# Qualification and evidence

**Status:** authoritative acceptance program for V2 signal research. Existing V1 evidence remains valid only for the claims it actually tested. In particular, software/replay tests for B2/B3/B4 do not by themselves establish predictive edge.

## Q0. Evidence standard

Every test or experiment produces an evidence capsule with:
- claim and qualification class;
- code/dependency revisions;
- schema, policy, universe, feature, model and cluster/cohort revisions;
- input manifest and point-in-time cut;
- clock/seed;
- exact invocation;
- environment;
- latency/cost assumptions;
- primary metric and predeclared decision rule;
- search count / candidate family;
- checks, result, limitations and artifact hashes.

Allowed result states: PASS, FAIL, BLOCKED, INCONCLUSIVE. Unknown is never PASS.

Qualification classes are separate:

1. **ENGINEERING**: implementation, invariants, replay, interface behavior.
2. **DESCRIPTIVE**: evidence artifacts faithfully represent observed trader behavior.
3. **HISTORICAL_PREDICTIVE**: frozen PIT historical holdout supports incremental forecasting value.
4. **LIVE_FORWARD**: frozen live shadow evidence supports the same claimed effect.
5. **ADVISORY_PRODUCTION**: promoted model/policy is deterministic, inspectable, operationally safe, and explicitly approved.

A pass in one class cannot be cited as a pass in another.

## Existing V1 evidence interpretation

The V1 Q10/Q11/Q15 suites establish useful engineering properties:
- exact posture arithmetic and missing-mass behavior;
- clone-resistant cluster aggregation;
- deterministic candidate mechanics;
- promotion/approval control flow.

They do **not** prove:
- that clipping improves prediction;
- that clustering improves forward returns;
- that profit-factor quality weighting improves prediction;
- that the hard-coded regime rule improves returns or risk-adjusted performance;
- that any V1 target is a calibrated confidence or expected-return forecast.

V2 documentation and future evidence must keep these claims separate.

## Acceptance scenarios

| ID | Class | Scenario | Observable pass condition |
|---|---|---|---|
| Q01 | ENGINEERING | Baseline inventory | Current branches, code paths, runtime model, existing tests and actually deployed scoring path are recorded. |
| Q02 | ENGINEERING | Observation identity/lifecycle | Existing tracker invariants for dedup, position transitions and uncertainty remain green. |
| Q03 | ENGINEERING | PIT replay | Same manifest/revisions produce identical descriptive and signal artifacts; later knowledge cannot alter historical as-known outputs. |
| Q04 | DESCRIPTIVE | Raw-bias preservation | >1x leverage remains recoverable in WalletEvidence even if bounded influence saturates; replay reproduces both values exactly. |
| Q05 | DESCRIPTIVE | Intent semantics | OPEN/ADD/REDUCE/CLOSE/FLIP are derived from economic position change; valuation/equity/reconciliation changes cannot fabricate intent flow. |
| Q06 | DESCRIPTIVE | State vs flow | Persistent unchanged bias changes state only through valuation/equity rules; it cannot repeatedly create new flow. +2x -> +1x is REDUCE with positive state and negative flow. |
| Q07 | DESCRIPTIVE | Age semantics | True position age, intent age and observation age remain distinct; unknown entry time is never replaced by seed time. |
| Q08 | DESCRIPTIVE | Independence | Exact clones do not increase cluster influence/breadth; unknown similarity remains unknown; cluster revisions are replayable PIT artifacts. |
| Q09 | DESCRIPTIVE | Relative conviction | Current bias rank uses only lagged history; low support returns UNKNOWN; no current/future sample leaks into the reference distribution. |
| Q10 | DESCRIPTIVE | Cohort PIT correctness | Alpha/control/anti-alpha labels are fitted only before their effective interval, frozen during evaluation, support-aware, and drift-visible. |
| Q11 | ENGINEERING | Feature lineage | WalletEvidence -> Independence/Cohort -> EnsembleEvidence -> PredictiveEvidence -> TradeSignal resolves exact parents, revisions, timestamps and missing reasons. |
| Q12 | ENGINEERING | Forward outcome attachment | Each frozen signal/evidence event can later receive 1m/5m/15m/1h/4h/24h returns and excursions without mutating the historical decision record. |
| Q13 | HISTORICAL_PREDICTIVE | Representation ladder | B1-B4 are compared to B0 on identical PIT manifests with predeclared metrics; each promoted rung shows incremental value or is rejected/inconclusive. |
| Q14 | HISTORICAL_PREDICTIVE | State/flow ablation | state-only, flow-only, state+flow are evaluated on identical events; any merge must beat the best separate representation after latency/costs. |
| Q15 | HISTORICAL_PREDICTIVE | Independence ablation | cluster-aware evidence is compared with equal-wallet evidence; clone correctness alone cannot count as economic improvement. |
| Q16 | HISTORICAL_PREDICTIVE | Relative-conviction ablation | trader-relative conviction must improve a predeclared metric or remain descriptive-only. |
| Q17 | HISTORICAL_PREDICTIVE | Skill divergence | frozen skill/anti-alpha cohort features must add robust holdout value beyond B4 before use in a signal policy. |
| Q18 | HISTORICAL_PREDICTIVE | Market/crowding context | each context family is added one at a time; predictor and risk-gate uses are evaluated separately. |
| Q19 | HISTORICAL_PREDICTIVE | Signal policy | deterministic state machine shows monotone/useful signal buckets, acceptable turnover, latency/cost robustness, and stable subperiod behavior. |
| Q20 | HISTORICAL_PREDICTIVE | Calibration | any confidence probability or expected-return estimate is calibrated on held-out outcomes; otherwise those fields remain structural/NULL. |
| Q21 | LIVE_FORWARD | Frozen shadow qualification | promoted historical candidate is frozen before the live interval and evaluated without repeated retuning/searching that interval. |
| Q22 | LIVE_FORWARD | Missing-expert degradation | controlled expert unavailability reduces support/confidence appropriately and does not amplify surviving votes or improve reported certainty. |
| Q23 | LIVE_FORWARD | Drift | strategy/cluster/cohort drift is detected and degrades support without silently rewriting historical assignments. |
| Q24 | ENGINEERING | Agent explanation | bounded inspection can answer direction, new-vs-old information, independent contributors, unusual conviction, missing evidence, baseline comparator, model revision and invalidation conditions. |
| Q25 | ENGINEERING | Adversarial matrix | all scenarios in RESEARCH R15 produce the declared semantics. |
| Q26 | ADVISORY_PRODUCTION | Production advisory gate | engineering + descriptive + required historical + live-forward gates pass, policy approval exists, runtime is deterministic/inspectable, and financial execution remains forbidden. |

