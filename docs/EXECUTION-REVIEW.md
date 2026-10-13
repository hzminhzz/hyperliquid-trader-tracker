# Expert-wallet execution and sampling review

Date: 2026-10-06. Status: PROPOSED DESIGN / RESEARCH REVIEW; not an implementation or promotion authorization.

Inspected tracker: `2f948f81094cfb18fa2799eb1dcce5c748973148`, `/home/quant/dev/hyperliquid-trader-tracker`.
Inspected engine: `ef8d7c19f4bfdf76b86d35827145679ae9d1c346`, `/home/quant/dev/hyperliquid-expert-ensemble`.
Both worktrees were clean before this documentation review. The supplied `/home/dev/hyperliquid-trader-tracker` path was outside this connector's allowed roots; the accessible checkout above was inspected instead. Existing engine tests: `PYTHONDONTWRITEBYTECODE=1 uv run --frozen --no-sync pytest -q -p no:cacheprovider`: 79 passed in 3.57s. No live health, complete wallet roster, empirical cluster count, source latency, or predictive-performance claim was verified in this review.

Claim labels: FACT = directly inspected code/documentation or cited research result; INFERENCE = a consequence reasoned from those facts; HYPOTHESIS = an empirical proposition; PARAMETER = a proposed initial setting, not an optimum. Repository symbols below refer to the pinned revisions, not any later checkout.

## 1. Decision

Keep the V2 evidence-to-policy separation. Replace the mandatory `fill -> order -> episode -> intent` dependency with an additive, point-in-time exposure path plus optional order/program/episode annotations.

An execution episode is an observable segment or an explicitly identified execution program. It is not proof of one investment decision. A fill can come from discretionary trading, an old limit order, liquidation, inventory control, or hedging an unobserved position. Inferred intent must not become accounting truth.

Recommended next research scale: approximately 120 observed wallets, retaining the original approximately 50 as a frozen comparator. Seek roughly 40-60 supported behavioral groups across the universe; 50-70 is plausible only as a hypothesis to measure. Never tune clustering to manufacture a desired count. Report active, reliable, per-instrument/per-horizon breadth rather than a single global count. Do not expand to 200 merely to increase agreement.

Recommended operational shape: authoritative event-driven accounting, one-minute evidence and ensemble cuts, and coalesced material-event interrupts. Five-minute scoring remains an experimental comparator; it should not be the default blind interval for a system evaluating one- and five-minute outcomes.

## 2. Verified repository findings and implications

| ID | Fact and code evidence | Inference / required gate |
|---|---|---|
| F01 | `src/ensemble/runtime.py::compute_targets` still calls `compute_equal_budget_consensus`; runtime does not invoke the V2 evidence/outcome modules. | V2 functions existing is not V2 running end to end. Prove one real receipt-to-outcome path before research promotion. |
| F02 | Runtime directly fetches account snapshots; `ProjectionStore.replace_wallet_snapshot` deletes/reinserts positions and resets opened_at, last_added_at and revision. `step` snapshots before consuming outstanding ledger events. | Two state-authority paths can overwrite one another without a proven snapshot/stream cut; ages can be observation ages rather than true holding ages. Rust must own account reconciliation and snapshot boundaries. |
| F03 | Tracker `listener.rs::handle_trades` uses the public trades stream. The canonical payload contains transition, delta, quantity_after, avg_entry and px, but no oid, twapId, crossed or liquidation classification. A repository-wide Rust search found no ingestion of these fields. | OID/program-aware grouping needs a qualified enrichment source. It cannot be recovered by guessing identifiers from public trade IDs. |
| F04 | `handle_trades` updates the in-memory seen set before durable receipt acceptance; append failure does not stop reduction, the book changes before `commit_transition`, and that result is discarded. Receipt ID can fall back to 0. | The live call site can violate fail-closed durability even if ledger-unit tests pass. Fault-inject append/commit failures and crash windows before claiming immutable complete observation. This is a code-inspection risk, not a reproduced incident. |
| F05 | Live public-trade dedup uses `(coin, tid)` and receipt source key `coin:tid`, not block time. Official documentation recommends `(block_time, coin, tid)` [S1]. | Same order pair across blocks can be lost. Preserve a documented scoped identity and distinguish duplicate delivery from a new economic fill. |
| F06 | `projection.py::apply_canonical_events` strips instrument namespace with `split(':')[-1]`, hardcodes some coverage fields, and applies POSITION_CHANGE without materializing canonical equity observations. | Instrument collisions, age/coverage loss and snapshot dependence need explicit acceptance cases. Default-perp support must not be presented as all-dex/spot support. |
| F07 | `wallet_evidence.py` classifies every nonzero verified quantity transition; there is no execution episode or materiality implementation. Flow membership uses only known_at; events with delta_bias=None are dropped and the remaining sum can be zero. | This can confuse missing normalization with inactivity and old backfill with fresh execution. Event age and coverage must accompany availability time. |
| F08 | Intent normalization is supplied per event as delta quantity times price divided by equity; its input contract does not itself pin freshness/scope of that denominator. | Add explicit normalization provenance. Independent per-fill scales do not guarantee round-trip cancellation. |
| F09 | `ensemble_evidence.py::_cluster_contribution` bounds state through bounded_influence but sums raw wallet flow. It checks only that wallet knowledge time is not in the future. | A highly leveraged wallet can dominate flow despite a nominal cluster budget. Old wallet window sums must be recomputed at the ensemble cut, not mixed as if current. Preserve raw measures and separately bound influence. |
| F10 | `independence.py` defaults to five active steps and one joint episode; one supported pair can classify a wallet as known, even when many other pair relationships are missing. Effective breadth is inverse squared cluster budgets. | These are synthetic-mechanics defaults, not statistical evidence of independence. Weight concentration is not correlation-adjusted independent sample size. |
| F11 | `outcomes.py` selects the first price at or after a target with no maximum lateness, and PricePoint has no received/known timestamp. net_return subtracts cost before `ablation.py::_directional_return` may negate it for a short. | A distant price can be mislabeled a short-horizon outcome. If these interfaces are composed directly, a short receives a positive cost term. Test direction before cost, delayed price availability, and latency exceeding the horizon. |
| F12 | The only ablation metric is mean directional return; promotion depends on sample count and a mean delta, without a dependence-aware interval in that function. | A sign-only metric cannot generally test magnitude preservation. Many fills/wallets sharing one market move are not independent observations. Promotion needs a research-level statistical gate. |
| F13 | `docs/evidence/v2/INDEX.md` marks V26 INCONCLUSIVE, V27/V28 NOT_ACTIVATED, V29 REJECTED / ENGINEERING_PASS and V30/V31 NOT_ACTIVATED. | No forward predictive qualification should be inferred from the passing 79-test suite or the V2 commit title. |

