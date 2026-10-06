# V2 forward-qualified expert signal engine

**Date:** 2026-10-06  
**Status:** V2 design with a proposed execution/sampling amendment. No implementation or predictive promotion is authorized by this document.

## Execution-review amendment, 2026-10-06

The current review is [EXECUTION-REVIEW](EXECUTION-REVIEW.md), with inspected code findings in its section 2 and proposed acceptance dependencies in [plan.md](../plan.md). The baseline critique below predates the current tracker `2f948f8` and engine `ef8d7c1`; see [STATUS](STATUS.md) for present implementation and test claims.

The proposed next-stage representation is an additive, point-in-time quantity path with optional order/program/segment annotations, not a mandatory fill-to-order-to-intent hierarchy. Execution does not prove discretionary belief. Native linkage is stronger evidence than a timing heuristic, but even a confirmed program can cross several position lifecycles.

State, net/gross flow and missingness must work without episode classification. Materiality controls prominence and coalesced interrupts, never observation retention. Normalize accumulated quantities using compatible common anchors; preserve event recency separately from receipt and knowledge time. Bound flow influence as well as state influence, without discarding raw measurements.

The initial proposed cadence is aligned one-minute wallet and ensemble evidence with bounded material-event interrupts. Five-minute scoring and exposure-event bars are registered comparators, not presumed inferior or superior. Approximately 120 observed wallets and 40-60 supported global behavioral groups are planning hypotheses; per-instrument active support must be measured and clustering must not be tuned to a desired count.

Live source/durability/snapshot ownership and outcome-cost correctness must be qualified before the next predictive experiment. Existing V2 functions and passing tests are not evidence that the live V2 pipeline is integrated or profitable. The exact state machines, detector uncertainty, adversarial cases and two-stage experiment are specified in EXECUTION-REVIEW; they remain proposed until accepted.

This document is the compact synthesis of the V2 redesign. Domain terms live in [CONTEXT](../CONTEXT.md); normative research semantics in [RESEARCH](RESEARCH.md); acceptance in [QUALIFICATION](QUALIFICATION.md); execution order in [plan](../plan.md).

## A. Critique of the current scoring model

### Verified facts

At inspected head:
- the Python production runtime computes per-wallet `raw_exposure = quantity * valuation_price / equity`, then `clip(raw_exposure / k, -1, 1)`;
- the production runtime aggregates those postures through equal-budget consensus;
- the runtime does not currently use the B2 cluster consensus, B3 quality weighting, or B4 regime rule;
- raw exposure is retained inside `ExpertPosture`, but downstream consensus contributions expose only the bounded posture;
- position projection contains position state but no canonical flow/intent artifact;
- clustering uses mean absolute bounded-posture distance and complete-link clustering;
- B3 quality weights use lagged profit factor with a minimum-support fallback;
- B4 high-volatility conditioning is a fixed 0.70 multiplier;
- existing engine tests pass, but candidate research tests are synthetic engineering tests rather than forward-return validation.

### Strengths worth preserving

1. Missing experts do not amplify survivors through renormalization.
2. Flat, abstaining and unavailable states are distinct.
3. Cluster budgets can neutralize exact clone inflation.
4. Replay and evidence lineage already exist.
5. The live path is deterministic and contains no LLM.
6. Rust/Python ownership is correctly separated.

### Main information losses / conceptual errors

1. **Early clipping discards leverage magnitude.** +1x and +5x become the same vote even though the difference may matter descriptively or predictively.
2. **State dominates the representation.** A persistent +0.7 and a fresh jump +0.05 -> +0.70 can look similar despite very different information arrival.
3. **Intent and passive drift are not canonical products.** A position change, mark change, equity change and reconciliation need different semantics.
4. **No trader-relative scale.** Absolute 0.8x may be exceptional for one trader and routine for another.
5. **No canonical portfolio concentration.** Same asset bias can mean concentrated conviction or one leg in a broad book.
6. **Independent breadth is under-expressed.** Cluster count/influence exists, but support/confidence needs effective independent breadth and unknown-similarity handling.
7. **Skill weighting is mechanically implemented before economic proof.** Profit factor is not automatically the right forecasting-quality statistic.
8. **Crowding/context is conflated with risk dampening in B4.** A fixed volatility multiplier is a policy hypothesis, not a validated law.
9. **Research qualification conflated mechanics and edge.** Passing a synthetic promotion test does not show forward predictive value.
10. **No account-independent TradeSignal state machine.** The current target magnitude is not yet a signal with explicit ENTER/REDUCE/EXIT/REVERSE semantics.

