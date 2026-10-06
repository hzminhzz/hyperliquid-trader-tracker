# V25: Forward outcome ledger qualification manifest

**Capsule ID:** V25  
**Milestone:** Forward-qualified expert signal engine V2  
**Issue:** #17 — V25: Build the forward outcome ledger  
**Status:** PASS  
**Qualification class:** ENGINEERING  
**Expert-engine commit:** `1bbd2ae`

## Deliverable

The expert engine now has an append-only SQLite forward-outcome ledger with:

- content-addressed sealed manifests;
- deterministic source/horizon work pairs;
- immutable per-horizon `OutcomeRecord` attachments;
- 1m, 5m, 15m, 1h, 4h, and 24h default horizons;
- raw, latency-adjusted, and cost-adjusted forward returns;
- MFE and MAE over the latency-adjusted path;
- context/coverage and late-restatement flags captured from emission time;
- idempotent append and conflict rejection;
- restart/resume by recomputing only missing manifest pairs.

Outcome attachment never mutates the original evidence or decision.

## Acceptance evidence

`tests/test_v25_outcomes.py` verifies Q12:

- exact return/latency/cost/MFE/MAE arithmetic;
- deterministic manifest identity independent of source ordering;
- idempotent duplicate append and rejection of conflicting rewrites;
- restart recovery returns only remaining source-horizon pairs;
- not-yet-knowable horizons produce no record.

## Verification

```text
uv run pytest -q tests/test_v25_outcomes.py
5 passed

uv run ruff check .
PASS

uv run ty check
PASS

uv run pytest -q
67 passed
```

## Exit

**PASS.** V26 can evaluate frozen representation variants against one reusable, immutable outcome substrate.