## Frozen experiment protocol

Before reading a holdout result, register:
- research question;
- candidate rung and exact feature family;
- universe construction;
- training, validation and holdout boundaries;
- all lags;
- signal sampling/event rules;
- target horizon(s);
- primary metric;
- secondary diagnostics;
- latency/cost model;
- parameter search space and maximum search count;
- promotion/rejection rule;
- what happens after INCONCLUSIVE.

Do not retrospectively change the rule and call the same holdout "forward".

## Point-in-time universe and survivorship

Two valid studies exist and must be labeled differently:

1. **Conditional fixed-universe study**: evaluates a wallet set chosen today over historical data. Useful for mechanism research but subject to selection/survivorship bias.
2. **Point-in-time selection study**: wallet admission/retirement rules themselves are reconstructed as-known. Required for claims about a discover/select/deploy process.

Never mix the labels.

Clone/cohort assignments and historical quality features also require PIT membership. Future-observed similarity cannot collapse historical wallets retroactively in an as-known evaluation.

## Forward outcome schema

For each evidence/signal emission, append an immutable OutcomeRecord keyed to signal_id/evidence_id with:

- price_at_signal and price_after_declared_latency;
- 1m, 5m, 15m, 1h, 4h, 24h forward return;
- return after declared cost model;
- MFE and MAE per horizon;
- realized volatility;
- funding/carry where relevant;
- coverage and independent breadth at emission;
- market/crowding context at emission;
- later correction/restatement flag;
- outcome knowledge time.

The attachment happens after outcomes become knowable; it never changes the original signal timestamp or features.

## Metrics and uncertainty

Primary predictive diagnostics:
- conditional mean/median return;
- information coefficient where appropriate;
- hit rate with uncertainty;
- monotonicity by signal-strength bucket;
- monotonicity by support/confidence bucket;
- calibration error if probabilistic;
- turnover/event rate;
- latency and cost sensitivity;
- regime/subperiod stability;
- independent-breadth sensitivity;
- missing-expert degradation;
- MFE/MAE;
- policy-level drawdown only for an explicitly defined strategy.