These findings do not justify a wholesale rewrite. They justify a narrow correctness gate and a thinner descriptive interface.

## 3. Source capability and universe scale

FACT: public WsTrade records do not expose wallet order IDs; user-fill records expose oid/crossed and optional liquidation data, and native TWAP records can expose twapId [S1]. `aggregateByTime` combines some matching-engine partial fills, not an entire multi-order investment decision [S2]. Native TWAPs use a documented scheduled slicing mechanism, but missed/partial executions and catch-up mean perfectly equal fills are not required [S3].

Treat native parent identifiers as stronger evidence than timing patterns. Treat missing metadata as UNKNOWN, not a negative label. Establish a field-capability matrix for the actual subscription/provider plan before adding an evidence dependency. Do not assume a vendor's discovery endpoint is a complete, low-latency execution feed. This review did not qualify the HyperX account plan or feed.

Documented public limits include 1,200 REST weight/minute/IP, weight 2 for clearinghouseState, weight 20 for most other info calls plus response-dependent costs, and 10 unique users for user-specific WebSocket subscriptions [S4]. These are documented limits, not a live quota probe.

Illustrative budget, one account scope per wallet and one request per minute:

| Observed wallets | Account-state polling weight/min | User-fill polling base weight/min |
|---:|---:|---:|
| 50 | 100 | 1,000 |
| 120 | 240 | 2,400 |
| 200 | 400 | 4,000 |

The table is arithmetic from [S4], excluding payload weights, other programs, retries and recovery. Thus 120 public-stream-observed wallets is a different engineering problem from 120 richly subscribed user accounts. More sockets do not remove an IP-scoped user limit. Use a qualified provider or a selectively enriched research panel within the documented allowance; do not design around bypassing limits.

HYPOTHESIS: 120 observed wallets provides better discovery and replacement coverage than 50 without committing to 200 low-quality voters. Admit wallets by prospective data availability, useful horizons, strategy diversity and independent novelty, not retrospective leaderboard returns alone. Record discovery/eligibility dates, source, reasons and exclusions. Historical results using today's selected wallets are conditional on that selected universe, not a survivorship-free universe study.

Maintain separate counts: observed; seeded; normalized; supported behavioral groups; active groups for this instrument/horizon; unknown-redundancy mass; and qualified predictive contributors. Missing/quiet/flat are different. A wallet with little history can provide provisional descriptive state while its relative scale and independence remain unknown.

For nonnegative active reliability weights a_g, report weight breadth `(sum a_g)^2 / sum(a_g^2)`. Do not rename it verified independence. With a sufficiently supported, lagged, regularized residual correlation matrix R, additionally report a variance-equivalent diagnostic `(sum a_g)^2 / (a' R a)` and its uncertainty. It is not a literal count of people. Common-market correlation and direct copying are different explanations.

