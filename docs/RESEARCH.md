# Ensemble semantics and research discipline

**Status:** target design and explicitly provisional research defaults. No ranking, clustering, consensus, or risk policy is qualified by this document. Follow [CONTRACTS](CONTRACTS.md) for identity and point-in-time evidence.

## R1. Separate facts, interpretation, and actions

A wallet position is a fact about a scoped account, not proof of a directional forecast, a stop, or the trader's complete portfolio. External hedges are usually unknown. Interpret normalized exposure as posture, not psychological conviction.

Keep these products separate: account view -> posture -> independent support -> consensus -> account advice. Each product records its exact parents and validity. Never blend estimated confidence, historical trader skill, current exposure, and source freshness into an opaque score.

The user supplies candidates. Record when each wallet was supplied, admitted, retired, and permitted to influence decisions. A backtest of today's selected winners is a conditional retrospective, not an unbiased historical selection experiment. Keep retired and failed experts in the evaluation record.

## R2. A simple interpretable baseline

For expert i and instrument a, compute signed equity exposure `e(i,a) = quantity * valuation_price / scoped_equity`. Use compatible scope and timestamps. Missing/nonpositive equity makes normalization unavailable. Valuation identifies mark/mid and is not silently treated as an executable price.

Baseline posture is `s(i,a) = clip(e(i,a) / k(i,a), -1, 1)`. The initial policy uses a fixed declared scale, with `k=1` as the transparent candidate default: 100% equity notional maps to unit posture. This is a research choice, not an optimized sizing claim. Always display uncapped exposure as well so clipping cannot hide leverage.

Trader-relative scales may later use lagged historical exposure distributions, with minimum support and a floor. Scale/model parameters are fixed for their declared evaluation interval; never estimate them using the trade currently being evaluated or future data.

Eligibility has an explicit instrument and holding-horizon scope. Separate fast trading from swing positions rather than making a scalper's exit cancel an unrelated swing signal. Unmapped instruments remain observable but cannot produce cross-venue account advice.

**Flat semantics:** a reliable flat expert in an eligible scope contributes zero and retains its allocated budget. An out-of-scope expert abstains by policy. Missing/unreliable state is unknown. Closing a long is not a short vote. Changing eligibility is a versioned policy event, not evidence of trading skill.

## R3. Fixed-budget hierarchical consensus

Let `b(g)` be a cluster budget and `a(i|g)` an expert's share within that cluster. Budgets sum to one over the policy's eligible scope. The full-data target is:

`C(a) = sum_g b(g) * sum_i a(i|g) * s(i,a)`.

Initial baseline: equal expert weights with optional user-declared duplicate groups. The next candidate uses equal cluster budgets and equal shares inside each cluster. Learned quality weights are a later, separately evaluated feature. Adding a wallet does not immediately change live denominators; membership is admitted through a scheduled versioned policy transition.

When weight mass `m` is unavailable, report its observed contribution `C_observed` and a conservative posture interval `[C_observed - m, C_observed + m]`, clipped to [-1,1]. This is a missing-information bound, not a statistical confidence interval. Missing mass is NOT reallocated to surviving experts. The policy can block new actionable advice when coverage is inadequate or the sign is ambiguous; preserve the last valid decision as expired evidence, not as a fresh instruction.

Each decision stores expert and cluster contributions, missing mass, raw wallet count, effective breadth, coverage, prior-target delta, and cause category: economic change, valuation change, reconciliation, policy change, or recovery. No arbitrary 'confidence 74%' is shown without a calibrated probability model.

Exact duplicate expertise should not gain influence from additional copies. Qualification includes cloning an expert many times and checking cluster influence, plus controls showing that merely same-direction but independently timed experts are not automatically collapsed.

## R4. Similarity is a versioned hypothesis

Measure redundancy per instrument and compatible horizon on an aligned time grid using only as-known valid states. A minute-grid is a starting research representation, not a requirement to recompute every matrix each minute. Persist incremental features; run pairwise/cluster work off the ingestion path.

Use signed posture changes, non-flat direction agreement, entry/exit timing overlap, and lead/lag diagnostics. PnL correlation is supporting context because shared market exposure can dominate it. Exclude flat-flat agreement, stale intervals, and fabricated entry times from similarity support. Count independent position episodes as well as rows; thousands of unchanged snapshots are not thousands of independent observations.

