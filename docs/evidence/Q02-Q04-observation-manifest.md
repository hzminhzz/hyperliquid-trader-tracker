# Q02-Q04: Observation Module Evidence Manifest

**Capsule ID:** Q02-Q04  
**Milestone:** Agent-operable expert ensemble V1  
**Issue:** #2 — S1: Deepen the existing observation module  
**Status:** PASS  
**Inspection Date:** 2026-10-06  
**Implementation Commit:** HEAD of `feat/agent-operable-expert-ensemble-v1`  

---

## 1. Summary of Changes & Scenarios Qualified

This manifest certifies completion of slice **S1**, establishing robust, explicit, and adversarial-tested observation semantics for the KonScanner Rust ingestion and reconciliation engine.

### Qualified Acceptance Scenarios

| Scenario ID | Name | Pass Condition | Verification Test | Status |
|---|---|---|---|---|
| **Q02** | Economic identity and lifecycle | Scoped composite identity `(coin, tid)`; repeated trade receipt applied once; cross-instrument shared IDs do not collide; two watched counterparties both update; open/add/reduce/close/flip lifecycle yields exact quantities and distinct provenance. | `tests/adversarial_observation.rs::test_q02_scoped_trade_key_no_cross_instrument_collision`, `test_q02_two_watched_counterparties_both_update`, `test_q02_lifecycle_transitions_exact_quantities` | PASS |
| **Q03** | Snapshot validity and races | Invalid or missing `assetPositions` payload strictly rejected; wallet quarantined rather than wiping positions; live fills during snapshot window skip stale snapshot without data loss (`ReconcileOutcome::SkippedRace`); generation increments protect retired wallets. | `tests/adversarial_observation.rs::test_q03_malformed_empty_snapshot_rejected`, `test_q03_reconciliation_race_skips_without_erasing_live_fill`, `test_q03_generation_retired_wallet_isolation` | PASS |
| **Q04** | Startup and incomplete history | Seeded position followed by same-direction fill emits `EventKind::Add`, never false `EventKind::Open`; desired wallet universe operates independently of Telegram chat subscribers; scoped equity and mark prices observed with provenance. | `tests/adversarial_observation.rs::test_q04_seeded_position_next_fill_is_add_not_open`, `test_q04_desired_universe_independent_of_subscribers`, `test_scoped_equity_and_mark_observations` | PASS |

---

## 2. Inspected Defect Repairs (G01–G09)

1. **G01 (Scoped Trade Deduplication):** Upgraded `SeenTids` in `src/tracker/book.rs` from scalar `i64` to `TradeKey { coin, tid }`. Trades on different instruments sharing a `tid` are no longer dropped. Duplicate receipts on reconnect are filtered idempotently.
2. **G02 (Decoupled Ingestion & Async Dispatch):** In `src/tracker/listener.rs`, WebSocket trade receipt is decoupled from authoritative close-PnL lookups and Telegram API dispatch via an asynchronous `tokio::sync::mpsc` channel (`10,000` buffer). Trade ingestion never stalls on Telegram network latency.
3. **G03 (Separate Desired Wallet Universe):** Extended `Registry` in `src/tracker/registry.rs` with `desired` address membership. Wallets can be monitored directly by the observation process regardless of whether any Telegram subscriber has subscribed.
4. **G04 (Explicit Reconciliation Outcomes):** Implemented `ReconcileOutcome` (`Applied`, `SkippedRace`, `Failed`) and `ReconcileSummary`. Discarded silent boolean return values; batch reconciliations log and track race-skipped versus failed seeds explicitly.
5. **G05 (Monotonic Revisions and Generation Protection):** Reseeding now increments monotonic `fill_epoch` / `state_revision` rather than resetting it via `drop_wallet`. `drop_wallet` increments `generation`, preventing retired-generation snapshots from corrupting re-admitted wallets.
6. **G06 (Strict Snapshot Validation):** `Enricher::account_positions` and `snapshot_positions` in `src/tracker/enrich.rs` require `assetPositions` to be present and a valid JSON array. Missing or malformed keys fail the snapshot and transition the wallet to `CoverageState::Quarantined` rather than silently wiping positions as empty.
7. **G07 (Explicit Coverage & History States):** Introduced `CoverageState` (`Unseeded`, `Seeded`, `Reconciling`, `Degraded`, `Quarantined`, `KnownFlat`) in `models.rs` and `InMemoryBook` to maintain the invariant that missing, stale, and known-flat states remain distinct.
8. **G08 (Startup Observation Consistency):** Verified seeded position lifecycle transitions so the next fill is correctly identified as an add.
9. **G09 (Shared Weighted Request Scheduler):** Created `RequestScheduler` in `src/tracker/scheduler.rs` enforcing a 1,200 weight/minute limit with a 200 weight reserve exclusively for crash recovery and reconciliation.

---

## 3. Test Evidence

- **Rust Suite:** `make check-rs`
  - Unit tests: 98 passed
  - Integration tests (`tests/adversarial_observation.rs`): 9 passed
  - Clippy: 0 warnings (`--all-targets -- -D warnings`)
  - Formatter: clean (`cargo fmt --all --check`)
- **Python Suite:** `make test`
  - Pytest: 79 passed
  - Ruff: clean
  - Ty: clean
- **Total Tests Passed:** 186 passed, 0 failed.
