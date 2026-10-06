# ADR-0002: Preserve descriptive evidence before predictive compression

**Date:** 2026-10-06  
**Status:** adopted for the V2 signal-engine architecture. Implementation and predictive qualification are pending.

## Context

The V1 ensemble normalizes wallet exposure as signed notional divided by equity and then bounds that value for aggregation. This is a useful anti-dominance baseline, but a bounded posture cannot represent every distinction required for predictive research:

- +1x and +5x leverage can collapse to the same posture;
- persistent state and new intent flow are not the same information;
- one expert's routine 1x exposure can be another expert's exceptional exposure;
- clone-resistant influence and predictive skill are different concepts;
- high agreement can coexist with severe crowding;
- software-correct research mechanics can exist without economic edge.

The system already has strong observation/replay/control infrastructure. Replacing that substrate is unnecessary. The decision is about the interpretation seam above authoritative account state.

## Decision

V2 preserves a tower of immutable descriptive evidence before any predictive compression:

```text
account view
  -> WalletEvidence
  -> Independence/Cohort artifacts
  -> EnsembleEvidence
  -> PredictiveEvidence
  -> deterministic TradeSignal
```

`WalletEvidence` retains raw unclipped bias, bounded influence, portfolio concentration, state, verified intent flow, age semantics, reliability and lineage.

State and flow remain separate through `EnsembleEvidence`. Relative conviction, skill divergence, market divergence and crowding are explicit dimensions rather than implicit changes to one consensus number.

Only feature families that pass the declared point-in-time historical and live-forward research gates may enter `PredictiveEvidence`. The deterministic `TradeSignal` policy consumes that promoted subset and still carries baseline comparators and evidence references.

Signal strength, confidence/support, crowding risk and expected return remain separate fields. Expected return stays unavailable until calibration is demonstrated.

## Alternatives considered

| Alternative | Attraction | Rejection reason |
|---|---|---|
| Keep V1 posture as the canonical representation | Smallest change | Irreversibly hides leverage magnitude and makes state/flow research awkward. |
| Replace clipping with raw-bias capital weighting | Retains leverage magnitude | Lets whales/leverage dominate influence and conflates capital-at-risk with forecasting skill. |
| Build one composite score immediately | Simple downstream interface | Makes ablations, causality and uncertainty opaque before predictive value is established. |
| Move directly to ML | Can model nonlinear interactions | Increases overfitting/search risk before simple feature families are understood. |
| Put scoring into Rust ingestion | Single process | Couples source truth with research policy and makes iteration/replay harder. |
| Treat B2/B3/B4 tests as economic qualification | Fast promotion | Their inspected tests establish mechanics, not forward-return advantage. |

## Consequences

The Python expert-engine gains a deeper evidence interface and more persisted feature artifacts, but callers need fewer ad hoc reconstructions.

Research becomes cheaper because every candidate reuses the same point-in-time evidence and forward-outcome substrate.

Some V1 constants and candidate mechanics are demoted from apparent policy to experiment parameters. Existing V1 behavior remains available as B0 and as a production fallback until a V2 policy is forward-qualified and explicitly approved.

The design can validly conclude that no predictive signal exists. Engineering quality alone is not considered economic success.

## Revisit triggers

Revisit this decision only if:
- preserving the evidence dimensions creates measured unacceptable resource cost that cannot be solved incrementally;
- forward research shows a simpler representation is sufficient across the qualified objective and the lost information has no diagnostic value;
- a materially different observation contract is required to reconstruct intent correctly.

Do not revisit merely because an end-to-end learned model is easier to prototype.

## Related authorities

[SIGNAL-V2](../SIGNAL-V2.md) · [RESEARCH](../RESEARCH.md) · [QUALIFICATION](../QUALIFICATION.md) · [SYSTEM](../SYSTEM.md) · [plan](../../plan.md)
