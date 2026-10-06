# V20: V1/B0 baseline and evidence-semantics manifest

**Capsule ID:** V20  
**Milestone:** Forward-qualified expert signal engine V2  
**Issue:** #12 — V20: Freeze the V1 baseline and evidence semantics  
**Status:** PASS  
**Qualification class:** ENGINEERING / DESCRIPTIVE BASELINE  
**Inspection date:** 2026-10-06

## Repository pins

- Tracker: `hzminhzz/hyperliquid-trader-tracker`
  - V2 milestone base: `dbf55ce`
  - milestone branch starts from that commit.
- Expert engine: `hzminhzz/hyperliquid-expert-ensemble`
  - inspected head: `bf88f7a0e02f1b7b27ca95666367fd4613f0f0a6`
- Runtime working directory: `/home/quant/dev/hyperliquid-expert-ensemble`
- Runtime entrypoint declared by systemd: `.venv/bin/ensemble run`
- Service state at this inspection: `inactive`. This is an observation, not a deployment requirement.

## B0 definition

B0 is the exact V1 live equal-budget bounded consensus implemented by:

- `src/ensemble/posture.py::compute_posture`
- `src/ensemble/consensus.py::compute_equal_budget_consensus`
- `src/ensemble/runtime.py::EnsembleRuntime.compute_targets`

For each configured expert and coin:

```text
raw_exposure_i = quantity_i * valuation_price / equity_i
posture_i      = clip(raw_exposure_i / k_scale, -1, 1)
weight_i       = 1 / N_configured_experts
C_observed     = sum(weight_i * posture_i for available/known-flat experts)
missing_mass   = sum(weight_i for unavailable experts)
bounds         = clip([C_observed - missing_mass, C_observed + missing_mass], -1, 1)
```

Missing expert mass is not redistributed to surviving experts.

The pinned machine-readable fixture is [V20-b0-fixture.json](V20-b0-fixture.json). Its exact expected output is:

```text
observed_target = 0.375
missing_mass    = 0.25
lower_bound     = 0.125
upper_bound     = 0.625
is_actionable   = true
blocker_code    = null
```

## Current V1 candidate constants

These values are implementation defaults at the pinned expert-engine revision. V2 treats them as candidate parameters, not demonstrated optima.

| Mechanism | Current default |
|---|---:|
| posture scale `k_scale` | 1.0 |
| max equity age | 15 minutes |
| consensus insufficient-coverage threshold | missing mass >= 0.50 |
| pairwise similarity min support | 5 active steps |
| complete-link cluster threshold | 0.20 |
| B3 quality min support | 5 trades |
| B3 quality floor | profit-factor score >= 0.1 |
| B4 high-volatility threshold | 3% mean absolute lagged return |
| B4 high-vol target multiplier | 0.70 |
| research promotion Sharpe-delta example/default | 0.15 |
| research promotion turnover ceiling example/default | 0.10 |
| live runtime target-change notification threshold | 0.05 |

None of these constants is promoted to V2 predictive policy solely because it exists in code.

## Universe semantics

The live V1 expert universe is the configured `ENSEMBLE_EXPERTS` list loaded by `RuntimeSettings.from_env`. Per coin, the runtime constructs one posture for every configured expert; a wallet with no projected position is treated as quantity zero if its equity is otherwise admissible.

Historical studies using today's configured/curated wallet list are therefore **conditional fixed-universe studies** unless the admission/retirement process itself is reconstructed point-in-time. They must not be described as survivorship-free wallet-selection studies.

## Qualification-class correction

Existing V1 evidence is retained but narrowed to what it actually demonstrates:

| Evidence | What it proves | What it does not prove |
|---|---|---|
| Q10 | engineering/descriptive correctness of normalization, eligibility states, missing-mass accounting, explanation | clipping improves forward returns; target is calibrated confidence |
| Q11 | engineering/descriptive clone resistance, complete-link behavior, low-support semantics | clustering improves forward returns |
| Q15 | engineering correctness of lagged-input mechanics, candidate gating and approval control flow | B3 quality weights or B4 regime dampening have predictive/economic value |

Historical predictive, live-forward, and advisory-production claims require the new V2 qualification classes.

## Verification

### Tracker

```text
make check
```

Result:
- ruff: PASS
- ty: PASS
- Python: 79 passed
- Rust library/unit/integration: 115 passed total
- clippy/fmt: PASS

### Expert engine

```text
uv run ruff check .
uv run ty check
uv run pytest -q
```

Result:
- ruff: PASS
- ty: PASS
- pytest: 39 passed

No runtime source, service configuration, credentials, or service state was changed by V20.

## Exit

**PASS.** B0 is pinned and exactly reproducible, universe-selection semantics are labeled correctly, and V1 evidence is separated from predictive qualification. V21 may deepen WalletEvidence without changing observation ownership.
