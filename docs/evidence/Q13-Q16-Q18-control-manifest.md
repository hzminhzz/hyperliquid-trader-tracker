# Q13-Q16-Q18: Operations Control, Authority, and Knowledge Retention Evidence Manifest

**Capsule ID:** Q13-Q16-Q18  
**Milestone:** Agent-operable expert ensemble V1  
**Issue:** #6 — S5: Make control safe and cheap  
**Status:** PASS  
**Inspection Date:** 2026-10-06  
**Application Repository:** `/home/quant/dev/hyperliquid-expert-ensemble`  
**Application Commit:** `bdcc955`  
**Tracker Commit:** HEAD of `feat/agent-operable-expert-ensemble-v1`  

---

## 1. Summary of Deliverables Qualified

This manifest certifies completion of slice **S5**, establishing the operations facade, typed plans, authority escalation defenses, idempotency receipts, prompt injection sanitization, and knowledge retention runbooks per Contracts [OPERATIONS](docs/OPERATIONS.md) O4–O8 and [QUALIFICATION](docs/QUALIFICATION.md) Q13, Q16, Q18:

### Qualified Acceptance Scenarios

| Scenario ID | Name | Pass Condition | Verification Test | Status |
|---|---|---|---|---|
| **Q13** | Authorized control | Expired plans rejected (`PLAN_EXPIRED`); insufficient role rejected (`PERMISSION_DENIED`); altered parameters on reused idempotency key rejected (`IDEMPOTENCY_KEY_REUSED_WITH_DIFFERENT_PARAMS`); valid repeated keys return original receipt; financial trading strictly forbidden (`FINANCIAL_EFFECT_FORBIDDEN`). | `tests/test_s5_qualification.py::test_q13_authorized_control_plan_expiry_and_preconditions`, `test_q13_financial_execution_forbidden`, `test_q13_idempotency_key_behavior` | PASS |
| **Q16 (Task 5)** | Revision drift re-planning | Precondition failure on revision drift halts execution without partial mutation; operator safely re-plans against updated revision. | `tests/test_s5_qualification.py::test_q16_task5_revision_drift_detection_and_safe_replan` | PASS |
| **Q16 (Task 6)** | Prompt injection defense | Hostile wallet labels attempting credential exfiltration, admin escalation, or risk limit tampering are rejected (`HOSTILE_LABEL_REJECTED`) and neutralized as passive data. | `tests/test_s5_qualification.py::test_q16_task6_reject_hostile_prompt_injection_labels` | PASS |
| **Q18** | Knowledge retention & runbooks | Defect fixtures link to verified runbooks in `RunbookRegistry`; superseded runbooks are discoverable as superseded. | `tests/test_s5_qualification.py::test_q18_knowledge_retention_and_superseded_runbooks` | PASS |
| **Handoffs & Jobs** | Bounded job control & O6 briefs | Bounded job cancellation supported; compact handoff briefs generated for subsequent agent sessions. | `tests/test_s5_qualification.py::test_job_cancellation_and_compact_handoff` | PASS |

---

## 2. Invariants & Security Boundaries (O4, O8)

1. **V1 Financial Execution Barrier:**
   - Real trading, order placement, and fund transfers are strictly forbidden in V1. Any plan requesting `is_financial_trade = True` or role `EXECUTION_OPERATOR` is blocked unconditionally with `FINANCIAL_EFFECT_FORBIDDEN`.
2. **Authority Hierarchy:**
   - Strict level checking: `OBSERVER` (1) < `RESEARCH_OPERATOR` (2) < `RUNTIME_OPERATOR` (3) < `POLICY_APPROVER` (4).
   - Privilege escalation without explicit grant returns `PERMISSION_DENIED`.
3. **Idempotency Guarantees:**
   - Exact repeat calls with the same key return the original receipt and do not re-apply effects.
   - Key reuse with tampered/altered parameters returns `IDEMPOTENCY_KEY_REUSED_WITH_DIFFERENT_PARAMS`.
4. **Precondition & Revision Drift Safety:**
   - Preconditions are validated at apply time against current runtime revision. Stale proposals return `PRECONDITION_FAILED`, preventing stale approvals from overwriting newly accepted evidence.
5. **Prompt Injection Sanitization:**
   - Untrusted inputs are stripped of control characters and tested against hostile intent patterns before being processed.

---

## 3. Test Evidence Across Repositories

- **Observation Tracker (`hyperliquid-trader-tracker`):**
  - `make check-rs`: 115 tests passed, 0 clippy warnings, clean formatting.
  - `make test`: 79 pytest tests passed.
  - Total: 194 tests passing.
- **Expert Ensemble (`hyperliquid-expert-ensemble`):**
  - `uv run ruff check .`: clean.
  - `uv run ty check`: clean.
  - `uv run pytest -q`: 17 passed in 0.14s.
- **Combined Test Count:** 211 passed across both repositories.
