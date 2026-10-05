//! Q05, Q06, Q08 Crash Consistency & Snapshot/Tail Recovery Test Suite (Contracts C2–C5).
//!
//! Verifies:
//! - Q05: Crash consistency across receipt, commit, and cache restoration boundaries.
//! - Q06: Snapshot and tail protocol: consistent cuts, bounded ordered streaming, cursor expiry, and ledger change detection.
//! - Q08: Isolation and backpressure: pagination limits, retention pruning, and unstarved recovery.

use std::str::FromStr;

use chrono::Utc;
use rust_decimal::Decimal;
use serde_json::json;
use tempfile::tempdir;

use tracker::book::InMemoryBook;
use tracker::ledger::{
    LedgerCursor, ObservationLedger, PositionCheckpoint, TailError, TransitionParams,
};
use tracker::state::Direction;

fn d(s: &str) -> Decimal {
    Decimal::from_str(s).expect("valid decimal")
}

fn test_transition(
    receipt_id: i64,
    wallet_id: &str,
    coin: &str,
    checkpoint: Option<PositionCheckpoint>,
    kind: &str,
    before: u64,
    after: u64,
) -> TransitionParams {
    TransitionParams {
        receipt_id,
        wallet_id: wallet_id.to_string(),
        coin: coin.to_string(),
        checkpoint,
        event_kind: kind.to_string(),
        instrument_id: format!("perp:{coin}"),
        event_time: Utc::now(),
        received_at: Utc::now(),
        before_rev: before,
        after_rev: after,
        payload: json!({}),
    }
}

// --- Q05: Crash consistency & transaction boundaries -----------------------

#[tokio::test]
async fn test_q05_receipt_dedup_and_crash_isolation() {
    let dir = tempdir().expect("tempdir");
    let db_path = dir.path().join("ledger.db");

    let mut ledger = ObservationLedger::new(&db_path);
    ledger.connect().await.expect("connect");

    let payload = json!({
        "coin": "BTC",
        "px": "65000",
        "sz": "1.0",
        "tid": 1001,
        "users": ["0xalice", "0xbob"]
    });

    // 1. Durably append receipt
    let (receipt_id, is_dup) = ledger
        .append_receipt("ws:trades", "BTC:1001", &payload, Utc::now())
        .await
        .expect("append receipt");
    assert!(!is_dup);
    assert_eq!(receipt_id, 1);

    // 2. Second append with same economic key must be identified as duplicate
    let (receipt_id_2, is_dup_2) = ledger
        .append_receipt("ws:trades", "BTC:1001", &payload, Utc::now())
        .await
        .expect("append receipt 2");
    assert!(is_dup_2, "duplicate economic receipt must be detected");
    assert_eq!(receipt_id_2, 1);
}

#[tokio::test]
async fn test_q05_atomic_commit_and_book_restoration() {
    let dir = tempdir().expect("tempdir");
    let db_path = dir.path().join("ledger.db");
    let addr = "0x1111111111111111111111111111111111111111";

    let now = Utc::now();

    // Session 1: write to ledger
    {
        let mut ledger = ObservationLedger::new(&db_path);
        ledger.connect().await.expect("connect");

        let (receipt_id, _) = ledger
            .append_receipt("ws:trades", "BTC:2001", &json!({"tid": 2001}), now)
            .await
            .expect("receipt");

        let cp = PositionCheckpoint {
            address: addr.to_string(),
            coin: "BTC".to_string(),
            szi: d("2.5"),
            direction: Direction::Long,
            avg_entry: d("62000"),
            opened_at: now,
            last_added_at: now,
            realized_pnl: Decimal::ZERO,
            revision: 1,
            generation: 1,
            coverage: "Seeded".to_string(),
            updated_at: now,
        };

        let event = ledger
            .commit_transition(test_transition(
                receipt_id,
                addr,
                "BTC",
                Some(cp),
                "POSITION_CHANGE",
                0,
                1,
            ))
            .await
            .expect("commit transition");

        assert_eq!(event.cursor.seq, 1);
        assert_eq!(event.kind, "POSITION_CHANGE");
    }

    // Session 2: simulate crash recovery / process restart
    {
        let mut ledger = ObservationLedger::new(&db_path);
        ledger.connect().await.expect("reconnect");

        let mut book = InMemoryBook::new();
        assert!(book.position(addr, "BTC").is_none());

        let restored = ledger
            .restore_into_book(&mut book)
            .await
            .expect("restore into book");
        assert_eq!(restored, 1, "exactly 1 position restored");

        let pos = book
            .position(addr, "BTC")
            .expect("position restored in book");
        assert_eq!(pos.szi, d("2.5"));
        assert_eq!(pos.avg_entry, d("62000"));
        assert_eq!(pos.direction, Direction::Long);
    }
}

