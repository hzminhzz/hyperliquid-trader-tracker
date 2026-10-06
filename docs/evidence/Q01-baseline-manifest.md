# Q01: Baseline Evidence Manifest

**Capsule ID:** Q01  
**Milestone:** Agent-operable expert ensemble V1  
**Issue:** #1 — S0: Establish a trustworthy baseline  
**Status:** PASS  
**Inspection Date:** 2026-10-06  
**Host:** Linux vmi2637217 7.0.0-34-generic x86_64  

---

## 1. Repository & Baseline Identity

- **Fork Repository:** `hzminhzz/hyperliquid-trader-tracker`
- **Parent Upstream:** `KonScanner/hyperliquid-trader-tracker`
- **Baseline Git Pin:** `ffff331aa0f52e299ee995e2c96246c1253712a7`
- **Active Branch:** `feat/agent-operable-expert-ensemble-v1`
- **Remotes Configured:**
  - `origin`: `https://github.com/hzminhzz/hyperliquid-trader-tracker.git` (fetch & push)
  - `upstream`: `https://github.com/KonScanner/hyperliquid-trader-tracker.git` (fetch & push)
- **Upstream Divergence:** 0 commits (`origin/main` is identical to `upstream/main` at `ffff331`).
- **Issue Tracker:** GitHub Issues on `hzminhzz/hyperliquid-trader-tracker` (Milestone 1, Issues #1–#10). REST API endpoint (`/repos/{owner}/{repo}/issues/{n}`) verified; GraphQL `projectCards` query avoided due to GitHub sunset deprecation.

---

## 2. Subsystem Ownership & Architecture Separation

- **Rust Subsystem (`src/main.rs`, `src/tracker/*.rs`):**
  - Mandatory KonScanner observation foundation.
  - Owns exchange-state reconstruction, trade ingestion, reconciliation, and (planned S2) durable persistence.
- **Inherited Python Subsystem (`src/tracker/*.py`, `tests/test_*.py`):**
  - Inherited legacy Telegram notification tracker matching the Rust port.
  - **Explicit boundary:** This inherited Python tracker is NOT the planned Python expert ensemble engine.
  - The planned expert ensemble (S3+) will be a separate application with its own transactional projection/offset store, evidence manifests, and replay runner.

---

## 3. Toolchain & Command Capabilities

| Tool / Runner | Version | Command | Result |
|---|---|---|---|
| Rust Compiler | `rustc 1.89.0 (29483883e 2025-08-04)` | `rustc --version` | Verified |
| Cargo | `cargo 1.89.0 (c24e10642 2025-06-23)` | `cargo --version` | Verified |
| Rust Formatter | `rustfmt 1.8.0-nightly` | `cargo fmt --all --check` | Clean |
| Rust Clippy | `clippy 0.1.89` | `cargo clippy --all-targets -- -D warnings` | 0 warnings, clean |
| Rust Test Suite | Built-in test runner | `cargo test` | 97 passed, 0 failed |
| Python Environment | CPython 3.14.6 via `uv 0.12.15` | `uv --version` | Verified |
| Python Linter | `ruff 0.8.x` | `uv run ruff check .` | Clean |
| Python Typechecker | `ty 0.0.1a1` | `uv run ty check` | Clean |
| Python Test Suite | `pytest 8.3.x` | `uv run pytest -q` | 79 passed, 0 failed |
| Combined Verification | `Makefile` target | `make check-rs` | PASS |
| Full Repository Check | `Makefile` target | `make check` | PASS |

---

## 4. Host Environment & VPS Resources

- **Operating System:** Linux Ubuntu 7.0.0-34-generic #34-Ubuntu SMP PREEMPT_DYNAMIC x86_64
- **CPU Cores:** 8 vCPUs (`nproc` = 8)
- **Memory (RAM):**
  - Total: 23 GiB
  - Used: 16 GiB
  - Buff/Cache: 6.3 GiB
  - Available: 7.2 GiB
  - Swap: 8.0 GiB (4.1 GiB used, 3.9 GiB free)
- **Disk Storage:**
  - Filesystem: `/dev/sda1` on `/`
  - Total: 436 GiB
  - Used: 284 GiB (66%)
  - Available: 153 GiB
- **Public Outbound IP:** `173.249.52.222`

---

## 5. Shared IP / API Consumers & Rate Limit Analysis

- **Co-located Processes & Docker Containers:**
  - `hummingbot/hummingbot:arcus-native` (Container `343fdf30ca5e`, running actively)
  - `hummingbot/hummingbot-api:arcus-native` (Container `d0fd6198185c`, listening on `127.0.0.1:8000`)
  - `postgres:16` (`hummingbot-postgres`, port `5432`)
  - `emqx:5` (`hummingbot-broker`, ports `1883`, `18083`)
  - `tailscale/tailscale:latest` (`hummingbot-tailscale`)
  - `infisical/agent-vault:0.40.0` (ports `14321`, `14322`)
- **Shared IP Resource Risk:**
  - Hummingbot is actively running on the identical public IP (`173.249.52.222`).
  - Hyperliquid REST endpoint imposes a shared 1,200 weight/minute per IP budget (`clearinghouseState` = weight 2).
  - Hyperliquid WebSocket limits to 10 unique users across user-specific WS subscriptions per connection.
  - **Operational Constraint:** The tracker cannot assume exclusive use of the 1,200 weight/minute API budget. A shared weighted request scheduler with recovery allowance (G09) is mandatory.

---

## 6. Bounded Public Read Probe

- **Target Endpoint:** `https://api.hyperliquid.xyz/info`
- **User-Agent:** `OpenAI File Downloader, XaiImageApiFetch/1.0` (mandatory per repository agent instructions)
- **Request Body:** `{"type":"meta"}`
- **Observed Response:**
  - HTTP Status: `200 OK`
  - Round-trip Latency: `0.333s`
  - Universe Content: 203 active and delisted asset entries (`BTC`, `ETH`, `SOL`, etc.)
- **Execution Limits:** Bounded read-only probe strictly. No order submission, no private wallet queries, and no Telegram broadcasts.

---

## 7. Scoped Initial Wallet Set & Live-Test Boundaries

- **Fork Public Visibility:** The repository is public. Private wallet addresses, Telegram Bot API tokens, and production evidence must never be committed to Git.
- **Live Trading Authority:** Zero financial execution authority in V1. No order placement or trading execution path enabled.
- **Test Scoping:** Testing relies on deterministic offline fixtures (`tests/` and crate tests) and bounded public read probes.
- **Initial Wallet Set:** Offline synthetic addresses (`0xa`, `0xb`, etc.) for unit/integration suites; public high-activity addresses for read-only live calibration when authorized.

---

## 8. Testable Baseline Findings for Inspected Gaps (G01–G11)

| Gap ID | Inspected Behavior | Reproducer / Verification Test | Status |
|---|---|---|---|
| **G01** | `SeenTids` deduplicates on raw scalar `tid: i64` without `(coin, address)` scoping; cross-instrument duplicate IDs collide. | `tests/test_baseline_gaps.py::test_g01_seen_tids_unscoped_dedup`, `src/tracker/book.rs::test_g01_seen_tids_unscoped_dedup_cross_coin` | Classified Defect |
| **G02** | `Listener::handle_trades` couples in-memory book mutation with synchronous PnL lookup and Telegram dispatch in the hot ingestion loop. | Code inspection: `listener.rs:104-124`. A slow Telegram response blocks trade ingestion. | Classified Architecture Gap |
| **G03** | Live dispatch depends on Telegram chat subscribers; startup derives tracked wallets only from chat subscriptions. | Code inspection: `app.rs:65-72`. If no Telegram subscriber exists, no wallets are tracked. | Classified Architecture Gap |
| **G04** | `reseed_wallet` silently replaces state; `seed_wallet` return value discarded in batch reconcile loops. | Code inspection: `enrich.rs:62-75`, `app.rs:250-265`. Uninspected failures do not raise alerts. | Classified Defect |
| **G05** | `fill_epoch` resets to 0 on `reseed_wallet` via `drop_wallet`, breaking monotonic revision guarantees. | `tests/test_baseline_gaps.py::test_g05_fill_epoch_resets_on_reseed`, `src/tracker/book.rs::test_g05_fill_epoch_resets_on_reseed` | Classified Defect |
| **G06** | Missing or malformed `assetPositions` in `clearinghouseState` is parsed as `[]`, wiping existing positions on reconcile. | `tests/test_baseline_gaps.py::test_g06_missing_asset_positions_treated_as_flat` | Classified Defect |
| **G07** | `seed_state_from_row` assigns `opened_at = fallback_ts` (now), masking true entry time and distorting trade duration. | `tests/test_baseline_gaps.py::test_g07_seeded_position_uses_observation_time` | Classified Defect |
| **G08** | Seeding precedes admission; trade feed filters unadmitted wallets, dropping fills occurring during seed. | Code inspection: `app.rs:180-205`. Fills during seed window are lost without an overlap cut. | Classified Defect |
| **G09** | Concurrency uses an unweighted semaphore; lack of shared rate-limit scheduler across reconcile, enrich, and recovery. | Code inspection: `enrich.rs:64`. Risk of IP rate-limit bans against Hyperliquid given co-located Hummingbot. | Classified Architecture Gap |
| **G10** | Universe discovery calls `{"type":"meta"}` without multi-dex / spot parameters; no HIP-3 or DEX-aware coverage. | Code inspection: `enrich.rs:90-105`, `resolve.rs:280-295`. | Classified Scope Limit |
| **G11** | Positions, epochs, and trade receipts are purely in-memory (`InMemoryBook`); SQLite database persists only `subscriptions`. | `tests/test_baseline_gaps.py::test_g11_database_persists_only_subscriptions` | Classified Architecture Gap |

---

## 9. Conclusion & Exit Recommendation

Baseline evidence criteria for scenario **Q01** are satisfied. The fork baseline, host resources, test suites, shared IP consumers, and reproducible limitations G01–G11 are classified with reproducible test coverage.

- **Milestone S0 Exit:** PASS.
- **Handoff to S1:** Ready. S1 focuses on resolving observation semantics (composite identity G01, monotonic epochs G05, strict snapshot validation G06, and decoupled ingestion G02).
