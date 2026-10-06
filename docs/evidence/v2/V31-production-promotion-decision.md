# V31: Production advisory promotion decision

**Capsule ID:** V31  
**Milestone:** Forward-qualified expert signal engine V2  
**Issue:** #23 — V31: Promote the qualified signal to production advisory  
**Status:** NOT_ACTIVATED  
**Qualification class:** ADVISORY_PRODUCTION  
**Upstream dependency:** V30 / #22 = NOT_ACTIVATED; no forward-qualified V2 signal exists

## Promotion decision

Production advisory promotion is prohibited without a forward-qualified signal and explicit policy approval.

Those prerequisites are absent. Therefore:

- no V2 policy-approval artifact is issued;
- no V2 signal revision is selected by the production runtime;
- no V2 advisory claim is made;
- rollback is unnecessary because no V2 production change occurred;
- the existing V1 advisory/runtime behavior remains the fallback.

## Runtime verification

The V2 expert-engine branch changes only new evidence/research/signal modules and their tests.

`src/ensemble/runtime.py` and `deploy/systemd/copytrade-ensemble.service` have **no diff** from expert-engine `main`.

Thus this milestone does not silently change live advisory policy while its predictive evidence is unqualified.

## Safety boundary

- Automated financial execution remains unavailable.
- The current V2 signal policy is disabled/rejected.
- Q24-Q26 production-advisory qualification is not claimed for V2.
- B0/V1 remains the descriptive/advisory baseline until a future candidate passes historical and live-forward gates and receives explicit approval.

## Revisit condition

A future production promotion requires:

1. a historically promoted signal candidate;
2. a frozen live-forward PASS under its registered rule;
3. explicit policy approval;
4. runtime integration with explanation, degradation, and rollback tests;
5. unchanged prohibition on financial execution unless separately specified and authorized.

## Exit

**Resolved by dependency: NOT_ACTIVATED.** The safe terminal outcome is to keep V1 advisory behavior and refuse V2 production promotion.
