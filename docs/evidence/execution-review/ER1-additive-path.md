# ER1 additive point-in-time path evidence

Date: 2026-10-06  
Issue: #25  
Status: **ENGINEERING_PASS**

## Implemented contract

- Raw signed quantity deltas are the additive primitive.
- Visibility requires `known_at <= knowledge_time`; horizon membership uses `event_time`.
- Net and gross quantity accumulate before one common valuation/equity anchor is applied.
- Missing/stale equity preserves raw quantity while normalized net/gross flow stays unavailable.
- Flow values expose net quantity, gross quantity, net/gross ratio, build rate, quantity excursion,
  recent-close context, support count, and first/last event time.
- Atomic flips allocate disjoint close/open legs whose deltas sum to the real fill.
- Mark/equity/reconciliation causes do not create execution-path events.
- Partial-history/open-age uncertainty remains distinct from event recency.

## Verification

Companion engine branch verification:

- Ruff: PASS.
- `ty check src`: PASS.
- Pytest: **88 passed**.
- Focused ER1 + existing V21 evidence tests: **14 passed**.

Key fixtures:

- split invariance across one fill versus ten children;
- rapid open/close: zero net, positive gross and excursion;
- +3 -> -2 flip: -3 close leg and -2 open leg, total -5;
- +2 -> +1: positive long state with negative reducing flow;
- old event newly known does not become fresh;
- stale/missing equity yields missing normalized flow rather than zero;
- non-execution causes cannot create path events.

## Qualification boundary

ER1 is descriptive evidence only. It does not claim episode/intent inference quality, independent breadth,
predictive value, clock superiority, or financial authority. The live-source ER0 capsule is still blocked
by absent recorded traffic; ER1 tests use deterministic fixtures and existing replay contracts.
