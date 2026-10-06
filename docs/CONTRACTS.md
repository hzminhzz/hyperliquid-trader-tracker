# Information and durability contracts

**Status:** normative target; implementation belongs to [plan.md](../plan.md). Terms are in [CONTEXT](../CONTEXT.md). This is the single authority for cross-module data semantics.

## Proposed execution-contract amendment, 2026-10-06

[EXECUTION-REVIEW](EXECUTION-REVIEW.md) identifies live call-site gaps against C1-C2; passing ledger-unit tests does not discharge those gaps. Before richer scoring, qualify receipt/commit failures and the snapshot/stream cut with both watched counterparties preserved. Python must not silently replace the authoritative account view through an independent snapshot path.

The proposed evidence extension preserves additive quantity deltas and full instrument identity, with optional observed order/native-parent links and versioned inferred segments. Missing metadata remains unknown. Segment classification never controls whether an economic fill is retained or whether raw state/flow can be produced. Virtual flip legs are disjoint allocations of the real fill, not additional executed quantity.

Every derived flow cut names event-time bounds, knowledge visibility, coverage, normalization anchors and window expiry. Unavailable normalized events remain missing, not known-zero. Late backfill or enrichment can produce a new current revision but cannot rewrite prior as-known outputs. Common-cut ensemble contributions replace prior contributions; they do not append votes for each child order. These extensions are proposed, not yet implemented or qualified.

## C1. Identity, time, and units

All products carry `schema_version`, `producer_revision`, `environment`, a stable ID, and provenance references. IDs are opaque and globally scoped; they must not be inferred from display labels.

- Instrument identity includes venue, network, market type, dex namespace, and source instrument key. Human ticker and broker mapping are separate versioned metadata.
- Wallet addresses are canonicalized without changing their identity. Watchlist membership has its own generation and effective/knowledge times.
- Economic trade identity includes source environment, instrument scope, source block/event time, and `tid`; per-wallet projections add wallet and leg identity. Verify field mapping against live payloads. `tid` alone is neither globally unique nor a monotonically increasing offset.
- Preserve a receipt identity for every accepted source delivery; duplicate receipts can point to one economic event. Different payloads claiming the same economic identity produce a conflict diagnostic, not silent overwrite.
- Record `event_time`, `received_at`, and `known_at` separately. Unavailable source time is null, not fabricated. Use an injectable clock and recorded timer ticks for deterministic replay.
- Cross-language money, prices, quantities, and fractional weights use decimal strings plus currency/unit metadata. Round to venue/broker increments only at the appropriate boundary. IDs that may exceed JavaScript's safe integer range use strings.
- A replay pins code, dependencies, schema, policy, universe, model, clock behavior, and input manifests. Ordering and tie-breaking are deterministic and specified; a sorted `tid` is not a source ordering guarantee.

## C2. Durable observation owner

The KonScanner Rust process is the sole writer of its observation ledger. Keep the existing pure `apply_fill` semantics where correct; surround them with durable ownership rather than replacing the reducer.

Required persistence transaction model:

1. Validate and durably append the relevant source receipt before declaring it accepted. Preserve enough original payload to reparse and reproduce its interpretation.
2. The reducer worker reads accepted receipts after its committed checkpoint. For each logical unit, atomically record economic deduplication, the new durable account revision/checkpoint, all emitted events, and the consumed-receipt checkpoint.
3. Publish the committed state to the memory cache and wake tail consumers only after that transaction commits. After failure, reconstruct the cache from the durable checkpoint plus unprocessed receipts.

An alternative storage layout may be chosen only if it demonstrates the same crash properties through the public test interface. A memory update followed by an unrelated outbox insert does not satisfy this contract.

Flip legs belong to one source change and are committed together. A public trade touching two watched wallets preserves both wallet projections; deduplication of the source trade cannot suppress its second counterparty.

Delivery is at-least-once. Idempotent consumers provide exactly-once application of an accepted logical event within their own transactional projection. Do not claim exactly-once network transport or complete exchange history.

Bound all queues, frame sizes, retry schedules, and storage. Durable batching is allowed with an explicit commit latency and crash-loss boundary; only committed receipts count as accepted. If persistence is unavailable, stop publishing trusted state, expose degradation, and record recovery coverage. A WebSocket cannot backpressure the exchange into retaining every missed trade for this application.

## C3. Canonical event family

| Family | Meaning | Downstream use |
|---|---|---|
| `OBSERVATION` | Accepted source receipt, with economic identity where present | Audit and infrastructure replay; not a direct consensus input |
| `POSITION_CHANGE` | Verified reducer transition: open/add/reduce/close or correlated flip legs | Replace the expert account projection at the given revision |
| `ACCOUNT_SNAPSHOT` | Validated scoped positions, equity, and freshness | Seed/refresh the account view, without inventing trading activity |
| `RECONCILE_RESULT` | Applied, unchanged, skipped-race, failed, or correction | Revise state/health and retain the reason and before/after diff |
| `MARK_OBSERVATION` | Typed mark/mid/oracle price at a known time | Valuation with explicit price provenance |
| `COVERAGE_CHANGE` | Gap, recovery interval, admitted scope, or expired evidence | Admission gates and historical validity |
| `HEALTH_CHANGE` | Transport, state, persistence, or delivery transition | Operational action, not a trader opinion |
| `UNIVERSE_CHANGE` | Desired/admitted/removed wallet or instrument membership | Point-in-time eligibility; not an economic position exit |

