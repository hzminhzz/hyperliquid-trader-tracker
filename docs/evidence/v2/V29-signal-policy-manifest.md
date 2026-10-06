# V29: Deterministic TradeSignal policy qualification manifest

**Capsule ID:** V29  
**Milestone:** Forward-qualified expert signal engine V2  
**Issue:** #21 — V29: Build the deterministic TradeSignal state machine  
**Engineering status:** PASS  
**Historical-predictive status:** REJECTED / NOT PROMOTED  
**Expert-engine commit:** `88e950d`

## Deliverable

The expert engine now exposes deterministic, account-independent:

- `PredictiveEvidence`;
- structured `ConfidenceComponents`;
- versioned `SignalPolicy`;
- `TradeSignal` with FLAT/LONG/SHORT state;
- ENTER/INCREASE/REDUCE/EXIT/REVERSE/NONE events;
- explicit signal strength, crowding risk, expected return, baselines, lineage, invalidation conditions, and blocker codes.

Signal strength, confidence/support, crowding risk, and expected return remain separate. An expected return is rejected unless a calibration revision is present.

## State-machine evidence

`tests/test_v29_signal_policy.py` verifies:

- disabled/unpromoted policy produces FLAT/NONE with `NO_PROMOTED_PREDICTIVE_EVIDENCE`;
- persistence alone emits NONE;
- same-direction strengthening emits INCREASE;
- same-direction weakening emits REDUCE without changing LONG/SHORT state;
- exit hysteresis produces EXIT;
- opposite entry-threshold crossing is required for REVERSE;
- missing promoted features block rather than guess;
- crowding remains separate from confidence and expected-return calibration.

## Current predictive decision

V26 promoted no evidence family. V27 and V28 were correctly not activated.

Therefore the current V2 `SignalPolicy` must remain disabled. There is no defensible historical-predictive candidate to send into live-forward qualification.

This is the V2 plan's valid negative terminal branch: descriptive evidence infrastructure exists, but predictive signal activation is explicitly rejected.

## Verification

```text
uv run pytest -q tests/test_v29_signal_policy.py
6 passed

uv run ruff check .
PASS

uv run ty check
PASS

uv run pytest -q
79 passed
```

## Exit

**Engineering PASS; predictive promotion REJECTED.** No forward-qualified V2 signal exists at this cut. V30's historical-promotion dependency is therefore not satisfied.
