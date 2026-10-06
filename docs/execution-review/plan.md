# Execution-review implementation and research plan

Status: APPROVED FOR ISSUE MATERIALIZATION by the user on 2026-10-06. Publishing this plan does not start implementation or authorize financial execution, deployment or predictive promotion.

Repository: `hzminhzz/hyperliquid-trader-tracker`. Active source plan: `docs/execution-review/plan.md`. This is the single-milestone projection of the seven ER0-ER6 packages already discussed in root `plan.md` and `docs/EXECUTION-REVIEW.md`; historical V1/V2 milestones and their evidence remain unchanged.

Dependency graph: `ER0 -> ER1 -> (ER2 optional annotations || ER3 coherent cuts) -> ER4 -> ER5 -> ER6`. ER4 requires ER1 and ER3; ER2 is a conditional prerequisite only for the R2 annotation arm. ER6 requires ER4/ER5 evidence. Current next step is ER0.

Shared constraints: two runtime processes; Rust owns observed account truth, Python owns interpretation; additive path before optional execution labels; point-in-time source/universe/normalization/cluster cuts; approximately 120 observed wallets with the actual original approximately 50 as comparator; cluster count is measured, not targeted; no retrospective or future-data qualification; retain negative and inconclusive research. Research issues remain pending when required future observations do not yet exist.

### Milestone: `Expert-wallet execution correctness and prospective research`

**Issue 1 -- ER0: Qualify source identity, durability, and single state authority**

**Depends on:** None. This is the initial actionable frontier.
**Ownership:** Rust observation path and its Python projection consumer; narrow contract changes only.
**Source:** Root plan ER0; execution review sections 2-4 and 9, especially F01-F06.

**Objective:** Establish one trustworthy receipt-to-position-to-projection path before richer scoring. Reproduce inspected risks before fixing them; preserve the existing reducer rather than rewriting the system.

**Deliverables**
- Versioned source-capability matrix for the actual feeds: public trades versus OID/native-program/taker/forced-execution metadata; namespace, history limits, timestamps, subscription/request budget and recovery reserve. Unsupported fields remain UNKNOWN. Discovery analytics are not assumed to be a complete live feed.
- Durable acceptance before trusted memory/publication advances; explicit receipt/transition failures; restart-safe economic dedup and both watched counterparty projections. Use scoped source identity including source block/event time, instrument and trade ID, qualified against actual payloads.
- Rust-owned positions, scoped equity, coverage and reconciliation. Remove or fence competing Python snapshot replacement; prove a snapshot/stream cut or quarantine the affected scope. Preserve true/unknown lifecycle ages, monotonic revisions and full instrument IDs.
- A bounded real-source receipt -> durable state -> canonical publication -> Python projection qualification, without starting/restarting production services. Reusable sanitized fixtures and a compact capability/inspection report.

**Acceptance criteria**
- [ ] Fault injection at receipt append, transition commit, publication and restart shows no published uncommitted state or lost unapplied accepted event.
- [ ] Duplicate WS/recovery deliveries, equal trade IDs in different source blocks, two watched counterparties and restart between their processing are handled exactly once per economic wallet perspective.
- [ ] Concurrent snapshot/tail races cannot let older ledger state overwrite a newer unqualified snapshot; corrections are explicit and never fabricated trades.
- [ ] Default/HIP-3/spot-like ticker collisions remain separate canonical instruments; unsupported markets are explicitly excluded rather than claimed covered.
- [ ] Snapshot refresh does not fabricate entry time or complete history. Malformed/empty/missing states and equity have distinct outcomes.
- [ ] A real receipt-to-projection evidence capsule records source/knowledge cuts, revisions, coverage, resource budget and observed failures. Missing required source access is BLOCKED, not synthetic live qualification.
- [ ] Affected Rust/Python tests and integration regressions pass; update STATUS/capabilities with exactly what was qualified.

**Out of scope:** Episode classification, alpha testing, broad infrastructure replacement, trading or predictive promotion.

**Completion evidence:** Link the reproduced-risk fixtures, before/after contract checks, real-source capsule, exact test commands and paired tracker/engine commit references. Do not cite ledger unit tests alone as full live-path qualification.

