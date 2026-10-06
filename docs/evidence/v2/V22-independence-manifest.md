# V22: Independence artifact V2 qualification manifest

**Capsule ID:** V22  
**Milestone:** Forward-qualified expert signal engine V2  
**Issue:** #14 — V22: Build the independence artifact V2  
**Status:** PASS  
**Qualification class:** DESCRIPTIVE  
**Expert-engine commit:** `f67871f`

## Deliverable

The expert engine now exposes immutable point-in-time `IndependenceArtifact` objects with:

- versioned feature-schema and artifact revisions;
- explicit training cutoff and effective-from timestamps;
- active-step and joint-episode support diagnostics;
- deterministic complete-link clustering;
- equal independent cluster budgets and effective breadth;
- conservative UNKNOWN-similarity pooling for unsupported newcomers;
- retained pair metrics and update-schedule metadata.

Pair metrics newer than the training cutoff are rejected rather than leaking future similarity into historical replay.

## Acceptance evidence

`tests/test_v22_independence.py` verifies Q08:

- 15 exact clones collapse to one independent cluster with breadth 1;
- complete-link avoids A-B-C chain-link merging;
- low-support/new experts share an UNKNOWN-similarity pool rather than manufacturing breadth;
- supported experts and unknown newcomers remain distinguishable;
- future similarity metrics are rejected for an earlier as-known cutoff;
- strategy drift creates a new artifact revision and does not mutate historical membership.

## Verification

```text
uv run pytest -q tests/test_v22_independence.py
6 passed

uv run ruff check .
PASS

uv run ty check
PASS

uv run pytest -q
52 passed
```

## Exit

**PASS.** V24 can consume a stable, point-in-time independence layer after V23 supplies trader-relative conviction.
