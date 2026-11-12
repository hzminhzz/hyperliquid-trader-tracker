# DI production activation: causal observation and descriptive intelligence

**Verified:** 2026-10-13 10:14 UTC (17:14 Asia/Bangkok)
**Scope:** Activate production **observation, shadow research, deterministic agent inspection and descriptive Telegram digest**, not predictive trade entries or financial execution.

## Release and authority

- Python ensemble release commits **`a3412f7`** (causal shadow collector, outcomes and archiving) and **`96163bd`** (descriptive board, Telegram digest, typed agent inspection, mode pinning and tested deployment). Existing tracker design documentation base **`204105b`**.
- Original tracker, ensemble and raw outcome services all continued running; the new collectors did **not** replace Rust exchange-state authority, the existing grade-based Telegram engine, or brokerage execution. All are on the existing VPS feature branch.
- Separate operational user-systemd timers active:
  1. `copytrade-opportunity-shadow.timer` (20s, 256 MiB max) tails the authoritative ledger and appends first-known wallet action observations.
  2. `copytrade-opportunity-outcomes.timer` (~10m, 384 MiB max) appends hypothetical mid-to-mid forward outcomes and same-coin momentum comparisons.
  3. `copytrade-flow-digest.timer` (09:00 and 21:00 Asia/Bangkok, 128 MiB max) emits a bounded descriptive summary with explicit NOT QUALIFIED warnings and no trade/size instructions.
- Actual first digest accepted and marked **DELIVERED_DESCRIPTIVE_ONLY**, single attempt, at **2026-10-13 10:10:04 UTC** (period `2026-10-13:0`). The service exited successfully, memory peak ~15 MiB. No credentials or wallet address roster is stored in repository.
- Persistent resource constraints applied to existing user services without restarting them: Rust tracker MemoryHigh/Max **128/384 MiB**, Python ensemble **2500/3500 MiB**, raw outcome recorder **600/1000 MiB**. All six underlying services/timers were active on reinspection. This limits bot growth but cannot fix unrelated host-level swap use or the historic outage.

## Agent-facing real production inspection

```bash
cd /home/quant/dev/hyperliquid-expert-ensemble
uv run --frozen --no-sync python -m ensemble.copy_shadow health
uv run --frozen --no-sync python -m ensemble.copy_shadow board --hours 4 --limit 5
uv run --frozen --no-sync python -m ensemble.copy_shadow changes --since-seq 232911 --limit 20
uv run --frozen --no-sync python -m ensemble.copy_shadow wallet --wallet <observed-wallet-id>
uv run --frozen --no-sync python -m ensemble.copy_shadow explain --id copy:<opportunity-id>
uv run --frozen --no-sync python -m ensemble.copy_shadow study --horizon 1h
```

The `copy explain` capsule returns frozen decision inputs and separately matured outcomes; it does not revise original as-known evidence. `copy board` deduplicates same-wallet directional events per instrument over the requested window and segregates long/short entries, de-risking and exits. It does **not** claim these addresses are independent clusters or statistically predictive. No size/entry price is estimated. Telegram rendering and send cadence are deliberately separate from the outcome model. The legacy grades A/B remain explicitly descriptive.

## Live verified snapshot

- Rust source ledger sequence **238,547**, shadow journal cursor **238,547**; new live shadow opportunities **528**, of which many are NO_COPY reductions/exits.
- Shadow outcome labels **2,414** and separately matched momentum controls **264**, all explicitly **HYPOTHETICAL MID / ASSUMED COST**. No label is a verified executable fill.
- Live Telegram delivery configured and successful; 215 tracked wallets, prior cluster estimate ~203, not independently skill-qualified.
- **172 Python tests pass**, Ruff clean, `ty` clean. Two user timer runs have Result=success; system and research health were observed as live and reporting.
- Historical scoring gap **2026-10-11 17:27 UTC -> 2026-10-12 14:45 UTC (~1278 min)** remains unrecoverable as contemporaneous predictions.
- Host swap free **0 MiB** at this audit, though approximately 6 GiB memory was available; DI0 host-wide root-cause and resource contention remain an explicit operational BLOCKER. Do not kill unrelated agent processes or silently reconstruct missing signals.

## Honest DI0–DI7 terminal status

| Goal | Verdict |
|---|---|
| Rust ledger + causal semantic interpretation + bounded live shadow recording | **OPERATIONAL**; native program metadata and additional coverage qualifications pending |
| Immutable wallet-level opportunity IDs, mode pinning and exact source cursor | **OPERATIONAL**; independent cross-wallet consensus and expiry not fully implemented/qualified |
| Latency-aware hypothetical outcomes + frozen price-only comparator | **OPERATIONAL**; cannot claim executable PnL without spread/fills/impact/funding |
| Independent predictive mechanism and complete research gate | **BLOCKED / INCONCLUSIVE**; insufficient untouched forward horizon/diverse independent episodes |
| Wallet skill, leader/follower, independence and entry sizing | **NOT QUALIFIED**; no autonomous financial decisions |
| Compact agent inspections, explainable decisions and descriptive Telegram | **OPERATIONAL**; does not replace old grade-based Telegram policy |
| Compressed, hash-verified export | **IMPLEMENTED**; no source deletion or native Parquet/sparse production writer migration |
| Host-wide OOM prevention across unrelated processes | **PARTIAL**; scoped caps installed but host swap remains exhausted |

**Go-live interpretation:** `PRODUCTION_DESCRIPTIVE_ONLY`. Production means resilient reporting of observations and research labels, **NOT** certified trading alpha. Automatic copy execution, long/short advice, sizing and predictive promotion stay disabled. Do not mark DI0–DI7 scientific terminal predicate PASS until untouched forward validation actually meets its preregistered acceptance gates.