Use block/bootstrap or episode-aware resampling where dependence is material. Rows created by an unchanged position are not independent observations.

Report effect size plus uncertainty, not p-values alone.

## Mandatory paired ablations

Use the same manifests and evaluation rules:

1. clipped-only vs raw-bias-preserved representation;
2. state-only vs flow-only vs state+flow;
3. equal wallets vs cluster budgets;
4. absolute bias vs trader-relative conviction;
5. no skill labels vs skill divergence;
6. alpha-only vs alpha-minus-anti-alpha;
7. no market context vs funding only;
8. no market context vs OI only;
9. no market context vs volatility/price-response only;
10. crowding unused vs crowding as risk gate vs crowding as directional predictor;
11. equal cluster weights vs lagged quality weights;
12. full promoted feature set minus each feature family;
13. zero/representative/stressed latency;
14. zero/representative/stressed costs;
15. complete universe vs controlled expert dropout.

A mechanism can remain operationally useful even if it has no predictive value: clustering may still be retained for clone resistance while its economic ablation is neutral. State that distinction explicitly.

## Signal state-machine qualification

The policy under test must be deterministic and versioned. Test:
- ENTER from FLAT;
- INCREASE without repeated ENTER;
- REDUCE while retaining direction;
- EXIT to FLAT;
- REVERSE only through declared opposite-side transition;
- no event on persistence alone;
- hysteresis/noise band behavior;
- missing-data invalidation;
- policy revision changes with explicit cause;
- replay equivalence.

All thresholds are experiment parameters until their selection protocol passes.

## Live-forward protocol

A live-forward candidate is frozen before the interval:
- code, feature schema, cluster/cohort artifacts or update schedule;
- signal policy;
- parameter values;
- evaluation horizons and primary metric.

During the interval:
- fixes for software correctness are allowed only with explicit version split;
- model/threshold retuning creates a new candidate and new forward clock;
- outcomes are appended automatically;
- failures, outages and missing experts remain in the sample;
- no cherry-picking "good" active periods.

A minimum duration/sample size is experiment-specific and must be registered from expected event frequency/uncertainty, not chosen after seeing PnL.

## Qualification labels

### Descriptive bias engine
May be claimed when Q03-Q12 and Q25 relevant descriptive cases pass:
- raw bias and bounded influence both preserved;
- state/flow/age/independence semantics correct;
- PIT lineage and replay correct;
- no claim of predictive edge required.

### Signal-engine MVP
May be claimed when:
- descriptive bias engine passes;
- B0-B4 research machinery exists;
- forward outcomes are attached;
- exact paired ablations can run;
- deterministic signal policy interface exists or remains intentionally disabled pending evidence;
- no unqualified confidence/expected-return field is populated.

### Forward-qualified signal engine
May be claimed only when:
- relevant Q13-Q20 historical gates pass;
- Q21-Q23 live-forward gates pass for a frozen candidate;
- effect remains useful after declared latency/costs;
- result survives predeclared robustness checks;
- negative searches are retained.

### Production advisory signal engine
May be claimed only when:
- forward-qualified signal engine passes;
- Q24-Q26 pass;
- model/policy revision is explicitly approved;
- runtime exposes current evidence and baseline comparator;
- rollback/degradation behavior is verified;
- no automated financial execution exists under this qualification.

## Agent ergonomics battery

A cold-start operator must answer using bounded interfaces:

1. Why is BTC LONG/SHORT/FLAT now?
2. Which clusters caused it and what are their contributions?
3. What changed since the prior signal?
4. How much of the current evidence is new flow versus persistent state?
5. Is current bias unusually large for those experts?
6. How many independent sources support/opppose it?
7. Which expert/cohort evidence is only descriptive versus promoted predictive evidence?
8. What historical holdout and live-forward evidence supports the current model revision?
9. What is missing/stale?
10. What would invalidate the signal?
11. What would B0/B1/B2 say on the same input cut?
12. Which model/policy revision emitted the signal?

Score correctness, evidence completeness, unsafe-effect count, tool calls, bytes/tokens, repeated work and time-to-diagnosis. Compactness cannot excuse omitted uncertainty.

## Documentation acceptance for this revision

This architecture/spec revision:
- may edit documentation, glossary, plan and ADRs;
- must not alter scoring runtime code, dependencies, secrets, service state or financial authority;
- must pass Markdown-link validation and `git diff --check`;
- must report the existing engine tests separately from predictive qualification.