Every state-affecting event includes before/after revisions, the affected scope, cause references, state validity, and available account context. Publish typed positions and snapshots even with no Telegram recipients.

Position data distinguishes quantity, average entry, valuation price type/time, observed-since time, actual opened-at time if known, and economic PnL provenance. Equity includes currency, account/dex scope, source field, and observation time. An account-wide value is not assumed to cover every vault, subaccount, spot balance, or builder dex.

Fees/funding and verified realized PnL can arrive as later enrichment. Keep estimated gross PnL distinct from verified exchange PnL and net-after-cost PnL. Append enrichment revisions; do not alter an already-published decision or block ingestion waiting for enrichment.

Example envelope, illustrative only:

```json
{
  "schema_version": "1",
  "event_id": "ledger-a:1042",
  "cursor": {"ledger_id": "ledger-a", "seq": "1042"},
  "kind": "POSITION_CHANGE",
  "environment": "mainnet",
  "instrument_id": "hyperliquid:mainnet:perp:default:BTC",
  "wallet_id": "wallet-example",
  "event_time": "2026-10-06T01:00:00.000Z",
  "received_at": "2026-10-06T01:00:00.050Z",
  "known_at": "2026-10-06T01:00:00.055Z",
  "cause_ids": ["receipt-example"],
  "before_revision": "17",
  "after_revision": "18",
  "payload": {"transition": "ADD", "quantity_before": "0.50", "quantity_after": "0.75", "quantity_unit": "BTC"},
  "quality": {"current_state": "valid", "history": "partial"}
}
```

## C4. Snapshot and tail protocol

The minimum machine interface is snapshot plus ordered tail; an optional stream only reduces wake-up latency. All reads are bounded and paginated.

- Snapshot is produced from one committed ledger cut and returns `ledger_id`, `snapshot_id`, `last_seq`, admitted universe revision, state revisions, and quality/coverage. Paginated snapshot pages remain pinned to that cut; an expired snapshot token forces a restart, never silent drift.
- Tail returns events strictly after an opaque cursor, with an explicit next cursor, page limit, high-water mark, and retention floor. A filtered tail must still advance its global scan cursor when it finds no matching events.
- The engine installs a snapshot and consumes its tail without overlap or gaps. On restart it resumes from its own committed consumer offset.
- The engine atomically persists projected account state, applied event identities, downstream decision references as appropriate, and the new consumer offset. A process crash between projection and acknowledgement cannot apply a delta twice.
- Ledger recreation changes `ledger_id`. Unknown/expired cursors return typed `CURSOR_EXPIRED` or `LEDGER_CHANGED`; the engine explicitly reboots from a snapshot and preserves the uncovered interval.
- Archives become durable before acknowledged retention is advanced. Retention tracks registered consumer leases; abandoned consumers cannot force unbounded disk growth. Eviction publishes the new floor and forces a snapshot, rather than pretending the missing tail exists.

A source snapshot is not this protocol's local ledger snapshot. A local committed cut is precise; a source account response and public trades may lack a demonstrated common boundary.

## C5. Source seeding and reconciliation

Use a desired-universe registry independent of chats. Admission proceeds through requested -> seeding -> observed -> eligible, with quarantine and retirement alternatives. Unknown positions do not enter consensus as zeros.

Before applying a source snapshot, validate its envelope and required fields, instrument scope, values, and response timing. Missing `assetPositions`, invalid equity, or malformed rows cannot silently erase a live portfolio. Distinguish valid empty state from invalid input.

Record request-start revision, watchlist generation, source timestamp where meaningful, response-receipt time, and outcome. Use a durable account revision separate from the inherited fill epoch. Do not reuse or reset revisions on reseed/removal. Concurrent snapshots must have a deterministic acceptance rule; stale work cannot overwrite a newer accepted revision or re-admit a retired wallet.

For startup/recovery, buffer relevant receipts while fetching snapshots and bounded fills history. Apply a snapshot/tail join only when a tested source boundary or validated matching procedure establishes non-overlap. Receipt time alone and 'apply every newer-looking trade' do not prove this. Otherwise preserve an explicit gap, restore current state, and withhold unsupported historical or transition claims.

Reconciliation returns `APPLIED`, `UNCHANGED`, `SKIPPED_RACE`, `INVALID`, or `FAILED`, with a state diff and evidence. A skipped result must not refresh the last-applied-reconcile timestamp. Repeated races have a bounded retry/budget policy and an explicit degraded outcome, not an infinite loop.

