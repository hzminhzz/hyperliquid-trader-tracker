# ADR-0001: Evidence-driven operating model over the KonScanner fork

**Date:** 2026-10-06. **Status:** adopted as the target design in the requested documentation revision; runtime implementation and qualification are pending. This records design direction, not operational authority.

## Context

The user requires KonScanner/hyperliquid-trader-tracker as infrastructure, has forked it, and wants an expanding human-expert ensemble that an agent can understand and operate with minimal wasted work. The inspected implementation is a notification-first, memory-state tracker. Its public-feed/reducer architecture is useful, but chat subscriptions, inline notification delivery, volatile dedup/state, and silent reconciliation are insufficient as the sole evidence boundary for a replayable ensemble. See [STATUS](../STATUS.md).

The earlier plan described components and a durable event sink, but did not fully specify atomicity, coherent inspection, command authority, or how operational learning accumulates. Simply adding MCP wrappers or more prose would expose those gaps rather than hide them behind a usable interface.

## Decision

Keep the upstream-tracked Rust fork as the authoritative observation owner and the new Python application as the interpretation/advisory owner. Use one tower of versioned products from receipt through account view, posture, independent support, consensus, advice, and command receipt. Every inference retains its evidence and uncertainty.

Deepen the observation interface into durable receipts/checkpoints plus snapshot/tail. Preserve the existing reducer where correct. Decouple Telegram and slow enrichment from ingestion and make the desired wallet universe independent of subscribers. Qualify observed implementation gaps before building dependent research.

Provide one inspect/explain/evaluate/plan/apply vocabulary, with thin CLI/API/MCP adapters generated from the same executable contracts. Proposals are not effects. Deterministic handlers enforce permissions, budgets, revisions, idempotency, and postconditions. Keep the agent outside the per-fill and safety-critical loops.

Use two independently owned local transactional stores initially, plus immutable research exports. PostgreSQL is conditional on measured concurrency or multi-host need. The engine never writes directly into the tracker's database or duplicates its exchange-position reducer.

Accretive improvement means verified incidents become regression fixtures, experiments become reproducible evidence, and accepted choices become versioned artifacts. It does not mean accumulating unbounded instructions or automatically promoting an agent's hypotheses.

## Alternatives considered

| Alternative | Attraction | Reason not selected |
|---|---|---|
| Extend the Telegram bot into the whole product | Fast initial UI and one repository | Couples delivery, expert policy, and source accounting; cannot support a reliable machine boundary through message parsing |
| Rewrite all ingestion in Python | One implementation language | Discards the user's required infrastructure and increases accounting/recovery work without evidence of benefit |
| Add a WebSocket-only event sink | Small fork patch | Cannot recover missed downstream delivery or make volatile state/outbox updates crash-atomic |
| Add many generic MCP tools over internal modules | Immediate tool availability | Forces agents to know ordering, internals, and implicit side effects; multiplies incompatible control surfaces |
| Adopt microservices, brokers, and PostgreSQL immediately | Familiar scaling pattern | Higher deployment/operational cost before demonstrated demand; does not by itself establish truthful state |
| One giant prompt or knowledge database | Easy to keep adding context | Does not enforce correctness and increases cold-start load; evidence should be retrieved by exact dependencies |

## Consequences

The fork patch is larger than an output adapter alone because observation acceptance and export must share a tested durability model. Keep changes narrowly located and retain upstream attribution/license. Two languages remain, but cross-language behavior is constrained by shared versioned contracts and conformance fixtures.

New inspection and replay work precedes advanced ranking. This delays sophisticated scoring but makes its results auditable and cheaper to iterate. Runtime learning is bounded by explicit promotion gates. Missing data may result in no actionable advice, which is preferable to manufactured certainty.

Future financial execution requires a separate specification and explicit authority. Repository edits, observation access, and this ADR do not authorize real trades.

## Revisit triggers

Revisit storage when measured writers, storage volume, or recovery requirements exceed the qualified local envelope. Revisit process boundaries only when interface/operations evidence shows a bottleneck. Revisit normalization/clustering through controlled research, not through infrastructure refactors. Preserve this record and supersede it explicitly rather than rewriting its historical rationale.

## Related authorities

[SYSTEM](../SYSTEM.md) · [CONTRACTS](../CONTRACTS.md) · [OPERATIONS](../OPERATIONS.md) · [plan](../../plan.md)
