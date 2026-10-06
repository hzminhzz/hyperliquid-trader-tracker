# V21: WalletEvidence qualification manifest

**Capsule ID:** V21  
**Milestone:** Forward-qualified expert signal engine V2  
**Issue:** #13 — V21: Deepen WalletEvidence  
**Status:** PASS  
**Qualification class:** DESCRIPTIVE  
**Expert-engine commit:** `fdd8607e7055729e4ecfa6e9fab78afcf96edc9c`

## Deliverable

The expert engine now exposes a canonical `WalletEvidence` seam in `src/ensemble/wallet_evidence.py` with:

- raw unclipped equity bias;
- bounded influence as a separate field;
- observable portfolio share;
- OPEN / ADD / REDUCE / CLOSE / FLIP intent classification;
- multi-horizon no-decay intent-flow windows keyed by `known_at`;
- distinct position, intent, and observation ages;
- explicit economic, valuation, equity, and reconciliation causes;
- input revision, evidence refs, reliability state, and missing reasons.

Valuation, equity, and reconciliation causes cannot manufacture intent events.

## Acceptance evidence

`tests/test_v21_wallet_evidence.py` verifies Q04-Q07 semantics:

- +5x raw bias remains +5 while bounded influence saturates at +1;
- +2x -> +1x is REDUCE, retains positive state, and creates -1 normalized flow;
- a +0.1 position add on a large existing position produces only +0.01 normalized flow;
- persistent unchanged state creates no new flow after prior intent ages out;
- events not yet known at the decision cut do not leak into flow;
- true position age, last-intent age, and observation age remain separate;
- seeded/partial history can leave position age unknown;
- missing equity yields unavailable bias, never fabricated zero.

## Verification

```text
uv run pytest -q tests/test_v21_wallet_evidence.py
7 passed

uv run ruff check .
PASS

uv run ty check
PASS

uv run pytest -q
46 passed
```

The tracker observation contract and runtime scoring policy were not changed in this slice.

## Exit

**PASS.** V22 and V23 may now build point-in-time independence and trader-relative conviction on the WalletEvidence seam.
