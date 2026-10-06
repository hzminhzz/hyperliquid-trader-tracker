# ER2 optional execution annotations

Date: 2026-10-06  
Issue: #26  
Status: **ENGINEERING_PASS / NATIVE-LIVE-SOURCE-BLOCKED**

## Implemented

The companion engine adds an optional annotation layer in \`execution_annotations.py\`.

- Native program IDs and OIDs are observed references only; none are fabricated.
- Inferred directional segments use deterministic \`CANDIDATE -> ACTIVE -> PAUSED -> TERMINAL\` states.
- Terminal causes distinguish direction change, flat boundary, timeout unknown, and interruption.
- Initial timer parameters are explicitly hypotheses: 90 second pause and 10 minute end.
- Schedule-like support requires at least four observations spanning at least two minutes.
- Labels are \`NATIVE_CONFIRMED\`, \`SCHEDULE_LIKE\`, \`DIRECTIONAL_SEGMENT_UNKNOWN\`,
  and \`INVENTORY_OR_MIXED\`.
- Concurrent/native program accounting preserves executed quantity after termination/cancellation.
- Annotation state does not modify ER1 quantity/state/flow or voting budgets.

## Verification

- Ruff: PASS.
- \`ty check src\`: PASS.
- ER2 focused tests: **9 passed**.
- Full engine suite: **97 passed**.

Fixtures cover native program termination, native linkage, pause/timeout timing, schedule support guard,
opposite flow, exact-flat boundaries, interruption, mixed market-making-like flow, terminal immutability,
and missing metadata.

## Qualification boundary

The local recorded source stream had no traffic during ER0 inspection, and public trades do not expose
native TWAP/OID metadata. Therefore live precision/recall for \`NATIVE_CONFIRMED\` or schedule-like
classification is **BLOCKED**, not fabricated. The tests are outcome-blind behavioral fixtures; no
future-return result was used to tune segmentation.

Iceberg/hidden-size classification is unavailable without qualified order/depth evidence.
