# Domain glossary

## Observation and knowledge

**Wallet**: one public account address. Multiple wallets may share a controller or strategy; one wallet is not necessarily one independent expert.

**Instrument**: a venue- and market-specific contract, including network/environment and perp-dex namespace. Similar ticker text does not establish economic equivalence.

**Observation**: a source payload received by this system, together with its provenance. Observing a trade does not establish complete account history.

**Receipt time**: when a payload reaches the system. **Event time**: when the source says it occurred. **Knowledge time**: when accepted information becomes available to a decision. These times are distinct.

**Source identity**: the source-defined identity of an economic event. **Receipt identity**: the identity of a particular delivery, including duplicate deliveries.

**Reconstructed position**: the best supported current quantity and cost basis for a wallet/instrument. It is not an inferred trading intention.

**Reconciliation**: comparison with an external account snapshot and, when justified, correction of reconstructed state. A correction is not proof that the trader just made a new trade.

**Coverage**: the documented scope and completeness of observation across wallets, instruments, and time. **Gap**: an interval or scope whose completeness is unknown or known to be insufficient.

**Known flat**: reliable evidence of zero position in an eligible instrument. **Abstention**: an expert is outside the applicable universe. **Unavailable**: the expert's state is not reliable enough to interpret. Neither abstention nor unavailable means flat or short.

## Expert behavior and ensemble evidence

**Expert**: a user-selected source of trading information, associated with a wallet and a declared eligible universe and horizon.

**Raw bias**: signed instrument notional divided by compatible scoped equity at a named knowledge cut. Raw bias is intentionally unclipped and may exceed +/-1. It describes capital exposure, not forecasting skill or probability of profit.

**Posture**: a bounded influence transform of raw bias used by a declared aggregation baseline. Posture exists to prevent one wallet's leverage from dominating a vote. It is not a substitute for raw bias and is not psychological conviction.

**Portfolio share**: the absolute notional of one instrument divided by the expert's total observed absolute portfolio notional over the same supported scope. It describes concentration only within what the system can observe.

**Intent event**: a verified position transition attributable to an economic position change: OPEN, ADD, REDUCE, CLOSE, or FLIP. Reconciliation, mark movement, and equity movement are not intent events.

**Intent flow**: signed change in equity-normalized exposure caused by intent events over a declared horizon. Flow asks what new position information the expert revealed; it excludes passive mark/equity drift by construction.

**Position age**: elapsed time since the start of the current position episode when that start is actually known. **Intent age**: elapsed time since the most recent intent event. Observation-start time must not be presented as a true position age.

**Relative conviction**: how unusual the magnitude of an expert's current raw bias is relative to that expert's own lagged historical distribution in the same declared scope. It is descriptive until forward evidence demonstrates predictive use.

**Quality weight**: a slow-moving, point-in-time assessment of an expert's demonstrated forecasting usefulness. It is predictive policy, not source reliability or capital size.

**Reliability**: whether current evidence is admissible. **Similarity**: measured behavioral redundancy, not proof of shared identity.

**Cluster**: a versioned group of behaviorally redundant experts within an instrument/horizon scope. A cluster is the default unit of independent influence after redundancy adjustment.

**Independent breadth**: the effective number of independent cluster budgets contributing admissible evidence at a decision cut. Raw wallet count is not breadth.

**Cohort**: a versioned point-in-time grouping used for research or interpretation, such as alpha, control, or anti-alpha. Cohort labels may use only information available before their effective period.

**State evidence**: the current positioning of admissible independent experts. It answers "what do they hold now?"

**Flow evidence**: recent intent events from admissible independent experts. It answers "what new information did they just reveal?"

**Skill divergence**: difference between lagged skill cohorts' state or flow evidence. It is a candidate predictive feature, not a fact about future returns.

**Market divergence**: difference between expert evidence and an independently defined market-positioning/context measure. It is meaningful only when the external measure and timestamp semantics are explicit.

**Crowding risk**: evidence that positioning or market structure may make the same-direction trade fragile, congested, or liquidation-sensitive. Crowding risk is separate from signal strength.

## Predictive signal and advice

**Predictive evidence**: a versioned set of descriptive features plus point-in-time historical outcome evidence that has passed the declared research gates for use by a signal policy.

**Signal strength**: a signed output of a promoted predictive model or deterministic rule. It represents directional predictive evidence under that model; it is not confidence.

**Confidence**: evidence about the reliability and statistical support of a signal, including data quality, independent support, sample support, and calibration uncertainty. Confidence is not the magnitude of the signal.

**Expected return**: a calibrated estimate of forward return for a declared horizon, latency, and cost model. It remains unavailable until calibration is demonstrated out of sample.

**Signal policy**: the versioned deterministic mapping from promoted predictive evidence to a signal state and signal event. Thresholds and hysteresis belong to the policy and are candidate parameters until qualified.

**Trade signal**: an account-independent, replayable signal-policy decision for one instrument and horizon. It contains evidence lineage, state/flow context, uncertainty, revisions, and the change from the prior signal. It is not a broker order.

**Consensus target**: the V1 bounded ensemble posture with contribution attribution and coverage. It remains a valid baseline and descriptive comparator, but it is not automatically a predictive trade signal.

**Advisory target**: an account-specific proposed position derived downstream from an account-independent trade signal or consensus baseline, with risk assumptions and validity conditions.

**Action delta**: the difference between an advisory target and a fresh, confirmed account position, not the difference from the previous notification.

**Risk budget**: a configured amount of admissible loss exposure. Remaining drawdown allowance is a constraint, not an instruction to risk it all.

## Control and evidence

**Worldview**: a coherent, explicitly scoped view of accepted state at named input cuts and policy revisions. It may truthfully report incomplete knowledge.

**Policy**: a versioned set of deterministic interpretation and control rules. **Proposal**: an unapplied change to policy or operation. **Receipt**: durable evidence of whether an authorized command was applied and what changed.

**Evidence capsule**: a bounded, reproducible collection of inputs, versions, outputs, checks, and limitations supporting one conclusion.

**Forward outcome**: the return, excursion, cost, and availability measurements attached after the fact to a previously frozen signal/evidence record at declared horizons. A forward outcome can evaluate a past decision; it cannot alter that decision's as-known inputs.

**As-known replay**: reconstruction using only information available at each historical decision. **Restated replay**: reconstruction using later corrections or backfills. These answer different questions and must remain distinguishable.

**Qualification**: demonstrated satisfaction of a named acceptance predicate at specified revisions. Software correctness, descriptive validity, historical predictive evidence, live-forward evidence, and production advisory qualification are distinct claims.

**Lesson**: an evidence-backed finding retained as a regression, operational rule, or decision record. A session narrative or untested explanation is not a validated lesson.