Start with a deterministic complete-link grouping candidate rather than an unconstrained graph connected-component rule: A resembling B and B resembling C must not automatically merge dissimilar A and C. A candidate artifact must explicitly contain its feature definitions, distances, thresholds, minimum joint history/episode support, missing-data rules, training cutoff, stable tie-break, and assignment version. Thresholds are selected in a predeclared development experiment, not silently inferred by the live agent.

Insufficient overlap is `INSUFFICIENT_OVERLAP`, not zero correlation or proof of independence. Unclassified experts use the declared conservative newcomer/manual-group budget until there is support. Freeze assignments between scheduled updates; use membership-change hysteresis so noisy cluster churn cannot become portfolio turnover.

Each pair/cluster explanation names the observations and features driving similarity. Similar behavior does not prove common ownership, insider status, or copying intent.

## R5. Target cadence and change control

Use event-driven updates to affected postures and targets, with a declared bounded aggregation/debounce window. Record that timing policy for replay. Periodic market/equity changes can also change normalized exposure and must have explicit causes.

Use no-trade bands and entry/exit hysteresis at the advisory boundary to control small fluctuations. Never debounce away coverage loss, a stop condition, or an explicit position close. A target is state, not an order stream. Repeated equivalent targets do not create repeated actions.

## R6. Account advice is an independent constraint engine

The target is informational until the account, instrument mapping, price, risk model, and rulebook are complete and fresh. The account rulebook must declare loss-floor formulas, daily reset timezone/calendar, trailing/static behavior, realized/unrealized treatment, commissions/swaps, and existing open risk. Do not infer these from a '5% daily / 10% total' label.

A proposed per-opportunity risk allocation comes from a bounded policy. Total remaining drawdown and daily headroom are ceilings, NOT the amount to multiply directly by consensus. Aggregate existing plus proposed stressed loss, correlation/concentration, fees, slippage, and safety reserves before approval. Multiple assets share one account budget.

Convert monetary risk into notional only with an explicit loss model: a validated stop distance plus cost/gap allowance or a conservatively qualified scenario loss per unit. Historical maximum drawdown and MAE are descriptive estimates, not guaranteed future loss limits. No stop or credible stress model means `UNSIZED`, with an exposure comparison only; never label a notional 'SAFE SIZE'.

Broker contract size, minimum lot, lot increment, currency conversion, price basis, and symbol equivalence are versioned. Round sizes downward where needed to respect the risk ceiling. Hyperliquid exposure and a prop firm's similarly named contract are not assumed identical.

Action delta uses fresh confirmed actual account holdings. Without account integration or a timestamped manual position ledger, show a target only; do not tell the user to 'add' a quantity inferred from the prior alert. Advice expires and must be rechecked before action. Notification-only delivery cannot guarantee prop-rule compliance or order execution.

## R7. Research ladder and promotion

Compare on the same point-in-time universe, input manifest, capital/risk policy, delay assumptions, and cost model:

- B0: user-selected single-expert reference, selected before the forward interval.
- B1: equal normalized experts.
- B2: duplicate/cluster adjustment with otherwise identical policy.
- B3: cluster adjustment plus lagged quality weights.
- B4: optional regime conditioning only after the simpler model is justified.

For each extra mechanism, retain the ablation, development search count, forward evaluation, cost/turnover change, drawdown, tail stress, coverage, and uncertainty. Reserve chronological evaluation periods; avoid random splitting of overlapping position histories. Fit scales, similarity, quality, and eligibility only on information available before evaluation. Include receipt/alert/manual-execution delays; the follower does not automatically receive the leader's fill price.

Promotion requires engineering gates plus evidence that the extra complexity is useful under the declared objective. Block/episode-based uncertainty analysis should respect temporal dependence. A failed economic improvement keeps the simpler baseline; it is not permission to keep searching the holdout. Quality weighting is not a prerequisite for the first useful monitoring/consensus product.

Track data quality and economics separately. A perfectly replayable engine may produce no profitable advantage. A profitable sample cannot excuse missing events or unsafe control.
