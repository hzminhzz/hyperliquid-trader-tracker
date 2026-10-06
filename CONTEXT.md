# Domain glossary

## Observation and knowledge

**Wallet**: one public account address. Multiple wallets may share a controller or strategy; one wallet is not necessarily one independent expert.

**Instrument**: a venue- and market-specific contract, including network/environment and perp-dex namespace. Similar ticker text does not establish economic equivalence.

**Observation**: a source payload received by this system, together with its provenance. Observing a trade does not establish complete account history.

**Receipt time**: when a payload reaches the system. **Event time**: when the source says it occurred. **Knowledge time**: when accepted information becomes available to a decision. These times are distinct.

**Source identity**: the source-defined identity of an economic event. **Receipt identity**: the identity of a particular delivery, including duplicate deliveries.

**Reconstructed position**: the best supported current quantity and cost basis for a wallet/instrument. It is not an inferred trading intention.

**Reconciliation**: comparison with an external account snapshot and, when justified, correction of reconstructed state. A correction is not proof that the trader just made a new trade.

**Coverage**: the documented scope and completeness of observation across wallets, instruments, and time. **Gap**: an interval or scope whose completeness is unknown or known to be insufficient.

**Known flat**: reliable evidence of zero position in an eligible instrument. **Abstention**: an expert is outside the applicable universe. **Unavailable**: the expert's state is not reliable enough to interpret. Neither abstention nor unavailable means flat or short.

## Ensemble and advice

**Expert**: a user-selected source of trading information, associated with a wallet and a declared eligible universe and horizon.

**Posture**: an expert's bounded, signed, normalized current exposure. It is an exposure proxy, not a claim about psychological conviction or probability of profit.

**Quality weight**: a slow-moving assessment of an expert's demonstrated usefulness. **Reliability**: whether current evidence is admissible. **Similarity**: measured behavioral redundancy, not proof of shared identity.

**Cluster**: a versioned group of behaviorally redundant experts within an instrument/horizon scope. **Independent breadth**: the effective number of information sources after redundancy adjustment.

**Consensus target**: a bounded ensemble posture with contribution attribution and coverage. It is not a broker quantity, a calibrated confidence probability, or a trade instruction.

**Advisory target**: an account-specific proposed position, with risk assumptions and validity conditions. **Action delta**: the difference between that target and a fresh, confirmed account position, not the difference from the previous notification.

**Risk budget**: a configured amount of admissible loss exposure. Remaining drawdown allowance is a constraint, not an instruction to risk it all.

## Control and evidence

**Worldview**: a coherent, explicitly scoped view of accepted state at named input cuts and policy revisions. It may truthfully report incomplete knowledge.

**Policy**: a versioned set of deterministic interpretation and control rules. **Proposal**: an unapplied change to policy or operation. **Receipt**: durable evidence of whether an authorized command was applied and what changed.

**Evidence capsule**: a bounded, reproducible collection of inputs, versions, outputs, checks, and limitations supporting one conclusion.

**As-known replay**: reconstruction using only information available at each historical decision. **Restated replay**: reconstruction using later corrections or backfills. These answer different questions and must remain distinguishable.

**Qualification**: demonstrated satisfaction of a named acceptance predicate at specified revisions. Software correctness and economic usefulness are separate qualifications.

**Lesson**: an evidence-backed finding retained as a regression, operational rule, or decision record. A session narrative or untested explanation is not a validated lesson.
