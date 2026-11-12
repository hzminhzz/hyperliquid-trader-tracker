# Decision intelligence: an agent-operable copy-opportunity system

**Design revision:** 2026-10-13. **Status:** proposed target contract and research program; NOT evidence that the live runtime implements or has qualified it. This document is authoritative for the new **copy-opportunity** seam. The existing [SYSTEM](SYSTEM.md), [CONTRACTS](CONTRACTS.md), [RESEARCH](RESEARCH.md), [OPERATIONS](OPERATIONS.md), [QUALIFICATION](QUALIFICATION.md), and root [plan](../plan.md) remain authoritative for their respective domains. Prior ER0-ER6 evidence stays historically true; no predictive or financial authority is conferred.

## D0. The system objective

**Maximize independently verified, executable decision information per unit of observation, compute, time, operational effort and financial risk.** Do not maximize watched-wallet count, database rows, Telegram volume, synthetic test counts or how favorable the historical chart looks.

The system is a **closed learning loop**, not a bot that averages positions:

1. **Observe** source facts, with coverage and separate event/receipt/knowledge times.
2. **Reconstruct** attributable signed economic exposure changes and the current supported state.
3. **Recognize** bounded, versioned candidate copy opportunities only when a follower could actually have known about them.
4. **Freeze** each opportunity's observable information, policy, independent support, alternative baselines and availability/latency assumptions.
5. **Evaluate** its eventual outcomes after valid prices arrive: economic source-trader outcome, copyable return and incremental forecast value are different questions.
6. **Learn** which wallet/coin/action/horizon/regime patterns have demonstrated *lagged* transferable value, shrinking unsupported groups to UNKNOWN.
7. **Decide** under an explicitly promoted policy, or abstain and explain why.
8. **Explain and improve** with evidence handles, failures, missingness, drift and bounded agent actions. Any policy change creates a new revision; past decisions remain immutable.

The most important optimization is *time to falsifiable answer*: reusable causal decisions and controls beat another week of correlated one-minute labels. No design can manufacture independent observations or a regime-diverse holdout from five days of runtime.

## D1. One authority graph, not more microservices

```text
Hyperliquid public stream + qualified account reads
     |
     v
Rust KonScanner [source authority]
  accepted receipts + trade identities
  position/equity/checkpoint/reconciliation + coverage
  durable canonical ledger: source event, receipt, known-at
     |
     +---- immutable ledger cursor / observation correction
     |
     v
Python engine [interpretation authority]
  A. additive wallet exposure path (always available when qualified)
  B. optional OID/native-program/inferred segment annotations
  C. frozen decision-point / copy-opportunity journal
  D. outcomes and matched controls
  E. lagged wallet/cluster evidence and qualified policy
     |
     +---- inspect / explain / evaluate / propose / apply [bounded agent facade]
     |
     +---- Telegram [thin human adapter, not a data source]
     |
     X no direct broker execution in V1/V2
```

Two existing independently owned processes and their stores remain the default; logical modules do not imply independent deployments. The Rust owner is never bypassed by a second Python position reducer or by scraping Telegram. Native exchange coverage, instrument namespace and missingness remain scoped and explicit. A new source adapter is introduced only when an existing source demonstrably cannot supply a necessary observed fact.

### Three different truths

- **Source truth:** an exchange event or account observation, including contradictions and uncertainty.
- **Analytic truth:** an interpretation computed from specific source revisions and a declared rule; not proof of trader belief.
- **Decision truth:** what our system could know and act on at a particular information cut; never rewritten by later enrichment.

A later source correction creates a restated research view and a separate audit of the original as-known view; it never improves the original copy opportunity retrospectively.

## D2. The minimum deep interfaces

Expose five deep interfaces, hiding storage, replay, and model plumbing:

| Interface | Main question | Returns and invariant |
|---|---|---|
| `observe.snapshot_tail(cut)` | What was observed and when? | Durable cursor, identity, scope, coverage and corrections; one state authority |
| `behavior.explain(wallet, instrument, cut)` | What objectively changed? | Exact signed path, lifecycle, raw net/gross flow, exposure, unknowns, optional annotations |
| `opportunities.list/get(cut, filters)` | What NEW follower decision became available? | Frozen opportunity state, provenance, first actionable time, lineage and reason for abstention |
| `evaluation.compare(manifest, controls)` | Did that available decision beat simple alternatives after costs? | Immutable paired outcomes, effective support, cost/latency sensitivity, negative results |
| `inspect/explain/plan/apply` | What is the system doing, what is blocked, and which bounded change is justified? | One coherent, revision-pinned facade, not direct arbitrary SQL, shell, or an agent in the trading path |