**Execution contract:** Read tracker `AGENTS.md`, `docs/STATUS.md`, `docs/EXECUTION-REVIEW.md`, and applicable contracts before work. Inspect actual branch, worktree, remotes, issues/comments, and both repositories; preserve unrelated/concurrent changes. Tracker: `/home/quant/dev/hyperliquid-trader-tracker`; engine: `/home/quant/dev/hyperliquid-expert-ensemble`. Inspection baselines were tracker `2f948f8` and engine `ef8d7c1`, not assumptions about execution-time HEAD. Keep Rust authoritative for exchange state and Python for interpretation. No financial execution, production deployment, credential changes, new paid subscriptions, or predictive promotion is authorized by this issue alone. Keep raw wallet lists/credentials outside the public repository. Use the available native coding executor, independent subagents for bounded work, and Herdr for long deterministic verification where available. Implement and verify the scoped work rather than only planning; record exact revisions, commands, evidence hashes, and limitations. Separate code completion from empirical qualification and retain BLOCKED/INCONCLUSIVE/REJECTED results.

**Issue 2 -- ER1: Build additive point-in-time wallet path evidence**

**Depends on:** Issue 1 / ER0.
**Ownership:** Python wallet evidence/projection interpretation; Rust only for a demonstrated missing authoritative datum.
**Source:** Root plan ER1; execution review sections 4-6 and 8-9, especially F07-F08.

**Objective:** Make the additive exposure path authoritative; state and flow must work without OIDs, episode labels or inferred investment intent.

**Deliverables**
- Exact signed quantity transitions with OPEN/ADD/REDUCE/CLOSE/FLIP, forced/unknown cause, full scope, source references and separate event, receipt and feature-availability times.
- At cut (t,K), recent flow includes only event_time in (t-h,t] and known_at <= K under qualified coverage. Store raw net quantity and gross absolute quantity. Normalize accumulated values using the same compatible recorded price/equity anchor: F_h=(P*/E*)*sum(delta_q); G_h=(P*/E*)*sum(abs(delta_q)). Preserve normalization epochs and raw sums.
- Raw state/bias plus bounded views; initial 1m/5m/15m/1h flow, gross flow, net-to-gross ratio, recency, build rate, quantity excursion and recent-close indicator. Retain 4h/24h where required by the declared horizon without expanding the model arbitrarily.
- Per-feature missingness/coverage and denominator freshness; true position age, observed age and first-material/segment age remain distinct. Mark/equity/normalizer/correction changes are not new execution.
- Materiality gates prominence/interrupt eligibility only. Accumulate child fills before thresholds. Keep exact small transitions. Implement a versioned absolute-floor plus lagged wallet/position-relative candidate; sparse scale estimates use explicit cohort shrinkage/UNKNOWN. Values in the review are sensitivity parameters, not optima.

**Acceptance criteria**
- [ ] Repartitioning the same timed quantity path into partial fills/OIDs preserves state, net/gross flow and bounded influence. Nonlinear transforms never run per child before accumulation.
- [ ] OPEN/CLOSE within 20 seconds preserves both events, zero net quantity flow, positive gross activity and excursion despite changing prices/equity.
- [ ] +3 -> -2 creates one atomic FLIP with disjoint -3/-2 legs; a separate close/reopen retains its real flat interval. No raw-fill/virtual-leg double counting.
- [ ] +2x -> +1x remains long state plus reducing flow; many tiny adds can cumulatively cross materiality.
- [ ] Missing/stale/nonpositive/incompatible equity yields missing normalized flow, not zero. Snapshot recovery can restore state without falsely restoring past flow coverage.
- [ ] Old backfill does not become fresh flow; later enrichment/normalization cannot rewrite an earlier as-known evidence record. Future inputs are rejected.
- [ ] Mark/equity-only movement and reconciliation produce no discretionary execution flow; observed hedges/forced activity retain explicit cause/scope.
- [ ] Deterministic replay and compact inspect/explain output expose raw path, anchors, uncertainty, causes and source lineage.

**Out of scope:** Mandatory episode detection, skill weighting, new prediction policy or execution authority.

**Completion evidence:** Versioned WalletEvidence contract, focused invariance/late-data/missingness fixtures, replay parity results and exact implementation revisions.

