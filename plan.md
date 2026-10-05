# Agent-operable expert ensemble: implementation plan

**Status:** revised design plan, 2026-10-06. No implementation slice is completed by this document. **Mandatory infrastructure:** the user's KonScanner fork. **Baseline:** `ffff331aa0f52e299ee995e2c96246c1253712a7`.

## Goal and terminal predicate

Produce a two-process, observation/advisory system that an agent can inspect accurately, explain causally, improve through bounded experiments, operate within explicit authority, and recover without inventing knowledge. The user supplies the expert universe. Discovery subscriptions, paid analytics, and real order execution are not prerequisites.

V1 is complete when S0-S8 have accepted evidence (or an optional research candidate is explicitly rejected in favor of its qualified baseline), the system operates in shadow/advisory mode, every target has replayable lineage and validity, agent operations pass the task battery, and no real execution authority is enabled. Economic advantage remains a separately reported result, never an assumed deliverable.

Authoritative references: [SYSTEM](docs/SYSTEM.md), [CONTRACTS](docs/CONTRACTS.md), [OPERATIONS](docs/OPERATIONS.md), [RESEARCH](docs/RESEARCH.md), [QUALIFICATION](docs/QUALIFICATION.md). [STATUS](docs/STATUS.md) lists inspected gaps G01-G11.

## Execution principles

Use one autonomous top-level goal over the dependency graph, not a manual continuation per small ticket. At each step inspect actual repository/branch/worktree/GitHub state and instructions; preserve legitimate concurrent changes; select the ready frontier; delegate independent bounded work through the available native executor; implement; diagnose ordinary failures; rerun; inspect the diff/evidence; and recompute dependencies. Long deterministic jobs may use the installed worker runner with checkpoints and budgets. Only commit/push, deploy, or change repository settings within authorization.

The agent may not weaken an acceptance predicate, fabricate a completed capability, spend beyond its budget, or enable financial execution to make the plan finish. A failed hypothesis is retained as a result; a failed correctness gate blocks dependent implementation/promotion.

## Dependency graph

```text
S0 baseline -> S1 correct observation semantics -> S2 durable publication
                                                    |
                                                    v
                                      S3 inspect + replay + evidence
                                             /             \
                                            v               v
                                  S4 baseline target    S5 bounded control
                                             \             /
                                              v           v
                                    S6 dedup candidate   S7 account advice
                                               \          /
                                                v        v
                                             S8 qualification
                                                    |
                                             S9 optional learning
```

S6 and S7 each depend on BOTH S4 and S5. They can be developed independently behind their interfaces; S8 integrates them. S9 is optional and must not block V1. Shared contract and migration edits have one owner.

## Slice ledger

| ID | Depends on | Deliverable | Acceptance evidence |
|---|---|---|---|
| S0 | none | Pinned baseline, capability and workload inventory | Q01 |
| S1 | S0 | Correct scoped observations and explicit uncertainty | Q02-Q04 |
| S2 | S1 | Durable receipts/checkpoints/outbox and snapshot/tail | Q05-Q06, Q08 |
| S3 | S2 | Inspectable evidence, projection/replay substrate | Q09, Q14; initial Q16 |
| S4 | S3 | End-to-end normalized equal-budget target with explanations | Q10, Q14 |
| S5 | S3 | Typed plan/apply, jobs, budgets, receipts, handoffs | Q13, Q16, Q18 |
| S6 | S4, S5 | Versioned similarity/dedup experiment and accepted baseline/candidate | Q11, Q15 |
| S7 | S4, S5 | Account-aware advisory output and optional Telegram adapter | Q12, Q08 |
| S8 | S6, S7 | Integrated shadow, recovery, load and agent qualification | Q07, Q14, Q16-Q18 |
| S9 | S8 | Optional quality/regime research and controlled promotion | Q15 |

### Milestone: Agent-operable expert ensemble V1

**Issue 1 — S0: Establish a trustworthy baseline**

**Scope:** inspect the fork, upstream relationship, existing Rust/Python implementations, source contracts, tests, deployed processes, and available VPS resources without changing services. Verify actual GitHub Issues availability before choosing a tracker. Configure an `upstream` remote only within the implementation authorization; do not rebase the user's work.

**Deliverable:** evidence manifest identifying the baseline, command capabilities, known failures, shared IP/API consumers, available compute/disk, scoped initial wallet set, and explicit live-test limits. Check that the inherited Python tracker is not confused with the new engine.