These are **conceptual interfaces**, not claims that equivalent CLI routes have already been implemented. Reuse the existing Python facade and Rust ledger. Do not add wrappers before the underlying contracts work.

## D3. State, action, execution program, and opportunity are different objects

### D3.1 Source and lifecycle

A `POSITION_CHANGE` has signed `quantity_before`, `delta` and `quantity_after` and a mechanically classified `OPEN/ADD/REDUCE/CLOSE/FLIP` where coverage supports it. Mark drift, equity changes, snapshots, funding and reconciliations do not become voluntary direction changes. Forced or unknown causes remain explicit; no unknown state becomes known-flat.

Maintain **one truth-preserving additive path**, with common-anchor normalized flow when equity and price scope are compatible. Preserve raw quantity when normalization is unavailable. The split/merge invariant applies to equivalent timed source paths, not to arbitrary reassignment of genuinely different times.

### D3.2 Optional execution interpretation

OID and exchange-confirmed programs are **observed links** only when the source truly provides them. Timing-derived segments are **tentative annotations** with labels such as SCHEDULE_LIKE and MIXED/UNKNOWN. One strategy may use multiple OIDs, one OID may cross lifecycles, and a market-making wallet can alternate both sides. Detector failure cannot erase exposure facts or create a new independent vote.

### D3.3 Signed flow is not a copy recommendation

**Critical rule:** reducing a long (`delta < 0` while remaining long) is a **REDUCE-LONG**, not automatically a fresh SHORT. Closing a short (`delta > 0`) is not automatically a new LONG. On a FLIP, distinguish the close leg from the genuinely new opposite open leg without double counting raw volume. A de-risking or opposing-flow indicator may be evaluated for reversal value **only as a separate, preregistered hypothesis**. The raw recorder's `sign(delta)` markout remains useful to describe execution-flow impact; it must not be called a directional copy-entry outcome.

### D3.4 First-class CopyOpportunity

An opportunity is the first *as-known* moment the proposed follower could consider a concrete directional action, not every child fill, snapshot, minute, static holding or Telegram resend. It can be `CANDIDATE`, `ACTIVE`, `INVALIDATED`, `EXPIRED` or `CENSORED`. Status transitions are append-only; first publication and first eligible entry are immutable. Subsequent fills or stronger consensus **update the same opportunity**, not increment the number of independent samples. A material opposite decision may end one opportunity and start a new one with distinct linked IDs.

Required fields (contract, implementation to be verified before use):

- Identity: `opportunity_id`, `parent/source_event_ids`, `wallets`, `instrument_id`, `horizon`, `decision_family`, optional `cluster_ids` and observed/heuristic segment refs.
- Knowledge: `event_time`, `received_at`, `known_at`, `decision_cut`, ledger cursor, wallet/market/cluster revisions, historical coverage; no forward information.
- Semantics: supported `source_action`, proposed `copy_direction` or `NONE`, signed net/gross change, prior exposure, new exposure, materiality/novelty, opposing/supporting independent breadth, uncertainty.
- Execution feasibility: `earliest_actionable_at`, observed quote/trade source and its known-at time, latency policy, spread/slippage/funding assumptions, venue mapping and censored/unsupported status. For a Telegram-specific strategy, delivery latency belongs in the actionability clock.
- Policy context: registered candidate/policy ID, prior state, entry/invalidation/expiry conditions, simple comparators, `reason_codes`, human explanation and traceable raw evidence IDs.
- Future labels live **only** in independently keyed outcome records, never in the immutable opportunity row.

Opportunities can also be `NO_COPY` due to closed/unknown state, stale price, insufficient independent support, market-making/hedged behavior, unsupported venue, risk uncertainty or unqualified policy. Recording why we abstained allows unbiased denominators and better agent diagnosis.

### D3.5 Keep both event and time-sampled evidence, at the right cost

Every accepted economic source event remains durable. Emit a decision-point when an objectively defined material change/new cluster information/coverage invalidation occurs. Use periodic **bounded aggregate checkpoints** to observe silent/flat periods, expiration, negative controls and drift. Do **not** perpetually materialize six forward horizons for every unchanged instrument-minute if the evaluation protocol does not need it. A fixed common market-time grid may still be generated *in sealed offline research partitions* for paired comparisons. Never let event-triggered samples win by selecting only favorable times.

