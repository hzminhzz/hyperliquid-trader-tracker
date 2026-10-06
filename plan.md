# Forward-qualified expert signal engine: V2 implementation/research plan

**Status:** V2 plan with a proposed execution/research amendment, 2026-10-06. Historical slice plans and evidence remain below; their completion must be read from the evidence index, not inferred from this plan. No implementation, issue creation, deployment or predictive promotion is authorized by this documentation review.

## Proposed next frontier: execution correctness before richer scoring

Read [EXECUTION-REVIEW](docs/EXECUTION-REVIEW.md) for the inspected findings, proposed state machines and registered experiment design. Current inspected heads are tracker `2f948f8` and engine `ef8d7c1`; the engine's 79 passing tests do not establish live V2 integration or predictive value. [STATUS](docs/STATUS.md) owns the current claim boundary.

Execution status on the milestone branch: ER0-ER6 implementation/protocol slices are complete. ER0/ER2/ER3 empirical live-source sub-gates remain BLOCKED by absent recorded traffic; ER4 has no prospective outcome-mature rows; ER5 has no independent untouched Stage-2 holdout; ER6 therefore keeps predictive emission disabled. See `docs/evidence/execution-review/`. New data may advance the evidence gates later without rewriting these completed implementation slices.

The following are the materialized execution work packages; their acceptance evidence, not this table, determines completion:

| Package | Depends on | Deliverable and acceptance |
|---|---|---|
| ER0: qualify source and state authority | None | Pin source field/coverage/rate capabilities; fault-inject receipt and transition failures, duplicate/restart/counterparty boundaries and snapshot races. Rust owns scoped position/equity/reconciliation authority. Demonstrate one real receipt-to-projection path. No new live-trading authority. |
| ER1: additive path evidence | ER0 | Preserve exact quantity transitions, net/gross flow, true/unknown ages, event-versus-knowledge time and normalization provenance. Split invariance, round-trip cancellation, stale-equity missingness and old-backfill non-freshness must pass. |
| ER2: optional execution annotations | ER1 | Explicit native parent links plus conservative causal segments; no requirement that a detector succeeds before state/flow exist. Qualify false merges/splits and delay without outcome-based tuning. Iceberg labels remain unavailable without the necessary source. |
| ER3: coherent cuts and bounded contributions | ER1; annotations optional | One-minute common cuts; deterministic expiry; coalesced material interrupts; raw descriptive flow plus bounded influence; supported per-instrument breadth and unknown redundancy. Clone, leader/follower, stale-cut and simultaneous-market-event fixtures pass. |
| ER4: representation experiment | ER1 and ER3; ER2 only for the annotation arm | Preregister state-only, additive path, and path-plus-annotation comparisons at identical cuts; market-only comparator, prospective approximately 50/120 wallet cuts, signed-cost and outcome-availability checks, dependence-aware intervals. Retain INCONCLUSIVE/REJECTED outcomes. |
| ER5: cadence and universe experiment | ER4 | Freeze the simplest supported representation and use a new untouched evaluation interval for 60s, 300s, hybrid interrupts and exposure-event clocks. Evaluate common market-time rows plus actual-action replay; report latency, costs, missingness and marginal universe value. |
| ER6: reconsider predictive qualification | ER4/ER5 evidence | Continue V29-V31 only when their economic and operational predicates are actually satisfied. Do not equate a descriptive engine or passing synthetic tests with qualification. |

Approximately 120 observed wallets is a research budget, not a required voter count. The original approximately 50 remain a comparator. Do not force clustering to yield 50-70 groups, and do not expand to 200 without measurable novelty/coverage or incremental value. One-minute and segmentation thresholds are test parameters, not proven optima. No fill is dropped by a materiality gate.

The previous V20-V31 issue materialization section is retained for traceability. Before any implementation, reconcile these packages with actual open issues and current code; do not blindly recreate or close historical issues.

## Goal and terminal predicate

Transform the existing expert-ensemble from a bounded descriptive consensus into a point-in-time-correct, causally legible, forward-qualified signal engine without weakening observation/replay correctness or enabling financial execution.

