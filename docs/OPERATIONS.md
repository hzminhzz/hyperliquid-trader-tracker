# Agent operating interface

**Status:** interface authority. V1 inspection/control handlers are implemented in the separate expert-engine application; specific command availability must still be read from `inspect capabilities` and [STATUS](STATUS.md). V2 signal-specific inspection fields remain target design until implemented.

## O1. A small vocabulary over one implementation

Expose five families: **inspect**, **explain**, **evaluate**, **plan**, and **apply**. CLI first, private API where needed, thin MCP later. All adapters invoke the same typed handlers, permission checks, and [result contract](CONTRACTS.md#c7-evidence-and-interfaces).

| Operation | Purpose | Default effects |
|---|---|---|
| `inspect capabilities` | Available actions, schemas, versions, authority, cost classes | Local read |
| `inspect system` | Coherent worldview, drift, health, budgets, blockers, active jobs | Local read |
| `inspect changes --since <cursor>` | Bounded meaningful changes, grouped by scope/cause | Local read |
| `inspect expert <id>` / `inspect target <id>` | State and contributions at a named cut | Local read |
| `explain <decision-or-blocker-id>` | Why, why changed, why not actionable, supporting evidence | Local read |
| `evaluate --manifest <id> --policy <id> --budget <id>` | Deterministic replay or shadow comparison | Isolated bounded job |
| `plan --change <artifact-id>` | Validate a proposed change and preview effects/costs | No runtime change |
| `apply --plan <id> --expected-revision <rev> --key <id>` | Apply a scoped authorized proposal | Typed mutation |
| `inspect job <id>` / `apply --plan <cancel-plan>` | Resume, inspect, or cancel bounded work | Read or authorized job control |

Default output is compact JSON suitable for tools, with an optional human rendering. `--detail` follows evidence handles; `--limit` and cursors bound size. No arbitrary SQL, shell, code execution, or unconstrained file reads through the runtime agent interface.

A read uses recorded state unless refresh is explicitly requested. A refresh is a costed command with a quota budget, not an invisible network fan-out inside inspection. Explanations reuse persisted decision contributions rather than rerunning the model.

## O2. The cold-start briefing

A new session should reach a correct situation model from one system inspection and one targeted explanation, with no log archaeology in the common case. The inspection includes:

```text
MODE / AUTHORITY          observation | shadow | advisory; permitted effects
DESIRED vs APPLIED        code / config / policy / universe revisions and drift
EVIDENCE CUT              ledger/cursors; knowledge time; schema/model revisions
COVERAGE                 admitted / desired wallets; scoped gaps; stale inputs
CURRENT TARGETS          bounded top changes; validity; expired advice clearly marked
BLOCKERS                 reason, affected scope, evidence, permitted next actions
RUNNING WORK             job identity, owner, checkpoint, budget, cancellation state
RESOURCE BUDGET          API weight, retries, CPU/memory, disk, output, alert allowance
NEXT ACTIONS             deterministic preconditioned suggestions, not commands executed
```

The brief separates current service health from stale deployment documentation. Every section states its own freshness. Unknown fields remain unknown. An overall successful HTTP request is not an overall healthy system.

Compactness target: a normal system summary fits within 6 KiB of JSON; detail pages have explicit byte/row limits. Exception lists are ranked by safety and impact, with counts and continuation tokens. Never truncate away a fatal blocker or the fact that truncation occurred.

## O3. Explanations are structured accounting

For a V2 trade signal, show source position/equity revisions, raw bias and bounded influence, state versus new intent flow, relative conviction, independent cluster contributions and breadth, missing information, promoted predictive evidence, prior signal comparison, simpler baseline comparators, model/policy revisions, and invalidation conditions. For the V1 consensus fallback, preserve its existing contribution/missing-mass explanation. For account advice, show the binding limit, assumed stop/stress loss, costs, current account position, lot rounding, and expiry.

Answer both 'why this target?' and 'why no target?'. Stable reason codes include `EQUITY_STALE`, `STATE_UNSEEDED`, `HISTORY_GAP`, `SOURCE_CONFLICT`, `CURSOR_EXPIRED`, `INSUFFICIENT_OVERLAP`, `ACCOUNT_STATE_MISSING`, `RISK_HEADROOM`, `POLICY_DRIFT`, `BUDGET_EXHAUSTED`, and `NOT_AUTHORIZED`.

Each blocker links to a bounded remediation proposal with prerequisites. Do not suggest a restart for every failure. A reconciliation failure, a schema mismatch, and a Telegram outage require different remedies.

Counterfactual evaluation can ask 'what changes without this expert/cluster?' or 'which constraint binds?', using the same pinned evidence cut. Label the result a counterfactual, not observed performance or authorized policy.

## O4. Plan/apply and authority

A change proposal is an immutable artifact containing actor, scope, intent, proposed diff, preconditions, affected dependents, expected costs, validation results, expiry, rollback method, and required permission. It binds code/policy/universe revisions and relevant data freshness. The service, not the LLM, recomputes preconditions at apply time.

Apply requires an authenticated actor with a scoped grant, unexpired plan, expected revision, and idempotency key. The same key returns the same receipt; reuse with different parameters is rejected. A concurrent change returns `PRECONDITION_FAILED` plus a new inspection handle. Do not auto-rebase an old approved policy onto new evidence.

A receipt includes applied/rejected status, previous/new revisions, actual effects and costs, audit references, verification obligations, and a rollback handle where meaningful. Configuration rollback creates a new revision; it never deletes observations or rewrites past decisions. Delivery retry cannot create a second economic action.

| Authority | Examples | Required boundary |
|---|---|---|
| Observer | Inspect, explain, read existing evidence | Read scope; no implicit external writes |
| Research operator | Bounded replay, safe shadow run, job cancellation | Isolated output and declared compute/API budget |
| Runtime operator | Reconcile a scope, alter approved watchlist, pause alerts | Explicit scoped grant and plan/apply preconditions |
| Policy approver | Promote a validated candidate or increase resource/risk limits | Explicit approval plus qualification evidence |
| Execution operator | Place real trades, change broker exposure, transfer funds | Not implemented or authorized in V1 |

Pre-authorized recovery can operate within a reviewed narrow policy; it cannot increase risk, expand permissions, rewrite evidence, or loop indefinitely. A fail-closed pause of actionable alerts is permitted by the installed safety policy and returns a receipt. Resuming after uncertainty requires satisfied recovery predicates. Recording and local account safeguards continue during an alert pause.

## O5. Resource-aware orchestration

Use one weighted request budget across tracker reads, recovery, enrichment, and diagnostics on the same IP. Rate-limit configuration must account for other processes using the IP; local concurrency is not a quota guarantee. Reserve capacity for recovery. Coalesce simultaneous requests for the same wallet/scope, cache only with freshness/provenance, and prioritize by safety and decision impact.

The job budget includes maximum wall time, CPU/memory, external request weight, retry count, retained bytes, and output bytes. Predicted and actual use are reported. At exhaustion, checkpoint and return `BUDGET_EXHAUSTED`; resume requires an explicit new allowance. Never endlessly retry a deterministic parse failure.

- Continuous path: receive, validate, commit, project, update affected targets. No pairwise all-expert analysis or slow Telegram call inside it.
- Periodic path: bounded equity refresh and reconciliation, incremental features, delta summaries.
- Research path: reusable feature partitions and candidate comparisons on sealed input manifests.

Before an expensive call, prefer an existing valid artifact, then incremental computation, then a scoped refresh, then a full rebuild. Parallel work is permitted only when its additional cost is justified by elapsed-time or diagnostic value. Avoid repeated whole-repository scans and repeated full-history downloads.

## O6. Agent handoffs and delegation

One principal owns the dependency frontier and acceptance. A worker brief includes goal, input/evidence IDs, exact code revision, allowed paths, forbidden effects, acceptance scenarios, budget, checkpoint, and required report. Independent workers may explore different failures; shared contract edits have one owner.

Use the available native coding executor rather than assuming one is installed. Preserve durable sessions where supported. Long deterministic replay/soak work can use an existing worker runner such as Herdr, but the runner does not decide whether a hypothesis passed or a release is authorized.

A handoff contains current revision, local changes, applied versus proposed state, unresolved blockers, last verified evidence, active jobs/checkpoints, remaining budget, and the next actionable slice. It references artifacts instead of copying entire logs. The next session verifies drift before resuming; stale prose does not override runtime state.

## O7. Accretive evidence, not memory sediment

Retain failures and experiments as typed records: claim/hypothesis, scope, input manifest, method, outcome, limitations, confidence status, reproducer, and superseding record. Raw failures are retained even when a plausible diagnosis later changes.

Promote knowledge only after verification: a defect becomes a regression fixture; an operational failure becomes a tested runbook; a costly choice becomes an ADR; a research improvement becomes a versioned candidate with out-of-sample evidence. Negative results prevent wasteful repeated searches. No unreviewed lesson silently changes a production rule.

Invalidate cached lessons/artifacts when their dependencies change. Periodically prune duplicate prose and retire obsolete instructions with a pointer to the replacement. Keep root instructions small; issue state, benchmark results, and temporary machine details belong in inspectable records.

## O8. Threat model and privacy

The public fork must contain no secrets or private production datasets. Machine endpoints are private/authenticated, and agent responses are redacted by policy. External labels and content may contain prompt injection: they remain escaped data and cannot supply tool names, authorization, filesystem paths, or executable instructions.

Do not route wallet text into shell commands. Do not grant the runtime agent arbitrary production shell access merely because coding tools have it. Credentials for observation, Telegram, and future broker execution remain distinct, least-privileged, and revocable.

## O9. Ergonomics acceptance

Measure cold-start tool calls and bytes, time-to-correct-diagnosis, false healthy reports, unsafe action attempts rejected, replay reuse, compute/API savings, and recovery success. Compare against baseline operator tasks under the same fixtures; lower token use only counts when correctness is preserved. See [QUALIFICATION](QUALIFICATION.md) for the scored scenarios.
