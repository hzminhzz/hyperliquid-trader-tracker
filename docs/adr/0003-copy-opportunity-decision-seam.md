# ADR-0003: Immutable copy opportunities as the agent's decision seam

**Date:** 2026-10-13
**Status:** Proposed architecture; not approved for production and not implemented

## Context

The deployed Rust observation ledger and Python ensemble collect trade/position changes and as-known one-minute consensus cuts. The source change stream includes far more ADD/REDUCE transitions than OPEN/CLOSE and the raw outcome recorder signs its markout by quantity delta. A long reduction can therefore look like short alpha if mistakenly treated as a copy entry. Repeated consensus snapshots and Telegram deliveries overlap and do not provide independent profitable trade counts. Existing ER1–ER6 code validates many representation/qualification mechanics but the production policy remains descriptive. An OOM incident caused a substantial missing-scoring interval; expanding the model before restoring resource integrity risks unreliable historical evidence.

The agent requires a small, truthful, actionable view of what it knew, when and why, linked to outcomes and negative controls without navigating every internal module.

## Decision

Introduce an immutable **CopyOpportunity** as the first available follower decision between descriptive WalletEvidence/EnsembleEvidence and any qualified TradeSignal. Keep three distinct representations: economic source event, optional execution annotation, and as-known follower opportunity. Store frozen original decision cuts separately from future outcomes and later restatements. Child fills and unchanged minute cuts cannot manufacture new independent opportunities. Normalize raw state/flow at compatible anchors; optional segments do not gate valid source exposure.

Evaluate follower decisions from first actionable time, with costs/latency and matched same-asset market-only baselines. Separate economic wallet returns, execution-flow markouts and transferable incremental alpha. Fail closed on unavailable execution quotes, stale source, missing coverage or unqualified policies. The agent orchestrates inspection, explanations, bounded science and typed change proposals; deterministic runtime alone owns source accounting, inference state, promotion gates and advisory delivery.

Retain the two-process Rust/Python ownership split. Improve source integrity and resource controls before pushing more data through the runtime. Preserve replayable canonical observations and sparse frozen decisions; rebuild common market-time comparator grids in bounded scientific jobs. Never treat a proposed interface as a deployed command.

## Alternatives and rejected trade-offs

- **Static consensus as entry:** cheapest change; fails to distinguish stale capital from new information and often proxies market trend.
- **One trigger per raw fill:** easy logging; inflates samples and mislabels reductions as opposite entries.
- **Mandatory inferred strategy/TWAP hierarchy:** appealing narratives; uncertain and source-dependent and needlessly blocks additive exposure research.
- **Telegram as a signal source:** low implementation cost; loses delivery-state, exact knowledge cut and independent evidence lineage.
- **A new agent/microservice/broker per stage:** clearer diagrams, but adds failure and maintenance seams with no demonstrated leverage.
- **Optimize historical positive outcomes first:** fast apparent wins; creates selection, overlap, leakage and unrepeatable claims.

## Consequences

Requires a precise opportunity identity/expiry and state machine, durable as-known knowledge cuts, causal pricing/latency research and source-provenance support. Must retain existing baselines and old observational outcome data under their correct labels, not silently migrate it to directional copy performance. Adds a deliberate shadow-only period and honest negative/unknown decisions. Positive trade performance is neither assumed nor authorized. Data-retention changes and any live financial effects require separate scoped authorization.

## References

[DECISION-INTELLIGENCE](../DECISION-INTELLIGENCE.md), [SYSTEM](../SYSTEM.md), [CONTRACTS](../CONTRACTS.md), [RESEARCH](../RESEARCH.md), [QUALIFICATION](../QUALIFICATION.md), [plan](../../plan.md), ADR-0001 and ADR-0002.