V2 is complete only when:
- descriptive bias/state/flow evidence is replayable and inspectable;
- the B0-B4 representation ladder can be evaluated on frozen point-in-time manifests;
- every emitted candidate signal can receive immutable multi-horizon outcomes;
- at least one deterministic signal policy clears historical and live-forward gates after realistic latency/costs;
- production advisory promotion is explicit and reversible;
- simpler baselines remain available as comparators;
- negative results are retained.

If no candidate demonstrates forward predictive value, V2 research may terminate with the descriptive engine plus a rejected signal-policy result. That is a valid scientific outcome.

Authorities: [SYSTEM](docs/SYSTEM.md), [CONTEXT](CONTEXT.md), [CONTRACTS](docs/CONTRACTS.md), [RESEARCH](docs/RESEARCH.md), [QUALIFICATION](docs/QUALIFICATION.md), [OPERATIONS](docs/OPERATIONS.md).

## Historical V2 starting point

- Tracker repository main: `9b55228`.
- Expert-engine repository main inspected at `bf88f7a`.
- Existing expert-engine suite: 39 tests passing at inspection.
- Live production runtime code currently computes only equal-budget V1 consensus.
- B2/B3/B4 mechanics exist as research functions, but their synthetic tests establish engineering behavior, not forward predictive edge.
- Current hard-coded research/runtime thresholds are candidate parameters, not proven optima.
- Rust observation ledger remains authoritative; do not redesign ingestion for V2 unless a required signal datum is unavailable from the contract.

## Design constraints

1. Preserve raw information before bounded aggregation.
2. State and flow are separate products.
3. Cluster is the unit of independent influence after redundancy adjustment.
4. Skill is separate from capital size and current conviction.
5. Confidence, signal strength, crowding risk and expected return are separate.
6. No threshold is called optimal without a registered experiment.
7. No LLM participates in scoring, accounting, signal-state transitions or risk enforcement.
8. No financial execution is added.
9. Research code uses sealed point-in-time manifests and reusable persisted features.
10. A later rung cannot erase or hide its simpler baseline.

## Dependency graph

```text
V20 inspect/freeze baseline
        |
        v
V21 WalletEvidence: raw bias + intent semantics
        |
        +-------------------+
        |                   |
        v                   v
V22 independence       V23 relative conviction
        \                   /
         \                 /
          v               v
            V24 EnsembleEvidence
                    |
                    v
            V25 outcome ledger
                    |
                    v
            V26 B0-B4 ablations
                    |
          +---------+---------+
          |                   |
          v                   v
 V27 cohort/skill       V28 context/crowding
          \                   /
           \                 /
            v               v
             V29 signal policy
                    |
                    v
             V30 live-forward
                    |
                    v
             V31 advisory promotion
```

V27 and V28 are optional until B0-B4 show useful structure. V29 may terminate as REJECTED if no evidence set earns predictive use.

## Work slices

### V20 — Freeze the V1 baseline and evidence semantics

**Depends on:** none.

**Deliverable**
- capture tracker/engine revisions, deployed runtime path, feature paths and current candidate constants;
- register B0 as the exact V1 live equal-budget bounded consensus;
- classify existing Q10/Q11/Q15 evidence by qualification class;
- record current wallet universe selection semantics and whether historical studies are conditional fixed-universe or true PIT selection.

**Verify**
- Q01;
- existing test suites remain green;
- no service/config/source changes.

**Exit**
- later experiments can reproduce B0 exactly and cannot cite synthetic candidate tests as predictive evidence.

### V21 — Deepen WalletEvidence

**Depends on:** V20.

**Scope**
Python expert-engine projection/feature layer only unless tracker contract lacks an indispensable event field.

**Deliverable**
- canonical WalletEvidence artifact;
- raw unclipped bias retained;
- bounded influence as a named baseline transform;
- portfolio share over observable scope;
- OPEN/ADD/REDUCE/CLOSE/FLIP intent events;
- multi-horizon flow features;
- position age, intent age and observation age;
- explicit cause separation for economic change vs valuation/equity/reconciliation.

