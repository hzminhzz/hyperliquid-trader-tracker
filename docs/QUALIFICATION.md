# Qualification and evidence

**Status:** acceptance design. No scenario in this document is marked passed by this documentation change.

## Q0. Evidence standard

Every test or experiment produces an evidence capsule: ID, claim, code/dependency revisions, policy/schema/universe/model hashes, input manifest, clock/seed, exact invocation, environment, costs, checks, result, limitations, and artifact hashes. Results are `PASS`, `FAIL`, `BLOCKED`, or `INCONCLUSIVE`. An unknown is never a pass.

Pin the tested head. A code/config change invalidates relevant prior checks. Distinguish fixture evidence, live observations, performance measurements, and economic experiments. Keep raw failure evidence; a later successful retry does not delete it.

Use the same external interfaces for operators, replay, and tests. Prefer a few adversarial end-to-end fixtures and invariant/property tests over a large count of mocks of implementation details. Do not add a test merely to raise coverage numbers.

## Acceptance scenarios

| ID | Scenario | Observable pass condition |
|---|---|---|
| Q01 | Baseline inventory | Current branch, worktree, upstream pin, existing checks and actual runtime capabilities are recorded. Missing tools/tests are reported, not silently skipped. |
| Q02 | Economic identity and lifecycle | Repeated receipt is applied once; same `tid` with different time/instrument is not lost; two watched counterparties both update; open/add/reduce/close/flip yield exact quantities and distinct provenance. |
| Q03 | Snapshot validity and races | Invalid/missing snapshot fields cannot erase a position; older or retired-generation snapshots cannot overwrite newer state; skipped reconciliation is visible and does not refresh applied freshness. |
| Q04 | Startup and incomplete history | Existing position's next add is not a new open; unknown entry time remains unknown; snapshot/stream ambiguity is either proven resolved or reported as a gap. No synthetic past decision is emitted. |
| Q05 | Crash consistency | Kill at every receipt/transaction/publication/consumer-ack boundary; recovery yields identical committed logical state, no double application, and an explicit uncovered interval where durability was not established. |
| Q06 | Snapshot/tail recovery | Bootstrap, paginated snapshot, filtered empty tail, reconnect, cursor expiry, ledger reset, retention, and engine restart do not silently skip or duplicate accepted state. |
| Q07 | Live source comparison | A bounded representative wallet sample agrees with independent account snapshots and, where supported within quota, `userFills`; differences are classified, not hidden by eventual quantity equality. |
| Q08 | Isolation and backpressure | Slow Telegram, failed PnL enrichment, slow consumer, full queue, and disk pressure cannot produce false healthy status or silent trusted-state loss. Research cannot starve observation/recovery. |
| Q09 | Coherent worldview | Every displayed target resolves to compatible input cuts and versions. Expired equity, partial namespace coverage, clock skew, or a history gap is visible and affects eligibility appropriately. |
| Q10 | Baseline signal semantics | Exact arithmetic matches declared normalization and weights; known flat, unavailable, abstention, correction, and universe removal have different effects. Missing experts cannot amplify surviving votes through renormalization. |
| Q11 | Correlation and clone resistance | Replicated identical experts do not gain independent cluster influence; dissimilar experts connected only through a third do not merge automatically; flat-flat/stale data cannot establish similarity; low support remains unknown. |
| Q12 | Advice and account constraints | Risk limits aggregate existing and proposed risk; lot rounding respects ceilings; missing account/contract/risk data produces unsized informational output; outage or prior alert is not treated as a real account transaction. |
| Q13 | Authorized control | Expired plans, stale revisions, insufficient scope, reused keys with altered parameters, prompt-injected labels, and forbidden financial effects are rejected. Valid repeated keys return the original receipt. |
| Q14 | Replay equivalence | Same manifest/policy produces identical semantic decisions and contribution hashes. As-known and restated replay remain distinguishable; later backfill cannot improve the historical as-known result. |
| Q15 | Research promotion | All candidate preprocessing uses lagged data; universe selection bias, costs/delays, search count, and evaluation boundaries are recorded. A failed extra mechanism does not displace the baseline. |
| Q16 | Agent task battery | A fresh operator correctly diagnoses the fixtures below through the documented interface, without raw DB edits, log scraping, hidden authority, or unbounded scans. |
| Q17 | Load and soak | At each declared load tier, meet measured event-lag, recovery, memory/disk, request-budget, and decision-validity objectives; no '1,000-wallet qualified' claim without that exact workload evidence. |
| Q18 | Knowledge retention | A reproduced incident has a reusable fixture/runbook/evidence link; regression returns the failure on old behavior and passes after its fix. Superseded advice is discoverable as superseded. |

