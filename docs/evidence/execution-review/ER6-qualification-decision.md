# ER6 qualification decision

Date: 2026-10-06  
Issue: #30  
Decision revision: \`er6:v1\`  
Status: **DESCRIPTIVE ENGINEERING COMPLETE / PREDICTIVE QUALIFICATION BLOCKED**

## Acceptance matrix

| Gate | Evidence | Result | What it means |
|---|---|---|---|
| ER0 | ER0 source/state authority | ENGINEERING_PASS / LIVE_SOURCE_CAPSULE_BLOCKED | Durable source/state contract qualified by faults/replay; no recorded live traffic capsule. |
| ER1 | ER1 additive path | ENGINEERING_PASS | Raw additive path and normalization/missingness semantics qualified. |
| ER2 | ER2 execution annotations | ENGINEERING_PASS / NATIVE-LIVE-SOURCE-BLOCKED | Optional annotation state machines work; native detector precision/recall is not live-qualified. |
| ER3 | ER3 coherent cuts | ENGINEERING_PASS / LIVE-COVERAGE-MEASUREMENT-BLOCKED | Cut/coalescing/budget semantics qualified; no real 50/120 coverage measurement. |
| ER4 | ER4 representation experiment | PROTOCOL_ENGINEERING_PASS / EMPIRICAL_EVALUATION_BLOCKED | R0/R1/R2 and outcomes are frozen; no prospective outcome-mature rows. |
| ER5 | ER5 cadence/universe experiment | PROTOCOL_ENGINEERING_PASS / NEW-HOLDOUT-BLOCKED | Independent Stage-2 design is frozen; no new untouched holdout rows. |

Historical V2 evidence remains unchanged: V26 INCONCLUSIVE, V27/V28 NOT_ACTIVATED,
V29 REJECTED / ENGINEERING_PASS, V30 NOT_ACTIVATED, V31 NOT_ACTIVATED.

## Deterministic decision

The companion engine implements a fail-closed qualification gate that keeps these classes separate:

1. engineering implementation;
2. descriptive evidence;
3. historical predictive evidence;
4. live-forward shadow evidence;
5. production advisory authority;
6. financial execution authority.

Current decision:

- engineering: PASS for the implemented/protocol slices, with empirical sub-gates explicitly blocked;
- descriptive: PASS on deterministic fixtures/replay;
- historical predictive: **BLOCKED**;
- live-forward: **NOT_RUN**;
- production advisory: **NOT_RUN**;
- predictive emission: **DISABLED**;
- promoted policy revision: **none**;
- expected-return calibration: **unavailable**;
- financial execution: **FORBIDDEN**.

Passing tests, closing a milestone, increasing wallet count, or observing a positive unqualified mean
cannot change those gates.

## Verification

Companion engine:
- Ruff: PASS.
- \`ty\` on ER6 qualification/capability modules: PASS.
- ER6 focused tests: **4 passed**.
- Full suite: **115 passed**.

Capability inspection now states the qualification boundary explicitly rather than reporting all
implemented surfaces as predictive qualification.

## Next evidence-dependent frontier

Collect prospective, point-in-time, outcome-mature ER4 evidence under the frozen protocol. Only a real
ER4 PASS followed by an independent ER5 PASS can make a frozen candidate eligible for a distinct V30
live-forward shadow gate. V31 production advisory remains a separate approval. Financial execution is
outside this system's authority.

This issue is complete even though prediction is not qualified; BLOCKED/INCONCLUSIVE is a valid
scientific terminal state until new evidence exists.