**Verify**
Q04-Q07 and adversarial cases:
- +2x -> +1x;
- unchanged +0.7 state;
- tiny add on huge old position;
- reconciliation correcting quantity;
- mark/equity-only changes.

**Exit**
No information required by later research is destroyed by the posture abstraction.

### V22 — Independence artifact V2

**Depends on:** V21.

**Deliverable**
- versioned similarity feature schema using intent/state behavior;
- support measured in active steps and episodes;
- complete-link baseline retained;
- effective breadth;
- UNKNOWN similarity/newcomer policy;
- PIT cluster training cutoff/update schedule.

**Verify**
Q08 plus clone, chain-link, low-support and strategy-drift fixtures.

**Exit**
15 clones cannot become 15 independent votes, and historical as-known replay cannot use future cluster knowledge.

### V23 — Trader-relative conviction

**Depends on:** V21.

**Deliverable**
- lagged per-expert/per-instrument bias distribution artifact;
- robust percentile/rank feature;
- minimum support and UNKNOWN semantics;
- drift diagnostics;
- no current-sample leakage.

**Verify**
Q09.

**Exit**
System can answer whether current exposure is unusual for the trader without claiming that unusual means predictive.

### V24 — Canonical EnsembleEvidence

**Depends on:** V22, V23.

**Deliverable**
One deep interface that assembles:
- state evidence;
- flow evidence by horizon;
- relative conviction summaries;
- independent breadth;
- supporting/opposing cluster contributions;
- missing/reliability diagnostics;
- B0/B1/B2 comparators;
- hooks for optional skill/context fields;
- exact lineage and revisions.

Do not collapse these into one score.

**Verify**
Q11 and agent explanation fixtures.

**Exit**
A caller can understand the situation without reassembling lower-level modules.

### V25 — Forward outcome ledger

**Depends on:** V24.

**Deliverable**
- immutable OutcomeRecord attachment for every evidence/signal emission;
- 1m, 5m, 15m, 1h, 4h, 24h outcomes;
- latency-adjusted and cost-adjusted returns;
- MFE/MAE;
- context/coverage at emission;
- late correction flag;
- incremental/resumable computation over sealed manifests.

**Verify**
Q12; outcome attachment cannot mutate historical feature values or knowledge times.

**Exit**
Every research question can use the same reusable outcome substrate instead of one-off scripts.

### V26 — B0-B4 frozen ablation program

**Depends on:** V25.

**Deliverable**
Run registered paired evaluations:
- B0 V1 bounded consensus;
- B1 raw-state representation;
- B2 independent-state;
- B3 independent-flow;
- B4 relative-conviction.

Required comparisons:
- clipped-only vs raw-preserved;
- state vs flow vs state+flow;
- equal-wallet vs cluster-aware;
- absolute vs relative conviction;
- latency/cost sensitivity;
- expert-dropout sensitivity.

**Verify**
Q13-Q16.

**Exit**
Each rung is PROMOTE, REJECT or INCONCLUSIVE with retained evidence. No parameter tuning on the final holdout.

### V27 — Skill/cohort divergence candidate

**Depends on:** V26 and only starts if prior evidence justifies it.

**Deliverable**
- PIT alpha/control/anti-alpha cohort artifact;
- lagged horizon-specific skill metrics with shrinkage;
- drift/decay policy;
- skill divergence feature;
- cluster-budget-preserving optional quality weighting.

**Verify**
Q10, Q17 and equal-weight ablation.

**Exit**
Use in predictive evidence only if incremental holdout value is robust; otherwise retain as research/descriptive metadata.

### V28 — Market divergence and crowding candidates

**Depends on:** V26 and only starts if data are available point-in-time.

**Deliverable**
Add one family per registered experiment:
- funding;
- OI;
- volatility/price response;
- liquidation/crowding metrics only when provenance/support are adequate.

Separate:
- directional predictor use;
- risk-gate use.