**Execution contract:** Read tracker `AGENTS.md`, `docs/STATUS.md`, `docs/EXECUTION-REVIEW.md`, and applicable contracts before work. Inspect actual branch, worktree, remotes, issues/comments, and both repositories; preserve unrelated/concurrent changes. Tracker: `/home/quant/dev/hyperliquid-trader-tracker`; engine: `/home/quant/dev/hyperliquid-expert-ensemble`. Inspection baselines were tracker `2f948f8` and engine `ef8d7c1`, not assumptions about execution-time HEAD. Keep Rust authoritative for exchange state and Python for interpretation. No financial execution, production deployment, credential changes, new paid subscriptions, or predictive promotion is authorized by this issue alone. Keep raw wallet lists/credentials outside the public repository. Use the available native coding executor, independent subagents for bounded work, and Herdr for long deterministic verification where available. Implement and verify the scoped work rather than only planning; record exact revisions, commands, evidence hashes, and limitations. Separate code completion from empirical qualification and retain BLOCKED/INCONCLUSIVE/REJECTED results.

**Issue 3 -- ER2: Add optional execution-program and segment annotations**

**Depends on:** Issue 2 / ER1. Independent of ER3 after ER1; ER3 and R0/R1 research must not wait for optional metadata.
**Ownership:** Python execution annotations; Rust/provider enrichment only where a qualified source supplies missing factual metadata.
**Source:** Root plan ER2; execution review sections 6-7 and 9.

**Objective:** Explain execution without treating an inferred segment as a verified investment decision or prerequisite for additive evidence.

**Deliverables**
- Preserve observed OID/native-program links separately from conservative inferred directional segments; native programs, orders and position lifecycles need not form a tree. Do not fabricate IDs from public trade identifiers.
- Implement deterministic segment states CANDIDATE, ACTIVE, PAUSED, TERMINAL: same-direction accumulation; materiality activation; inactivity pause/resume; opposite-direction/lifecycle boundary termination; TIMEOUT_UNKNOWN versus INTERRUPTED. Late metadata yields a new annotation revision, never a rewritten past decision.
- Native program status follows qualified source facts. Concurrent native programs remain distinct; cancellation of unexecuted remainder does not reduce executed exposure. A parent can link several semantic segments or flip legs.
- Evidence labels NATIVE_CONFIRMED, SCHEDULE_LIKE, DIRECTIONAL_SEGMENT_UNKNOWN and INVENTORY_OR_MIXED. Use causal distinct bursts/orders, path/gap/size dispersion, duration and supported taker-notional fraction. Missing native IDs are not negative examples.
- Versioned, predeclared parameter sensitivities: initial 90s pause/10min end and a four-burst/two-minute schedule-like support guard are hypotheses only. Compare cadence-adaptive gaps using prior distinct bursts, not raw partial-fill arrivals.
- Outcome-blind detector evaluation against confirmed-native and sanitized manually audited discretionary/pyramiding/market-making snippets; report coverage, false merges/splits, delay and unknowns. Iceberg/hidden-size claims remain unavailable without qualified order-depth evidence.

**Acceptance criteria**
- [ ] Missing OID/native metadata does not block state, net/gross flow or ER3; base segment machinery works and the unavailable rich-source arm is explicitly BLOCKED/UNKNOWN.
- [ ] State transitions/timers replay deterministically; timeout completion becomes knowable only when the timer fires, never backdated to the last fill.
- [ ] Long-lived orders, pauses/repricing, dust counterflow, two simultaneous same-side programs, flip-through-zero and cancelling a remainder preserve accounting and parent linkage.
- [ ] Alternating market-making fills are not confidently relabeled as one discretionary direction. Detector labels cannot grant extra voting weight.
- [ ] Native-confirmed claims have qualified source references; schedule-like claims expose support/uncertainty rather than uncalibrated probabilities.
- [ ] Detector evaluation is frozen before predictive outcomes are inspected; no return-optimized segmentation. Feature-disabled replay matches ER1 exactly.

**Out of scope:** Universal intent recovery, HMM/HSMM by default, fills-only iceberg certainty or blocking simpler research on provider limitations.

**Completion evidence:** State-machine fixtures, annotation schema/revisions, source capability result and outcome-blind detector report. Close only implemented/tested scope; do not claim unavailable metadata has been qualified.

