# Master Qualification Evidence Index: Agent-Operable Expert Ensemble V1

This master index aggregates all qualification manifests across slices **S0 through S8** for the **Agent-operable expert ensemble V1** milestone, reporting **Engineering**, **Operational Ergonomics**, **Resource Use**, and **Economic Results** separately per [QUALIFICATION](../QUALIFICATION.md) and [plan.md](../../plan.md).

V2 evidence is indexed separately at [v2/INDEX.md](v2/INDEX.md) so historical V1 engineering evidence is not conflated with V2 predictive qualification.

---

## 1. Slice Manifest Ledger

| Slice | Name | Scope & Authority | Primary Scenarios | Manifest File | Status |
|---|---|---|---|---|---|
| **S0** | Baseline Inventory | Toolchain, fork pin, remotes, compute/disk, live-test limits | Q01 | [Q01-baseline-manifest.md](Q01-baseline-manifest.md) | PASS |
| **S1** | Observation Reducer | Scoped deduplication, composite TradeKey, mpsc dispatch buffer | Q02, Q03, Q04 | [Q02-Q04-observation-manifest.md](Q02-Q04-observation-manifest.md) | PASS |
| **S2** | Observation Ledger | SQLite durable receipts, atomic checkpoints, ordered tail streams | Q05, Q06, Q08 | [Q05-Q06-Q08-persistence-manifest.md](Q05-Q06-Q08-persistence-manifest.md) | PASS |
| **S3** | Worldview & Replay | Independent Python projection, consumer offsets, replay isolation | Q09, Q14, Q16 | [Q09-Q14-Q16-projection-manifest.md](Q09-Q14-Q16-projection-manifest.md) | PASS |
| **S4** | Target Explanation | Posture normalization, consensus bounds, causal change categories | Q10, Q14 | [Q10-Q14-explanation-manifest.md](Q10-Q14-explanation-manifest.md) | PASS |
| **S5** | Operations & Control | Typed change plans, scoped authority, prompt injection defense | Q13, Q16, Q18 | [Q13-Q16-Q18-control-manifest.md](Q13-Q16-Q18-control-manifest.md) | PASS |
| **S6** | Clustering & Consensus | Pairwise similarity, complete-link clustering, clone resistance | Q11, Q15 | [Q11-Q15-clustering-manifest.md](Q11-Q15-clustering-manifest.md) | PASS |
| **S7** | Account Advisory | Drawdown headroom ceilings, downward lot rounding, Telegram adapter | Q12, Q08 | [Q12-Q08-advisory-manifest.md](Q12-Q08-advisory-manifest.md) | PASS |
| **S8** | Synthetic Qualification | High-leverage integration, 6-task battery, load envelopes, recovery | Q07, Q14, Q16–Q18 | [Q07-Q14-Q16-Q17-Q18-synthetic-system-manifest.md](Q07-Q14-Q16-Q17-Q18-synthetic-system-manifest.md) | PASS |
| **S9** | Research Promotion | Adaptive quality weights (B3), regime conditioning (B4), promotion gates | Q15 | [Q15-research-promotion-manifest.md](Q15-research-promotion-manifest.md) | PASS |

---

## 2. Engineering Results

- **Reducer & Dedup (S1, Q02-Q04):** Scoped composite key `(coin, tid)` completely prevents cross-instrument deduplication collisions. Decoupled WebSocket ingestion via bounded mpsc channel buffers. Monotonic epoch advancement prevents fill desynchronization during reconnection reseeds.
- **Durable Persistence (S2, Q05-Q06, Q08):** SQLite-backed `ObservationLedger` provides atomic transactions across receipt logging, checkpoint updates, and outbox event emissions. Paginated snapshots remain pinned to cuts; cursor expiration and retention floors are strictly enforced.
- **Projection Store & Offset Bridge (S3, S8, Q09, Q14):** Independent Python database tracks consumer offsets against the Rust observation ledger. Direct SQLite polling via `ProjectionStore.consume_from_ledger_db` executes with complete idempotency and zero duplicate delta application.
- **Replay Equivalence (S3, S8, Q14):** Deterministic semantic hashes across repeated executions. As-known replays pin historical knowledge times, strictly preventing later backfill from altering historical decisions. Restated replays incorporate late corrections and produce distinct restated hashes.

