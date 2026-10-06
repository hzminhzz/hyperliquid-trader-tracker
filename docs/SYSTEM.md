# System design: an evidence-driven trader ensemble

**Status:** target design, not implemented. Read [STATUS](STATUS.md) for inspected reality. This design retains the approved KonScanner fork as mandatory infrastructure.

## Purpose and optimization order

Turn a user-curated set of wallets into understandable, reproducible, account-aware advice. Make an agent effective by giving it a small, truthful model of the whole system and bounded ways to improve it, rather than requiring it to infer state from logs and unrelated tools.

Optimize in this order: correctness of observation; explicit uncertainty; replayability; bounded authority; useful decisions; operator effort; resource cost. More autonomy, more trades, more tracked wallets, or more abstractions are not success metrics by themselves.

The operating loop is:

```text
observe -> establish what is known -> explain the target -> propose a change
    ^                                                        |
    |                                                        v
retain validated lesson <- verify outcome <- apply authorized bounded change
```

Agents supervise the slow operating/research loop. Deterministic code owns ingestion, accounting, consensus calculation, constraints, and authorization. There is no LLM call per fill.

## One tower of linked abstractions

| Level | Product | Question answered | Responsibility hidden behind its interface |
|---|---|---|---|
| L0 | Observation ledger | What did we receive, from where, and when? | Reconnects, identities, receipts, persistence, coverage |
| L1 | Account view | What position and equity can we support now? | Reducer, snapshot races, corrections, scope, reconciliation |
| L2 | Expert posture | What comparable exposure does this expert express? | Eligibility, normalization, lagged scale, reliability |
| L3 | Independent support | How much of this information is redundant? | Similarity estimation, clustering, membership stability |
| L4 | Consensus target | What does the ensemble imply, and why did it change? | Contributions, fixed budgets, aggregation, hysteresis |
| L5 | Advisory target | What does that imply for this account under explicit constraints? | Contract mapping, risk model, existing risk, account rules |
| L6 | Proposal and receipt | What may the operator change, and what actually happened? | Authorization, preconditions, idempotency, verification |

Each product carries its input cut, policy/model revision, scope, validity, limitations, and evidence references. A higher level may narrow or invalidate admissibility; it cannot erase lower-level uncertainty. A target is inspectable through its exact dependencies, not through reconstructed explanations generated later by an LLM.

**Information is not authority.** External observations can update knowledge but cannot issue commands. An attractive backtest cannot grant permission to trade. A successful command receipt is not proof of a profitable strategy.

## Five deep modules; two runtime processes

1. **Observation ledger, Rust/KonScanner.** Own raw watched-event receipts, position reduction, equity/scope observations, health, checkpoints, and a durable ordered publication stream. Hide transport and recovery mechanics behind snapshot/tail/inspect operations. Preserve the upstream reducer and public-feed design, adding narrow verified infrastructure changes.
2. **Ensemble, Python.** Consume authoritative observation products idempotently. Own expert posture, versioned similarity/cluster artifacts, consensus, and contribution explanations. It must not maintain a second competing exchange-position reducer.
3. **Account adviser, Python.** Transform consensus into an optional, bounded account-specific target. Missing broker/account/rule data yields informational output, not a guessed executable quantity.
4. **Evidence/replay, shared contract.** Persist enough exact inputs and versions to reproduce decisions. Run the same deterministic logic in live, replay, and shadow modes, with external writes disabled in replay.
5. **Operations facade.** Expose inspection, explanations, bounded jobs, and typed changes over the preceding modules. CLI, HTTP, and later MCP are thin adapters to the same handlers and schemas; they are not separate control implementations.

```text
Hyperliquid public feeds + account reads
                  |
    Rust KonScanner observation process
      receipts -> reducer/checkpoint -> durable stream
                  |
        snapshot + ordered tail contract
                  |
    Python ensemble/advisory process
      expert state -> support -> target -> account constraints
                  |
      explanation + decision + optional Telegram alert

Operations facade -> scoped commands to each owner -> command receipts
Evidence lineage <- every accepted input, artifact, decision, and receipt
```

