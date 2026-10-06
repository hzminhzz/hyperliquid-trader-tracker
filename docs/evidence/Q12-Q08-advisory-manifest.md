# Q12-Q08: Account Advisory, Risk Ceilings, and Notification Evidence Manifest

**Capsule ID:** Q12-Q08  
**Milestone:** Agent-operable expert ensemble V1  
**Issue:** #8 — S7: Produce bounded account advice, not assumed copying  
**Status:** PASS  
**Inspection Date:** 2026-10-06  
**Application Repository:** `/home/quant/dev/hyperliquid-expert-ensemble`  
**Application Commit:** `7c31631`  
**Tracker Commit:** HEAD of `feat/agent-operable-expert-ensemble-v1`  

---

## 1. Summary of Deliverables Qualified

This manifest certifies completion of slice **S7**, delivering the account adviser, explicit rulebook constraints, loss models, downward lot rounding, unsized informational fallbacks, and the meaningful-change Telegram notification adapter per [RESEARCH](../RESEARCH.md) R6 and [QUALIFICATION](../QUALIFICATION.md) Q12, Q08:

### Qualified Acceptance Scenarios

| Scenario ID | Name | Pass Condition | Verification Test | Status |
|---|---|---|---|---|
| **Q12** | Advice and account constraints | Risk limits aggregate existing open risk against total and daily loss floors; sizing respects the allowable headroom ceiling; lot rounding rounds downward to lot step; missing broker spec or loss model produces `UNSIZED` informational output; unobserved account position omits action delta. | `tests/test_s7_qualification.py::test_q12_account_drawdown_headroom_and_trade_ceilings`, `test_q12_downward_lot_rounding_and_min_lot`, `test_q12_missing_data_produces_unsized_fallback`, `test_q12_unknown_account_position_omits_action_delta` | PASS |
| **Q08** | Notification adapter & meaningful change | Telegram adapter filters micro-fluctuations (<5%) while preserving notifications for closes, direction flips, and status changes; formats clean HTML cards. | `tests/test_s7_qualification.py::test_q08_telegram_notification_filtering_and_formatting` | PASS |

---

## 2. Invariants & Mathematical Rules Qualified (R6, Q12)

1. **Drawdown Headroom & Risk Ceilings:**
   - Total floor: $\text{floor}_{\text{total}} = \text{starting\_equity} \times (1 - \text{max\_total\_drawdown\_pct})$.
   - Remaining total headroom: $\text{headroom}_{\text{total}} = \max(0, \text{current\_equity} - \text{floor}_{\text{total}} - \text{existing\_open\_risk})$.
   - Daily floor: $\text{floor}_{\text{daily}} = \text{daily\_starting\_equity} \times (1 - \text{max\_daily\_drawdown\_pct})$.
   - Remaining daily headroom: $\text{headroom}_{\text{daily}} = \max(0, \text{current\_equity} - \text{floor}_{\text{daily}} - \text{existing\_open\_risk})$.
   - Allowable trade risk: $\min(\text{current\_equity} \times \text{max\_risk\_per\_trade\_pct}, \text{headroom}_{\text{total}}, \text{headroom}_{\text{daily}})$.
   - If allowable risk $\le 0$, output transitions to `BLOCKED` with blocker code `RISK_HEADROOM_EXHAUSTED`.
2. **Downward Lot Rounding:**
   - Unit risk: $\text{unit\_risk} = \text{stop\_distance\_pct} + \text{fee\_rate} + \text{slippage\_rate}$.
   - Max allowable units: $\text{allowable\_risk} / (\text{unit\_risk} \times \text{valuation\_price})$.
   - Rounded units: $\lfloor \text{raw\_units} / \text{lot\_step} \rfloor \times \text{lot\_step}$.
   - Min lot floor: if rounded units $< \text{min\_lot}$, sized to $0.0$.
3. **UNSIZED Fallback:**
   - If broker instrument specification or explicit stop loss distance is missing, advice is marked `UNSIZED` with `target_quantity = None` and `action_delta = None`. No trade is guessed without an explicit loss model.
4. **Action Delta Integrity:**
   - If account position is confirmed/known: $\text{delta} = \text{target\_quantity} - \text{held\_quantity}$.
   - If account position is unobserved/unknown: $\text{action\_delta} = \text{None}$. The system displays target quantity only, strictly refusing to infer an incremental add/reduce order.

---

## 3. Test Evidence Across Repositories

- **Observation Tracker (`hyperliquid-trader-tracker`):**
  - `make check-rs`: 115 tests passed, 0 clippy warnings, clean formatting.
  - `make test`: 79 pytest tests passed.
  - Total: 194 tests passing.
- **Expert Ensemble (`hyperliquid-expert-ensemble`):**
  - `uv run ruff check .`: clean.
  - `uv run ty check`: clean.
  - `uv run pytest -q`: 26 passed in 0.81s.
- **Combined Test Count:** 220 passed across both repositories.
