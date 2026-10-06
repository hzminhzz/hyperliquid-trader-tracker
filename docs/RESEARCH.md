# Ensemble semantics and V2 signal research

**Status:** authoritative research and signal-engine specification. V1 consensus remains the live descriptive baseline. Nothing in this document promotes a predictive model by prose alone.

## R1. Separate observation, description, prediction, and action

The system has four different epistemic layers:

1. **Observation**: exchange/account facts and their uncertainty.
2. **Description**: deterministic representations of expert behavior such as raw bias, flow, concentration, age, redundancy, and cohort state.
3. **Prediction**: only features and mappings that demonstrate incremental forward value under the frozen evaluation protocol.
4. **Action/advice**: account-independent signal state, then optional account-specific sizing under a separate risk engine.

No downstream layer may silently convert an unknown into zero, a descriptive feature into a forecast, or a statistically useful forecast into execution authority.

The user supplies candidate wallets. Record admission and retirement knowledge times. Today's selected winners cannot be backfilled into historical universes and called point-in-time selection.

## R2. Canonical wallet evidence

For expert i, instrument a, and knowledge cut t:

### R2.1 Raw equity bias

[
b_{i,a,t} = rac{q_{i,a,t} p_{a,t}}{E_{i,t}}
]

where quantity q, valuation price p, and scoped equity E must be compatible in scope and knowledge time. If E is missing, non-positive, stale, or scope-incompatible, bias is unavailable.

Raw bias is unclipped. Values beyond +/-1 are retained because leverage magnitude may contain useful descriptive information.

### R2.2 Bounded influence

A baseline vote may transform raw bias:

[
v_{i,a,t}(k) = operatorname{clip}(b_{i,a,t}/k,-1,1)
]

The transform protects aggregation from leverage domination. It deliberately discards leverage magnitude beyond k, so every persisted evidence record MUST retain raw bias alongside bounded influence.

Any k is a versioned candidate parameter until evaluated. The current V1 k=1 remains a transparent baseline, not an optimum.

### R2.3 Portfolio concentration

For the observable compatible portfolio scope A:

[
c_{i,a,t} =
rac{|N_{i,a,t}|}
{sum_{jin A}|N_{i,j,t}|}
]

where N is signed notional. This distinguishes a 1x BTC position that dominates the wallet from a 1x BTC hedge inside a much larger multi-asset book. Unknown external venues remain an explicit limitation.

### R2.4 Intent transitions and flow

A verified economic position transition is classified as OPEN, ADD, REDUCE, CLOSE, or FLIP from before/after signed exposure. Reconciliation corrections, valuation changes, and equity-only changes MUST NOT create intent flow.

Define exposure change attributable to economic intent:

[
Delta b^{intent}_{i,a,e} =
b^{counterfactual after event}_{i,a,e}
-
b^{before event}_{i,a,e}
]

using the same valuation/equity cut where feasible so passive price/equity drift does not masquerade as trader action.

For a declared lookback h:

[
f_{i,a,t}^{(h)} =
sum_{e: t-h < known(e)le t} Delta b^{intent}_{i,a,e},K_h(t-known(e))
]

where K_h is a declared deterministic decay kernel. Start with no-decay window sums as the baseline; exponential decay is a later candidate.

### R2.5 Ages

- **position_age**: since the true current episode OPEN/FLIP if known.
- **intent_age**: since the most recent OPEN/ADD/REDUCE/CLOSE/FLIP.
- **observation_age**: since the latest admissible observation.

These are different. Seed time cannot fabricate position age.

### R2.6 Trader-relative conviction

For a lagged, point-in-time history H of |b| in the same scope:

[
r_{i,a,t} = F^{past}_{i,a}(|b_{i,a,t}|)
]

where F is the empirical CDF or another predeclared robust rank estimator fitted only before t. Report direction separately: signed relative conviction may be sign(b)*r.

Insufficient history yields UNKNOWN, not neutral 0.5. Relative conviction is descriptive until an ablation proves predictive value.

## R3. Independence before agreement

Raw wallet count is not evidence breadth.

Similarity artifacts are versioned by instrument/horizon and trained only on as-known history. Candidate features include:

- signed posture/bias changes;
- OPEN/ADD/REDUCE/CLOSE/FLIP timing;
- non-flat direction agreement;
- lead/lag structure;
- episode overlap;
- optional portfolio concentration patterns.