PARAMETER: a planning target is 40-60 supported groups globally and useful breadth across the main markets; no minimum global target is a promotion rule. Expansion to 200 should require measurable new coverage/diversity or incremental out-of-sample value, without worse observation completeness or latency.

## 4. Architecture and ownership

```text
Rust observation authority
  immutable source receipts -> deduplicated economic fills
  -> exact scoped position transitions + coverage/reconciliation facts
                         |
Python evidence module  |
  additive quantity path + common-anchor normalization
  + optional order/native-program/inferred-segment annotations
  -> WalletEvidence at a named common cut
                         |
lagged independence and normalization artifacts
  -> bounded cluster contributions + unbounded descriptive measures
  -> EnsembleEvidence
                         |
registered research -> PredictiveEvidence -> deterministic TradeSignal
                         |
optional account advice; financial execution remains disabled

Outcome observations attach later; they never rewrite prior evidence.
```

Keep two processes, not one service per artifact. The episode label is not required to compute quantity, state, net flow or gross turnover. An unavailable detector must not disable these simpler features. Unknown order IDs must not be invented. No LLM belongs in reduction, episode transitions, normalization, clustering application or scoring.

Each artifact references receipt/economic-event IDs, full instrument scope, position/coverage revision, event cut, receipt and acceptance/knowledge times, input watermark, normalization revision, universe/cluster/feature/policy revisions, and its cause. Late enrichment creates a new revision; past decisions remain immutable. Separate source receipt time from the moment the Python feature became usable.

Prove a coherent snapshot/stream cut or mark state approximate/quarantined. A snapshot repairs current state, not missing historical execution. Stale equity invalidates equity normalization but need not erase known raw quantity. Equity refresh, mark refresh, funding, transfers, cluster changes and missing-data recovery are not discretionary flow.

## 5. Exposure, flow and materiality

State remains `b(t) = q(t) * p(t) / E(t)` with compatible scope, freshness and source refs. This is observed leverage, not total wealth or directional belief.

Retain raw quantity deltas as the additive primitive. At a common evidence cut (t, K), define visible recent events by both `event_time in (t-h,t]` and `known_at <= K`, with eligible continuous coverage. Backfilled older events do not become fresh flow just because K is recent. Recent late events may enter the current cut if timely enough; no historical evidence is overwritten.

For one common point-in-time price/equity anchor P*, E* across the events being summarized:

`F_h(t,K) = P* / E* * sum(delta_q_e)`

`G_h(t,K) = P* / E* * sum(abs(delta_q_e))`

Keep buys and sells separately when useful. Use the same anchor across horizons at a cut when comparing build rates. Freeze and record the anchor epoch; if a later epoch revalues an existing flow window, classify that as normalization/valuation change, not a newly executed quantity. Store raw sums so revaluation is explainable. Do not subtract differently scaled horizon values without reconciling their units.

This construction gives zero net quantity flow to an exact round trip, even when exit price differs. Its gross turnover remains positive. A sum of independently normalized fills is a different cash/execution-weighted measure and should not silently replace inventory flow.

Split invariance is narrow and testable: repartitioning the same timed quantity path into more fills/OIDs must not change endpoint state, additive flow or voting budget. Executing earlier versus later genuinely changes time-local information; time-path features need not be invariant to that change. Apply clipping, percentiles and materiality after accumulation, never separately to every child fill.

Materiality must control semantic prominence and interrupts, not observation retention, exact state updates or the additive flow ledger. Small fills accumulate. Always retain exact OPEN/CLOSE/FLIP, but do not let a dust OPEN interrupt the whole ensemble.

For a segment anchor q0 and fixed compatible P*, E*, let `A = abs(q-q0)*P*/E*`. A transparent candidate rule for ADD/REDUCE prominence is:

`A >= absolute_floor AND (A >= alpha * lagged_typical_segment_size OR abs(q-q0)/max(abs(q0), q_floor) >= eta)`.

The relative-to-position branch detects a meaningful exit in a small book; the absolute floor prevents dust from becoming a large percentage. The typical size distribution uses prior completed, independently segmented activity, not current/future fills and not a percentile of individual TWAP children. Estimate with wallet-level shrinkage toward a declared cohort for insufficient support. Threshold-free continuous sizes remain available to research.

PARAMETERS for an initial sensitivity test only: absolute_floor=0.005 equity units; alpha=0.10; eta=0.10; lagged 30-day scale; at least 30 completed segments before using an unshrunk wallet scale. These are not validated recommendations. Position-share change and portfolio-share change are additional context, not sole gates: they are unstable near flat or when other positions/markets move. Equity freshness/scope failures produce missing normalized magnitude, never zero.

Influence is a separate bounded view: preserve raw state/flow and apply a declared bounded transform to each accumulated wallet feature before fixed within-cluster aggregation. One large leveraged wallet must not bypass the cluster budget through an unbounded flow channel. Evidence updates replace a cluster's current contribution; they never append another vote.