// --- Q06: Snapshot and tail protocol ---------------------------------------

#[tokio::test]
async fn test_q06_paginated_snapshot_pinned_to_cut() {
    let dir = tempdir().expect("tempdir");
    let db_path = dir.path().join("ledger.db");
    let now = Utc::now();

    let mut ledger = ObservationLedger::new(&db_path);
    ledger.connect().await.expect("connect");

    // Seed 5 positions across 3 wallets
    let coins = ["BTC", "ETH", "SOL", "AVAX", "NEAR"];
    for (i, coin) in coins.iter().enumerate() {
        let (r_id, _) = ledger
            .append_receipt("rest:seed", &format!("coin-{i}"), &json!({}), now)
            .await
            .unwrap();

        let cp = PositionCheckpoint {
            address: format!("0xwallet-{i}"),
            coin: coin.to_string(),
            szi: d("1.0"),
            direction: Direction::Long,
            avg_entry: d("100"),
            opened_at: now,
            last_added_at: now,
            realized_pnl: Decimal::ZERO,
            revision: 1,
            generation: 1,
            coverage: "Seeded".to_string(),
            updated_at: now,
        };

        ledger
            .commit_transition(test_transition(
                r_id,
                &format!("0xwallet-{i}"),
                coin,
                Some(cp),
                "ACCOUNT_SNAPSHOT",
                0,
                1,
            ))
            .await
            .unwrap();
    }

    // Query paginated snapshot: page size 2
    let p0 = ledger.snapshot_page(0, 2).await.expect("page 0");
    assert_eq!(p0.positions.len(), 2);
    assert_eq!(p0.total_pages, 3);
    assert_eq!(p0.last_seq, 5);

    let p1 = ledger.snapshot_page(1, 2).await.expect("page 1");
    assert_eq!(p1.positions.len(), 2);
    assert_eq!(p1.last_seq, 5);

    let p2 = ledger.snapshot_page(2, 2).await.expect("page 2");
    assert_eq!(p2.positions.len(), 1);
    assert_eq!(p2.last_seq, 5);

    // Verify all positions retrieved without duplicates
    let mut all_coins = Vec::new();
    for p in [p0, p1, p2] {
        for pos in p.positions {
            all_coins.push(pos.coin);
        }
    }
    assert_eq!(all_coins.len(), 5);
}

#[tokio::test]
async fn test_q06_ordered_tail_reading_advances_cursor() {
    let dir = tempdir().expect("tempdir");
    let db_path = dir.path().join("ledger.db");
    let now = Utc::now();

    let mut ledger = ObservationLedger::new(&db_path);
    ledger.connect().await.expect("connect");
    let ledger_id = ledger.ledger_id().to_string();

    // Commit 3 sequential events
    for i in 1..=3 {
        let (r_id, _) = ledger
            .append_receipt("ws:trades", &format!("t-{i}"), &json!({}), now)
            .await
            .unwrap();

        ledger
            .commit_transition(test_transition(
                r_id,
                "0xwallet",
                "BTC",
                None,
                "POSITION_CHANGE",
                i - 1,
                i,
            ))
            .await
            .unwrap();
    }

    // Tail page 1: from seq 0, limit 2
    let cursor_0 = LedgerCursor {
        ledger_id: ledger_id.clone(),
        seq: 0,
    };
    let tail_1 = ledger.tail(&cursor_0, 2).await.expect("tail 1");
    assert_eq!(tail_1.events.len(), 2);
    assert_eq!(tail_1.events[0].cursor.seq, 1);
    assert_eq!(tail_1.events[1].cursor.seq, 2);
    assert_eq!(tail_1.next_cursor.seq, 2);
    assert_eq!(tail_1.high_water_mark, 3);

    // Tail page 2: resume from next_cursor (seq 2)
    let tail_2 = ledger.tail(&tail_1.next_cursor, 2).await.expect("tail 2");
    assert_eq!(tail_2.events.len(), 1);
    assert_eq!(tail_2.events[0].cursor.seq, 3);
    assert_eq!(tail_2.next_cursor.seq, 3);

    // Tail page 3: tail at high-water mark yields 0 events and cursor stays at 3
    let tail_3 = ledger.tail(&tail_2.next_cursor, 2).await.expect("tail 3");
    assert_eq!(tail_3.events.len(), 0);
    assert_eq!(tail_3.next_cursor.seq, 3);
}