Flat-flat intervals, stale states, corrections presented as trades, and fabricated entry times are excluded from support.

Complete-link clustering remains the first deterministic candidate because it resists chain-link merging. The existing V1 posture-distance implementation is a software baseline, not the final similarity definition.

For cluster g, define one fixed cluster budget before seeing current direction. Exact clones divide that budget. Independent breadth SHOULD report both raw cluster count and an effective breadth measure, for example:

[
B_{eff}=1/sum_g w_g^2
]

where w_g are normalized independent cluster budgets actually admissible at the decision cut.

Unknown similarity is not independence. New or low-support wallets use a declared newcomer/manual-group policy.

## R4. Cohorts and skill must be point-in-time

Cohort labels are research artifacts, not permanent identities. Candidate cohorts:

- **alpha cohort**: lagged evidence of positive forecasting value;
- **control cohort**: insufficient or neutral evidence;
- **anti-alpha cohort**: lagged evidence of systematically adverse forecasting value.

Cohort assignment must use only pre-period outcomes, have minimum support and shrinkage, and be frozen during each evaluation interval. A trader can change strategy; therefore cohort persistence and drift diagnostics are required.

Do not weight by wallet capital. Skill weighting, if used, is based on lagged forecasting evidence at declared horizons and must preserve cluster budgets so clones cannot regain influence through quality weighting.

## R5. Cohort state, flow, and divergence

For cluster-level feature x_g and cluster budget w_g:

[
State_{a,t} = sum_g w_g,x^{state}_{g,a,t}
]

[
Flow^{(h)}_{a,t} = sum_g w_g,x^{flow,(h)}_{g,a,t}
]

Maintain them separately.

Candidate skill divergence:

[
D^{skill}=Evidence_{alpha}-Evidence_{anti}
]

Candidate market divergence:

[
D^{mkt}=Evidence_{experts}-Context_{market}
]

Do not define market context by a same-source transformation that mechanically duplicates the expert feature. Funding, open interest, liquidation metrics, volatility, and price response are optional context families and must each earn incremental value.

## R6. Uncertainty is multidimensional

Never publish one opaque confidence number from unrelated causes.

Persist at least:

- coverage/reliability mass;
- independent breadth;
- similarity support;
- relative-conviction support;
- skill/cohort support;
- model calibration support;
- latency/freshness;
- explicit missing-information reasons.

The existing V1 missing-mass interval remains useful for bounded posture consensus. It is not a statistical confidence interval.

**Signal strength**, **confidence**, **crowding risk**, and **expected return** are distinct outputs.

Expected return MUST remain null until a model is calibrated for a specific horizon and latency/cost assumption.

## R7. Crowding and context

Crowding is primarily a risk/context variable, not automatically a bearish or bullish predictor.

Candidate descriptive inputs:

- expert cohort leverage concentration;
- independent-cluster directional concentration;
- funding;
- open interest level/change;
- liquidation proximity where defensible;
- market volatility;
- price response after expert flow;
- broad market positioning from an independent source.

Universal same-direction expert positioning can mean strong information, clone/crowd redundancy, or liquidation fragility. The model must not infer which without evidence.

## R8. Canonical evidence artifacts

Persist immutable, replayable artifacts at these seams:

### WalletEvidence
- expert_id, instrument_id, horizon
- raw_bias
- bounded_influence by named baseline transform
- portfolio_share
- position_age / intent_age / observation_age
- last_intent_event
- multi-horizon intent_flow
- reliability/missing reasons
- as_of, knowledge_time
- input evidence refs

### IndependenceArtifact
- model_revision
- training_cutoff
- pair support diagnostics
- cluster memberships
- cluster budgets
- effective breadth
- newcomer/unknown-similarity policy

### CohortArtifact
- cohort_revision
- training/evaluation cutoff
- membership and support
- lagged performance definitions
- shrinkage/uncertainty
- drift flags

### EnsembleEvidence
- instrument_id, horizon
- state evidence
- flow evidence by horizon
- relative-conviction summary
- independent breadth
- skill divergence
- market divergence if available
- crowding/context vector
- coverage/uncertainty
- supporting/opposing clusters
- causal changes
- baseline comparators

### PredictiveEvidence
Only promoted features and transformations, with:
- candidate/model revision
- frozen feature schema
- training cutoff
- calibration revision
- latency/cost assumptions
- historical support handles
- current feature values
- uncertainty/support

### TradeSignal
See R10.