A module is a responsibility, not necessarily a package or deployment. Keep the Python side a modular monolith. Do not add Kafka, a vector database, a general agent framework, or a service per box.

## Storage and repository boundaries

The tracker remains an upstream-tracked fork. Keep Rust infrastructure changes local to observation and export. The new Python engine belongs in a separate application repository when implementation reaches S3 (`/home/quant/dev/hyperliquid-expert-ensemble`); do not disguise it as the inherited `src/tracker/*.py`. This fork temporarily holds the cross-system specification; migrate each document's ownership once the engine repository exists, leaving pointers rather than duplicate specifications.
Use the tracker's SQLite store for a durable receipt/checkpoint/outbox design with one logical writer. Python initially uses its own SQLite database for projections, consumer offsets, policies, and decisions. Do not have two processes write the same database. Export sealed historical partitions to Parquet for DuckDB/Polars research. PostgreSQL is an upgrade for measured concurrent-write or multi-host needs, not a prerequisite for an expert count.

**This revises the earlier mandatory-PostgreSQL proposal.** The important commitments are transactional ownership and replay, not a database brand. Keep engine repositories/storage independent of the tracker implementation behind the published contract.

Retain watched economic observations and the market/equity inputs actually used. It is not necessary to archive the entire all-market firehose indefinitely. Unwatched messages can be filtered with bounded diagnostics; newly added wallets do not magically gain historical coverage.

## Coherent worldview, not a pile of latest values

The agent's default view must show six distinct things: desired configuration, applied configuration, observed facts, inferred targets, unapplied proposals, and authorization. Bind all displayed calculations to explicit compatible watermarks; show unavailable fields instead of combining inconsistent cuts.

A compact inspection answers:

- Which code, policy, universe, and artifacts are deployed versus desired?
- What scope is covered, how fresh is it, and what is uncertain?
- What changed since the last inspection and which targets are affected?
- What is blocked, what evidence proves it, and which bounded action can resolve it?
- What budgets remain, which jobs are running, and what permissions are available?

Drill down by stable IDs. Summary results contain short explanations and evidence handles; raw payloads are fetched only on demand. Common checks are cached against revisions, never against an unqualified wall-clock TTL alone.

## Causal dependency and invalidation

The dependency path is explicit: receipt/snapshot -> account revision -> posture -> cluster/model revision -> consensus decision -> advisory -> command receipt. Record edge IDs and content hashes. Changing a policy, correcting a position, expiring equity, or changing universe membership invalidates the affected descendants and reusable caches.

Use scoped invalidation: a corrupt SOL wallet observation should not force every BTC calculation to rerun. Conversely, an account-wide equity correction can affect that wallet's entire posture vector. A new cluster artifact can affect all targets in its declared instrument/horizon scope. Corrections create new decisions; published history is never silently overwritten.

## Healthy operation and degraded operation

Fresh state is not equivalent to complete history. Track separately transport health, accepted state freshness, historical coverage, delivery health, and economic-evidence confidence. No single green badge can conceal a missing dependency.

Unavailable expert weight is retained as an unknown budget, not redistributed automatically among healthy experts. Missing information must not manufacture a stronger trade signal. An explicit flat observation can reduce a posture; an outage cannot masquerade as a trader closing. Independent local account risk controls remain active even when advice is blocked.

The recorder should continue during notification pauses and research jobs. Safety-stop and recovery reads take priority over analytics. Overflow or storage exhaustion is explicit degradation, not silent event dropping.

## Agent-accretive development

A useful lesson progresses from observation -> reproducible evidence -> regression fixture or benchmark -> reviewed rule/artifact -> measured improvement. Keep unsuccessful experiments and limitations, not just winners. Learned weights and operating heuristics require promotion gates; they do not become policy merely because an agent wrote them down.

Accretion should reduce future work: reuse validated fixtures, content-addressed datasets, incremental sufficient statistics, cached explanations, and one canonical command vocabulary. Periodically retire superseded material. A large memory file is not a substitute for a small entrypoint plus retrievable evidence.

The measurable standard for ergonomics is in [QUALIFICATION](QUALIFICATION.md); interface details are in [OPERATIONS](OPERATIONS.md) and [CONTRACTS](CONTRACTS.md).