**Verify**
Q18 plus missing/context-staleness cases.

**Exit**
Only promoted families enter PredictiveEvidence.

### V29 — Deterministic signal state machine

**Depends on:** V26 plus any promoted V27/V28 features.

**Deliverable**
- canonical TradeSignal artifact;
- FLAT/LONG/SHORT state;
- ENTER/INCREASE/REDUCE/EXIT/REVERSE/NONE event;
- explicit signal strength;
- structured confidence components;
- crowding risk separate;
- expected_return NULL unless calibration passes;
- invalidation conditions;
- baseline comparators;
- versioned thresholds/hysteresis.

**Verify**
Q19-Q20, Q24-Q25.

**Exit**
Historical holdout demonstrates useful signal behavior after latency/costs, or the signal policy is explicitly rejected.

### V30 — Frozen live-forward qualification

**Depends on:** V29 historical promotion.

**Deliverable**
Freeze one candidate before live shadow evaluation. Auto-attach outcomes and record outages/missing experts. No retuning the same candidate during its forward clock.

**Verify**
Q21-Q23.

**Exit**
FORWARD-QUALIFIED only if the registered live-forward rule passes. Otherwise retain/reject and start a new candidate version if further research is justified.

### V31 — Production advisory promotion

**Depends on:** V30.

**Deliverable**
- policy approval artifact;
- advisory runtime selects explicit promoted signal revision;
- explanations show current model plus B0/B1/B2 comparators;
- rollback to prior qualified policy;
- degraded/missing-data behavior;
- Telegram/inspection surfaces show signal state/event and evidence, not only target magnitude.

**Verify**
Q24-Q26 and existing authority/replay/load gates.

**Exit**
May be called **production advisory signal engine**. Financial execution remains unavailable.

## What is explicitly not in this plan

- automatic order execution;
- automatic portfolio authority;
- end-to-end neural/LLM scoring;
- online self-training weights;
- general-purpose feature platform;
- new message broker/microservices for scoring;
- auto-discovery claims without PIT selection history;
- expected-return numbers without calibration;
- universal fixed promotion constants;
- capital-weighted voting;
- single opaque confidence score.

## Acceptance-name mapping

**Descriptive bias engine**
V20-V24 pass Q03-Q11/Q25 descriptive cases.

**Signal-engine MVP**
Descriptive engine + V25 + runnable V26 research ladder + typed TradeSignal interface, even if predictive emission remains disabled.

**Forward-qualified signal engine**
Relevant V26-V29 historical gates plus V30 live-forward gate pass.

**Production advisory signal engine**
Forward-qualified + V31 operational/approval gates pass.

## Execution guidance

When implementation is explicitly authorized:
- inspect actual tracker and expert-engine branches first;
- preserve concurrent changes;
- implement in the expert-engine repository by default;
- touch tracker contracts only if V21 proves required information is unavailable;
- work dependency-aware from the ready frontier;
- use bounded parallel subagents only for independent slices;
- use reusable sealed manifests/checkpoints for long experiments;
- diagnose failures and continue rather than weakening predicates;
- review diff against the exact slice acceptance criteria;
- commit/push coherent completed work only when authorized;
- retain FAIL/BLOCKED/INCONCLUSIVE evidence;
- recompute the frontier after each accepted slice.

# V1 history

The prior Agent-operable expert ensemble V1 milestone remains historical evidence of the observation, persistence, projection, baseline consensus, clone-resistance, advisory and operations infrastructure. It is not deleted or reinterpreted as proof of predictive edge. V2 begins from that qualified engineering substrate.

### Milestone: Forward-qualified expert signal engine V2

**Issue 1 — V20: Freeze the V1 baseline and evidence semantics**

**Depends on:** none

**Goal:** establish the immutable B0/V1 reference and stop engineering evidence from being mistaken for predictive evidence.

**Deliverable**
- capture tracker/engine revisions, deployed runtime path, feature paths, universe semantics, and current candidate constants;
- register B0 as the exact V1 live equal-budget bounded consensus;
- classify existing Q10/Q11/Q15 evidence by qualification class;
- label historical universe studies as conditional fixed-universe or point-in-time selection.