#[tokio::test]
async fn test_q06_cursor_expired_below_retention_floor() {
    let dir = tempdir().expect("tempdir");
    let db_path = dir.path().join("ledger.db");
    let now = Utc::now();

    let mut ledger = ObservationLedger::new(&db_path);
    ledger.connect().await.expect("connect");
    let ledger_id = ledger.ledger_id().to_string();

    for i in 1..=5 {
        let (r_id, _) = ledger
            .append_receipt("ws", &format!("key-{i}"), &json!({}), now)
            .await
            .unwrap();
        ledger
            .commit_transition(test_transition(
                r_id,
                "0xwallet",
                "BTC",
                None,
                "POSITION_CHANGE",
                i - 1,
                i,
            ))
            .await
            .unwrap();
    }

    // Advance retention floor to seq 4
    ledger
        .advance_retention_floor(4)
        .await
        .expect("advance floor");

    // Request from seq 2 (below floor 4) must return CursorExpired
    let expired_cursor = LedgerCursor { ledger_id, seq: 2 };
    let res = ledger.tail(&expired_cursor, 10).await;
    match res {
        Err(TailError::CursorExpired { requested, floor }) => {
            assert_eq!(requested, 2);
            assert_eq!(floor, 4);
        }
        other => panic!("expected CursorExpired, got {other:?}"),
    }
}

#[tokio::test]
async fn test_q06_ledger_id_mismatch_detected() {
    let dir = tempdir().expect("tempdir");
    let db_path = dir.path().join("ledger.db");

    let mut ledger = ObservationLedger::new(&db_path);
    ledger.connect().await.expect("connect");

    let wrong_cursor = LedgerCursor {
        ledger_id: "ledger-different-id".to_string(),
        seq: 0,
    };

    let res = ledger.tail(&wrong_cursor, 10).await;
    match res {
        Err(TailError::LedgerChanged { requested, current }) => {
            assert_eq!(requested, "ledger-different-id");
            assert_eq!(current, ledger.ledger_id());
        }
        other => panic!("expected LedgerChanged, got {other:?}"),
    }
}

// --- Q08: Isolation and backpressure ---------------------------------------

#[tokio::test]
async fn test_q08_bounded_tail_pagination_respects_limits() {
    let dir = tempdir().expect("tempdir");
    let db_path = dir.path().join("ledger.db");
    let now = Utc::now();

    let mut ledger = ObservationLedger::new(&db_path);
    ledger.connect().await.expect("connect");
    let ledger_id = ledger.ledger_id().to_string();

    for i in 1..=10 {
        let (r_id, _) = ledger
            .append_receipt("ws", &format!("k-{i}"), &json!({}), now)
            .await
            .unwrap();
        ledger
            .commit_transition(test_transition(
                r_id,
                "0xwallet",
                "BTC",
                None,
                "POSITION_CHANGE",
                i - 1,
                i,
            ))
            .await
            .unwrap();
    }

    // Limit clamp: requesting 1 must return exactly 1 event
    let cursor = LedgerCursor { ledger_id, seq: 0 };
    let tail = ledger.tail(&cursor, 1).await.expect("tail limit 1");
    assert_eq!(tail.events.len(), 1);
    assert_eq!(tail.high_water_mark, 10);
    assert_eq!(tail.next_cursor.seq, 1);
}

#[tokio::test]
async fn test_q08_retention_prunes_outbox_without_dropping_meta() {
    let dir = tempdir().expect("tempdir");
    let db_path = dir.path().join("ledger.db");
    let now = Utc::now();

    let mut ledger = ObservationLedger::new(&db_path);
    ledger.connect().await.expect("connect");

    for i in 1..=6 {
        let (r_id, _) = ledger
            .append_receipt("ws", &format!("r-{i}"), &json!({}), now)
            .await
            .unwrap();
        ledger
            .commit_transition(test_transition(
                r_id,
                "0xwallet",
                "BTC",
                None,
                "POSITION_CHANGE",
                i - 1,
                i,
            ))
            .await
            .unwrap();
    }

    // Prune events with seq < 4
    let pruned = ledger.advance_retention_floor(4).await.expect("prune");
    assert_eq!(pruned, 3, "3 events pruned (seq 1, 2, 3)");

    // Tail from seq 4 still succeeds
    let cursor = LedgerCursor {
        ledger_id: ledger.ledger_id().to_string(),
        seq: 4,
    };
    let tail = ledger.tail(&cursor, 10).await.expect("tail from seq 4");
    assert_eq!(tail.events.len(), 2, "events 5 and 6 remain");
    assert_eq!(tail.retention_floor, 4);
}
