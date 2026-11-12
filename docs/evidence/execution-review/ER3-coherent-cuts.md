# ER3 coherent cuts and bounded influence

Date: 2026-10-06  
Issue: #27  
Status: **ENGINEERING_PASS / LIVE-COVERAGE-MEASUREMENT-BLOCKED**

## Implemented

- Deterministic aligned one-minute evidence cuts carry event/knowledge cut, source watermark,
  source revision, normalizer revision, independence revision, feature revision, universe revision,
  and cause.
- Material interrupts are coalesced per instrument/cluster with a three-second initial candidate
  delay. Later revisions replace earlier pending contributions; they do not append extra votes.
- WalletEvidence retains raw normalized flow; EnsembleEvidence bounds each wallet flow to [-1, 1]
  before applying fixed cluster budgets.
- Missing normalized flow remains missing mass rather than numeric zero.
- Missing wallets do not renormalize surviving cluster budgets.
- Existing lagged independence artifacts and clone grouping remain immutable inputs to a cut.

## Verification

Companion engine:
- Ruff: PASS.
- \`ty check src\`: PASS.
- ER3 focused tests: **4 passed**.
- Full engine suite: **101 passed**.

Fixtures cover deterministic cut IDs/revisions, queued OPEN->CLOSE style replacement, leveraged-flow
budget bounding, and fixed missing mass without survivor amplification. Existing V22/V24 suites still
cover clone grouping, complete-link behavior, low-support newcomer pools, future-information rejection,
and lineage explanation.

## Empirical boundary

The existing local observation ledger was empty at ER0 inspection, so this issue cannot honestly
measure live 50-versus-120 wallet coverage, latency, supported behavioral breadth, or interrupt load.
No wallet roster is fabricated or committed to the public repository. Those operational measurements
remain **BLOCKED** until authorized recorded traffic and the private prospective universe manifest are
available.

This is a descriptive integration gate only. It does not establish predictive value, a superior clock,
or financial authority.
