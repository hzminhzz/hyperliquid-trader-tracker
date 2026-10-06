# Q10-Q14: Target Explanation, Normalization, and Consensus Evidence Manifest

**Capsule ID:** Q10-Q14  
**Milestone:** Agent-operable expert ensemble V1  
**Issue:** #5 — S4: Deliver the first complete target explanation  
**Status:** PASS  
**Inspection Date:** 2026-10-06  
**Application Repository:** `/home/quant/dev/hyperliquid-expert-ensemble`  
**Application Commit:** `fd190b9`  
**Tracker Commit:** HEAD of `feat/agent-operable-expert-ensemble-v1`  

---

## 1. Summary of Deliverables Qualified

This manifest certifies completion of slice **S4**, delivering declared eligibility, fixed-scale normalization, equal-budget baseline consensus (B1), unknown-weight bounds, causal change categories, and contribution ledgers per [RESEARCH](../RESEARCH.md) and [QUALIFICATION](../QUALIFICATION.md):

### Qualified Acceptance Scenarios

| Scenario ID | Name | Pass Condition | Verification Test | Status |
|---|---|---|---|---|
| **Q10** | Baseline signal semantics | Exact arithmetic matches declared normalization ($e = q \cdot px / \text{equity}$, $s = \text{clip}(e/k, -1, 1)$); flat, abstention, unavailable, and removed states have distinct semantic effects; missing mass is not reallocated to amplify surviving votes. | `tests/test_s4_qualification.py::test_q10_exact_fixed_scale_normalization_arithmetic`, `test_q10_distinct_effects_of_eligibility_states`, `test_q10_no_renormalization_missing_experts_cannot_amplify_survivors` | PASS |
| **Q14** | Replay equivalence | Pinned input manifests produce identical target explanations and decision hashes across repeated executions under deterministic replay. | `tests/test_s3_qualification.py::test_q14_replay_equivalence_and_isolation` | PASS |
| **Explanations** | Causal attribution & blockers | `explain_target` surfaces causal change categories (`ECONOMIC_CHANGE`, `VALUATION_CHANGE`, `EQUITY_CHANGE`, `RECONCILIATION`), posture intervals, and per-expert contribution ledgers; `explain_blocker` provides explicit remediation advice. | `tests/test_s4_qualification.py::test_causal_explanations_and_contribution_ledger` | PASS |

---

## 2. Invariants & Mathematical Rules Qualified (R2, R3, Q10)

1. **Fixed-Scale Normalization:**
   - Exposure formula: $e(i,a) = \text{quantity} \times \text{valuation\_price} / \text{scoped\_equity}$.
   - Posture formula: $s(i,a) = \text{clip}(e(i,a) / k, -1.0, 1.0)$ with $k = 1.0$.
   - Missing or nonpositive equity renders normalization unavailable (`UNAVAILABLE`), never guessing or fabricating state.
2. **Distinct Eligibility States:**
   - `ELIGIBLE`: active in-scope expert with valid equity.
   - `KNOWN_FLAT`: reliably verified flat (0 size) contributing zero posture ($0.0$) without adding to missing mass.
   - `ABSTAINING`: out of scope by policy (horizon/instrument), contributes zero posture without voting.
   - `UNAVAILABLE`: missing/stale equity (>15m) or unseeded state; contributes its weight to missing mass $m$.
   - `REMOVED`: dropped from universe.
3. **Equal-Budget Consensus Without Renormalization (R3 & Q10 Invariant):**
   - Each expert has weight $w_i = 1/N$.
   - Observed target: $C_{\text{observed}} = \sum_{i \in \text{available}} w_i \cdot s_i$.
   - Missing mass: $m = \sum_{i \in \text{unavailable}} w_i$.
   - Missing mass is **NOT** reallocated to surviving experts. If peers go offline or have stale equity, surviving experts keep their original weight $w_i$ and do not gain amplified influence.
   - Posture interval: $[C_{\text{observed}} - m, C_{\text{observed}} + m]$ clipped to $[-1.0, 1.0]$.
   - Actionability: blocked (`AMBIGUOUS_BOUNDS` or `INSUFFICIENT_COVERAGE`) if $m \geq 0.50$ or interval spans zero when $C_{\text{observed}} \neq 0$.

---

## 3. Test Evidence Across Repositories

- **Observation Tracker (`hyperliquid-trader-tracker`):**
  - `make check-rs`: 115 tests passed, 0 clippy warnings, clean formatting.
  - `make test`: 79 pytest tests passed.
  - Total: 194 tests passing.
- **Expert Ensemble (`hyperliquid-expert-ensemble`):**
  - `uv run ruff check .`: clean.
  - `uv run ty check`: clean.
  - `uv run pytest -q`: 10 passed in 0.19s.
- **Combined Test Count:** 204 passed across both repositories.
