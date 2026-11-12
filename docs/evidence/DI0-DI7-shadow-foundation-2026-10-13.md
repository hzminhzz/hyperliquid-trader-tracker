# DI0–DI7 execution evidence: shadow opportunity foundation

**As-of:** 2026-10-13 UTC
**Scope:** VPS production audit, Python ensemble shadow-only implementation, local bounded retrospective replay and cost-aware control experiment, plus independently capped prospective source/outcome shadow timers **deployed at approximately 2026-10-13 06:55 UTC**. Financial execution not enabled.
**Repositories inspected:** tracker `b33e095` baseline; engine `e95cd22` baseline, on matching feature branches; new changes are on those branches, not a deployed production revision.

## Verified work

| Phase | Code/evidence | Honest state |
|---|---|---|
| DI0 source integrity | User services active; ledger caught up. Kernel/system OOM logs show repeated kills on Oct 11; the one-minute scoring record has 1278-minute absence from Oct 11 17:27 UTC to Oct 12 14:45 UTC. Swap ~8GiB used, many unrelated VPS processes. New bounded read-only `copy health` reports source, gaps, runtime research qualification and host memory. Kernel messages denied to nonprivileged user; cause/root-owner not established. | **PARTIAL / operational remediation blocked** |
| DI1 semantic actions | `copy_opportunities.py` parses source delta and exact before/after quantity, enforces declared transition, and separates source action from follower direction. REDUCE_LONG/CLOSE_SHORT cannot create new opposite entries; partial data/late source blocks. Existing `wallet_evidence.py` and `execution_annotations.py` remain independent source/research helpers. | **ENGINEERING_PASS in shadow/replay; production path not integrated** |
| DI2 immutable decision journal | New `OpportunityJournal` with stable first-source identity, transactional ledger cursor, append-only source updates, ten-minute per-wallet/instrument/direction ADD coalescing, no-copy exclusions and fixed action-latency stamp. Tested restart, gap and split actions. | **ENGINEERING_PASS for candidate semantics and independent prospective Rust-ledger journal; timers live, but full expiry/cluster aggregation not yet qualified** |
| DI3 economic labels and controls | `copy_research.py`, `copy_shadow.py` emit price-known-at, assumed-latency and assumed-cost mid-to-mid labels with missing-price censoring; one-hour lagged same-coin momentum is separately frozen as a comparison. No actual spread, slippage, funding, order fills or broker latency captured. | **HYPOTHETICAL/EXPLORATORY; executable PnL BLOCKED** |
| DI4 evidence evaluation | `copy_study.py` joins immutable paired candidates and controls, counts one first opportunity per coin per 4-hour bucket (24-hour for 24h labels), requires day support and ALWAYS reports no OOS qualification. | **EXPLORATORY/INCONCLUSIVE; forward holdout BLOCKED** |
| DI5 skill | `shrink_wallet_skill` uses prior days only and a day-mean contribution with zero-prior shrinkage; missing day support is UNKNOWN. No qualified live skill/risk weights or leader/follower cohort. | **RESEARCH HELPER only** |
| DI6 agent operating interface | `ensemble copy scan/evaluate/inspect/health/study/explain/export/verify` CLI and bounded JSON health capsule; no live Telegram decision adapter. | **CLI ENGINEERING_PASS; separate source/outcome shadow timers deployed, notification integration BLOCKED** |
| DI7 archive | Bounded streaming gzip NDJSON partition export, hash manifest, verification. Optional Parquet adapter is unavailable in current `.venv`; historical one-minute production SQLite writer has not been optimized or pruned. | **ARCHIVE ENGINEERING_PASS; native Parquet/storage migration BLOCKED** |

## Production observational snapshot and retrospective counts

- Rust ledger consumed **232,509** rows in three bounded retrospective scan passes; **31,561** first wallet-level opportunities resulted. The action mix included 19,957 REDUCE_LONG, 4,539 REDUCE_SHORT, 3,173 ADD_LONG, 318 ADD_SHORT, 1,514 OPEN_LONG, 258 OPEN_SHORT, 1,478 CLOSE_LONG, 293 CLOSE_SHORT and 31 FLIP actions. The remainder of trade transitions were coalesced into earlier opportunities; account snapshots advanced cursor but were not counted as copy decisions.
- These observations involve 189 distinct wallets and 114 instruments during Oct 6–13. **They are not 31,561 independent alpha signals.** Wallet overlap, market shocks and coalescing are not fully qualified.
- A bounded 150-opportunity audit of Oct 8/9 observational records generated 44 directional source opportunities with hypothetical forward labels and 106 no-copy de-risk/exit records. It created 170 paired momentum controls across horizons.
- The exploratory 1-hour subset with matched controls had **34** paired source events that collapsed to **15 coin/time buckets on one UTC day**. Its observed mean relative to the simple momentum comparator was **+73.45 bps**, **NOT predictive qualification**: one-day selection, partial retrospective period, overlapping assets, hypothetical mid fills, a chosen control and no untouched holdout. No trading decision or sizing claim follows.
- The first **10,000** shadow opportunities were exported as bounded gzip NDJSON to a separate private VPS research file, manifest SHA-256 `9112bb98701b51bf163459a03e5c486faef794eaf588fdd07a7d98f3622ff370`, verified intact. Neither ledger nor historical SQLite was deleted.

## Verification

From `/home/quant/dev/hyperliquid-expert-ensemble`:

```text
uv run --frozen --no-sync pytest -q -p no:cacheprovider
166 passed
uv run --frozen --no-sync ruff check src tests
All checks passed!
uv run --frozen --no-sync ty check src
All checks passed!
```

The first 27 focused tests covered source semantics, no-copy exits, flip, restart/gap, 2s latency, assumed signed cost and immutable controls; later tests add integration replay, deduped study and hash export verification. These tests are software correctness proof, not backtested or live signal quality proof.

## Remaining gates / high-ROI next actions

1. Resolve shared VPS OOM/root causes with host-wide memory and cgroup evidence or reduce unrelated agent/process memory safely; qualify zero silent source drops and identify impact of historical gap. Never reconstruct false fills into the gap.
2. **Prospective shadow capture started at source cursor 232,911 around 06:55 UTC** and recorded 12 candidate decisions by approximately 06:57 UTC (not independent signals). Audit the full raw trade -> first actionable market quote -> matured outcome chain and process restart recovery. Bounded timers use 256/384 MiB MemoryMax; the existing Telegram policy remains unchanged.
3. Integrate no-copy/opportunity capsules with source-aware cluster independence and Telegram **only after verification**; keep the current production descriptive advisory untouched until then.
4. Preregister DI4 fresh independent accumulation and de-risking warning experiments, cost/slippage sensitivities, controls and untouched evaluation period. No retrospective result or one-day proxy can satisfy this gate.
5. Add native Parquet streaming if a qualified offline Polars/Arrow environment is installed and profile optimized sparse journal vs current minute-by-coin outcomes. The current 2.5GiB research SQLite is preserved and still growing; no automatic retention/prune is authorized by this evidence.

**Claim boundary:** Observations and historical samples are genuine; the new shadow code is not the deployed Telegram system. No predictive feature, executable trade, trading size or financial execution is qualified by this slice.
