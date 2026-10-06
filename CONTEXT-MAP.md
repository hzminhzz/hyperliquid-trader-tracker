# Context map

This is the navigation map, not a second specification. Start with [AGENTS.md](AGENTS.md) and [docs/STATUS.md](docs/STATUS.md). Terms are defined in [CONTEXT.md](CONTEXT.md).

| Context | Owns | Does not own | Contract authority |
|---|---|---|---|
| Observation | Hyperliquid receipts, reconstructed state, reconciliation, coverage | Trader quality or investment intent | [CONTRACTS](docs/CONTRACTS.md) |
| Ensemble | Expert posture, similarity, clusters, consensus, attribution | Exchange position accounting or broker execution | [RESEARCH](docs/RESEARCH.md) |
| Account advice | Account constraints, proposed target, incremental risk, advisory validity | Expert selection or source truth | [RESEARCH](docs/RESEARCH.md) |
| Operations | Inspection, scoped commands, budgets, receipts, capability discovery | Bypassing the preceding contexts | [OPERATIONS](docs/OPERATIONS.md) |
| Evidence | Immutable inputs, decision lineage, replay, evaluation, retained regressions | Rewriting past decisions or granting authority | [QUALIFICATION](docs/QUALIFICATION.md) |

The system composes these contexts through one linked evidence model; they are NOT instructions to deploy five services. [SYSTEM](docs/SYSTEM.md) defines the two-process runtime and ownership. [plan.md](plan.md) defines the implementation frontier. [ADR-0001](docs/adr/0001-agent-operating-model.md) records the costly choices.

## Document authority

- `STATUS.md`: dated, inspected capability inventory; not a deployment health report.
- `SYSTEM.md`: target system and module responsibilities.
- `CONTRACTS.md`: normative information, identity, durability, and delivery rules.
- `SIGNAL-V2.md`: compact V2 scoring/signal architecture synthesis and current claim boundary.
- `EXECUTION-REVIEW.md`: dated code audit and proposed execution/sampling amendment, including state machines, source limits, adversarial cases and the minimum experiment. It is not implementation or predictive-promotion authorization. The root plan identifies the proposed next frontier; existing evidence manifests retain their historical meaning.
- `RESEARCH.md`: normative signal semantics, feature definitions, research ladder, validation, and account-advice rules.
- `OPERATIONS.md`: agent-facing interface and operational authority.
- `QUALIFICATION.md`: executable acceptance scenarios and evidence requirements.
- `plan.md`: dependency-aware work slices; never evidence that a slice passed.
- `docs/DESIGN.md` and `docs/hyperliquid-api-map.md`: inherited reference material, with explicit fork corrections. They do not override the target documents.

When implementation diverges, report the mismatch and update the relevant authority; do not silently treat either code or prose as proof of intended behavior. A capability becomes available only through implementation plus verification, not by adding a command example to a document.
