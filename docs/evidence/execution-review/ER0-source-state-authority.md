# ER0 source/state authority evidence

Date: 2026-10-06  
Milestone: Expert-wallet execution correctness and prospective research  
Issue: #31  
Status: **ENGINEERING_PASS / LIVE_SOURCE_CAPSULE_BLOCKED**

## Scope

ER0 establishes one authoritative account-state path before richer scoring.

- Rust durably accepts a watched public trade before state publication.
- One economic public trade commits all watched-wallet projections atomically.
- Receipt identity includes source time, instrument scope, and trade id.
- Pending receipts are resumable after a crash; committed redeliveries are ignored.
- Reconciliation and live-fill publication share one async state gate.
- Rust reconciliation emits durable `ACCOUNT_SNAPSHOT` events containing current positions and account equity.
- Python no longer polls `clearinghouseState` or replaces account state independently.
- Python preserves full instrument ids and keeps seeded/partial-history open ages unknown.

Companion engine revision: `b443a5a` on
`feat/expert-wallet-execution-correctness-prospective-research`.

## Reproduced risks and regression coverage

| Risk | Durable regression |
|---|---|
| Same `(coin, tid)` in different source blocks/times collides | `test_q02_same_coin_tid_in_different_source_times_are_distinct` |
| Crash after receipt append leaves an unresumable duplicate | `test_q05_pending_duplicate_resumes_and_multi_counterparty_commit_is_atomic` |
| Two watched counterparties can be half committed | same multi-counterparty atomic batch test |
| Reconcile can overwrite a live fill | shared state gate plus existing reconciliation-race regression |
| Snapshot/equity state exists only in memory | `test_q04_seed_snapshot_is_durable_and_publishes_equity_from_rust` |
| Python snapshot can compete with Rust tail | direct snapshot replacement is disabled and tested |
| Instrument namespace is stripped | `test_account_snapshot_is_rust_owned_and_preserves_instrument_scope` |
| Seed observation time is presented as true entry time | Python stores partial-history `opened_at = None` and tests it |

## Verification

Tracker:

- `make check-rs`: PASS.
- Rust unit tests: 98 passed.
- `tests/adversarial_observation.rs`: 11 passed.
- `tests/crash_recovery.rs`: 9 passed.
- Clippy runs with `-D warnings`; `cargo fmt --all --check` passes.

Companion engine at `b443a5a`:

- `uv run --frozen --no-sync ruff check src tests`: PASS.
- `uv run --frozen --no-sync ty check src`: PASS.
- `PYTHONDONTWRITEBYTECODE=1 uv run --frozen --no-sync pytest -q -p no:cacheprovider`: 81 passed.

## Live-source qualification boundary

The existing local observation ledger was inspected read-only at the ER0 evidence cut. It exists, but
`ledger_meta.current_seq = 0` and contains no receipts/outbox events. No production service was
started or restarted because the issue does not authorize deployment.

Therefore the real-source receipt-to-projection capsule is **BLOCKED by absent recorded source
traffic**, not replaced with synthetic evidence. ER0 code correctness is qualified by the durable
regression suite above; real-feed field mapping, latency, and operational coverage remain explicitly
unqualified until a separately authorized recorder produces observations.

This blocked live-source sub-gate must not be relabeled as predictive qualification or production
readiness.

## Authority

No financial execution, deployment, credential change, paid subscription, or predictive promotion
is introduced by ER0.
