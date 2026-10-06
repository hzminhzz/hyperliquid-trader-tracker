# Q15: Adaptive Quality Weighting, Regime Conditioning, and Research Promotion Evidence Manifest

**Capsule ID:** Q15  
**Milestone:** Agent-operable expert ensemble V1  
**Issue:** #10 — S9: Optional adaptive quality and regime research  
**Status:** PASS  
**Inspection Date:** 2026-10-06  
**Repositories:**  
- Observation Tracker: `/home/quant/dev/hyperliquid-trader-tracker`  
- Expert Ensemble Application: `/home/quant/dev/hyperliquid-expert-ensemble` (commit `80d2731`)  

---

## 1. Summary of Deliverables Qualified

This manifest certifies completion of slice **S9**, delivering the research candidate ladder (B3 lagged quality weighting and B4 regime conditioning) under strict frozen forward-evaluation and promotion authority gates per [RESEARCH](../RESEARCH.md) R7 and [QUALIFICATION](../QUALIFICATION.md) Q15:

### Qualified Acceptance Scenarios

| Scenario ID | Name | Pass Condition | Verification Test | Status |
|---|---|---|---|---|
| **Q15** | Research candidate ladder (B3, B4) | Quality weighting enforces train/eval temporal separation and min-support neutrality; regime conditioning dampens exposure under high volatility; promotion requires cleared Sharpe/turnover bar and policy approver grant; failed candidates retain negative ablation records while preserving baseline. | `tests/test_s9_research.py::test_b3_lagged_quality_weighting_and_min_support_neutrality`, `test_b3_quality_weighted_intra_cluster_consensus`, `test_b4_regime_detection_and_volatility_dampening`, `test_q15_candidate_promotion_and_retained_negative_results` | PASS |

---

## 2. Invariants & Mathematical Rules Qualified (R7, Q15)

1. **Temporal Train/Evaluation Separation:**
   - Quality metrics (profit factor, win rate, trade count) are strictly calculated over historical windows prior to the evaluation cutoff.
   - Forward holdout events cannot retroactively influence historical quality scores.
2. **Support Threshold & Neutral Default:**
   - Experts with trade counts $< \text{min\_support}$ (default 5 trades) receive neutral default score $1.0$, preventing small-sample score inflation or newcomer starvation.
   - Scores are normalized so intra-cluster weights sum to unit cluster budget.
3. **Regime Conditioning (B4):**
   - Realized return volatility over historical window determines regime: $\text{vol} \ge 3\% \implies \text{HIGH\_VOLATILITY}$.
   - Under `HIGH_VOLATILITY`, consensus posture is dampened by $0.70\times$ to mitigate tail stress while retaining causal category `VALUATION_CHANGE`.
4. **Promotion Authority & Retained Negative Results:**
   - Clearing the pre-declared evidence bar requires forward Sharpe delta $\ge 0.15$ AND turnover delta $\le 0.10$.
   - A candidate clearing the bar without an explicit `AuthorityRole.POLICY_APPROVER` grant transitions to `AWAITING_APPROVAL`, strictly prohibiting autonomous self-promotion.
   - Candidates failing the Sharpe bar or exceeding the turnover ceiling transition to `RETAIN_BASELINE`. Negative results, development search counts, and ablation tables are permanently retained in the evaluation archive.

---

## 3. Test Evidence Across Repositories

- **Observation Tracker (`hyperliquid-trader-tracker`):**
  - `make check-rs`: 115 tests passed, 0 clippy warnings, clean formatting.
  - `make test`: 79 pytest tests passed.
- **Expert Ensemble (`hyperliquid-expert-ensemble`):**
  - `uv run ruff check .`: clean.
  - `uv run ty check`: clean.
  - `uv run pytest -q`: 37 tests passed (all 4 S9 research tests passing).
- **Combined Test Count:** **231 tests passing** across the workspace.
