# Inspected status and implementation gaps

**Inspection:** 2026-10-06. **Code baseline:** `ffff331aa0f52e299ee995e2c96246c1253712a7`.

This page records code inspection, not a running-service health check. The fork is `hzminhzz/hyperliquid-trader-tracker`; its VPS checkout is `/home/quant/dev/hyperliquid-trader-tracker`. At inspection the code matched the fork's `main`. The local remote was `origin`; no `upstream` remote was configured. Recheck these facts before implementation.

## Capability inventory

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

## Current frontier

**S0 baseline qualification completed:** see [docs/evidence/Q01-baseline-manifest.md](evidence/Q01-baseline-manifest.md) for the verified evidence manifest (Q01).

**S1 observation module completed:** see [docs/evidence/Q02-Q04-observation-manifest.md](evidence/Q02-Q04-observation-manifest.md) for Q02–Q04 acceptance evidence. Composite trade identities, monotonic revisions, strict snapshot validation, `ReconcileOutcome`, separate desired wallet universe, scoped equity and mark observations, decoupled ingestion via mpsc, and a shared weighted `RequestScheduler` are verified across 186 tests.

**S2 persistence engine completed:** see [docs/evidence/Q05-Q06-Q08-persistence-manifest.md](evidence/Q05-Q06-Q08-persistence-manifest.md) for Q05, Q06, Q08 acceptance evidence. Durable receipts ledger, economic dedup index, atomic checkpoint/outbox transitions, canonical event envelope, consistent snapshot pagination, bounded ordered tail stream, and retention pruning are implemented and verified across 194 tests.

**S3 projection and inspection engine completed:** see [docs/evidence/Q09-Q14-Q16-projection-manifest.md](evidence/Q09-Q14-Q16-projection-manifest.md) for Q09, Q14, Q16 acceptance evidence. The separate Python application repository is initialized at `/home/quant/dev/hyperliquid-expert-ensemble` with its own independent SQLite database (`projection.db`), consumer offset tracking, isolated as-known versus restated replay runner, job checkpoints, and compact JSON inspection commands (`inspect capabilities`, `inspect system`, `inspect changes`, `evidence lookup`), qualified across 200 tests.

**S4 target explanation completed:** see [docs/evidence/Q10-Q14-explanation-manifest.md](evidence/Q10-Q14-explanation-manifest.md) for Q10, Q14 acceptance evidence. Declared eligibility states, fixed-scale normalization, equal-budget consensus (B1) without survivor amplification, posture uncertainty bounds, causal change categories, contribution ledgers, and read-only target/blocker explanations are verified across 204 tests.

Implementation frontier advances to **S5** in [plan.md](../plan.md): make control safe and cheap (operations facade, typed plans, expected revisions, grants, receipts, and bounded job control).
