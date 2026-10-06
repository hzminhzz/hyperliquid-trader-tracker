# V24: Canonical EnsembleEvidence qualification manifest

**Capsule ID:** V24  
**Milestone:** Forward-qualified expert signal engine V2  
**Issue:** #16 — V24: Create canonical EnsembleEvidence  
**Status:** PASS  
**Qualification class:** DESCRIPTIVE  
**Expert-engine commit:** `133c1e6`

## Deliverable

The expert engine now exposes one immutable `EnsembleEvidence` interface that assembles, without collapsing:

- independent state evidence and missing mass;
- multi-horizon intent-flow evidence and missing mass;
- trader-relative conviction summary;
- effective independent breadth;
- per-cluster supporting/opposing contributions;
- reliability and missing-information diagnostics;
- B0 equal-wallet bounded, B1 equal-wallet raw, and B2 independent-state comparators;
- wallet, independence, conviction, feature, and universe revisions;
- optional skill divergence, market divergence, and crowding fields that remain unset until later slices qualify them.

`explain_ensemble_evidence` renders the persisted artifact directly, so agents do not reconstruct model logic from raw tables.

## Acceptance evidence

`tests/test_v24_ensemble_evidence.py` verifies:

- clone-adjusted B2 state remains distinct from simpler B0/B1 baselines;
- missing cluster mass is never reallocated to survivors;
- state and flow remain separate dimensions;
- explanations include lineage, support, baselines, and unset extension fields;
- future wallet evidence and not-yet-effective independence artifacts are rejected.

## Verification

```text
uv run pytest -q tests/test_v24_ensemble_evidence.py
5 passed

uv run ruff check .
PASS

uv run ty check
PASS

uv run pytest -q
62 passed
```

## Exit

**PASS.** V25 can attach immutable forward outcomes to evidence IDs without changing the evidence itself.