Preserve complete observed lifecycle metadata when a validated reconciliation is unchanged. Seeded unknown entry times remain unknown. If a correction changes quantity, emit `RECONCILE_RESULT` with replacement state and reason; do not fabricate an entry, exit, or trader-quality observation.

## C6. Validity and uncertainty

Keep separate dimensions: transport liveness, state freshness, reconciliation status, historical completeness, instrument coverage, and delivery health. Inactivity in a wallet is not by itself stale state; freshness depends on feed continuity and successful state validation.

A worldview includes an input-cut vector, evaluation time, policy/model hashes, and a dependency list. The engine accepts only states allowed by the policy's skew/freshness bounds. Out-of-date equity can block posture while an observed position remains valid for display.

Coverage loss makes an affected advisory non-actionable. Do not silently re-normalize missing expert weight onto survivors. Preserve last valid targets for explanation, clearly expired; whether to alter a real account is controlled separately by the account's independent risk policy.

## C7. Evidence and interfaces

Use one shared result envelope for CLI/API/MCP: request ID, schema version, status, scope, revisions/watermarks, concise summary, typed result, reason codes, evidence handles, pagination, truncation indicator, and cost/budget usage. Machine outputs must not require parsing prose. Human text is a rendering of the same result.

Evidence handles resolve to content-addressed artifacts with producer version, parent IDs, validity scope, access classification, and retention policy. A decision explanation contains exact arithmetic contributions and applied constraints. 'Confidence' is not a probability unless a separate calibration artifact supports that interpretation.

Schema evolution is additive within a compatible version; semantic changes require a new version and migration fixture. Unsupported major versions stop interpretation with a typed error. Generate future JSON Schema, CLI help, examples, and thin MCP definitions from the same executable contracts once implemented; prose alone is not schema enforcement.

## C8. Security and replay boundary

Bind machine interfaces locally/private by default; authenticate and authorize any remote exposure. Observation credentials never include exchange signing authority. Redact secrets from evidence and error payloads. Validate untrusted labels and source text as data, including when rendered in Telegram or returned to an agent.

Replay uses isolated storage and an effect-disabled environment: no Telegram sends, broker writes, live configuration edits, or credential inheritance. Live effects require an explicit, separate adapter plus authority. Runtime side effects are controlled by [OPERATIONS](OPERATIONS.md), not by source-event content.

## C8. V2 scoring and signal artifacts

V2 introduces an interpretation seam above the authoritative account projection. These artifacts are owned by the Python expert-engine and never become source truth for exchange accounting.

**WalletEvidence** MUST preserve raw unclipped equity bias and any bounded influence transform as separate fields. It also carries portfolio share where observable, position/intent/observation ages, the most recent verified intent event, multi-horizon intent flow, reliability/missing reasons, input state revisions, `as_of`, `knowledge_time`, and evidence references. Reconciliation, mark-only change, and equity-only change cannot be encoded as OPEN/ADD/REDUCE/CLOSE/FLIP intent.

**IndependenceArtifact** carries feature/model revision, training cutoff, pair support diagnostics, cluster membership, cluster budgets, effective breadth, unknown-similarity/newcomer treatment, and assignment effective time. As-known replay cannot use an artifact trained on later behavior.

**CohortArtifact** carries lagged cohort membership, support/shrinkage diagnostics, training/evaluation cutoffs, effective interval, and drift flags. Alpha/control/anti-alpha labels are research interpretations, not permanent wallet identities.

**EnsembleEvidence** carries state evidence and flow evidence separately, relative-conviction summaries, independent breadth, supporting/opposing clusters, missing information, optional skill/market divergence and crowding/context values, causal changes, and V1/simple-baseline comparators. It is descriptive until a promoted model explicitly consumes selected fields.

**PredictiveEvidence** contains only feature families that passed the declared research gate for its model revision. It binds the frozen feature schema, training cutoff, calibration revision if any, latency/cost assumptions, historical-support handles, current values, and support/uncertainty. An unpromoted descriptive feature cannot silently enter this artifact.

**TradeSignal** is account-independent and contains `signal_id`, instrument, horizon, state (`FLAT|LONG|SHORT`), event (`ENTER|INCREASE|REDUCE|EXIT|REVERSE|NONE`), signal strength, structured confidence/support, optional expected return, crowding risk, supporting/opposing clusters, missing information, causal changes, invalidation conditions, baseline comparators, evidence refs, feature/model/policy/universe revisions, `as_of`, and `knowledge_time`. `expected_return` MUST be absent/null until a calibration qualification exists for that horizon and assumptions.

**OutcomeRecord** is appended only after the future horizon becomes knowable. It references the immutable original evidence/signal and records declared-latency prices, forward returns, cost-adjusted returns, MFE/MAE, context/coverage at emission, correction/restatement flags, and outcome knowledge time. Attaching an outcome can never alter the original decision or its as-known features.

All V2 artifacts use deterministic serialization/hash rules for replay equivalence. Model or policy changes create new artifacts and decisions rather than rewriting prior ones. See [RESEARCH](RESEARCH.md) and [QUALIFICATION](QUALIFICATION.md).