**Execution contract:** Read tracker `AGENTS.md`, `docs/STATUS.md`, `docs/EXECUTION-REVIEW.md`, and applicable contracts before work. Inspect actual branch, worktree, remotes, issues/comments, and both repositories; preserve unrelated/concurrent changes. Tracker: `/home/quant/dev/hyperliquid-trader-tracker`; engine: `/home/quant/dev/hyperliquid-expert-ensemble`. Inspection baselines were tracker `2f948f8` and engine `ef8d7c1`, not assumptions about execution-time HEAD. Keep Rust authoritative for exchange state and Python for interpretation. No financial execution, production deployment, credential changes, new paid subscriptions, or predictive promotion is authorized by this issue alone. Keep raw wallet lists/credentials outside the public repository. Use the available native coding executor, independent subagents for bounded work, and Herdr for long deterministic verification where available. Implement and verify the scoped work rather than only planning; record exact revisions, commands, evidence hashes, and limitations. Separate code completion from empirical qualification and retain BLOCKED/INCONCLUSIVE/REJECTED results.

**Issue 4 -- ER3: Integrate coherent evidence cuts and bounded cluster influence**

**Depends on:** Issue 2 / ER1. ER2 is optional and not a blocker.
**Ownership:** Python runtime, ensemble evidence, independence and existing inspection facade; Rust for authoritative watermarks/coverage.
**Source:** Root plan ER3; execution review sections 3-5, 7-9 and 11, especially F01/F09/F10.

**Objective:** Run an integrated descriptive V2 pipeline with coherent clocks, bounded contribution updates and explicit independence support, without enabling predictive trading.

**Deliverables**
- Aligned one-minute WalletEvidence and EnsembleEvidence cuts with named watermarks/revisions. Recompute window expiry, ages and missingness even for inactive wallets; do not mix stale precomputed flow windows as current.
- Coalesced material-event interrupts per instrument/cluster (initial 2-5s candidate), driven by unannounced accumulated change since the last emitted contribution. Large events replace current contributions, never append votes. Coverage invalidation is immediate; human notification cadence is separate.
- Preserve unbounded descriptive state/flow, but apply declared bounded accumulated-feature transforms before fixed within-cluster aggregation. No survivor amplification. Normalizer/cluster/coverage/window-expiry causes are not new trader executions.
- Lagged versioned behavioral grouping with explicit insufficient pair support, copying/lead-lag versus common-market distinctions, fixed budgets and unknown-redundancy mass. Report global and per-instrument/horizon observed/eligible/active groups. Weight breadth is concentration, not proven independence. Never tune clustering to a desired count.
- Freeze a prospective observed-wallet registry near 120 when qualified candidates/capacity exist; preserve the actual original approximately 50 subset and admission/retirement knowledge times. Do not silently substitute an outcome-selected roster or automatically expand to 200.
- Extend existing inspect/explain operations with coherent-cut lineage, additive change attribution, opposing/supporting/missing evidence and truthful IMPLEMENTED/QUALIFIED/PROPOSED/BLOCKED capability states. No new service or per-fill LLM.

**Acceptance criteria**
- [ ] A bounded real receipt -> authoritative state -> WalletEvidence -> EnsembleEvidence trace runs with the same code used by deterministic replay; V1 remains reproducible as a comparator.
- [ ] Same timed path with arbitrary fill/OID splitting gives identical budgets and contributions; a high-leverage wallet cannot dominate through uncapped flow.
- [ ] Fifteen clones, delayed followers, leader dropout, unknown pair support and a common news shock preserve fixed budgets and truthful breadth without declaring shared control from coincidence.
- [ ] Quiet-market expiry, late arrivals, stale evidence cuts, material-trigger storms and queued OPEN-then-CLOSE never emit obsolete current state or duplicate novelty.
- [ ] Snapshots/normalizer/cluster revisions with no fills are explained as non-execution changes; missing sources do not renormalize survivors.
- [ ] Measured coverage/latency/resource results at actual 50/120 availability are recorded, including recovery reserve and unmet capacity. No fabricated roster or load/edge qualification.
- [ ] Read-only inspection can explain a contribution back to its cluster/member/path/anchor/source; financial execution stays disabled.

**Completion evidence:** Integrated replay/live-source capsule, cadence/influence/clone regressions, private universe manifest hash, capability report and measured resource/coverage results.

