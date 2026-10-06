# Q05-Q06-Q08: Observation Ledger & Persistence Evidence Manifest

**Capsule ID:** Q05-Q06-Q08  
**Milestone:** Agent-operable expert ensemble V1  
**Issue:** #3 — S2: Commit facts before publishing them  
**Status:** PASS  
**Inspection Date:** 2026-10-06  
**Implementation Commit:** HEAD of `feat/agent-operable-expert-ensemble-v1`  

---

## 1. Summary of Changes & Scenarios Qualified

This manifest certifies completion of slice **S2**, establishing transactional SQLite persistence, atomic receipt logging, economic deduplication indexing, outbox event generation, and the snapshot/tail reader protocol per Contracts C2–C5 in `docs/CONTRACTS.md`.

### Qualified Acceptance Scenarios

| Scenario ID | Name | Pass Condition | Verification Test | Status |
|---|---|---|---|---|
| **Q05** | Crash consistency | Kill at receipt/transaction/publication boundary; atomic transaction prevents orphan receipts or partial position mutations; process restart restores identical in-memory book state without data loss; duplicate receipts are deduplicated idempotently. | `tests/crash_recovery.rs::test_q05_receipt_dedup_and_crash_isolation`, `test_q05_atomic_commit_and_book_restoration` | PASS |
| **Q06** | Snapshot and tail recovery | Consistent cuts return pinned snapshot pages without drift or duplicates; tail stream orders strictly by sequence number; cursor expiry below retention floor returns typed `CursorExpired`; ledger ID change returns `LedgerChanged`. | `tests/crash_recovery.rs::test_q06_paginated_snapshot_pinned_to_cut`, `test_q06_ordered_tail_reading_advances_cursor`, `test_q06_cursor_expired_below_retention_floor`, `test_q06_ledger_id_mismatch_detected` | PASS |
| **Q08** | Isolation and backpressure | Tail pagination strictly respects limit bounds; high-water mark reports true unread lag; advancing retention floor prunes historical outbox events without corrupting metadata or active checkpoints. | `tests/crash_recovery.rs::test_q08_bounded_tail_pagination_respects_limits`, `test_q08_retention_prunes_outbox_without_dropping_meta` | PASS |

---

## 2. Persistence Architecture & Contracts (C2–C5)

1. **Durable Receipt Ledger (C2 Step 1):** `ObservationLedger::append_receipt` records raw source payloads into `receipts(receipt_id, source, source_key, payload, received_at, status)` with unique indexing on `(source, source_key)`. Duplicate receipts return `is_duplicate = true` and avoid double-processing.
2. **Atomic Transition Transaction (C2 Step 2):** `ObservationLedger::commit_transition` executes within a single SQLite transaction:
   - Updates receipt status to `COMMITTED`.
   - Upserts open positions into `position_checkpoints(address, coin, ...)` or deletes closed positions.
   - Advances `current_seq` in `ledger_meta`.
   - Appends canonical versioned events into `outbox_events`.
   - Commits transaction before cache publication or tail consumption.
3. **Cache Reconstruction (C2 Step 3):** `ObservationLedger::restore_into_book` re-seeds `InMemoryBook` from committed checkpoints on process restart without requiring network reseeding.
4. **Canonical Event Envelope (C3):** `CanonicalEvent` carries `schema_version`, `event_id`, `cursor: { ledger_id, seq }`, `kind`, `environment`, `instrument_id`, `wallet_id`, `event_time`, `received_at`, `known_at`, `cause_ids`, `before_revision`, `after_revision`, `payload`, and `quality`.
5. **Snapshot & Tail Protocol (C4):**
   - `ObservationLedger::snapshot_page` paginates position checkpoints pinned to `last_seq`.
   - `ObservationLedger::tail` reads events strictly after `cursor.seq`, advancing `next_cursor` monotonically and validating `retention_floor` and `ledger_id`.
6. **Retention & Backpressure (C4/C5):** `advance_retention_floor` prunes events strictly older than the specified sequence number, maintaining bounded disk usage while preserving the ledger checkpoint.

---

## 3. Test Evidence

- **Rust Suite:** `make check-rs`
  - Unit tests: 98 passed
  - Adversarial observation tests (`tests/adversarial_observation.rs`): 9 passed
  - Crash recovery & tail tests (`tests/crash_recovery.rs`): 8 passed
  - Clippy: 0 warnings (`--all-targets -- -D warnings`)
  - Formatter: clean (`cargo fmt --all --check`)
- **Python Suite:** `make test`
  - Pytest: 79 passed
  - Ruff: clean
  - Ty: clean
- **Total Tests Passed:** 194 passed, 0 failed.