## B. Conceptual model

The system should model **information arrival from independent expert strategies**, not merely average current exposure.

Core decomposition:

```text
Observation truth
  -> WalletEvidence
       state: raw bias, concentration, age
       flow: OPEN/ADD/REDUCE/CLOSE/FLIP deltas
       uncertainty
  -> IndependenceArtifact
       redundancy, cluster budgets, breadth
  -> Relative/Cohort artifacts
       unusual-for-trader exposure
       lagged skill cohorts
  -> EnsembleEvidence
       independent state
       independent flow
       divergence
       crowding/context
       missing/support
  -> PredictiveEvidence
       promoted features only
  -> deterministic SignalPolicy
  -> TradeSignal
  -> optional AccountAdviser
```

The central design rule is **preserve descriptive dimensions until evidence proves a useful predictive compression**.

## C. Mathematical definitions

For expert i, asset a, time t:

**Raw bias**
[
b_{i,a,t}=q_{i,a,t}p_{a,t}/E_{i,t}
]

**Bounded influence**
[
v_{i,a,t}(k)=clip(b_{i,a,t}/k,-1,1)
]

**Observed portfolio share**
[
c_{i,a,t}=|N_{i,a,t}|/sum_j|N_{i,j,t}|
]

**Intent flow over horizon h**
[
f^{(h)}_{i,a,t}=sum_e Delta b^{intent}_{i,a,e}K_h(t-known(e))
]

**Relative conviction**
[
r_{i,a,t}=F^{past}_{i,a}(|b_{i,a,t}|)
]

**Cluster state/flow**
[
X_{g}=sum_{iin g}a(i|g)X_i
]

**Independent ensemble evidence**
[
X_{ens}=sum_gw_gX_g
]

**Effective breadth**
[
B_{eff}=1/sum_gw_g^2
]

**Skill divergence candidate**
[
D^{skill}=X_{alpha}-X_{anti}
]

**Market divergence candidate**
[
D^{mkt}=X_{experts}-X_{market}
]

No formula above is automatically a prediction. A signal policy may use only empirically promoted features.

## D. Tower of abstractions

| Layer | Product | Descriptive/predictive | What it discards |
|---|---|---|---|
| L0 | Observation ledger | factual | nothing intentionally beyond source validation |
| L1 | Account view | factual | transport detail not needed by account state |
| L2 | WalletEvidence | descriptive | no raw-bias loss; bounded influence is an additional view |
| L3 | IndependenceArtifact | descriptive/research | duplicate influence; keeps support diagnostics |
| L4 | Relative/Cohort artifacts | descriptive/research | no current/future leakage permitted |
| L5 | EnsembleEvidence | descriptive | preserves state, flow, support, divergence separately |
| L6 | PredictiveEvidence | predictive | excludes unpromoted feature families |
| L7 | TradeSignal | predictive policy decision | compresses evidence into state/event but retains lineage/comparators |
| L8 | AdvisoryTarget | account-specific | applies account constraints; does not alter signal history |
| L9 | OutcomeRecord | evaluation | arrives later; cannot mutate prior decisions |

All artifacts carry as_of, knowledge_time, revisions and evidence refs.

## E. Canonical typed artifacts

Required V2 contracts:

1. `WalletEvidence`
2. `IndependenceArtifact`
3. `CohortArtifact`
4. `EnsembleEvidence`
5. `PredictiveEvidence`
6. `TradeSignal`
7. `OutcomeRecord`

The most important contract is:

```text
TradeSignal {
  signal_id
  instrument_id
  horizon
  state: FLAT|LONG|SHORT
  event: ENTER|INCREASE|REDUCE|EXIT|REVERSE|NONE

  state_evidence
  flow_evidence
  relative_conviction
  independent_breadth
  skill_divergence?
  market_divergence?
  crowding_risk?

  signal_strength
  confidence_components
  expected_return?

  supporting_clusters
  opposing_clusters
  missing_information
  causal_changes
  invalidation_conditions
  baseline_comparators

  evidence_refs
  feature_revision
  model_revision
  policy_revision
  universe_revision
  as_of
  knowledge_time
}
```

`expected_return` stays null until calibrated.

## F. Research ladder

- B0 current V1 bounded equal-wallet consensus.
- B1 raw-state preservation.
- B2 clone/cluster-adjusted independent state.
- B3 independent flow added as a separate channel.
- B4 trader-relative conviction.
- B5 frozen skill/anti-alpha divergence.
- B6 one market/crowding family at a time.
- B7 deterministic signal policy.
- B8 calibrated expected-return model.
- B9 learned model/weights only if transparent models leave material edge.