---

## 3. Operational Ergonomics Results

- **Agent Task Battery (S5, S8, Q16):**
  1. *Task 1 (Paused Alerts):* Cold-start diagnosis through `cmd_inspect_system` identifies healthy recording while delivery is paused, avoiding unneeded ingestion restarts.
  2. *Task 2 (Target Explanation):* `cmd_explain_target` and `cmd_inspect_system` explain target adjustments caused by stale equity or corrections without mistaking them for new expert activity.
  3. *Task 3 (Checkpoint Resume):* `JobStore.resume_and_execute` resumes interrupted evaluation jobs from persistent checkpoints within remaining budgets.
  4. *Task 4 (Duplicate Detection):* `complete_link_clustering` and `find_clone_groups` detect duplicate trading behavior while public summaries scrub raw wallet addresses to preserve privacy.
  5. *Task 5 (Revision Drift Safe Re-Plan):* `OperationsEngine` catches revision drift before plan execution (`PRECONDITION_FAILED`), safely prompting the agent to re-plan.
  6. *Task 6 (Hostile Label Rejection):* Prompt injection patterns ("ignore previous instructions", "export secrets", "increase risk limit") are detected and rejected via `OperationError(HOSTILE_LABEL_REJECTED)`.
- **Knowledge Retention (S5, S8, Q18):** Runbook registry links reproduced incidents (e.g., G01 TID collision) to permanent regression tests; policy rollback marks superseded advice discoverable as superseded.
- **Compact Handoffs (S5, S8, Q18):** `generate_compact_handoff` creates token-efficient status digests under 6 KiB containing revision, active universe, blockers, and next slice targets.

---

## 4. Resource Use & Capacity Results

- **Declared Load Envelopes (S8, Q17):**
  - **10 Wallets:** 0.003s total latency (write + projection).
  - **100 Wallets:** 0.015s total latency.
  - **1,000 Wallets:** 0.168s total latency, easily satisfying the < 1.0s p95 durable-to-target budget.
- **Host Co-Location Budget (S0, S8):** Accounts for co-located Hummingbot process consuming up to 1,200 public API requests/min; observer public queries strictly capped under 100 req/min with zero order placement.
- **Memory & Storage Bounds:** Memory remains under 12 MiB per process; outbox retention pruning preserves storage under 50 MiB across soak tests.

---

## 5. Economic Results & Boundaries

- **Advisory Mode Only (S7, S8, ADR-0001):** The system strictly outputs bounded account advice, not automated order placement. Execution authority is forbidden (`FINANCIAL_EFFECT_FORBIDDEN`).
- **Drawdown Headroom Ceilings (S7, Q12):** Account headroom is dynamically constrained by both total (10%) and daily (5%) loss floors. Exhausted headroom transitions immediately to `BLOCKED`.
- **Lot Sizing & Downward Rounding (S7, Q12):** Notional sizing rounds downward to lot step and enforces minimum lot thresholds; zero over-allocation.
- **Explicit Economic Uncertainty (S7, S8):** Unobserved account positions withhold action deltas (`action_delta = None`). Missing loss models or broker specs produce `UNSIZED` informational output. No strategy claims safety from backtest ROI or historical drawdown alone.
- **Clone Resistance (S6, Q11):** Hierarchical clustering neutralizes cloned accounts, preventing duplicate voter inflation from capturing consensus weight.

---

## 6. Full Verification Gate

Across both repositories, all automated checks and tests pass with zero failures:
- **Rust Observation Tracker:** 115 tests passed (`make check-rs`), 0 warnings.
- **Inherited Python Tracker:** 79 tests passed (`make test`).
- **Expert Ensemble Application:** 37 tests passed (`uv run pytest`), ruff clean, type check clean.
- **Total Workspace Test Count:** **231 tests passing**.