## D4. Three evaluation ledgers, no ambiguous PnL

1. **Trader behavior and skill:** what the source wallet did, what we can observe of execution/position lifecycle, and what was missing; not necessarily complete realized PnL. An open/incompletely seeded episode cannot be treated as a fully known round trip.
2. **Follower opportunity markout:** mid-to-mid diagnostic first, then a **latency-aware executable-price or conservative cost model** from the first *actually available* follower action time. Cost once per actual entry/exit/rebalance, not once per unchanged minute. If quote/depth/funding data is unavailable, mark net result HYPOTHETICAL or BLOCKED, never a proven copy fill.
3. **Incremental predictive value:** paired contrast versus contemporaneous same-coin/direction market-only controls (momentum, volatility, price response, and where available OI/funding), plus strong simple baselines. An unusually rising altcoin is not wallet alpha merely because followers and price rise together.

Attach frozen 5m/15m/1h/4h horizons by mechanism and an exploratory 24h/swing horizon; choose one preregistered primary endpoint per study and report the rest as secondary. Always report entry delay, sample support, dependence, effective independent days/episodes/clusters, confidence intervals, market contribution, copy turnover and incremental edge net of costs. Scoring correctness tests do not count as empirical success.

### Opportunity-specific study (initial bounded set)

| Candidate | Trigger at knowledge time | Main counterexample/control |
|---|---|---|
| Fresh accumulation | New or material independent OPEN/ADD, lagged wallet-relative size and non-stale equity | Same-coin comparable momentum/news moves without wallet novelty |
| Skilled de-risking | Lagged-skill cohort REDUCE/CLOSE without new short; separate EXIT warning | Ordinary profit-taking after a rally, not an automatic bearish trade |
| Leader versus follower | Independently established lead/lag behavioral groups trading before/after market response | Clones, public news shock, shared market exposure and post-price chasers |
| Trader specialization | Lagged copyable markout by coin, action, regime, horizon with shrinkage | Source PnL, capital size or selecting winners after seeing the test period |

Start with the first two; defer complex leader/follower learning until sufficient longitudinal support exists. Reject hypotheses honestly.

### Selection and science protections

- Train/update clusters, normalizers, skill cohorts, calibration, candidate thresholds and wallet universes on past data only. Unseen current wallets are UNKNOWN, not "unskilled".
- A wallet is not independent merely because it has a different address; compute effective breadth and cluster/event support, leave-cluster-out and common-shock controls.
- Count eligible negative/no-opportunity periods and explicitly blocked rows; account for selection bias, missingness, censoring and market-wide dependence.
- Group multiple fills/alerts for one opportunity; do not count multiple future horizons or overlapping windows as independent trades. Use chronological purged splits and day/cluster block uncertainty. Do not optimize into the final frozen holdout.
- A historical result that becomes positive only when holding longer in a rising market does not establish added information.
- Never silently promote an exploratory positive result. `DESCRIPTIVE -> RESEARCH_CANDIDATE -> HISTORICAL_PASS -> LIVE_FORWARD_PASS -> APPROVED_ADVISORY` are separate transitions with human authorization where required. Expected return remains null until qualified calibration.

## D5. One agent-ready situation model

An agent should be able to answer in **one bounded inspection**:

- What is actually running and what is implemented versus planned? Name code, schema, wallet universe, candidate/policy revisions and mode.
- Are source, interpreted state, opportunity journal, outcome pricing, qualification and Telegram **independently healthy**? How stale or missing is each?
- What changed economically since the last cursor? Newly opened/added/reduced/closed/uncertain opportunities, affected clusters, and why.
- Which available decisions would matter, and why? Show source action, independent support, previous state, price move *before* detection, latency, ability to trade, blockers and expiration.
- What have we learned? Market-baseline-adjusted, cost-aware evidence by horizon, with negative results, regime support and whether out-of-sample gates remain blocked.
- Which **safe** bounded next action has the greatest expected information gain per API/computation/storage dollar? Show preconditions, evidence, estimated cost and permission; never automatically trade.

Canonical conceptual views, to be exposed through the **existing facade** rather than arbitrary terminal scraping:

