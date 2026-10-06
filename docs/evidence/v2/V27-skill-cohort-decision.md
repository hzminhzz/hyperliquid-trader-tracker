# V27: Skill/cohort divergence decision

**Capsule ID:** V27  
**Milestone:** Forward-qualified expert signal engine V2  
**Issue:** #19 — V27: Test skill and cohort divergence  
**Status:** NOT_ACTIVATED  
**Qualification class:** HISTORICAL_PREDICTIVE  
**Upstream dependency:** V26 / #18 = INCONCLUSIVE, no promoted evidence family

## Conditional-start decision

V27 is explicitly conditional: it starts only if the B0-B4 program provides evidence that justifies added complexity.

V26 produced no point-in-time historical V2 outcome sample and promoted no candidate. Therefore the activation predicate for V27 is false.

Implementing alpha/control/anti-alpha cohort weighting now would:

- add model/search complexity without a qualified simpler baseline;
- create no valid incremental holdout comparison;
- risk reintroducing the exact engineering-vs-edge conflation V2 was designed to prevent.

## Current treatment

- No V27 cohort artifact is promoted.
- No skill-divergence feature enters `PredictiveEvidence`.
- Existing V1 B3 quality-weighting code remains historical research machinery only.
- Q10 cohort PIT correctness and Q17 incremental predictive value are **not claimed**.
- Search count for V27 at this cut: **0**.

## Revisit condition

Reopen a new V27 candidate only after V26 or a successor produces a promoted simpler evidence representation on a sealed point-in-time historical holdout with reusable outcomes.

## Exit

**Resolved by precondition: NOT_ACTIVATED.** This is the required low-complexity behavior for the current evidence state, not a failed implementation.