## 6. Exact reference state machines

These are proposed deterministic semantics, not implemented capability. Source units use exact decimals and venue precision. Never use floating-point zero. A display dust threshold does not change the accounting state.

### 6.1 Position lifecycle

Coverage qualification is orthogonal to signed position. The usable sign states are FLAT, LONG and SHORT; UNKNOWN means the underlying position is not established.

| Input / before -> after | Output |
|---|---|
| Valid initial snapshot, including flat | Seed state with PARTIAL_HISTORY when applicable; no OPEN/flow inferred. |
| Qualified economic fill, 0 -> nonzero | OPEN; begin a position lifecycle. |
| Same sign, absolute quantity increases | ADD, even below prominence threshold. |
| Same sign, absolute quantity decreases but nonzero | REDUCE. |
| Nonzero -> exact zero | CLOSE; end that lifecycle. |
| Sign changes in one fill | Atomic FLIP containing a close leg to zero and an open leg from zero; preserve their joint parent event. |
| Sign crosses zero across separate fills | Preserve CLOSE then OPEN and the real flat interval; link a possible reversal, do not invent an atomic FLIP. |
| Mark/equity/funding/transfer update without a trade | Update the relevant descriptive facts; no execution delta. |
| Liquidation/forced reduction | Correct quantity and retain lifecycle event with forced/unknown cause; exclude from discretionary-flow prediction unless separately qualified. |
| Reconciliation correction | Explicit correction; close any unsupported episode as interrupted; no fabricated discretionary trade. |
| Missing/ambiguous source sequence | Preserve observations, mark affected state/flow incomplete, reconcile; do not infer CLOSE. |

For a +3 to -2 flip, the two virtual legs are -3 and -2; their sum equals the one real -5 fill. Features consume either allocated legs or the raw fill, never both. Known native parent linkage can span the legs and multiple semantic segments.

### 6.2 Inferred directional execution segment

Use conservative same-direction, continuous-coverage segments as the initial annotation. They are intentionally not a universal decision detector. A long trade campaign can contain several segments; an order/native program can also span several segments. Keep those relationships explicit instead of forcing a tree.

Maintain one active inferred segment per wallet/instrument. Its status is CANDIDATE, ACTIVE, PAUSED or TERMINAL; material_seen is retained through a pause. Attributes include direction, lifecycle, first/last event and knowledge times, accumulated net/gross quantity, anchors, parent refs, segmentation revision and termination reason.

| State/input | Deterministic transition |
|---|---|
| No active segment + qualified live economic fill | Start CANDIDATE; add the fill and emit its exact lifecycle event immediately. |
| CANDIDATE + same direction, same lifecycle segment, gap below end threshold | Accumulate; ACTIVE when cumulative materiality is reached. |
| ACTIVE + compatible fill | Accumulate and revise attributes; do not create another vote or reset first-material time. |
| Inactivity reaches pause threshold | PAUSED; position remains unchanged. |
| PAUSED + compatible fill before end threshold | Resume CANDIDATE/ACTIVE according to material_seen. |
| Any segment + opposite-side fill | End prior segment as DIRECTION_CHANGE; process the new fill as the first of the opposite segment. |
| Exact flat or flip boundary | End the old lifecycle segment; if needed start a segment for the new leg. Preserve a shared native-program link rather than merging the lifecycles. |
| Inactivity reaches end threshold | TERMINAL with TIMEOUT_UNKNOWN; never claim the investment decision completed. |
| Source gap or state correction | TERMINAL with INTERRUPTED; no completion inference. |
| Late historical fill or metadata | Append correction/annotation revision for future use; never replay it as a fresh online segment or revise an old emitted prediction. |

An opposite-side microfill deliberately fragments this conservative heuristic. Raw path features remain intact, and native program linkage can connect the pieces. Tolerating small counterflow or merging multiple segments is a later ablation, not hidden complexity in the accounting path. Mixed market making should not be forced into clean discretionary episodes.

PARAMETERS: initial pause=90s and end=10min. After sufficient lagged support from distinct child/burst timestamps, compare pause=max(90s,3*tau) and end=max(10min,10*tau), with a declared maximum gap and clock policy. tau is not calculated from raw partial-fill arrival gaps. These settings are segmentation conventions to test, not TWAP ground truth. Finalization is known only when the timer fires; a backdated last-fill timestamp must not leak that conclusion earlier.

### 6.3 Native execution-program lifecycle

Store explicit native parent states separately: OBSERVED/EXECUTING, FINISHED, TERMINATED or ERROR, as supplied by the qualified source. An active program can exist while its latest semantic segment is paused or closed. A cancellation says the unexecuted remainder was cancelled; it does not reduce already executed exposure. Store intended/executed size and completion fraction only when directly observed. Concurrent native programs remain distinct even when their fills have the same sign.