Each artifact carries input cut, schema/model/policy revision, as_of, knowledge_time, limitations, and evidence references. Lower-level information is never silently destroyed.

## R9. Research ladder

The ladder is deliberately finer than V1 so each source of claimed edge is isolated.

- **B0 V1-bounded**: current equal-wallet bounded posture consensus.
- **B1 raw-state**: preserve raw bias; aggregate only with bounded influence at the final vote seam.
- **B2 independent-state**: B1 plus clone/cluster budgets.
- **B3 independent-flow**: B2 plus intent flow as a separate output; no forced state+flow merge.
- **B4 relative-conviction**: B3 plus trader-relative conviction.
- **B5 skill-divergence**: B4 plus frozen alpha/control/anti-alpha cohort contrasts.
- **B6 market-divergence/crowding**: B5 plus one context family at a time.
- **B7 deterministic signal policy**: predeclared transparent mapping from the promoted evidence set to signal state/event.
- **B8 calibrated expected-return model**: only if B7 leaves material exploitable structure and calibration is stable.
- **B9 learned weighting/model**: only after transparent alternatives fail or a learned model shows robust incremental value.

A later rung does not inherit promotion automatically. Every rung is compared to the strongest simpler qualified predecessor on the same frozen manifests.

## R10. Signal policy and TradeSignal

The V2 signal engine is a deterministic state machine over promoted predictive evidence.

Canonical contract:

```text
TradeSignal
  signal_id
  instrument_id
  horizon

  state                  FLAT | LONG | SHORT
  event                  ENTER | INCREASE | REDUCE | EXIT | REVERSE | NONE

  state_evidence
  flow_evidence
  relative_conviction
  independent_breadth
  skill_divergence?
  market_divergence?
  crowding_risk?

  signal_strength
  confidence_components
  expected_return?       # null until calibrated

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
```

Signal event semantics compare the new policy state with the prior signal state. A trader REDUCE can weaken a long signal without creating a short signal. A flip may create REVERSE only if the policy crosses both exit and opposite-entry conditions.

Hysteresis/noise bands are policy parameters selected before holdout evaluation. No constant is "optimal" because it exists in code.

## R11. Exact evaluation protocol

### R11.1 Dataset cuts

Use chronological point-in-time cuts:
- development/training;
- validation/model-selection;
- frozen historical holdout;
- live-forward shadow interval.

Wallet admission, cohort labels, relative scales, cluster artifacts, and context data must be available as-known at each cut.

Today's wallet set may be evaluated as a conditional fixed-universe study, but that result MUST be labeled conditional and cannot claim survivorship-free discovery.

### R11.2 Forward outcomes

For every emitted evidence/signal record, attach later outcomes at fixed horizons:

1m, 5m, 15m, 1h, 4h, 24h

plus any strategy-specific horizon selected before evaluation.

Record:
- raw forward return;
- return after declared latency;
- return after cost/slippage model;
- maximum favorable/adverse excursion;
- volatility over horizon;
- market regime/context;
- availability/coverage at emission;
- whether a later correction would have changed restated evidence.

### R11.3 Metrics

Evaluate:
- conditional mean/median forward returns;
- hit rate with uncertainty;
- rank/linear information coefficient where meaningful;
- monotonicity across signal-strength buckets;
- monotonicity across confidence/support buckets;
- calibration for any probability/expected-return output;
- turnover and event rate;
- latency sensitivity;
- cost sensitivity;
- regime stability;
- independent breadth sensitivity;
- missing-expert degradation;
- crowding-state performance;
- MFE/MAE;
- drawdown for any explicit policy backtest;
- search count and family-wise/model-selection burden.

Use block/bootstrap or episode-aware uncertainty where temporal dependence is material. Do not treat repeated unchanged states as independent samples.

### R11.4 Ablations

At minimum run paired frozen-manifest ablations:

1. raw bias retained vs early clipping-only representation;
2. state-only vs flow-only vs state+flow;
3. equal wallets vs independent cluster budgets;
4. absolute bias vs relative conviction;
5. no skill cohorts vs skill divergence;
6. no anti-alpha vs anti-alpha divergence;
7. no market context vs each context family individually;
8. no crowding control vs crowding as risk gate vs crowding as predictor;
9. equal cluster weights vs lagged quality weights;
10. full model vs removal of each promoted feature family;
11. zero/low/high latency and realistic cost assumptions;
12. full expert availability vs controlled dropout.

Every ablation stores the exact manifest and result, including negative results.