**Execution contract:** Read tracker `AGENTS.md`, `docs/STATUS.md`, `docs/EXECUTION-REVIEW.md`, and applicable contracts before work. Inspect actual branch, worktree, remotes, issues/comments, and both repositories; preserve unrelated/concurrent changes. Tracker: `/home/quant/dev/hyperliquid-trader-tracker`; engine: `/home/quant/dev/hyperliquid-expert-ensemble`. Inspection baselines were tracker `2f948f8` and engine `ef8d7c1`, not assumptions about execution-time HEAD. Keep Rust authoritative for exchange state and Python for interpretation. No financial execution, production deployment, credential changes, new paid subscriptions, or predictive promotion is authorized by this issue alone. Keep raw wallet lists/credentials outside the public repository. Use the available native coding executor, independent subagents for bounded work, and Herdr for long deterministic verification where available. Implement and verify the scoped work rather than only planning; record exact revisions, commands, evidence hashes, and limitations. Separate code completion from empirical qualification and retain BLOCKED/INCONCLUSIVE/REJECTED results.

**Issue 5 -- ER4: Run the frozen state/path/episode representation experiment**

**Depends on:** Issues 2 and 4 / ER1 and ER3. Issue 3 / ER2 is required only for R2 annotations; R0/R1 must proceed independently.
**Ownership:** Python outcomes/ablation/research runner and immutable research evidence; narrow recorder additions only for required price availability.
**Source:** Root plan ER4; execution review sections 2 (F11-F13), 9-10.

**Objective:** Decide whether additive path features and optional execution annotations add predictive information beyond state and observable market trends.

**Deliverables**
- First qualify real evidence -> outcome plumbing: price event/receipt/knowledge time; explicit latency and maximum price lateness; invalid/censored outcomes; direction before costs; executable-price/cost/funding assumptions. Reject latency exceeding the evaluated horizon.
- Preregister R0 state-only; R1 state plus additive net/gross flow, recency/build rate; R2 R1 plus segment duration/pause/native-or-schedule status/supported taker fraction. Same one-minute instrument cuts, transparent frozen predictor/preprocessing, market-only control and exact current V1 comparator.
- Primary horizon 15m; secondary 1h; attach 1m/5m/15m/1h/4h/24h without post-hoc horizon selection. Use magnitude-sensitive held-out forecast metrics plus rank/conditional-return and realistic action diagnostics, not sign-only promotion.
- Prospective original-approximately-50 versus expanded-approximately-120 manifests, identical periods/eligibility, and predeclared cluster-preserving subsets. Log missingness, silence, rejected triggers, unavailable annotations and censored outcomes.
- Fit scales/clusters/detectors/models only on earlier inputs. Purge overlapping label intervals; preserve synchronized cross-asset/time dependence. Paired day/multi-day block uncertainty, leave-cluster-out and high-activity-day sensitivity; count all trials and retain negatives.
- Bounded resumable runner and immutable report with practical-effect threshold, support/stopping rule, hashes and terminal scientific status. Insufficient future observations remain pending/BLOCKED, not invented data or completed qualification.

**Acceptance criteria**
- [ ] Flat prices with positive costs lose money for both directions; shorts never reverse the cost sign. Long gaps, delayed price availability and latency beyond horizon are censored/rejected correctly.
- [ ] Evidence IDs/cuts and outcomes reproduce from sealed inputs; later corrections cannot repair earlier as-known predictions.
- [ ] The protocol, primary metric, minimum useful effect, model/search budget and actual available train/eval intervals are frozen before evaluation outcomes are inspected.
- [ ] Every representation uses common decision rows and comparable controls; R2 unavailable is recorded without blocking R0/R1. Wallet/fill counts are not independent sample counts.
- [ ] Outcome-mature real observations produce a paired report with uncertainty, support, coverage, costs and all registered failures/negative results. Synthetic tests validate mechanics only.
- [ ] PASS/REJECT/INCONCLUSIVE follows the registered rule; no automatic policy activation. A launched collection job or unfilled future holdout is not issue completion.

**Out of scope:** Adaptive reuse of this holdout for cadence selection, online self-training, forced profitability or deployment.

**Completion evidence:** Frozen manifest, signed-cost/availability regressions, reproducible runner, outcome-mature comparison report and honest research decision.