## 7. TWAP, iceberg and copying detection

Use an evidence hierarchy, not a single hard TWAP flag:

1. NATIVE_CONFIRMED: a qualified twapId/program record links the child fills. A shared oid confirms an order relationship, not an investment decision.
2. SCHEDULE_LIKE: causal same-wallet/instrument bursts have compatible direction, repeated activity and a roughly stable cumulative quantity-versus-time path. Retain distinct-order count if available, elapsed gaps, robust gap/size dispersion, path-fit residual, pause/resume events and known liquidity-taking fraction. Do not require identical child sizes or exact 30-second gaps.
3. DIRECTIONAL_SEGMENT_UNKNOWN: adequate quantity path but insufficient metadata/support.
4. INVENTORY_OR_MIXED: repeated two-sided turnover or unclear attribution; this is an abstention from an intent claim, not deletion of observations.

PARAMETER: do not call a timing heuristic schedule-like before at least four distinct child/burst observations and a meaningful elapsed span (initially two minutes). This is only a support guard. Missing native IDs are not negative training examples. If the public trade side and buyer/seller ordering can be qualified as sufficient to derive liquidity-taking role, use that derived field with provenance rather than requiring a redundant user-fill request; this mapping was not live-qualified in this review. Calibrate against confirmed native examples plus outcome-blind manually audited discretionary, pyramiding and market-making snippets. Report false merges, false splits, first-detection delay, coverage and parameter sensitivity. Do not optimize segmentation directly against future returns.

Native-ID inference may remain optional if it does not improve held-out predictions beyond simple net/gross flow and build rate. Vaglica et al. describe inferred hidden-order sequences as proxies, which supports this uncertainty distinction [S6]. No HMM/HSMM is required in the MVP.

Iceberg detection is a separate data problem. Credible replenishment evidence needs order-add/modify/cancel and displayed-size observations; fills alone do not identify an iceberg or its hidden size. CME research uses precisely such order-depth evidence [S7]; it is not a validated Hyperliquid detector. Emit UNKNOWN without the required feed.

For copying, compare lagged causal exposure changes, repeated lead-lag relationships, normalized size paths, entries/exits and inactivity patterns across many distinct campaigns. Test against a market-conditioned coincidence baseline. One simultaneous BTC trade or one common funding source is not proof of copying. Store direct ownership evidence separately from behavioral similarity. A common market shock can create correlation without common control.

Freeze clustering on a slow schedule and use only earlier data. Keep UNKNOWN pair support explicit; one supported comparison does not establish independence from all other wallets. Conservative unknown budgets must not count as verified breadth. For confirmed copy families, a prospectively chosen reliable/early representative can avoid averaging away the leader's early move; compare this with the existing fixed within-cluster mean. Never choose the most profitable or extreme member using the evaluation period. Broader behavioral groups can retain fixed mean aggregation. A follower fill updates the same budget, not a new independent arrival.

## 8. Cadence and feature contract

| Layer | Initial proposed cadence / rule |
|---|---|
| Receipt, accounting and coverage | Every source event, durably accepted before observable state advances. |
| Additive feature accumulators | Incrementally on events; explicit expiry as windows advance. |
| WalletEvidence and EnsembleEvidence | Aligned one-minute cuts. Reuse unchanged state by reference, but recompute ages, window expiry and missingness at the cut. |
| Material-event interrupt | Coalesce at instrument/cluster level over an initial 2-5 seconds, after a consistent ingestion watermark. Current contribution replaces prior contribution. |
| Coverage loss / invalidation | Immediately mark evidence unavailable; do not wait for the score timer. No automatic financial action is authorized. |
| Equity/reconciliation | Rust-owned budgeted schedule, initially 1-5 minutes depending on scope/activity; freshness requirements override convenience. Preserve a recovery reserve. |
| Wallet scale | Lagged daily artifact; freeze within each declared epoch. |
| Cluster membership | Lagged weekly initial schedule; drift alerts can occur sooner without retroactive membership changes. |
| Human summary | Approximately five minutes or meaningful change; independent of prediction cadence. |

These are PARAMETERS. Event-triggered and clock-triggered snapshots require identical evidence-cut semantics. The interrupt accumulator measures unannounced change since the last emitted contribution, not repeatedly the entire size of an already-material segment. A qualifying OPEN/CLOSE/FLIP can request an interrupt; dust cannot. Coalescing emits the latest coherent contribution while preserving every underlying semantic event, so a queued OPEN followed immediately by CLOSE cannot publish an obsolete open state. Do not wake a fleet of LLM agents every minute.

Pure fill-count or order-count bars recreate execution-noise weighting. Raw-dollar bars can give large wallets control of the clock. A competing exposure-bar policy should accumulate normalized bounded cluster-level activity, maintain a maximum idle timeout and still be assessed on a common calendar grid. Endogenous sampling is not automatically more informative. Duration research supports modeling irregular arrival times, not a universal superiority claim for activity bars [S8].

