# V26: Frozen B0-B4 ablation program evidence

**Capsule ID:** V26  
**Milestone:** Forward-qualified expert signal engine V2  
**Issue:** #18 — V26: Run the frozen B0-B4 ablation program  
**Engineering status:** PASS  
**Historical-predictive status:** INCONCLUSIVE  
**Expert-engine commit:** `5535eee`

## Research machinery delivered

The expert engine now implements a frozen paired-ablation protocol with:

- immutable pre-holdout registrations and content hashes;
- paired complete-case evaluation on identical source IDs;
- explicit minimum sample, predeclared delta, search-budget, latency, and cost assumptions;
- PROMOTE / REJECT / INCONCLUSIVE outcomes;
- append-only retention of positive, negative, and inconclusive results;
- mandatory registration families for clipped-vs-raw, state-vs-flow, state+flow, equal-vs-cluster, absolute-vs-relative, latency/cost, and expert-dropout sensitivity.

The protocol refuses to reinterpret insufficient support as a pass.

## Current data audit

At the qualification cut:

```text
observation.db / outbox_events: 0 events
projection.db / projected_positions: 6 current rows
projection.db / expert_equity: 3 current rows
V2 outcome stores: none
```

The current projection is therefore current-state/snapshot evidence, not a point-in-time historical V2 evidence/outcome sample. It cannot support the frozen historical ablations required by Q13-Q16.

## B0-B4 result

| Comparison | Status | Search count | Reason |
|---|---|---:|---|
| B0 bounded -> B1 raw state | INCONCLUSIVE | 0 | no PIT V2 outcome sample |
| B1 raw state -> B2 independent state | INCONCLUSIVE | 0 | no PIT V2 outcome sample |
| B2 independent state -> B3 independent flow | INCONCLUSIVE | 0 | no PIT V2 outcome sample |
| B3 flow -> B4 relative conviction | INCONCLUSIVE | 0 | no PIT V2 outcome sample |
| state vs state+flow | INCONCLUSIVE | 0 | no PIT V2 outcome sample |
| latency/cost sensitivity | INCONCLUSIVE | 0 | no PIT V2 outcome sample |
| expert-dropout sensitivity | INCONCLUSIVE | 0 | no PIT V2 outcome sample |

No candidate is promoted. No holdout was searched or retuned.

## Verification

```text
uv run pytest -q tests/test_v26_ablation.py
6 passed

uv run ruff check .
PASS

uv run ty check
PASS

uv run pytest -q
73 passed
```

The tests verify paired sample identity, frozen promotion/rejection rules, insufficient-support INCONCLUSIVE behavior, search-budget rejection, mandatory ablation registration, and durable retention of negative/inconclusive results.

## Dependency consequence

V27 skill/cohort divergence and V28 market/crowding candidates are explicitly conditional on V26 evidence justifying additional complexity. That precondition is **not met** at this cut. They must not be promoted or added to PredictiveEvidence merely to keep the milestone moving.

V29 may still implement the deterministic signal-policy seam, but absent a promoted evidence family it must end as an explicitly rejected/disabled predictive policy rather than inventing a forward-qualified signal.

## Exit

**Engineering PASS; historical-predictive INCONCLUSIVE.** This is a valid scientific result under the V2 plan. The simpler descriptive engine remains authoritative.
