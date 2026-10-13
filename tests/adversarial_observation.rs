//! Q02-Q04 Adversarial Observation Qualification Suite
//!
//! Verifies observation module invariants:
//! - Q02: Economic identity, scoped deduplication, multi-counterparty updates, and lifecycle transitions.
//! - Q03: Strict snapshot validation, race condition visibility, and generation protection.
//! - Q04: Startup seeding correctness, incomplete history isolation, and desired universe independence.

use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use chrono::{TimeZone, Utc};
use rust_decimal::Decimal;
use serde_json::{Value, json};
use tempfile::tempdir;

use tracker::book::{InMemoryBook, ReconcileOutcome, SeenTids};
use tracker::config::Settings;
use tracker::enrich::Enricher;
use tracker::hl_client::InfoClient;
use tracker::ledger::{LedgerCursor, ObservationLedger};
use tracker::models::{CoverageState, parse_account_equity};
use tracker::registry::Registry;
use tracker::resolve::{resolve_deltas, seed_state_from_row};
use tracker::state::{Direction, EventKind};

fn d(s: &str) -> Decimal {
    Decimal::from_str(s).expect("valid decimal")
}

fn ts() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 10, 6, 12, 0, 0).unwrap()
}

// --- Q02: Economic identity and lifecycle -----------------------------------

#[test]
fn test_q02_scoped_trade_key_no_cross_instrument_collision() {
    let mut seen = SeenTids::new(100);
    let shared_tid = 888_888;

    // Trade 1 on BTC with shared_tid
    assert!(
        !seen.check_and_add_scoped("BTC", shared_tid),
        "first trade on BTC should be accepted"
    );

    // Trade 2 on ETH with identical tid must NOT collide
    assert!(
        !seen.check_and_add_scoped("ETH", shared_tid),
        "trade on ETH with shared tid must not be dropped by BTC trade"
    );

    // Trade 3 on SOL with identical tid must NOT collide
    assert!(
        !seen.check_and_add_scoped("SOL", shared_tid),
        "trade on SOL with shared tid must not be dropped"
    );

    // Replay of Trade 1 on BTC must be recognized as duplicate
    assert!(
        seen.check_and_add_scoped("BTC", shared_tid),
        "duplicate trade on BTC must be rejected"
    );
}

#[test]
fn test_q02_same_coin_tid_in_different_source_times_are_distinct() {
    let mut seen = SeenTids::new(100);
    let tid = 42_4242;

    assert!(!seen.check_and_add_scoped_at("BTC", tid, 1_791_288_000_000));
    assert!(seen.check_and_add_scoped_at("BTC", tid, 1_791_288_000_000));
    assert!(
        !seen.check_and_add_scoped_at("BTC", tid, 1_791_288_001_000),
        "the documented economic identity includes source/block time"
    );
}

#[test]
fn test_q02_two_watched_counterparties_both_update() {
    let mut book = InMemoryBook::new();
    let mut watchlist = HashSet::new();
    let buyer = "0xaaaa000000000000000000000000000000000001";
    let seller = "0xbbbb000000000000000000000000000000000002";
    watchlist.insert(buyer.to_string());
    watchlist.insert(seller.to_string());

    let trade = json!({
        "coin": "BTC",
        "px": "60000.0",
        "sz": "1.5",
        "time": 1791288000000_i64,
        "tid": 10001,
        "users": [buyer, seller],
        "hash": "0x123abc"
    });

    let fills = resolve_deltas(&trade, &watchlist);
    assert_eq!(fills.len(), 2, "both counterparties must resolve fills");

    // Apply buyer fill (+1.5 BTC)
    let buyer_fill = fills.iter().find(|f| f.address == buyer).unwrap();
    assert_eq!(buyer_fill.delta, d("1.5"));
    let res_a = book.ingest(
        &buyer_fill.address,
        &buyer_fill.coin,
        buyer_fill.delta,
        buyer_fill.px,
        buyer_fill.ts,
    );
    assert_eq!(res_a.events.len(), 1);
    assert_eq!(res_a.events[0].kind, EventKind::Open);

    // Apply seller fill (-1.5 BTC)
    let seller_fill = fills.iter().find(|f| f.address == seller).unwrap();
    assert_eq!(seller_fill.delta, d("-1.5"));
    let res_b = book.ingest(
        &seller_fill.address,
        &seller_fill.coin,
        seller_fill.delta,
        seller_fill.px,
        seller_fill.ts,
    );
    assert_eq!(res_b.events.len(), 1);
    assert_eq!(res_b.events[0].kind, EventKind::Open);

    // Verify independent position states
    let pos_a = book.position(buyer, "BTC").expect("buyer position exists");
    assert_eq!(pos_a.szi, d("1.5"));
    assert_eq!(pos_a.direction, Direction::Long);

    let pos_b = book
        .position(seller, "BTC")
        .expect("seller position exists");
    assert_eq!(pos_b.szi, d("-1.5"));
    assert_eq!(pos_b.direction, Direction::Short);
}