**Verify:** run existing Rust checks (`make check-rs`) and the inherited Python checks when relevant (`make check`), after inspecting dependency/tool requirements. Record actual results and network/build effects. Perform only bounded public read probes; no Telegram broadcasts or order placement merely to qualify infrastructure. Convert G01-G11 into testable baseline findings; do not assume all prose claims are implemented.

**Exit:** baseline evidence exists and the reproducible limitations are classified. An existing failing test may be carried into S1 only as an explicit defect with its required repair; no green baseline claim.

**Issue 2 — S1: Deepen the existing observation module**

**Scope:** narrow Rust changes around `listener.rs`, `book.rs`, `enrich.rs`, `app.rs`, `resolve.rs`, and registry/config interfaces as warranted. Keep the existing pure reducer unless a failing behavior proves a change necessary.

**Deliverable:** composite economic identities, explicit persistent-ready revisions/generations, strict snapshots, applied/skipped/failed reconciliation outcomes, separate desired wallet universe, scoped equity/mark observations, and coverage states. Decouple production observation from chat recipients and move slow PnL/delivery work out of the ingestion loop. Introduce a shared weighted request scheduler with recovery allowance.

**Verify:** add the Q02-Q04 adversarial traces; use `cargo test --locked` and `make check-rs` against the actual head. Observe duplicate IDs across instruments, two watched counterparties, bad empty snapshots, concurrent reseeds, retired wallets, and partially known position histories. Compare public fields with provider evidence from S0.

**Exit:** the observation semantics are explicit and tested; unsupported scope and missing history remain visible. No durable-publication claim yet.

**Issue 3 — S2: Commit facts before publishing them**

**Scope:** Rust persistence and export only. Implement C2-C5 in [CONTRACTS](docs/CONTRACTS.md) over the existing SQLite infrastructure, with migrations and one logical writer. Do not add a broker or a competing position engine.

**Deliverable:** durable receipt ledger, economic dedup index, state/checkpoint/outbox transaction, versioned event envelope, consistent snapshot pages, bounded ordered tail, retention floors, and backpressure. A stream is optional notification of available durable data, never the sole recovery channel. Notification/PnL adapters consume committed events independently.

**Verify:** Q05/Q06/Q08 crash and recovery matrix through the external publication interface. Kill between acceptance, transaction commit, cache publication, consumer application, and acknowledgement. Check migrations/restore on copies of fixtures. Run the Rust checks plus the new contract tests; their executable names must be exposed in capability/help output, not hidden in a developer's notebook.

**Exit:** the consumer can stop, restart, resume, or rebootstrap truthfully; no accepted event is silently lost, and source gaps are not confused with local durability.

**Issue 4 — S3: Give the agent eyes before advanced intelligence**

**Scope:** create the separate Python application at an explicitly recorded repository/path; until then keep its implementation out of this fork. Build its own transactional projection/offset store, evidence manifests, and replay runner. Both processes retain independent database ownership.

**Deliverable:** actual `inspect capabilities`, `inspect system`, `inspect changes`, and evidence lookup; consistent worldviews; input-lineage IDs; isolated as-known versus restated replay; machine-readable validation/errors; basic job checkpoints. Implement these vertical capabilities rather than scaffolding every future module.

**Verify:** Q09/Q14 plus initial cold-start Q16 tasks. Same pinned accepted stream gives the same projected state after interruption. A future/unimplemented command is absent from available capabilities. Compact JSON retains fatal blockers and explicit truncation. Existing Rust checks and Python integration tests pass on the relevant repositories.

**Exit:** an operator can identify actual capabilities, health, coverage, drift, and evidence without reading internal tables or reconstructing logs.

**Issue 5 — S4: Deliver the first complete target explanation**

**Scope:** Python posture/ensemble and read-only explanation handlers.

**Deliverable:** declared eligibility, fixed-scale normalization, equal-budget baseline, unknown-weight bounds, target history, causal change categories, and contribution ledger. One expert can travel all the way from observed state to a correct explainable target before expanding the universe. Include market/equity changes and correction events, not only fill triggers.

**Verify:** Q10/Q14 with exact fixture arithmetic. Flat, abstention, unavailable, removed, and corrected state are distinct. The identical target is produced live and in as-known replay under the same timer policy. Expired equity cannot silently produce a fresh posture.

**Exit:** useful observation-to-consensus capability exists even without learned weights, clustering, or account execution.

**Issue 6 — S5: Make control safe and cheap**

**Scope:** operations facade and owner-specific command handlers, not a generic shell API.

**Deliverable:** typed proposals, immutable plans, grants, expected revisions, idempotency, cost budgets, receipts, bounded job start/resume/cancel, pause/recovery policy, and compact handoffs. Generate CLI/API schemas/help from one implementation. MCP is optional after the CLI contract is stable.

