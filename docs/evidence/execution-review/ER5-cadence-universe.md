# ER5 cadence and universe experiment

Date: 2026-10-06  
Issue: #29  
Status: **PROTOCOL_ENGINEERING_PASS / NEW-HOLDOUT-BLOCKED**

## Frozen Stage-2 protocol

ER5 is structurally separate from ER4 and rejects any holdout that overlaps or begins at/before the
ER4 holdout end.

Registered cadence arms:

- fixed 60 seconds;
- fixed 300 seconds;
- 60 seconds plus three-second coalesced material interrupts;
- normalized exposure-event sampling with a five-minute maximum idle timeout.

The comparison uses a common one-minute market-time grid and only the latest prediction actually
available at each row. Action replay is chronological and charges costs only when target exposure
changes; repeated identical predictions do not create artificial turnover.

Universe labels are original_50 and prospective_120. No 200-wallet arm is registered.

## Verification

Companion engine:
- Ruff: PASS.
- \`ty\` on ER5 module: PASS.
- ER5 focused tests: **5 passed**.
- Full engine suite: **111 passed** after a single transient wall-clock benchmark miss; the isolated
  capacity benchmark rerun passed in 0.26s and the subsequent full suite passed in 0.87s.

Fixtures cover holdout separation, as-known common-grid sampling, idle expiry to missing rather than
zero, chronological target-change costs, and absent new holdout rows.

## Decision

There is no new untouched prospective holdout after ER4 because the local recorder currently has no
recorded evidence rows. Stage 2 therefore returns **BLOCKED / NO_NEW_UNTOUCHED_HOLDOUT_ROWS**.

No claim is made that 60s, 300s, hybrid interrupts, or activity bars are superior. No claim is made
that ~120 wallets improve on ~50. The simplest existing design remains the operational research
default until outcome-mature paired evidence exists.