#[test]
fn test_q02_lifecycle_transitions_exact_quantities() {
    let mut book = InMemoryBook::new();
    let addr = "0x1111111111111111111111111111111111111111";

    // 1. Open long 2.0 @ 100
    let r1 = book.ingest(addr, "ETH", d("2.0"), d("100"), ts());
    assert_eq!(r1.events[0].kind, EventKind::Open);
    let p = book.position(addr, "ETH").unwrap();
    assert_eq!(p.szi, d("2.0"));
    assert_eq!(p.avg_entry, d("100"));

    // 2. Add long 1.0 @ 130 -> 3.0 @ 110 avg
    let r2 = book.ingest(addr, "ETH", d("1.0"), d("130"), ts());
    assert_eq!(r2.events[0].kind, EventKind::Add);
    let p = book.position(addr, "ETH").unwrap();
    assert_eq!(p.szi, d("3.0"));
    assert_eq!(p.avg_entry, d("110"));

    // 3. Reduce 1.0 @ 140 -> 2.0 @ 110 avg, realized PnL = +30
    let r3 = book.ingest(addr, "ETH", d("-1.0"), d("140"), ts());
    assert_eq!(r3.events[0].kind, EventKind::Reduce);
    let p = book.position(addr, "ETH").unwrap();
    assert_eq!(p.szi, d("2.0"));
    assert_eq!(p.avg_entry, d("110"));
    assert_eq!(p.realized_pnl, d("30"));

    // 4. Close 2.0 @ 120 -> position dropped, closed trade emitted
    let r4 = book.ingest(addr, "ETH", d("-2.0"), d("120"), ts());
    assert_eq!(r4.events[0].kind, EventKind::Close);
    assert!(book.position(addr, "ETH").is_none());
    assert!(r4.closed_trade.is_some());

    // 5. Flip: from flat, sell 2.0 @ 150 opens short
    let r5 = book.ingest(addr, "ETH", d("-2.0"), d("150"), ts());
    assert_eq!(r5.events[0].kind, EventKind::Open);
    let p = book.position(addr, "ETH").unwrap();
    assert_eq!(p.szi, d("-2.0"));
    assert_eq!(p.direction, Direction::Short);

    // 6. Flip across zero: buy 3.0 @ 140 -> closes short 2.0, opens long 1.0
    let r6 = book.ingest(addr, "ETH", d("3.0"), d("140"), ts());
    let kinds: Vec<_> = r6.events.iter().map(|e| e.kind).collect();
    assert_eq!(kinds, vec![EventKind::Close, EventKind::Open]);
    let p = book.position(addr, "ETH").unwrap();
    assert_eq!(p.szi, d("1.0"));
    assert_eq!(p.direction, Direction::Long);
}

// --- Q03: Snapshot validity and races ---------------------------------------

struct MockClient {
    response: Value,
}

#[async_trait::async_trait]
impl InfoClient for MockClient {
    async fn info(&self, _body: Value) -> Result<Value, tracker::exceptions::TrackerError> {
        Ok(self.response.clone())
    }
}