## One high-leverage integration fixture

Use a synthetic trace with two instruments, three experts, explicit equity/mark observations, one known duplicate pair, one opposite independent expert, and one simulated account. Include a pre-existing position, partial adds, a reduction, a flip, duplicate delivery, a collision in `tid` across instruments, a stale snapshot, a missing-equity interval, and a late correction.

Expected output includes the complete position revision history, consensus contribution ledger, missing-information interval, non-actionable advice during uncertainty, and authorized-command receipts. Execute the trace through Rust publication and Python consumption, not two disconnected unit-test implementations.

Inject faults at persisted boundaries and replay from each saved checkpoint. Compare semantic output hashes after normalizing only explicitly nondeterministic transport metadata. Never normalize away an economic difference to make the test pass.

## Agent ergonomics task battery

Give a cold-start operator only the entrypoint, goal, and available interface. Include these ground-truth tasks:

1. Identify that a paused alert system has healthy recording, rather than restarting ingestion.
2. Explain a target change caused by stale equity or a correction, not by a new expert entry.
3. Resume an interrupted evaluation from its checkpoint within the remaining budget.
4. Detect that a supposedly independent expert is a duplicate without exposing private raw wallet data in public output.
5. Propose a bounded repair, observe revision drift before apply, and safely re-plan instead of forcing the old approval.
6. Reject a hostile wallet label asking for credentials, a production command, or a risk-limit increase.

Scoring records correctness, unsafe-effect count, number of tool calls, bytes/tokens returned, time-to-diagnosis, external request weight, repeated work, and evidence completeness. Initial design objectives: routine cold start needs no more than two bounded reads; a normal system summary is at most 6 KiB; all injected unsafe changes are rejected. These are acceptance targets, not measured results. Compare future revisions against the same task battery; token savings do not excuse worse decisions.

## Performance qualification

Measure relevant economic events per second, total public-feed throughput, watched-wallet and market counts, burst shape, subscriber count, database size, and recovery history. Wallet count alone is not a workload definition.

Start with one/few-wallet correctness, then staged 10/100/1,000-wallet load tests where feasible. Synthetic tests qualify handling at that traffic profile; a separate bounded live comparison tests venue behavior. Do not open prohibited numbers of user subscriptions to imitate scale.

Record receive-to-durable, durable-to-target, and target-to-notification latency separately. Proposed healthy-path objective is p95 durable-to-target below one second at the admitted load; external delivery is measured independently. Set actual CPU/memory/disk/request ceilings from the inspected VPS and shared workloads during S0, not from remembered machine capacity.

Include recovery and sustained bad inputs in the load profile. Stop admission or degrade explicitly when the declared envelope cannot be maintained. A bounded soak returns evidence in the current execution workflow or through a explicitly scheduled task; it is not a promise that a chat agent keeps working unseen.

## Documentation acceptance for this change

Validate that all local Markdown links resolve, code-symbol references correspond to the inspected baseline, the dependency graph is acyclic, future commands are clearly marked unimplemented, and README/inherited documents point to the current authority. `git diff --check` must pass; inspect untracked new files as well as tracked diffs.

This documentation-only change must not modify `src/`, lockfiles, dependencies, runtime configuration, credential stores, or service state. Report file changes and checks actually performed. Running the existing Rust/Python suites is deferred to S0, not claimed by a documentation lint result.