- `inspect system`: <6 KiB, one coherent cut; source/state/opportunity/outcome/Telegram health plus active budget, qualified vs proposed capabilities.
- `inspect changes --since cursor`: bounded deltas, gap/coverage changes, new opportunity IDs and economically meaningful state changes, not a dump of minute rows.
- `inspect opportunity ID` and `explain ID`: one **decision capsule** with immutable inputs, source-action semantics, cutoff, quote/latency, support, controls, exclusions, hypothesis/policy version, outcome maturity and why no action.
- `inspect wallet ID`: compact behavioral fingerprint and drift *as of a cut*, no post-cut leakage.
- `evaluate --manifest ID --policy ID --budget ID`: asynchronous only as an explicit scheduled/authorized bounded research job; deterministic outputs and resumable checkpoints; read-only default.
- `plan/apply` retain typed, versioned scoped authority, idempotency, expected-revision and receipts. Recovery may pause unsafe actionable alerts, never change risk or switch on execution.

Agent interfaces MUST reveal implemented capabilities; these view names are proposed extensions, not available commands today. Telegram and dashboards only render the frozen underlying decision capsule. Silence, stale/unqualified evidence and no suitable opportunity are *successful abstention* rather than transport errors.

### Decision capsule shape (compact example, NOT live output)

```json
{
  "kind": "CopyOpportunity",
  "id": "opaque-stable-id",
  "mode": "SHADOW",
  "instrument": "hyperliquid:mainnet:perp:default:BTC",
  "source_action": "ADD_LONG",
  "copy_direction": "LONG",
  "first_known_at": "2026-10-13T00:00:00Z",
  "earliest_actionable_at": null,
  "readiness": "NO_COPY",
  "blockers": ["EXECUTABLE_QUOTE_NOT_QUALIFIED", "PREDICTIVE_GATE_BLOCKED"],
  "independent_support": {"clusters": null, "coverage": "UNKNOWN"},
  "expected_return": null,
  "lagged_baseline_ref": null,
  "outcome_ref": null,
  "ledger_ref": "opaque-source-ref"
}
```

Use honest nulls rather than invented fills, confidence, stop prices or probabilities.

## D6. Resource strategy and recoverability

Hot-path correctness wins over research convenience. Define **three priority classes**, not three new services by default:

1. **P0 continuous source integrity**: capture, commit, checkpoint, bounded replay, source recovery. Small fixed per-event memory; bounded queues; never await Telegram or expensive evaluation inside ingestion.
2. **P1 decision production and safety**: incremental exposure path, candidate dedup/expiry, opportunity durability, costed quote capture, visibility/gap and alert eligibility.
3. **P2 scientific jobs**: labels, matched baseline generation, clustering/cohort fitting, sensitivity sweeps, sealed exports. Preemptible/restartable; own CPU/memory/I/O quotas; no ability to starve P0.

Storage: retain immutable canonical observations/checkpoints and frozen *sparse* opportunities in transactional local stores. Export sealed historical raw and derived partitions to hash-verified Parquet with schema/provenance manifests; query with Polars/DuckDB only in the slow research path. Keep a small indexed hot catalog with watermarks and outcome due times. Prune/compact only after independent integrity checks, reproducible replay, defined retention, backup and explicit operator authorization. No automatic deletion of existing 2.5 GB research data or public-source evidence by this design revision.

Instrument source quotas, high-water marks, backlog age, source-to-ledger lag, state-to-opportunity lag, outcome queue lag, last successful snapshot and trade/coverage gaps separately. Detect OOM/cgroup kills, track memory headroom, pause P2 and preserve P0 via process quotas. A healthy restarted service is **not** complete data. Recovered partial states remain marked PARTIAL_HISTORY.

For efficiency: reuse one market-feed update across eligible opportunities; coalesce same-wallet/cluster child changes; memoize immutable input revisions; calculate outcomes only for registered arms and required negative controls. Measure bytes/decision, memory p95, external request weight/opportunity, CPU/decision and diagnostic tool calls. Scale watched wallets only if they add **marginal supported information**, not to reach a round number.

## D7. Revisions, learning, controls

Each durable artifact includes `schema_version`, `source_ledger_id:seq`, `feature_cut`, `universe_revision`, `cluster_revision`, `policy_revision`, `known_at`, `coverage`, and content/replay hash where applicable. An experiment record has hypothesis, frozen manifest, controls, baselines, effects, intervals, resource budget, negative result, precise limitation and reproducer.

The feedback loop is:

```text
observed anomaly/idea
  -> isolate source + causal decision capsule
  -> registered falsifiable hypothesis
  -> bounded historical test (including simple controls)
  -> independent forward gate
  -> human-reviewed policy proposal
  -> typed, idempotent policy revision / rollback receipt
  -> monitored live shadow outcomes + drift
  -> versioned reusable regression/evidence
```

Do not let an LLM change its own quality weights based on freshly observed price movement, infer confidence from one winning coin, tune clusters to a desired number, change costs after outcomes or use outcome-labeled segment boundaries. Agent memory is a retrieval index to durable evidence, not a second source of truth.

## D8. Failure semantics and what to show the user

Differentiate `NO_TRADE`, `FLAT_KNOWN`, `HISTORY_INCOMPLETE`, `SOURCE_STALE`, `RECONCILIATION_CORRECTION`, `OPPORTUNITY_NOT_MATERIAL`, `REDUCE_NOT_SHORT`, `DUPLICATE_CHILD`, `MARKET_ONLY_EXPLAINS_MOVE`, `PRICE_OR_COST_UNKNOWN`, `NOT_YET_MATURE`, `UNSUPPORTED_MARKET`, `PREDICTIVE_NOT_QUALIFIED`, `TELEGRAM_NOT_DELIVERED` and `OUTAGE_GAP`. Every reason has an owning module and first permitted remediation action.

A strong current wallet long posture with no new flow is context, not an ENTER event. An apparent win on an unsupported HIP-3 reference market is censored, not relabeled with a default-perp mark. A missing stream interval invalidates affected novelty and entry timing even if snapshot recovery restores current exposure.

## D9. Evidence-grounded motivation and current boundaries (2026-10-13)

The inspected live system has 215 configured wallets and ~203 provisional clusters. On the read-only audit approximately 57,233 economic `POSITION_CHANGE` events comprised 29,475 ADD, 24,220 REDUCE, 1,762 OPEN, 1,746 CLOSE and 30 FLIP. The current raw outcomes recorder assigns direction from `sign(delta)`, including reductions, and therefore measures signed execution-flow markouts, **not** verified copy-entry alpha. The production ensemble writes minute-by-instrument descriptive consensus cuts; the scientific R0/R1/R2 research functions are not yet the live policy. The observational grade A/B result is highly concentrated in a few rising assets, and no incremental predictive signal has passed the registered historical/live-forward gate.

During an October 11-12 incident the scoring stream had a measured ~21h18m missing-minute interval and the service logs recorded OOM kills. The prospective research SQLite grew to ~2.5 GB in roughly 4.75 days. Those are **observed bottlenecks**, not proofs of a single root cause; a resource profile and coverage capsule are required before choosing a fix. Preserve the outage as a permanent missing-evidence boundary.

See [STATUS](STATUS.md) for capability classification and [root plan](../plan.md) for dependency-aware implementation slices. Current running processes remain advisory/shadow; no new financial execution is authorized.

## D10. Explicitly rejected shortcuts

- "Just gather another week" without creating more independent, properly timed decisions.
- Treating reduction/closure trade signs as an opposite fresh copy entry.
- Treating 1,000 correlated minute snapshots as 1,000 trades, or a high win rate in a coin rally as incremental alpha.
- Requiring uncertain TWAP/order intent detectors before valid additive exposure and entry studies can proceed.
- Adding 100 more wallets before measuring novel independent cluster evidence.
- Replacing Rust observation authority, adding Kafka/PostgreSQL/microservices, an LLM on every fill, or scraping Telegram as the evidence source.
- Rebuilding research outputs each inspection, unbounded alerts/retries, rewriting as-known history, promoting without untouched forward data, or deleting source evidence to save disk.

## D11. Design gates before implementation

Decisions made here are default *design recommendations*, not deployment approval: (1) retain Rust/Python ownership; (2) first-class immutable CopyOpportunity; (3) flow-markout and copy-entry labels must never be conflated; (4) sparse decision journal plus reproducible common-grid research; (5) resource isolation and integrity before complex scoring; (6) a small typed agent inspection facade; (7) scientific null results are acceptable terminal outcomes.

Open factual gates: actual source availability of OID/native intent, per-venue executable price/slippage inputs, current outage root cause, actual recoverable historical coverage, wallet/cohort effective independent support, and minimal latency achievable by the follower. Resolve with read-only inspections and tests. These gates must not be papered over by guesses or prevent independent work on available source facts.
