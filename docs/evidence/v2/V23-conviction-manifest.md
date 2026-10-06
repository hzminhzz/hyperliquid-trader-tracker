# V23: Trader-relative conviction qualification manifest

**Capsule ID:** V23  
**Milestone:** Forward-qualified expert signal engine V2  
**Issue:** #15 — V23: Add trader-relative conviction  
**Status:** PASS  
**Qualification class:** DESCRIPTIVE  
**Expert-engine commit:** `f9f63e0`

## Deliverable

The expert engine now exposes frozen `RelativeConvictionArtifact` distributions and point-in-time scoring:

- absolute raw-bias history fitted strictly before the training cutoff;
- robust empirical percentile scoring with midpoint tie handling;
- signed percentile separated from magnitude percentile;
- explicit minimum-support and bias-unavailable states;
- median-scale drift diagnostics with configurable threshold;
- immutable artifact revision and support metadata.

Samples known at or after the training cutoff are excluded, so today's or future exposure cannot improve an historical reference distribution.

## Acceptance evidence

`tests/test_v23_conviction.py` verifies Q09:

- low support returns `INSUFFICIENT_SUPPORT`, never a neutral fabricated score;
- current exposure is ranked relative to the trader's own lagged history;
- direction is applied only after magnitude percentile is calculated;
- observations at/future of the training cutoff do not leak;
- a large recent scale shift is flagged by drift diagnostics;
- missing current raw bias remains unavailable.

## Verification

```text
uv run pytest -q tests/test_v23_conviction.py
5 passed

uv run ruff check .
PASS

uv run ty check
PASS

uv run pytest -q
57 passed
```

## Exit

**PASS.** V24 may combine WalletEvidence, independence, and relative conviction into canonical EnsembleEvidence without collapsing them into one score.