**Acceptance**
- Q01 passes;
- current tracker and expert-engine tests remain green;
- no service/config/source changes;
- B0 can be reproduced exactly from a pinned manifest.

---

**Issue 2 — V21: Deepen WalletEvidence**

**Depends on:** Issue 1 / V20

**Goal:** preserve the descriptive information currently lost or blurred by posture-only scoring.

**Deliverable**
- canonical WalletEvidence artifact;
- raw unclipped bias plus named bounded-influence transform;
- observable portfolio share;
- OPEN/ADD/REDUCE/CLOSE/FLIP intent semantics;
- multi-horizon intent flow;
- position age, intent age, observation age;
- explicit economic-change vs valuation/equity/reconciliation causes.

**Acceptance**
- Q04-Q07 pass;
- +2x -> +1x is REDUCE with positive state and negative flow;
- unchanged state cannot repeatedly create flow;
- tiny add on a large old position remains a small flow event;
- reconciliation and mark/equity-only changes cannot fabricate intent.

---

**Issue 3 — V22: Build the independence artifact V2**

**Depends on:** Issue 2 / V21

**Goal:** make redundancy and independent breadth explicit and point-in-time correct.

**Deliverable**
- versioned intent/state similarity schema;
- active-step and episode support diagnostics;
- deterministic complete-link baseline;
- effective breadth;
- UNKNOWN-similarity/newcomer policy;
- point-in-time cluster training cutoff and update schedule.

**Acceptance**
- Q08 passes;
- clone, chain-link, low-support, and drift fixtures pass;
- 15 clones cannot gain 15 independent votes;
- as-known replay cannot use future cluster knowledge.

---

**Issue 4 — V23: Add trader-relative conviction**

**Depends on:** Issue 2 / V21

**Goal:** distinguish routine exposure from exposure that is unusual for the same trader.

**Deliverable**
- lagged expert/instrument bias-distribution artifact;
- robust percentile/rank feature;
- minimum-support and UNKNOWN semantics;
- drift diagnostics;
- strict no-current/no-future leakage.

**Acceptance**
- Q09 passes;
- low-support experts remain UNKNOWN rather than neutral;
- historical replay reproduces only distributions knowable at the decision cut.

---

**Issue 5 — V24: Create canonical EnsembleEvidence**

**Depends on:** Issues 3-4 / V22-V23

**Goal:** expose one deep, inspectable ensemble interface without prematurely collapsing evidence into a magic score.

**Deliverable**
- state evidence;
- flow evidence by horizon;
- relative-conviction summaries;
- independent breadth;
- supporting/opposing cluster contributions;
- missing/reliability diagnostics;
- B0/B1/B2 comparators;
- exact evidence lineage and revision handles;
- extension slots for later skill/context features.

**Acceptance**
- Q11 and agent-explanation fixtures pass;
- callers can explain the ensemble without reconstructing lower-level modules;
- state, flow, confidence/support, and crowding remain separable.

---

**Issue 6 — V25: Build the forward outcome ledger**

**Depends on:** Issue 5 / V24

**Goal:** make all future research reuse one immutable outcome substrate.

**Deliverable**
- OutcomeRecord keyed to evidence/signal IDs;
- 1m, 5m, 15m, 1h, 4h, 24h forward returns;
- declared-latency and cost-adjusted returns;
- MFE/MAE;
- context/coverage at emission;
- late-correction/restatement flag;
- incremental resumable computation over sealed manifests.

**Acceptance**
- Q12 passes;
- attaching outcomes never mutates historical features, timestamps, or decisions;
- identical sealed manifests produce identical outcome joins.

---

**Issue 7 — V26: Run the frozen B0-B4 ablation program**

**Depends on:** Issue 6 / V25

**Goal:** test whether the simpler descriptive representations contain real predictive information before adding more complexity.

