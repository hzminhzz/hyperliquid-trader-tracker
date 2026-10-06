# V30: Live-forward qualification decision

**Capsule ID:** V30  
**Milestone:** Forward-qualified expert signal engine V2  
**Issue:** #22 — V30: Freeze and run live-forward qualification  
**Status:** NOT_ACTIVATED  
**Qualification class:** LIVE_FORWARD  
**Upstream dependency:** V29 / #21 historical predictive promotion = REJECTED

## Dependency decision

V30 requires a historically promoted V29 candidate before a live-forward clock can start.

No such candidate exists:

- V26 historical B0-B4 comparisons are INCONCLUSIVE because no point-in-time V2 outcome sample exists.
- V27 and V28 were not activated.
- V29 state-machine engineering passed, but predictive activation was rejected and the policy remains disabled.

Starting a live-forward interval for an unqualified candidate would violate the registered V2 research protocol.

## Current treatment

- No V2 forward candidate is frozen.
- No live-forward clock is claimed.
- No Q21-Q23 forward result is claimed.
- No correctness fix or parameter change is attributed to a nonexistent forward candidate.
- The outcome ledger and signal-policy machinery remain available for a future newly registered candidate.

## Revisit condition

A future V30 candidate must begin with a new explicit historical promotion record, frozen code/feature/model/policy revisions, and a predeclared forward evaluation rule before observing its live-forward outcomes.

## Exit

**Resolved by dependency: NOT_ACTIVATED.** The system correctly refuses to manufacture a forward-qualified label when the historical gate did not pass.