Keep first-class descriptive dimensions:

| Keep/add | Role |
|---|---|
| Raw scoped quantity/bias; bounded influence | Preserve magnitude without unlimited votes. |
| Net flow at 1m/5m/15m/1h | Directional exposure change. Keep 4h/24h optional for matching holding horizons. |
| Gross flow/turnover and net-to-gross ratio | Distinguish monotone building from round-trip churn. |
| Event-time recency, known/receipt lag, coverage, denominator age | Separate freshness from activity and missingness. |
| Position age, segment elapsed duration, first-material age | Separate old holdings from new execution. |
| Peak/trough quantity excursion and recent close indicator | Preserve information erased by endpoint snapshots. |
| Build rate in consistent units | Distinguish fast versus slow accumulation. |
| Observed taker-notional fraction + support fraction | Execution context, never inferred from absent crossed fields. |
| Relative size/conviction and observed portfolio share | Context with lagged scales and explicit scope. |
| Cluster contribution, opposing/supporting mass and unknown mass | Interpretable aggregation. |

Keep per-fill/OID count only for operational diagnostics or optional execution characterization, never voting power or independent statistical sample count. Defer raw second-difference acceleration, completion forecasts, inferred hidden size, skill/anti-alpha weights and complex regime models until the simpler representation demonstrates value. A stable fast-versus-slow build-rate contrast is an optional single-family ablation, not a dozen correlated features added together.

## 9. Adversarial acceptance suite

| Case | Required observable outcome |
|---|---|
| One fill versus 100 partial fills with the same timed total | Same endpoint, net/gross flow and influence. |
| New OID for every child | No increase in cluster budget or repeated novelty from identity churn. |
| Same `(coin,tid)` in different source blocks | Distinct economic fills under the documented identity. |
| Duplicate WS/recovery deliveries; watched buyer and seller | Delivery dedup; both wallet perspectives applied once; no loss of one counterparty on retry. |
| Append failure, transition failure, restart between two counterparties | No published uncommitted state; deterministic replay of unapplied economic perspectives. |
| OPEN then CLOSE within 20 seconds, flat at the minute | Two lifecycle events, zero net flow, positive gross turnover, nonzero excursion; no stale open interrupt emitted after close. |
| +3 -> -2 in one fill | Atomic flip; disjoint -3/-2 legs; total delta -5, not -10. |
| +3 -> 0 -> -2 in separate fills | Genuine flat interval preserved. |
| +2x -> +1x | Long state plus negative reducing flow, not an automatic short forecast. |
| Many individually tiny adds | Cumulative prominence eventually crosses; no quantity is discarded. |
| Pyramiding after pauses / cancels / repricing | Path preserved; uncertain decision grouping; no false native identity. |
| Two simultaneous same-side native TWAPs | Separate native parents, one wallet contribution. |
| Alternating market-making fills | Large gross, low net, mixed annotation; no artificial discretionary conviction. |
| Long one contract, short correlated contract or unobserved external hedge | Scope and hedge ambiguity explicit; no unsupported global directional intent. |
| Mark rises or equity falls with constant quantity | State valuation changes, no new execution flow. |
| Stale, zero or negative equity; recent equity after a formerly missing fill | Normalized flow missing at the earlier cut; cannot silently become known-zero or be retroactively repaired. |
| Old fills arrive after recovery | Correction/restatement, not a new live buying burst. |
| Long-lived OID spans hours or a program crosses zero | Preserve path/lifecycle segmentation and linkage, not one timestamp for all fills. |
| Fifteen clones, delayed copies, then leader dropout | Fixed copy-family budget; explicit lag/coverage; no survivor amplification or new breadth. |
| Many independent wallets react to one announcement | Shared-market dependence recorded without declaring common ownership. |
| Cluster revision or normalizer refresh with no fills | Attribution to model/scale revision, not new trader flow. |
| Default/HIP-3/spot ticker collision | Distinct canonical instrument IDs throughout. |
| Same economic history, different receipt ordering | Past as-known outputs may differ correctly; restated economic output converges where ordering is resolvable. |
| Price gap crosses outcome target; latency exceeds horizon | Outcome invalid/censored, not an arbitrarily late substitute. |
| Flat market plus positive costs, both long and short | Negative net result in both directions. |
| Thousands of wallet observations during one BTC move | One shared market outcome structure, not thousands of independent trials. |

## 10. Minimum decision experiment

### Gate 0: correctness and data availability

Before predictive comparison, qualify the receipt-to-state-to-evidence-to-outcome path, source fields, rate budget, missingness and the adversarial suite. Record a rich-metadata panel within authorized source limits to measure how much order/program annotations actually add. Do not delay useful baseline collection while building a universal intent classifier.