**Verify:** Q13/Q16/Q18. Reject stale/expired plans, authority escalation, cross-scope changes, prompt-injected labels, budget overruns, and replay effects. Resume a job instead of rerunning its full input. A repeated valid command key yields the original receipt.

**Exit:** routine maintenance/research can be delegated within explicit permissions without unrestricted production access. Real financial execution remains unavailable.

**Issue 7 — S6: Test independence rather than count wallets**

**Scope:** Python research and immutable similarity/cluster artifacts. This can proceed alongside S7 after S4/S5.

**Deliverable:** aligned causal posture features, support diagnostics, deterministic complete-link candidate, stable membership versions, clone-resistant hierarchical aggregation, and baseline-versus-candidate evaluation. Record thresholds/search space before forward evaluation. Manual groups/equal baseline remain usable when data cannot support inferred independence.

**Verify:** Q11/Q15. Test exact clones, independent same-direction experts, chain-link clustering traps, sparse overlaps, flat-heavy series, and missing data. Keep latency/cost assumptions and point-in-time universe fixed between candidates. Account for membership-change turnover.

**Exit:** the module and explanations work; economic evaluation either accepts the candidate or records rejection/inconclusive evidence and retains the baseline. Do not force profitable results or optimize the held-out period until the candidate wins.

**Issue 8 — S7: Produce bounded account advice, not assumed copying**

**Scope:** separate Python account adviser and output adapter; no real order-placement module.

**Deliverable:** explicit account rulebook, broker/instrument mappings, confirmed/manual account-state input, per-opportunity and portfolio risk ceilings, stop/stress assumptions, unsized fallback, rounding, expiry, and meaningful-change Telegram messages. Keep diagnostic trader alerts optional and separate. Unknown account position prevents incremental order advice.

**Verify:** Q12/Q08. Exercise daily resets, floating loss, existing correlated positions, fees/slippage, unavailable stop assumptions, unmapped instruments, stale manual entries, and notification retry. No recommendation claims safety from historical drawdown alone.

**Exit:** complete/fresh inputs yield explainable bounded advice; otherwise the system returns informational targets and named blockers. Sending any production notifications requires an approved recipient/configuration.

**Issue 9 — S8: Qualify the synthetic system**

**Scope:** integrated Rust/Python deployment in isolated shadow/advisory mode, evidence and task batteries, recovery and staged scale.

**Deliverable:** one evidence index showing engineering, operational ergonomics, resource use, and economic results separately. Demonstrate real bounded source comparison, event capture/replay, downstream outage, source gap, policy rollback, retention recovery, and an interrupted agent session.

**Verify:** Q07/Q14/Q16-Q18 and regression scenarios affected by the integrated head. Run declared staged load and bounded soak jobs, preserving checkpoints/artifacts. Evaluate 10/100/1,000-wallet capacity only with an explicit stream/market/activity/recovery workload. Account for other VPS workloads and provider limits. Independently inspect worker evidence rather than accepting a success narrative.

**Exit:** all mandatory correctness/authority gates pass, resource envelope is declared, agent task correctness is demonstrated, and remaining economic uncertainty is explicit. Deployment remains advisory, not automated execution.

**Issue 10 — S9: Optional adaptive quality and regime research**

Introduce lagged quality weighting or regime conditioning one family at a time, only after simpler baselines are useful. Require train/evaluation separation, uncertainty, ablations, negative results, and explicit promotion authority. This is not on V1's critical path.

# Plan history

## What changed from M0-M14

The previous milestones are superseded by the slice ledger above. M0-M3 become S0-S2; M4 becomes S3; M5-M6 become S4; M7-M9 become S6; M10 becomes optional S9; M11-M12 become S7; M13 becomes S8. M14 real execution remains out of scope. S5 adds the missing control/evidence loop early rather than bolting on agent access after the trading logic.

Mandatory PostgreSQL, a giant matrix recomputed on each fill, and one microservice per conceptual box are removed. Unqualified 'safe sizing', silent correction, missing-as-flat behavior, and broker-state inference from alerts are explicitly rejected. The design still requires KonScanner infrastructure, a durable handoff, a separate Python engine, and optional diagnostic Telegram.

## Documentation work delivered in this revision

Add a routed entrypoint, context map/glossary, inspected status/gaps, coherent system model, contracts, operations, research, qualification, and ADR. Mark inherited design/API assumptions as historical where superseded. Validate links, dependency structure, source references, and documentation diff hygiene. This revision does not itself run S0, install dependencies, change runtime code, modify services, publish issues, or grant new trading authority.
