# Q09-Q14-Q16: Projection, Replay, and Inspection Evidence Manifest

**Capsule ID:** Q09-Q14-Q16  
**Milestone:** Agent-operable expert ensemble V1  
**Issue:** #4 — S3: Give the agent eyes before advanced intelligence  
**Status:** PASS  
**Inspection Date:** 2026-10-06  
**Application Repository:** `/home/quant/dev/hyperliquid-expert-ensemble`  
**Application Commit:** `6e3461f`  
**Tracker Commit:** HEAD of `feat/agent-operable-expert-ensemble-v1`  

---

## 1. Summary of Deliverables & Repository Boundaries

In conformance with [SYSTEM](docs/SYSTEM.md) and [CONTRACTS](docs/CONTRACTS.md), the expert ensemble engine is housed in an independent application repository rather than disguising itself as the inherited tracker:

- **Location:** `/home/quant/dev/hyperliquid-expert-ensemble`
- **Database Ownership:** Operates its own independent SQLite database (`projection.db`), tracking consumer offsets against the Rust observation ledger.
- **Python Environment:** Managed via `uv` with independent dependencies (`pydantic`, `pytest`, `ruff`, `ty`).

### Qualified Acceptance Scenarios

| Scenario ID | Name | Pass Condition | Verification Test | Status |
|---|---|---|---|---|
| **Q09** | Coherent worldview | Worldview resolves to compatible input cuts and versions; unseeded or quarantined observation states surface as typed blockers (`STATE_QUARANTINED`); stale equity threshold (>15m) blocks consensus targets (`EQUITY_STALE`). | `tests/test_s3_qualification.py::test_q09_coherent_worldview_and_blockers` | PASS |
| **Q14** | Replay equivalence | Pinned input manifests produce identical semantic positions and deterministic decision hashes across repeated executions; as-known replay isolates knowledge time, proving that later corrections/backfill cannot alter historical as-known decisions. | `tests/test_s3_qualification.py::test_q14_replay_equivalence_and_isolation` | PASS |
| **Q16 (Task 1)** | Paused alert diagnosis | Operator inspects system during paused alert delivery; briefing explicitly confirms `ingestion.status = "RECORDING_HEALTHY"`, preventing false restarts of healthy ingestion. | `tests/test_s3_qualification.py::test_q16_task1_diagnose_paused_alerts_vs_healthy_ingestion` | PASS |
| **Q16 (Task 2)** | Stale equity explanation | Target change caused by stale equity observation is explained causally with blocker code `EQUITY_STALE` and specific remediation, rather than inferred as a new expert entry. | `tests/test_s3_qualification.py::test_q16_task2_explain_stale_equity_blocker` | PASS |
| **Q16 (Task 3)** | Bounded checkpoint resume | Interrupted evaluation halts at budget exhaustion with `CHECKPOINTED` status; adding budget resumes from the exact saved step without restarting full evaluation. | `tests/test_s3_qualification.py::test_q16_task3_resume_interrupted_evaluation_within_budget` | PASS |
| **Ergonomics** | Capabilities & Briefing Size | `inspect capabilities` strictly omits unimplemented/future commands; compact system briefing fits comfortably within the 6 KiB budget constraint. | `tests/test_s3_qualification.py::test_capabilities_omit_unimplemented_commands`, `cmd_inspect_system` size assertion | PASS |

---

## 2. Verification Evidence Across Repositories

- **Observation Tracker (`hyperliquid-trader-tracker`):**
  - `make check-rs`: 115 tests passed, 0 clippy warnings, clean formatting.
  - `make test`: 79 pytest tests passed.
  - Total: 194 tests passing.
- **Expert Ensemble (`hyperliquid-expert-ensemble`):**
  - `uv run ruff check .`: all checks passed.
  - `uv run ty check`: all checks passed.
  - `uv run pytest -q`: 6 passed in 0.07s.
- **Combined Test Count:** 200 passed across both repositories.