Register universe membership prospectively. Observe approximately 120 wallets, but retain the original approximately 50 subset. Log all eligible decisions including silence, missingness, rejected triggers, unavailable outcomes and detector uncertainty. Keep the full raw timed path needed for every comparator.

### Stage 1: do episodes add information?

At identical one-minute instrument cuts compare three small, predeclared representations with the same transparent predictor and market controls:

- R0: state only.
- R1: R0 plus additive net/gross flow and recency/build rate.
- R2: R1 plus the small episode annotation family: elapsed duration, pause state, native/schedule-like status and supported taker fraction.

Primary outcome: 15-minute forward return. Secondary: 1-hour. Attach 1m/5m/15m/1h/4h/24h outcomes, but do not select the winning horizon after inspection. Compare against a market-only predictor using lagged returns/volatility and other actually recorded inputs, to test whether wallets add information beyond observable price trends.

R0/R1 collection and comparison need not wait for an unavailable R2 metadata source; record R2 as BLOCKED/INCONCLUSIVE instead of withholding valid additive evidence. A small regularized linear predictor with frozen preprocessing is enough. Use held-out squared-error improvement and rank/conditional-return diagnostics; a sign-only score cannot test whether preserved amplitude is useful. Frozen state+flow rules may also be used, but their parameters must be set before the holdout.

### Stage 2: which clock captures that information?

Freeze the simplest supported representation from Stage 1, then compare on a new untouched evaluation interval: fixed 60s, fixed 300s, 60s plus coalesced material interrupts, and normalized exposure-event bars with a maximum idle timeout. Stage 1 selection consumes its holdout; reusing that period for adaptive Stage 2 validation is not out-of-sample confirmation. Do not mix a new feature family into a clock comparison. The fully event-driven comparator must define quiet-market expiry and stale-state handling explicitly.

Every policy is evaluated on the same one-minute instrument grid using its latest actually available prediction, plus an executable-action replay for interrupt timing. Event emitters do not get more favorable rows because they emit more often. Charge costs only on a declared trade/position change, not on every unchanged prediction snapshot. Log computation and delivery delay.

### Universe comparison and statistics

Compare the original approximately 50 with the expanded approximately 120 using the same periods and eligibility rules. Also report predeclared, stratified, cluster-preserving subsets from the expanded pool, since the original set may have a different selection bias. Keep policy and budget conventions explicit; missing wallets do not silently renormalize survivors. Do not test 200 unless additional observed candidates are genuinely available and selected without outcome leakage.

Fit scales, clusters, detector thresholds and predictor parameters on earlier data. Purge training labels that overlap evaluation; a shared six-horizon study needs at least the longest actual label span accounted for, including price/latency conventions. A conservative initial boundary excludes the preceding 24 hours of training labels, then verifies actual interval intersections. Keep synchronized assets together in time blocks. Do not treat two wallets forecasting the same coin/time return as independent responses.

Use paired day or multi-day block resampling of the full cross-asset panel, with block length reflecting the longest outcome and dependence. Add leave-cluster-out and high-activity-day sensitivity. Report all registered comparisons and negative results. A fill-count bootstrap is invalid for this purpose.

PARAMETER: an initial collection plan is 1-2 weeks for operational/calibration work followed by at least four untouched weeks; longer horizons and sparse clusters will commonly require more. This is not a power guarantee. Predeclare a minimum useful effect and continue until an interval distinguishes improvement, practical equivalence or unresolved uncertainty. Interval uncertainty, distinct market periods and supported clusters matter more than raw fill count.

Correct signed economic evaluation uses direction before costs: e.g. `direction * log(P_exit/P_entry) - round_trip_cost`, with the trading replay using executable sides, fees, slippage, funding and actual position changes. Prices must have admissible availability and maximum lateness. Never apply the short sign to an already cost-subtracted unsigned return.

Promotion rule: retain the simpler design unless incremental out-of-sample information and realistic trading utility justify complexity without worse reliability. An INCONCLUSIVE result is not a promotion. Short-horizon continuation during execution is not automatically long-horizon trader alpha: order-splitting persistence and execution impact/decay are relevant alternative explanations [S5, S9].

## 11. Agent-operable design

The agent should need one coherent evidence cut, not simultaneous mental reconstruction of two databases and several clocks. Deepen the existing inspection facade rather than adding another framework.

Proposed interface operations, not claims of existing commands:

- Inspect capabilities with statuses IMPLEMENTED, QUALIFIED, PROPOSED and BLOCKED.
- Inspect one wallet/instrument at an as-known cut: state, net/gross path, normalization, segment status, scope and missingness.
- Explain the change between two evidence IDs with additive cause attribution: EXECUTION, MARK, EQUITY, NORMALIZER, CLUSTER, COVERAGE or WINDOW_EXPIRY.
- Explain one ensemble contribution through cluster/member/position/receipt references, including opposing and missing evidence.
- Replay a bounded interval with pinned versions and compare two registered designs.
- Inspect an experiment's manifest, support, rejected trials, outcome maturity and promotion authority.