Every rung must beat or justify itself against the strongest simpler qualified predecessor.

## G. Ablation and forward validation

Mandatory paired ablations:
- raw-preserved vs clipped-only;
- state vs flow vs state+flow;
- equal wallets vs clusters;
- absolute vs relative conviction;
- no skill vs skill divergence;
- no anti-alpha vs alpha-minus-anti-alpha;
- market context families individually;
- crowding as no-use vs risk gate vs predictor;
- equal cluster weights vs quality weights;
- full promoted model minus each feature family;
- latency/cost sensitivity;
- expert dropout.

Every frozen evidence/signal event receives outcomes at 1m, 5m, 15m, 1h, 4h and 24h.

Measure conditional forward returns, hit rate, IC, monotonicity, calibration, turnover, latency sensitivity, cost sensitivity, regime stability, independent breadth, MFE/MAE, drawdown for explicit strategies, and degradation under missing experts.

Use chronological PIT data. Retain search counts and negative results.

## H. Failure modes and adversarial behavior

- **15 clones same trade:** one cluster influence; breadth approximately one.
- **great trader, low current conviction:** skill can remain positive; current relative conviction remains low.
- **bad cohort opposite good cohort:** preserve disagreement; only qualified skill divergence may use it predictively.
- **everyone highly leveraged long:** strong state plus crowding risk; confidence does not automatically increase.
- **huge whale:** raw bias visible, vote bounded.
- **stale equity:** normalized evidence unavailable.
- **tiny add on huge old book:** small positive flow, large existing state.
- **+2x -> +1x:** REDUCE and negative flow while state remains long.
- **experts long while price collapses:** persistent state does not create repeated new information.
- **two clusters disagree:** expose opposing evidence and breadth; do not hide behind a net average.
- **copying begins later:** cluster revision changes prospectively when support qualifies.
- **best expert changes strategy:** drift reduces support; stale skill does not persist forever.
- **new wallet:** current descriptive bias allowed; skill/relative conviction UNKNOWN.

## I. Minimum viable V2

MVP is V20-V26 in [plan](../plan.md):

1. preserve raw bias;
2. derive canonical intent/flow;
3. separate ages;
4. retain cluster independence;
5. compute lagged relative conviction;
6. persist EnsembleEvidence;
7. attach forward outcomes;
8. run B0-B4 frozen ablations.

Only then introduce a TradeSignal state machine if predictive structure is demonstrated.

## J. Explicitly out of V2

Do not build:
- LLM/NN per-signal scoring;
- online self-training;
- auto execution;
- generalized feature platform;
- new broker/microservice stack;
- capital-weighted voting;
- magic confidence score;
- unsupported liquidation/context feeds;
- expected returns without calibration;
- historical universe claims without PIT selection.

## K. Dependency-aware plan

The implementation frontier is:
`V20 -> V21 -> (V22,V23) -> V24 -> V25 -> V26 -> optional (V27,V28) -> V29 -> V30 -> V31`.

Default implementation ownership is the separate Python expert-engine repository. Modify tracker contracts only if V21 proves a required causal datum cannot be reconstructed from existing authoritative events.

## L. Acceptance criteria

### Descriptive bias engine
- raw bias + bounded influence replay exactly;
- state/flow/age semantics pass;
- independence and uncertainty are explicit;
- relative conviction is PIT-correct;
- no predictive claim required.

### Signal-engine MVP
- descriptive engine passes;
- reusable forward outcomes exist;
- B0-B4 exact ablations run;
- typed TradeSignal interface exists or emission is intentionally disabled awaiting evidence.

### Forward-qualified signal engine
- historical predictive gates pass for a frozen candidate;
- live-forward frozen evaluation confirms the effect;
- realistic latency/cost robustness is acceptable;
- no holdout/forward retuning contamination.

### Production advisory signal engine
- forward qualification passes;
- deterministic inspectable policy is explicitly approved;
- runtime explains current signal and simpler baselines;
- degradation/rollback is verified;
- financial execution remains forbidden.

## Things we explicitly should not claim now

At the current inspected state, the system may claim a qualified V1 descriptive/advisory engineering substrate. It may **not** claim a forward-qualified predictive signal engine. B2/B3/B4 candidate code exists, but forward economic value has not been established by the inspected tests.
