# Inspected status and implementation gaps

## ER0 implementation: source/state authority, 2026-10-06

ER0 (#31) is **ENGINEERING_PASS / LIVE_SOURCE_CAPSULE_BLOCKED** on the milestone branch. Rust now durably accepts watched trades before publishing state, commits all watched-wallet projections of one economic trade atomically, includes source time in trade identity, and serializes reconciliation publication against live fills. Rust reconciliation also emits durable `ACCOUNT_SNAPSHOT` events with current positions and account equity. The companion Python engine revision `b443a5a` consumes those snapshots, no longer polls `clearinghouseState` as a competing state writer, preserves full instrument IDs, and keeps partial-history open ages unknown.

Verification: tracker `make check-rs` passes (98 Rust unit tests, 11 adversarial-observation tests, 9 crash-recovery tests); companion engine passes Ruff, `ty`, and **81 tests**. The existing local observation ledger was inspected read-only but has `current_seq = 0`, so no real source receipt-to-projection capsule or latency/coverage qualification is claimed. See [ER0 evidence](evidence/execution-review/ER0-source-state-authority.md). No deployment, financial execution, or predictive promotion was performed.

ER1 (#25) is now **ENGINEERING_PASS** in companion engine revision `79d3e5c`: raw signed quantity is the additive primitive; event time controls horizon membership while knowledge time controls visibility; net/gross flow is accumulated before one common normalization anchor; stale/missing equity preserves raw flow while normalized flow is unavailable. The engine passes Ruff, `ty`, and **88 tests**. See [ER1 evidence](evidence/execution-review/ER1-additive-path.md).

ER2 (#26) is **ENGINEERING_PASS / NATIVE-LIVE-SOURCE-BLOCKED** in companion engine revision `4c537a4`: native references remain observed-only, inferred segments are deterministic optional annotations, and mixed/two-sided activity abstains from a directional story. The engine passes Ruff, `ty`, and **97 tests**. See [ER2 evidence](evidence/execution-review/ER2-execution-annotations.md).

ER3 (#27) is **ENGINEERING_PASS / LIVE-COVERAGE-MEASUREMENT-BLOCKED** in companion engine revision `1da1321`: coherent minute cuts carry all revisions/watermarks, material interrupts coalesce to the latest contribution, normalized wallet flow is bounded before fixed cluster budgets, and missing mass is not silently reallocated. The engine passes Ruff, `ty`, and **101 tests**. See [ER3 evidence](evidence/execution-review/ER3-coherent-cuts.md).

ER4 (#28) is **PROTOCOL_ENGINEERING_PASS / EMPIRICAL_EVALUATION_BLOCKED** in companion engine revision `f3ae0be`: R0/R1/R2 are frozen, direction-before-cost outcome semantics and price-lateness censoring are qualified, and absent/immature prospective data returns BLOCKED rather than promotion. The engine full suite passes **106 tests**. See [ER4 evidence](evidence/execution-review/ER4-representation-experiment.md).

The next implementation frontier is ER5. ER0's absent-live-traffic sub-gate remains explicit and must not be converted into a production or predictive claim.

## Latest inspection: execution and sampling review, 2026-10-06

Tracker HEAD inspected: `2f948f81094cfb18fa2799eb1dcce5c748973148`. Expert-engine HEAD inspected: `ef8d7c19f4bfdf76b86d35827145679ae9d1c346`. Both working trees were clean before the documentation-only review. The accessible tracker checkout was `/home/quant/dev/hyperliquid-trader-tracker`; the supplied `/home/dev/hyperliquid-trader-tracker` path was outside the connector's allowed roots.

`PYTHONDONTWRITEBYTECODE=1 uv run --frozen --no-sync pytest -q -p no:cacheprovider` passed **79 engine tests in 3.57s**. No Rust suite, live deployment health, complete wallet roster, measured cluster count, or predictive-performance validation was run in this review.

The current runtime still computes V1 equal-wallet consensus rather than running the V2 evidence pipeline. V2 descriptive functions exist, but optional order/program metadata, execution segmentation, materiality and aligned live evidence cuts remain unqualified. Code inspection also identified live durability error-handling risks, competing snapshot/projection authority, missing-flow ambiguity, namespace loss and outcome/cost composition risks. These are documented findings, not claims of reproduced production incidents; see [EXECUTION-REVIEW section 2](EXECUTION-REVIEW.md#2-verified-repository-findings-and-implications).

The [V2 evidence index](evidence/v2/INDEX.md) retains V26 INCONCLUSIVE, V29 REJECTED / ENGINEERING_PASS and V30/V31 NOT_ACTIVATED. No new predictive promotion is supported by this inspection. The next recommended frontier is correctness qualification, additive path evidence, then registered representation/cadence experiments. Proposed design changes are recorded in [EXECUTION-REVIEW](EXECUTION-REVIEW.md) and [plan.md](../plan.md); they are not implementation authorization.

## Earlier V2 baseline inspection

**Inspection:** 2026-10-06. **Historical V1 baseline:** `ffff331aa0f52e299ee995e2c96246c1253712a7`. **Current tracker head inspected for V2:** `9b55228`. **Current expert-engine head inspected for V2:** `bf88f7a`.

This page records code inspection, not a running-service health check. The fork is `hzminhzz/hyperliquid-trader-tracker`; its VPS checkout is `/home/quant/dev/hyperliquid-trader-tracker`. The separate scoring application is `/home/quant/dev/hyperliquid-expert-ensemble`. At the V2 inspection both checkouts matched their `origin/main`; the tracker also had an `upstream` remote for KonScanner. Recheck these facts before implementation.

## V2 scoring inspection

Verified at the current expert-engine head:

- Production `EnsembleRuntime.compute_targets()` uses `compute_equal_budget_consensus`; B2 cluster consensus, B3 quality weighting, and B4 regime conditioning are not on the live scoring path.
- `compute_posture()` retains `raw_exposure` but downstream consensus contributions use bounded `clipped_posture`; V2 therefore treats early clipping as an information-loss risk rather than deleting it outright.
- The projection store maintains current positions/equity and replayable offsets, but there is no canonical WalletEvidence artifact for intent flow, relative conviction, independent breadth, or multi-horizon outcomes.
- Similarity uses bounded-posture distance with flat-flat exclusion and deterministic complete-link clustering.
- B3 quality weights currently use lagged profit factor; B4 uses a fixed high-volatility threshold and 0.70 dampening. These are candidate mechanics, not demonstrated predictive optima.
- `uv run pytest -q` passed **39 tests** during this inspection.
- Both `copytrade-ensemble.service` and `copytrade-tracker.service` returned `inactive` at the inspection instant. This is a service-state observation only, not a statement about intended deployment configuration.

**Claim boundary:** V1 engineering/descriptive evidence is real and useful. The inspected tests do not establish forward predictive edge for B2/B3/B4. See [SIGNAL-V2](SIGNAL-V2.md) and [QUALIFICATION](QUALIFICATION.md).

## Historical capability inventory

| Capability | Evidence at baseline | Status |
|---|---|---|
| Public trade feed and watched-address filter | `Listener` and `resolve_deltas` in [listener.rs](../src/tracker/listener.rs), [resolve.rs](../src/tracker/resolve.rs) | Implemented; live qualification not performed in this documentation change |
| Position lifecycle reducer | `apply_fill` in [state.rs](../src/tracker/state.rs), corresponding tests | Implemented; current test suite not run in this change |
| Account seeding and rotating reconciliation | [enrich.rs](../src/tracker/enrich.rs), `reconcile_loop` in [app.rs](../src/tracker/app.rs) | Implemented with limitations below |
| Persistent subscriber watchlists | [db.rs](../src/tracker/db.rs) | Implemented |
| Durable observations, state checkpoints, machine event stream | Book and dedup are memory-only in [book.rs](../src/tracker/book.rs) | Not implemented |
| Account-equity product for normalization | Enricher extracts positions/leverage, not a durable equity observation | Not implemented |
| Expert normalization, clustering, consensus, prop advice | No separate expert engine in this checkout | Not implemented |
| Agent inspect/explain/plan/apply/evaluate facade | No such executable or API at baseline | Specified only |
| Runtime capability manifest and evidence graph | No such implementation at baseline | Specified only |
| Automated order execution | Outside the approved V1 scope | Not authorized |

The inherited Python files are a tracker implementation, not the proposed Python ensemble. The fork's public visibility means production wallet lists, Telegram credentials, account information, and raw operational evidence must live outside Git.

## Evidence-backed gaps that change the plan

| ID | Inspected behavior | Consequence and planned response |
|---|---|---|
| G01 | `Listener::handle_trades` deduplicates only `tid` through `SeenTids` | Use a scoped composite source identity; test different trades sharing a `tid`. Do not interpret `tid` as an ordered cursor. |
| G02 | Listener mutates the memory book, awaits close-PnL lookup, then awaits Telegram dispatch | Persistence, slow enrichment, and delivery need separate ownership. A Telegram stall must not stall observation. |
| G03 | Live dispatch depends on subscriber recipients; startup derives tracked wallets from chat subscriptions | Introduce a first-class desired wallet universe independent of Telegram. Preserve subscriber behavior as an optional adapter. |
| G04 | `reseed_wallet` silently replaces state; `seed_wallet` discards its applied/skipped return value | Emit explicit applied/skipped/failed reconciliation outcomes. A successful HTTP response alone cannot mark state reconciled. |
| G05 | The fill epoch resets in `drop_wallet`, called by reseeding | It is not a durable monotonically increasing state revision. Add a separate persistent revision; protect overlapping snapshots and watchlist generations. |
| G06 | Missing/malformed `assetPositions` can be interpreted as an empty collection in `account_positions` | Strictly validate snapshots before replacement. Missing is not proof of flat. Record malformed inputs and quarantine the affected scope. |
| G07 | Seeded position timestamps use the current time; fresh seed snapshots replace position state | Do not present observation-start time as actual entry time or claim complete trade-lifecycle PnL. Preserve provenance and partial-history flags. |
| G08 | Seeding precedes admission and the feed filters unadmitted wallets | Startup can miss activity. Buffering alone does not prove a snapshot/stream cut. Qualification must establish ordering or report a gap. |
| G09 | Seed concurrency is bounded, but that is not a shared weighted request-rate budget | Introduce one budget for reconciliation, enrichment, recovery, and diagnostic requests. Bound retries and reserve capacity for recovery. |
| G10 | Universe discovery requests default `meta` without explicit multi-dex orchestration | Do not claim all HIP-3 or spot coverage. Publish the actual covered namespace and instrument set. |
| G11 | Positions and recent identities are not persisted | An outbox added only after memory mutation is insufficient for crash consistency. Durable input, reducer checkpoint, and emitted-event ownership must be specified together. |

These are code-level findings and design risks, not claims of observed production losses. Reproduce each relevant defect before changing implementation. Keep the KonScanner reducer and infrastructure; patch verified gaps rather than rewriting the tracker.

## External contracts to re-probe during qualification

The official [subscription schema](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/websocket/subscriptions) identifies counterparties as buyer/seller and describes `tid` as a hash; its suggested identity includes block time and coin. The same source distinguishes snapshot deliveries. Indexed documentation was consulted on the inspection date; a live probe is still required.

The official [rate-limit reference](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/rate-limits-and-user-limits) documents a shared 1,200-weight REST budget per minute/IP, weight 2 for `clearinghouseState`, and 10 unique users across user-specific WebSocket subscriptions. Other requests and response sizes can consume more. Treat values as versioned provider configuration, not permanent assumptions.

The official [Info reference](https://hyperliquid.gitbook.io/Hyperliquid-docs/for-developers/api/info-endpoint) bounds fills history, including a 10,000-fill availability limit for `userFillsByTime`. Therefore an external snapshot can repair current quantity without recovering all missed historical activity. Time pagination requires overlap and duplicate handling, particularly at equal timestamps; do not blindly increment past the last timestamp.

## Historical V1 slice completion

**S0 baseline qualification completed:** see [docs/evidence/Q01-baseline-manifest.md](evidence/Q01-baseline-manifest.md) for the verified evidence manifest (Q01).

**S1 observation module completed:** see [docs/evidence/Q02-Q04-observation-manifest.md](evidence/Q02-Q04-observation-manifest.md) for Q02–Q04 acceptance evidence. Composite trade identities, monotonic revisions, strict snapshot validation, `ReconcileOutcome`, separate desired wallet universe, scoped equity and mark observations, decoupled ingestion via mpsc, and a shared weighted `RequestScheduler` are verified across 186 tests.

**S2 persistence engine completed:** see [docs/evidence/Q05-Q06-Q08-persistence-manifest.md](evidence/Q05-Q06-Q08-persistence-manifest.md) for Q05, Q06, Q08 acceptance evidence. Durable receipts ledger, economic dedup index, atomic checkpoint/outbox transitions, canonical event envelope, consistent snapshot pagination, bounded ordered tail stream, and retention pruning are implemented and verified across 194 tests.

**S3 projection and inspection engine completed:** see [docs/evidence/Q09-Q14-Q16-projection-manifest.md](evidence/Q09-Q14-Q16-projection-manifest.md) for Q09, Q14, Q16 acceptance evidence. The separate Python application repository is initialized at `/home/quant/dev/hyperliquid-expert-ensemble` with its own independent SQLite database (`projection.db`), consumer offset tracking, isolated as-known versus restated replay runner, job checkpoints, and compact JSON inspection commands (`inspect capabilities`, `inspect system`, `inspect changes`, `evidence lookup`), qualified across 200 tests.

**S4 target explanation completed:** see [docs/evidence/Q10-Q14-explanation-manifest.md](evidence/Q10-Q14-explanation-manifest.md) for Q10, Q14 acceptance evidence. Declared eligibility states, fixed-scale normalization, equal-budget consensus (B1) without survivor amplification, posture uncertainty bounds, causal change categories, contribution ledgers, and read-only target/blocker explanations are verified across 204 tests.

**S5 safe control completed:** see [docs/evidence/Q13-Q16-Q18-control-manifest.md](evidence/Q13-Q16-Q18-control-manifest.md) for Q13, Q16, Q18 acceptance evidence. Typed change proposals, immutable plans, scoped authority roles, idempotency receipts, prompt injection defenses, revision drift rejection, job cancellation, compact handoffs, and knowledge retention runbooks are verified across 211 tests.

**S6 similarity and clustering completed:** see [docs/evidence/Q11-Q15-clustering-manifest.md](evidence/Q11-Q15-clustering-manifest.md) for Q11, Q15 acceptance evidence. Aligned posture similarity features, flat-flat exclusions, support diagnostics, deterministic complete-link clustering, clone-resistant hierarchical aggregation (B2), and research promotion gates are verified across 215 tests.

**S7 account advice completed:** see [docs/evidence/Q12-Q08-advisory-manifest.md](evidence/Q12-Q08-advisory-manifest.md) for Q12, Q08 acceptance evidence. Explicit account rulebook, drawdown headroom calculations, downward lot rounding, UNSIZED fallbacks, action delta derivation under holdings uncertainty, and meaningful-change notification filtering are verified across 220 tests.

**S8 synthetic system qualification completed:** see [docs/evidence/Q07-Q14-Q16-Q17-Q18-synthetic-system-manifest.md](evidence/Q07-Q14-Q16-Q17-Q18-synthetic-system-manifest.md) and master index [docs/evidence/INDEX.md](evidence/INDEX.md) for Q07, Q14, Q16, Q17, Q18 acceptance evidence. The high-leverage integration fixture (Rust publication to Python consumption), real bounded source comparisons, deterministic replay isolation, the complete 6-task agent ergonomics battery, staged load and soak capacity evaluation (10/100/1,000 wallets meeting < 1.0s p95 budget), and incident recovery/runbooks are verified across 227 tests.

**S9 research promotion completed:** see [docs/evidence/Q15-research-promotion-manifest.md](evidence/Q15-research-promotion-manifest.md) for Q15 acceptance evidence. Controlled research candidates (B3 lagged quality weighting and B4 regime conditioning), strict train/eval temporal separation, min-support score neutral defaults, high-volatility tail dampening, search count multiple-testing tracking, and explicit policy approver gates with retained negative ablation records are verified across 231 tests.

**Milestone completion:** All issues #1 through #10 (slices S0 through S9) in milestone **Agent-operable expert ensemble V1** have verified evidence manifests and pass all qualification criteria under isolated shadow/advisory operation.