Keep the facade deterministic and read-only by default. Machine output should be compact, typed and paginated, with expand-by-ID detail. Every proposed command has explicit scope, input revisions, permitted effects and resource budget. Unknown is a first-class result; no confident natural-language story can override a missing source field.

Each agent-relevant lesson is a reproduced fixture plus one concise decision/rationale, not a growing transcript. Canonical glossary terms stay in CONTEXT.md; this review owns proposed execution/sampling details; historical evidence manifests remain historical. Status pages point to inspected commits and exact test results. Keep wallet addresses, account credentials and production raw evidence outside the public repository.

## 12. Decision frontier and proposed sequencing

No implementation approval is implied. The following recommendations resolve the design branches for this review but still require explicit acceptance before scoped work begins:

| Decision | Recommended answer | What can falsify it |
|---|---|---|
| Must episodes precede flow? | No; additive path is canonical and episodes annotate. | Episode-only representation materially outperforms without coverage/latency cost. |
| Default scoring clock? | One minute plus bounded interrupts. | Five-minute comparator is equivalent after costs and latency for the chosen horizon. |
| Observation scale? | Approximately 120, original 50 retained as comparator. | New wallets add no useful coverage/breadth or damage data quality. |
| Native metadata dependency? | Optional enrichment; mandatory only for confirmed native labels. | Qualified data coverage makes the richer feed cheaper and more reliable than the baseline. |
| Materiality role? | Control prominence/interrupts, never delete path information. | No expected reason to discard immutable accounting; alternative display rules are empirical. |
| Predictive promotion? | Blocked until correctness and prospective evidence gates pass. | Only new qualifying evidence can clear the gate. |

Proposed dependency order, not new tracker issues: source/durability/ownership qualification -> additive path and missingness -> optional episode annotations -> aligned cuts and capped influence -> registered representation comparison -> registered clock/universe comparison -> predictive-policy qualification. Reopen only the affected acceptance predicates; do not relabel old synthetic tests as forward evidence or erase prior manifests.

## 13. Primary-source notes

These sources motivate tests; none establishes an optimum wallet count, detector threshold or scoring clock for this system. Research findings from other venues require transfer validation.

- [S1] Hyperliquid, WebSocket subscriptions and schemas. Public trade identity, user-fill fields and native TWAP linkage. https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/websocket/subscriptions
- [S2] Hyperliquid, Info endpoint. Partial-fill aggregation and bounded fill history. https://hyperliquid.gitbook.io/Hyperliquid-docs/for-developers/api/info-endpoint
- [S3] Hyperliquid, Order types. Native TWAP scheduling and catch-up behavior. https://hyperliquid.gitbook.io/hyperliquid-docs/trading/order-types
- [S4] Hyperliquid, Rate limits and user limits. Documented public request/subscription constraints. https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/rate-limits-and-user-limits
- [S5] Toth, Palit, Lillo and Farmer, Why is order flow so persistent?, published 2015; arXiv 1108.1632. Their LSE analysis attributes short-timescale persistence largely to order splitting, not independent herding. This motivates separating repeated execution from breadth. https://arxiv.org/abs/1108.1632
- [S6] Vaglica, Lillo and Mantegna, Statistical identification with hidden Markov models of large order splitting strategies in an equity market (2010). Hidden-order patches are probabilistic proxies, not directly observed intent. https://arxiv.org/abs/1003.2981
- [S7] Zotikov and Antonov, CME Iceberg Order Detection and Prediction (2019). Detection uses detailed order/depth and replenishment evidence, illustrating why fills-only certainty is unjustified. https://arxiv.org/abs/1909.09495
- [S8] Engle and Russell, Forecasting Transaction Rates: The Autoregressive Conditional Duration Model, NBER 4966 (1994; published 1998). Irregular durations carry structure; this does not prove activity sampling improves this forecast. https://www.nber.org/papers/w4966
- [S9] Donier and Bonart, A Million Metaorder Analysis of Market Impact on the Bitcoin (2015 version). Execution trajectory and impact decay make temporary pressure a plausible confound to persistent information. https://arxiv.org/abs/1412.4503
- [S10] Cont, Kukanov and Stoikov, The Price Impact of Order Book Events (2014). Their equity evidence concerns order-book imbalance, including additions/cancellations, not this selected-wallet flow. Do not call wallet flow market-wide OFI. https://arxiv.org/abs/1011.6402
- [S11] Joseph, Riedl, Pentland and Moro, When Influence Misleads: Informational and Strategic Limits of Social Learning in Trading Networks (2025 preprint). Popularity is not demonstrated forecasting skill; follower counts should not supply predictive weights. This is not Hyperliquid validation. https://arxiv.org/abs/2507.01817