**Execution contract:** Read tracker `AGENTS.md`, `docs/STATUS.md`, `docs/EXECUTION-REVIEW.md`, and applicable contracts before work. Inspect actual branch, worktree, remotes, issues/comments, and both repositories; preserve unrelated/concurrent changes. Tracker: `/home/quant/dev/hyperliquid-trader-tracker`; engine: `/home/quant/dev/hyperliquid-expert-ensemble`. Inspection baselines were tracker `2f948f8` and engine `ef8d7c1`, not assumptions about execution-time HEAD. Keep Rust authoritative for exchange state and Python for interpretation. No financial execution, production deployment, credential changes, new paid subscriptions, or predictive promotion is authorized by this issue alone. Keep raw wallet lists/credentials outside the public repository. Use the available native coding executor, independent subagents for bounded work, and Herdr for long deterministic verification where available. Implement and verify the scoped work rather than only planning; record exact revisions, commands, evidence hashes, and limitations. Separate code completion from empirical qualification and retain BLOCKED/INCONCLUSIVE/REJECTED results.

**Issue 6 -- ER5: Compare scoring clocks and wallet-universe scale out of sample**

**Depends on:** Issue 5 / ER4. Freeze its simplest supported representation before this evaluation; do not reuse Stage 1 selection data as fresh validation.
**Ownership:** Python shadow/replay research and experimental clock policies; no production execution.
**Source:** Root plan ER5; execution review sections 3, 8 and 10.

**Objective:** Decide whether interrupts or exposure-event sampling outperform simple clocks, and whether approximately 120 observed wallets improve on the original approximately 50.

**Deliverables**
- Preregister fixed 60s, fixed 300s, 60s plus coalesced material interrupts, and normalized bounded cluster-activity bars with a maximum idle timeout. Freeze representation/normalization/model, predictor inputs and cost conventions across arms.
- Evaluate on a new untouched interval and the same one-minute market-time grid, using only each policy's latest actually available prediction. Supplement with executable-action replay for timing; extra emissions must not create favorable extra sample rows.
- Compare original and expanded prospective universes over identical periods, plus declared cluster-preserving subsets. Retain eligibility/unknown/missing budgets; distinguish wallet count from active supported information.
- Measure forecast value, latency, turnover/costs, funding/slippage assumptions, observation gaps, computation and delivery burden. Charge costs only on actual declared position changes, not unchanged score snapshots.
- Apply ER4 dependence-aware uncertainty and leakage controls. Report practical equivalence/uncertainty and marginal useful breadth/coverage; expansion to 200 requires a separate supported decision, not a count target.

**Acceptance criteria**
- [ ] Stage 2 preprocessing/representation/protocol are frozen before the new holdout; unavailable future data remains pending/BLOCKED. Do not adapt to Stage 2 results and call the same sample out of sample.
- [ ] Pure event/activity arms specify inactivity expiry, materiality reset, bounded cluster activity and missing-data handling; fill/OID/dollar-size fragmentation cannot control voting power.
- [ ] Common-grid comparisons use as-known availability, not future or backfilled values; interrupt-action replay preserves chronological fills/costs and cancels stale pending state.
- [ ] Sparse, bursty, rapid round-trip, clone/follower and source-gap fixtures pass for every clock. Fixed 300s is retained if practically equivalent; hybrid is not presumed superior.
- [ ] Outcome-mature paired results report intervals/effect sizes, sample support, coverage and resource tradeoffs for clocks and actual available universes.
- [ ] Recommend the simplest supported design or declare no decisive result; do not manufacture 40-60/50-70 clusters or imply more wallets guarantees edge.

**Out of scope:** New feature/model families, automatic 200-wallet expansion, production activation, or treating Stage 1 selection as independent confirmation.

**Completion evidence:** Sealed independent Stage 2 manifest, common-grid/action replay tests, measured comparison report and documented cadence/universe decision. A future collection schedule alone is not completion.

**Execution contract:** Read tracker `AGENTS.md`, `docs/STATUS.md`, `docs/EXECUTION-REVIEW.md`, and applicable contracts before work. Inspect actual branch, worktree, remotes, issues/comments, and both repositories; preserve unrelated/concurrent changes. Tracker: `/home/quant/dev/hyperliquid-trader-tracker`; engine: `/home/quant/dev/hyperliquid-expert-ensemble`. Inspection baselines were tracker `2f948f8` and engine `ef8d7c1`, not assumptions about execution-time HEAD. Keep Rust authoritative for exchange state and Python for interpretation. No financial execution, production deployment, credential changes, new paid subscriptions, or predictive promotion is authorized by this issue alone. Keep raw wallet lists/credentials outside the public repository. Use the available native coding executor, independent subagents for bounded work, and Herdr for long deterministic verification where available. Implement and verify the scoped work rather than only planning; record exact revisions, commands, evidence hashes, and limitations. Separate code completion from empirical qualification and retain BLOCKED/INCONCLUSIVE/REJECTED results.

