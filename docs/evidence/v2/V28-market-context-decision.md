# V28: Market divergence and crowding context decision

**Capsule ID:** V28  
**Milestone:** Forward-qualified expert signal engine V2  
**Issue:** #20 — V28: Test market divergence and crowding context  
**Status:** NOT_ACTIVATED  
**Qualification class:** HISTORICAL_PREDICTIVE  
**Upstream dependency:** V26 / #18 = INCONCLUSIVE, no promoted evidence family

## Conditional-start decision

V28 starts only when point-in-time context data are available and the simpler V26 ladder justifies additional complexity.

Neither condition is met:

1. V26 has no promoted predictive evidence family.
2. The current V2 evidence stores contain no sealed historical funding, open-interest, liquidation/crowding, or price-response context aligned to historical decision knowledge times.

Current/public API availability is not equivalent to point-in-time historical context evidence.

## Current treatment

- Funding, OI, volatility/price-response, and liquidation/crowding families do not enter `PredictiveEvidence`.
- Crowding remains an optional descriptive/risk concept in the schema, unset in current evidence.
- Directional-predictor and risk-gate variants are not compared because a valid paired holdout cannot be formed.
- Q18 historical predictive value is **not claimed**.
- Search count for V28 at this cut: **0**.

## Revisit condition

Start a new V28 candidate only after a sealed, provenance-preserving context dataset exists at the same point-in-time cuts as the promoted simpler evidence and immutable forward outcomes.

## Exit

**Resolved by precondition: NOT_ACTIVATED.** No market-context complexity is added without point-in-time evidence.
