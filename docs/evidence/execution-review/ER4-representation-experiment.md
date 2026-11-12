# ER4 frozen representation experiment

Date: 2026-10-06  
Issue: #28  
Status: **PROTOCOL_ENGINEERING_PASS / EMPIRICAL_EVALUATION_BLOCKED**

## Frozen protocol

- R0: state only.
- R1: state + additive net/gross flow + recency + build rate.
- R2: R1 + segment elapsed + pause state + execution label + supported taker fraction.
- R2 annotations are optional; unavailable annotations do not block R0/R1.
- Primary horizon: 15m. Secondary: 1h.
- Attached outcomes: 1m, 5m, 15m, 1h, 4h, 24h.
- Market control: lagged return and realized volatility.
- Planned universe labels: original_50 and prospective_120; actual private manifests must be supplied prospectively.
- Protocol hash is content-derived; freeze must precede holdout.

## Outcome corrections

- Outcome sources carry direction.
- Economic net return applies direction before subtracting costs.
- Flat price plus positive costs is negative for both long and short.
- Price observations may carry availability time.
- A maximum price-lateness bound censors late horizon/latency prices instead of silently using a distant point.
- Outcome known time reflects price availability.

## Verification

Companion engine:
- Ruff: PASS.
- Changed ER4 modules: \`ty\` PASS.
- V25 + ER4 focused tests: **10 passed**.
- Full engine suite: **106 passed**.

## Decision

The local observation ledger had no recorded events at the ER0 cut. Therefore there are no prospective,
outcome-mature real R0/R1/R2 rows to evaluate. The readiness contract returns **BLOCKED /
NO_RECORDED_EVIDENCE_ROWS** rather than a promotion result.

No predictive representation is promoted. The protocol is ready for later prospective data without
changing the frozen design.