**Issue 7 -- ER6: Decide predictive qualification without forced promotion**

**Depends on:** Issues 5 and 6 / ER4 and ER5 evidence. A documented negative/inconclusive result is an input; a missing future-data job is not completed evidence.
**Ownership:** Python predictive/signal/advisory gates, evidence index and operational documentation.
**Source:** Root plan ER6; execution review sections 10-12; historical V29-V31 evidence remains immutable.

**Objective:** Produce an auditable decision about the next qualified system, without converting engineering success into a prediction claim or financial authority.

**Deliverables**
- Consolidated acceptance matrix linking ER0-ER5 fixtures, code revisions, source/universe coverage, representation/cadence evidence, trial counts and limitations. Reconcile old V2 labels with what they actually tested; never erase prior failures or restate them as predictive passes.
- Deterministic fail-closed use of promoted PredictiveEvidence only. Retain the simplest descriptive baseline and explicit expected-return/calibration availability. Recheck replay, missingness, invalidation and signal-state policy gates.
- If evidence earns further evaluation, freeze a candidate and define the distinct live-forward V30 gate; implement/qualify only within authorized shadow scope. Production advisory V31 requires separate explicit approval, operational rollback and its own acceptance evidence. Do not reopen/duplicate historical issues without an explicit scope decision.
- If edge is absent or unresolved, document REJECTED/INCONCLUSIVE and keep predictive emission disabled. This is a valid scientific conclusion, not permission to tune until profitable.
- Final inspect/explain/evaluate entrypoints, compact evidence lineage, capability statuses, runbook and regression-linked lessons so a new agent can establish the system state without reconstructing past chats.

**Acceptance criteria**
- [ ] The conclusion is fully traceable to frozen real evidence and separate engineering, descriptive, historical-predictive, live-forward and production-advisory qualifications.
- [ ] No promoted feature/policy exists solely because tests passed, a milestone closed, sample counts grew or a positive unqualified mean appeared.
- [ ] Negative/inconclusive research leaves descriptive operation useful and predictive emission disabled; pending future observations are clearly identified rather than marked passed.
- [ ] Any further live-forward candidate has frozen inputs/policy/latency/cost assumptions, measurable invalidation and a separate prospective gate; no unapproved production change occurs.
- [ ] Financial execution remains forbidden; no account target or deployment is activated as a side effect of this issue. Unknown expected returns remain null until calibrated.
- [ ] Documentation accurately lists implemented versus qualified capabilities, rollback/failure procedures, source limitations and the next evidence-dependent frontier.

**Completion evidence:** Signed/versioned qualification decision, cross-repository acceptance/evidence map and cold-start inspection path. Completion means a defensible decision, including rejection; it does not require a profitable signal or production promotion.

**Execution contract:** Read tracker `AGENTS.md`, `docs/STATUS.md`, `docs/EXECUTION-REVIEW.md`, and applicable contracts before work. Inspect actual branch, worktree, remotes, issues/comments, and both repositories; preserve unrelated/concurrent changes. Tracker: `/home/quant/dev/hyperliquid-trader-tracker`; engine: `/home/quant/dev/hyperliquid-expert-ensemble`. Inspection baselines were tracker `2f948f8` and engine `ef8d7c1`, not assumptions about execution-time HEAD. Keep Rust authoritative for exchange state and Python for interpretation. No financial execution, production deployment, credential changes, new paid subscriptions, or predictive promotion is authorized by this issue alone. Keep raw wallet lists/credentials outside the public repository. Use the available native coding executor, independent subagents for bounded work, and Herdr for long deterministic verification where available. Implement and verify the scoped work rather than only planning; record exact revisions, commands, evidence hashes, and limitations. Separate code completion from empirical qualification and retain BLOCKED/INCONCLUSIVE/REJECTED results.