**Deliverable**
- registered paired evaluations for B0 V1 bounded consensus, B1 raw state, B2 independent state, B3 independent flow, and B4 relative conviction;
- clipped-only vs raw-preserved;
- state vs flow vs state+flow;
- equal-wallet vs cluster-aware;
- absolute vs relative conviction;
- latency/cost sensitivity;
- expert-dropout sensitivity.

**Acceptance**
- Q13-Q16 pass;
- every rung ends PROMOTE, REJECT, or INCONCLUSIVE with retained evidence;
- final holdout is never retuned;
- negative results and search counts are retained.

---

**Issue 8 — V27: Test skill and cohort divergence**

**Depends on:** Issue 7 / V26, and starts only if prior evidence justifies added complexity

**Goal:** test whether lagged skill cohorts add information beyond B4.

**Deliverable**
- point-in-time alpha/control/anti-alpha cohort artifact;
- lagged horizon-specific skill metrics with shrinkage;
- drift/decay policy;
- skill-divergence feature;
- cluster-budget-preserving optional quality weighting.

**Acceptance**
- Q10 and Q17 pass;
- equal-weight ablation is retained;
- only robust incremental holdout value permits predictive use;
- otherwise cohort data remains descriptive/research-only.

---

**Issue 9 — V28: Test market divergence and crowding context**

**Depends on:** Issue 7 / V26, and starts only when point-in-time context data are available

**Goal:** determine whether market context improves prediction or should remain only a risk control.

**Deliverable**
- one registered experiment per context family: funding, OI, volatility/price response, and supported liquidation/crowding inputs;
- separate directional-predictor and risk-gate variants;
- provenance/freshness semantics for every context feature.

**Acceptance**
- Q18 passes;
- missing/stale context fails visibly;
- only feature families with incremental holdout value enter PredictiveEvidence.

---

**Issue 10 — V29: Build the deterministic TradeSignal state machine**

**Depends on:** Issue 7 / V26 plus any promoted Issue 8-9 features

**Goal:** convert promoted predictive evidence into an account-independent, deterministic signal without hiding its causes.

**Deliverable**
- TradeSignal artifact;
- FLAT/LONG/SHORT state;
- ENTER/INCREASE/REDUCE/EXIT/REVERSE/NONE event;
- signal strength;
- structured confidence/support;
- separate crowding risk;
- expected_return NULL unless calibrated;
- invalidation conditions;
- simpler baseline comparators;
- versioned thresholds and hysteresis.

**Acceptance**
- Q19-Q20 and Q24-Q25 pass;
- persistence alone emits NONE;
- REDUCE can retain direction;
- REVERSE requires the declared opposite-side transition;
- historical holdout remains useful after realistic latency/costs, or the policy is explicitly rejected.

---

**Issue 11 — V30: Freeze and run live-forward qualification**

**Depends on:** Issue 10 / V29 historical promotion

**Goal:** verify the frozen candidate on genuinely new shadow data without forward-period retuning.

**Deliverable**
- frozen code/feature/model/policy revisions before the forward clock;
- automatic outcome attachment;
- outage, missing-expert, and drift evidence retained;
- version splits for any correctness fixes.

**Acceptance**
- Q21-Q23 pass;
- no threshold/model retuning is attributed to the same forward candidate;
- FORWARD-QUALIFIED is granted only by the registered live-forward rule.

---

**Issue 12 — V31: Promote the qualified signal to production advisory**

**Depends on:** Issue 11 / V30

**Goal:** make the forward-qualified signal the explicit advisory policy with safe rollback and complete explanations.

**Deliverable**
- policy approval artifact;
- runtime selection of one explicit promoted signal revision;
- current signal plus B0/B1/B2 comparators in inspection/explanations;
- rollback to the prior qualified policy;
- verified degraded/missing-data behavior;
- Telegram/inspection surfaces show signal state, event, evidence, and blockers.

**Acceptance**
- Q24-Q26 and existing authority/replay/load gates pass;
- production advisory claim is allowed only after forward qualification and approval;
- financial execution remains forbidden.