## R12. Promotion gates

Software correctness is necessary but not economic qualification.

A candidate can be:
- **IMPLEMENTED**: code/tests exist;
- **REPLAY-VALID**: deterministic PIT replay and lineage pass;
- **HISTORICALLY-SUPPORTED**: predeclared historical holdout shows incremental value;
- **FORWARD-QUALIFIED**: frozen live-forward shadow period confirms the effect within declared uncertainty;
- **ADVISORY-PROMOTED**: explicit policy approval enables the model in advisory runtime.

Promotion requires:
- no correctness regression;
- no point-in-time violation;
- predeclared primary metric and decision rule;
- materially acceptable turnover/latency/cost behavior;
- robustness across reasonable subperiods/regimes;
- retained negative results and search count;
- explicit approval.

Do not use arbitrary fixed Sharpe/turnover constants as universal gates. Candidate acceptance thresholds belong in an experiment registration and must be justified before the holdout is observed.

## R13. Minimum viable V2

Build only enough to test the central thesis:

1. persist raw bias beside bounded influence;
2. derive reliable intent events and multi-horizon flow;
3. expose position age vs intent age;
4. use existing clone/cluster adjustment as an independence layer;
5. add lagged relative conviction with UNKNOWN on low support;
6. persist EnsembleEvidence with state and flow separate;
7. attach frozen multi-horizon forward outcomes;
8. compare B0-B4 using exact ablations;
9. add a deterministic TradeSignal state machine only after at least one evidence representation demonstrates useful holdout structure;
10. run live shadow forward qualification before advisory promotion.

## R14. Explicitly out of V2 MVP

Do NOT build yet:
- neural networks, LLM scoring, or opaque end-to-end ML;
- auto-discovered wallet universes presented as unbiased historical selections;
- online self-updating production weights;
- complex regime ensembles;
- a generalized feature store platform;
- Kafka/new microservices solely for scoring;
- automatic financial execution;
- expected-return numbers without calibration;
- liquidation/crowding models that require unsupported data;
- capital-weighted voting;
- a single magic "confidence score";
- automatic promotion from backtest results.

## R15. Adversarial expected behavior

| Scenario | Correct behavior |
|---|---|
| 15 clone wallets open the same trade | One independent cluster budget; breadth near one, not fifteen; clone event retained for diagnosis. |
| Excellent trader uses unusually low bias | State direction can support the side, but relative conviction flags weak/unusual commitment; skill does not manufacture conviction. |
| Bad cohort opposes good cohort | Preserve both; skill divergence may strengthen good-cohort evidence only if cohort labels are lagged and qualified. |
| Every cohort is highly leveraged long | Strong descriptive state plus high crowding risk; confidence does not rise mechanically with crowding. |
| One whale dominates capital | Raw bias remains visible; bounded/cluster influence prevents capital size from dominating vote. |
| Stale equity | Wallet evidence unavailable for normalized bias; missing mass/support rises; no guessed fresh signal. |
| Tiny new trade atop huge old position | Flow reflects only the small incremental change; state retains the large old bias. |
| Bias moves +2x to +1x | REDUCE intent, positive state remains; flow is negative, not a new short. |
| Experts remain long while price falls | State persists; flow may be zero; price-response/context can penalize confidence only if empirically promoted. |
| Two independent clusters disagree | Preserve opposing cluster evidence and reduced net signal; do not average away disagreement diagnostics. |
| Formerly independent trader begins copying | New similarity artifact can merge influence only after point-in-time support and membership policy permit it. |
| Best expert changes strategy | Drift diagnostics lower support; old quality cannot persist indefinitely without requalification. |
| New wallet with no history | Current raw bias may be descriptive; relative conviction/skill are UNKNOWN; conservative newcomer influence policy applies. |

## R16. Resource and agent ergonomics

Persist reusable feature/state partitions and forward outcomes incrementally. Research jobs consume sealed manifests and checkpoints; do not recalculate the full world on each experiment.

An agent must be able to answer from bounded interfaces:
- why the signal has this direction;
- which clusters contributed;
- what changed now vs persisted;
- whether current exposure is unusual for each trader;
- how much support is independent;
- which evidence is predictive vs merely descriptive;
- how the signal compares with B0/B1/B2;
- what data are missing;
- what would invalidate the signal;
- which model/policy revision produced it;
- what historical and live-forward evidence supports that revision.

Negative experiments are first-class evidence.