#[tokio::test]
async fn test_q03_malformed_empty_snapshot_rejected() {
    let book = Arc::new(Mutex::new(InMemoryBook::new()));
    let addr = "0x2222222222222222222222222222222222222222";

    // Set existing position in book
    book.lock()
        .expect("book mutex poisoned")
        .ingest(addr, "BTC", d("1.0"), d("50000"), ts());
    assert!(
        book.lock()
            .expect("book mutex poisoned")
            .position(addr, "BTC")
            .is_some()
    );

    // Client returns a JSON object missing "assetPositions" entirely
    let client: Arc<dyn InfoClient> = Arc::new(MockClient {
        response: json!({
            "marginSummary": {
                "accountValue": "10000.0"
            }
        }),
    });

    let enricher = Enricher::new(Settings::default(), Arc::clone(&book), client);
    let outcome = enricher.seed_wallet(addr).await;

    // Must FAIL, not wipe position!
    assert_eq!(
        outcome,
        ReconcileOutcome::Failed,
        "malformed snapshot must return ReconcileOutcome::Failed"
    );
    assert_eq!(
        book.lock()
            .expect("book mutex poisoned")
            .coverage_state(addr),
        CoverageState::Quarantined,
        "failed wallet must be quarantined"
    );
    assert!(
        book.lock()
            .expect("book mutex poisoned")
            .position(addr, "BTC")
            .is_some(),
        "existing position must NOT be erased by malformed snapshot"
    );
}

#[tokio::test]
async fn test_q03_reconciliation_race_skips_without_erasing_live_fill() {
    let book = Arc::new(Mutex::new(InMemoryBook::new()));
    let addr = "0x3333333333333333333333333333333333333333";

    // Capture initial epoch
    let initial_epoch = book.lock().expect("book mutex poisoned").fill_epoch(addr);

    // Live fill lands while snapshot is pending
    book.lock()
        .expect("book mutex poisoned")
        .ingest(addr, "SOL", d("10.0"), d("150"), ts());
    let current_epoch = book.lock().expect("book mutex poisoned").fill_epoch(addr);
    assert_ne!(initial_epoch, current_epoch);

    // Reseed attempt with old expected epoch must return SkippedRace
    let stale_snapshot = vec![seed_state_from_row(
        addr,
        "SOL",
        d("5.0"),
        Some(d("140")),
        ts(),
    )];
    let outcome = book.lock().expect("book mutex poisoned").reseed_wallet(
        addr,
        stale_snapshot,
        HashMap::new(),
        Some(initial_epoch),
    );

    assert_eq!(outcome, ReconcileOutcome::SkippedRace);
    let pos = book
        .lock()
        .expect("book mutex poisoned")
        .position(addr, "SOL")
        .unwrap()
        .clone();
    assert_eq!(pos.szi, d("10.0"), "live fill was preserved");
}

#[test]
fn test_q03_generation_retired_wallet_isolation() {
    let mut book = InMemoryBook::new();
    let addr = "0x4444444444444444444444444444444444444444";

    assert_eq!(book.generation(addr), 0);
    book.ingest(addr, "BTC", d("1.0"), d("60000"), ts());

    // Drop wallet (e.g. subscriber untracks)
    book.drop_wallet(addr);
    assert_eq!(
        book.generation(addr),
        1,
        "drop_wallet must advance generation"
    );
    assert!(book.position(addr, "BTC").is_none());

    // Monotonic state revision must NOT reset to 0
    assert!(
        book.state_revision(addr) > 0,
        "monotonic revision is preserved"
    );
}

// --- Q04: Startup, incomplete history, and desired universe -----------------

#[test]
fn test_q04_seeded_position_next_fill_is_add_not_open() {
    let mut book = InMemoryBook::new();
    let addr = "0x5555555555555555555555555555555555555555";

    let seeded = vec![seed_state_from_row(
        addr,
        "BTC",
        d("3.0"),
        Some(d("55000")),
        ts(),
    )];
    let outcome = book.reseed_wallet(addr, seeded, HashMap::new(), None);
    assert_eq!(outcome, ReconcileOutcome::Applied);
    assert_eq!(book.coverage_state(addr), CoverageState::Seeded);

    // Next buy fill is an ADD, never false OPEN
    let res = book.ingest(addr, "BTC", d("1.0"), d("60000"), ts());
    assert_eq!(res.events.len(), 1);
    assert_eq!(
        res.events[0].kind,
        EventKind::Add,
        "next fill on pre-existing position must be ADD"
    );
    let p = book.position(addr, "BTC").unwrap();
    assert_eq!(p.szi, d("4.0"));
}

