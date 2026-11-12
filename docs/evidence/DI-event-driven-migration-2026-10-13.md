# Production migration: consensus-minute research to event-driven journal

**Cutover timestamp:** 2026-10-13 ~11:04 UTC
**Authorization:** User explicitly approved immediate migration of the research pipeline.
**Authority:** Recording and descriptive production only. No model promotion, automatic orders, trade-entry/sizing recommendations or retrospective-fabricated predictions.

## What changed in production

The existing Rust tracker remains sole source of accepted account changes and position transitions. The Python ensemble still computes contemporaneous descriptive wallet/cluster consensus for its **existing** Telegram advisory, and still consumes the authoritative Rust ledger. **Its research writer no longer writes the legacy `signal_cuts` or `forward_outcomes`**. It now writes only valid, source-known default-perp Hyperliquid minute reference mids into a separate transactional SQLite `market-reference.db`, without creating artificial consensus outcomes. It does not price unsupported HIP-3 instruments. The source-known price checkpoint clock remains distinct from event/decision/entry time.

`copytrade-opportunity-shadow.timer` continues to emit bounded ledger-derived wallet-level CopyOpportunities and no-copy exclusions to `copy-opportunities-live.db`. `copytrade-opportunity-outcomes.timer` now reads `market-reference.db`, evaluates bounded events over 5m/15m/1h/4h/24h horizons and records hypothetical cost-aware outcomes and a same-asset, prior-1h price-only control in `copy-outcomes-live.db`. Its timer has a normal successful run following the cutover.

`copytrade-outcomes.service`, previously an independent raw signed-fill markout recorder writing `outcomes.db`, was stopped and **disabled** after the event-driven research path was verified. Raw fill markouts and directional copy-entry outcomes are distinct analytical targets; the historical raw-fill archive remains intact and may be consulted in offline research. It is not the promoted copy entry engine.

`copytrade-flow-digest.timer` remains active; its messages remain **DESCRIPTIVE ONLY**, not predictive trade advice. No user wallet, leverage or execution privileges changed.

## Safeguards and observed verification

- Before cutover, seeded **184,675** exact historical `reference_mids` from the preceding ~28 hours into the new store. Source `known_at` timestamps and instrument namespaces were retained exactly, without backdating newly actionable opportunities. Immutable old ledger, cuts, outcome labels and reference data were neither modified nor deleted.
- Post-cutover archived `prospective-research.db` contains **885,037 legacy cuts** and **5,075,083 legacy forward outcomes**; the last cut belongs to **2026-10-13 11:04:00 UTC**. The counts stayed unchanged through at least 11:07 UTC, after more market-price checkpoints were recorded.
- New `market-reference.db` grew from **184,675** to **185,271** price rows by 11:07 UTC. The new `copy-outcomes-live.db` continued to append event-driven results and matched momentum controls.
- Source ledger and main ensemble were active following the single restart; observed consumed/source lag **0** in the immediate post-cutover health capsule. The six-style timer set remained active except the intentionally retired raw `copytrade-outcomes.service`.
- New mode is **default `ENSEMBLE_RESEARCH_MODE=price_only`**, supported alongside explicit legacy compatibility for replay/rollback and `off` for maintenance only. Production health reports `EVENT_DRIVEN_PRIMARY_REFERENCE_ONLY_NOT_ER4_QUALIFIED`, `legacy_consensus_cuts_enabled=false`, `legacy_forward_outcomes_enabled=false`.
- Historical gaps including **1278 minutes (Oct 11–12)** remain historically invalid; this cutover does not heal them. VPS 8 GiB swap was still essentially exhausted. Existing per-service cgroup memory limits are preserved.
- **Final post-cutover verification: 176 Python tests passed; Ruff and `ty` both passed.** Engine migration commit **`69b7fba`** on `feat/expert-wallet-execution-correctness-prospective-research`. Production price-only store and event journal remained active during and after verification.

## Paths and deliberate inactive archives

| Role | Path / process | State |
|---|---|---|
| Immutable canonical observations | `observation.db`; `copytrade-tracker.service` | ACTIVE |
| Legacy descriptive consensus, forward outcomes and prices | `prospective-research.db` | PRESERVED, NO NEW WRITES |
| Legacy raw signed-fill outcomes | `outcomes.db`; `copytrade-outcomes.service` | PRESERVED DB; SERVICE DISABLED |
| Continuous qualified mid-price reference | `market-reference.db`; price-only ensemble capture | ACTIVE |
| Event-ledger opportunity journal | `copy-opportunities-live.db`; shadow timer | ACTIVE |
| Event-level outcomes and controls | `copy-outcomes-live.db`; research timer | ACTIVE |
| Descriptive Telegram and agent interface | main ensemble and flow-digest timer | ACTIVE, NO FINANCIAL ACTIONS |

## Reproducible operations and rollback

Read-only verification:

```bash
cd /home/quant/dev/hyperliquid-expert-ensemble
uv run --frozen --no-sync python -m ensemble.copy_shadow health
uv run --frozen --no-sync python -m ensemble.price_checkpoints stats
uv run --frozen --no-sync python -m ensemble.copy_shadow study --horizon 1h
systemctl --user is-active copytrade-tracker.service copytrade-ensemble.service copytrade-opportunity-shadow.timer copytrade-opportunity-outcomes.timer copytrade-flow-digest.timer
systemctl --user is-enabled copytrade-outcomes.service
```

**Rollback is deliberate, not automatic.** Keep all existing files and record a safe cursor. Explicitly set `ENSEMBLE_RESEARCH_MODE=legacy` in the ensemble service environment and restart **only** that ensemble service to resume old minute cuts. To point event outcomes at the legacy reference store, revert the separate outcome service's `--prices` argument and reload/restart its timer. Reenable raw-fill recorder only if historical raw-flow analytics are required and its resource budget has been verified. This manual rollback would increase storage and correlated labels; no archived observations should be relabeled as newly actionable. Do not silently restore old signals during unrelated restarts.

## Claim boundary

**Operational migration complete** for research writing and event-level evaluation. **Scientific qualification incomplete**: wallet-cluster independence and lagged skill, actual spread/slippage/funding/executable fills, robust holdout market-control excess alpha, unsupported market sources and hosted VPS stability remain outstanding. Existing grade-based Telegram consensus may remain as a descriptive comparator; it has **not** become an event-validated trading signal.