#[tokio::test]
async fn test_q04_seed_snapshot_is_durable_and_publishes_equity_from_rust() {
    let book = Arc::new(Mutex::new(InMemoryBook::new()));
    let addr = "0x5656565656565656565656565656565656565656";
    let client: Arc<dyn InfoClient> = Arc::new(MockClient {
        response: json!({
            "marginSummary": {
                "accountValue": "25000",
                "totalRawUsd": "24000",
                "totalMarginUsed": "1000"
            },
            "assetPositions": [{
                "position": {
                    "coin": "BTC",
                    "szi": "2.0",
                    "entryPx": "60000",
                    "leverage": {"type": "cross", "value": 3}
                }
            }]
        }),
    });
    let dir = tempdir().expect("tempdir");
    let mut ledger = ObservationLedger::new(dir.path().join("observation.db"));
    ledger.connect().await.expect("ledger connect");
    let ledger = Arc::new(ledger);
    let enricher = Enricher::new(Settings::default(), Arc::clone(&book), client)
        .with_ledger(Arc::clone(&ledger));

    assert_eq!(enricher.seed_wallet(addr).await, ReconcileOutcome::Applied);
    {
        let stored = book.lock().expect("book mutex poisoned");
        assert_eq!(
            stored.position(addr, "BTC").expect("position").szi,
            d("2.0")
        );
        assert_eq!(
            stored.equity(addr).expect("equity").account_value,
            d("25000")
        );
    }

    let tail = ledger
        .tail(
            &LedgerCursor {
                ledger_id: ledger.ledger_id().to_string(),
                seq: 0,
            },
            10,
        )
        .await
        .expect("tail");
    assert_eq!(tail.events.len(), 1);
    assert_eq!(tail.events[0].kind, "ACCOUNT_SNAPSHOT");
    assert_eq!(tail.events[0].payload["equity"]["account_value"], "25000");
    assert_eq!(tail.events[0].payload["positions"][0]["coin"], "BTC");
    assert_eq!(
        tail.events[0].payload["positions"][0]["opened_at_known"],
        false
    );
    assert_eq!(
        ledger
            .snapshot_page(0, 10)
            .await
            .expect("snapshot")
            .positions
            .len(),
        1
    );
}

#[test]
fn test_q04_desired_universe_independent_of_subscribers() {
    let mut registry = Registry::new();
    let addr = "0x6666666666666666666666666666666666666666";

    // Add directly to desired universe
    registry.add_desired(addr);
    assert!(registry.is_desired(addr));
    assert!(registry.is_tracked(addr));
    assert!(registry.addresses().contains(addr));

    // A chat subscribes
    registry.subscribe(101, addr, "Alpha");
    assert_eq!(registry.subscribers(addr).len(), 1);

    // Chat unsubscribes: wallet must NOT be orphaned because it is still in desired universe
    let (existed, is_orphan) = registry.unsubscribe(101, addr);
    assert!(existed);
    assert!(
        !is_orphan,
        "wallet in desired universe is not orphaned when subscriber leaves"
    );
    assert!(
        registry.is_tracked(addr),
        "still tracked in desired universe"
    );
    assert!(registry.addresses().contains(addr));
}

#[test]
fn test_scoped_equity_and_mark_observations() {
    let mut book = InMemoryBook::new();
    let addr = "0x7777777777777777777777777777777777777777";

    // Test AccountEquity parse and store
    let raw = json!({
        "marginSummary": {
            "accountValue": "125000.75",
            "totalRawUsd": "100000.0",
            "totalMarginUsed": "25000.75"
        }
    });

    let equity = parse_account_equity(addr, &raw, ts())
        .expect("parse succeeds")
        .expect("equity exists");
    assert_eq!(equity.account_value, d("125000.75"));
    assert_eq!(equity.total_raw_usd, Some(d("100000.0")));
    assert_eq!(equity.total_margin_used, Some(d("25000.75")));

    book.set_equity(equity);
    let stored = book.equity(addr).expect("equity retrieved");
    assert_eq!(stored.account_value, d("125000.75"));

    // Test MarkObservation store and retrieve
    book.set_mark("BTC", d("65000.5"), ts());
    let mark = book.mark("BTC").expect("mark retrieved");
    assert_eq!(mark.px, d("65000.5"));
    assert_eq!(mark.coin, "BTC");
}
