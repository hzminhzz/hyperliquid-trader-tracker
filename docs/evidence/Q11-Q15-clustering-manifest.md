# Q11-Q15: Similarity, Complete-Link Clustering, and Promotion Evidence Manifest

**Capsule ID:** Q11-Q15  
**Milestone:** Agent-operable expert ensemble V1  
**Issue:** #7 — S6: Test independence rather than count wallets  
**Status:** PASS  
**Inspection Date:** 2026-10-06  
**Application Repository:** `/home/quant/dev/hyperliquid-expert-ensemble`  
**Application Commit:** `2533b01`  
**Tracker Commit:** HEAD of `feat/agent-operable-expert-ensemble-v1`  

---

## 1. Summary of Deliverables Qualified

This manifest certifies completion of slice **S6**, establishing aligned posture similarity features, deterministic complete-link clustering, clone-resistant hierarchical aggregation (B2), and research promotion evaluation per [RESEARCH](../RESEARCH.md) R3–R7 and [QUALIFICATION](../QUALIFICATION.md) Q11, Q15:

### Qualified Acceptance Scenarios

| Scenario ID | Name | Pass Condition | Verification Test | Status |
|---|---|---|---|---|
| **Q11** | Correlation and clone resistance | Replicated identical experts do not gain independent cluster influence (10 clones receive aggregate cluster budget identical to 1 expert); complete-link clustering avoids chain-link traps; flat-flat series and sparse overlap (<5 steps) return `INSUFFICIENT_OVERLAP`. | `tests/test_s6_qualification.py::test_q11_clone_resistance_neutralizes_replicated_experts`, `test_q11_avoid_chain_link_clustering_traps`, `test_q11_flat_flat_and_low_support_excluded` | PASS |
| **Q15** | Research promotion gates | Candidate evaluation compares baseline B1 against B2 candidate under strict evidence gates; promotes B2 when clone inflation is neutralized with low turnover (<0.05); strictly retains baseline B1 when no advantage is demonstrated or turnover penalty binds. | `tests/test_s6_qualification.py::test_q15_research_promotion_gates_and_baseline_retention` | PASS |

---

## 2. Invariants & Mathematical Rules Qualified (R3, R4, R7)

1. **Flat-Flat Exclusion & Overlap Support (R4):**
   - Flat intervals ($s_A = 0$ and $s_B = 0$) are strictly excluded from similarity support to prevent shared inactivity from counterfeiting agreement.
   - Minimum joint support floor: if active steps $< 5$, returns distance $1.0$ with diagnostic status `INSUFFICIENT_OVERLAP`.
2. **Complete-Link Agglomerative Clustering (R4):**
   - Cluster distance definition: $D(C_1, C_2) = \max_{u \in C_1, v \in C_2} D(u, v)$.
   - Immunity to chain-linking: if $D(A,B) \le \tau$ and $D(B,C) \le \tau$ but $D(A,C) > \tau$, complete-link strictly keeps $A$ and $C$ in separate clusters.
   - Deterministic tie-breaking on member IDs ensures reproducible cluster artifacts across replays.
3. **Clone-Resistant Hierarchical Aggregation (R3):**
   - Formula: $C(a) = \sum_g b(g) \sum_{i \in g} a(i|g) s(i,a)$ where $b(g) = 1/|G|$ and $a(i|g) = 1/|g|$.
   - Invariant: Clones sharing cluster $g$ divide the single cluster budget $b(g)$. Adding $K$ identical copies scales individual weights to $1/(|G| \cdot K)$, leaving aggregate influence completely unchanged.
4. **Research Promotion Principle (R7):**
   - Baseline B1 (equal-budget) remains authoritative unless candidate B2 demonstrates verified improvement.
   - Inconclusive or high-turnover experiments explicitly retain baseline B1.

---

## 3. Test Evidence Across Repositories

- **Observation Tracker (`hyperliquid-trader-tracker`):**
  - `make check-rs`: 115 tests passed, 0 clippy warnings, clean formatting.
  - `make test`: 79 pytest tests passed.
  - Total: 194 tests passing.
- **Expert Ensemble (`hyperliquid-expert-ensemble`):**
  - `uv run ruff check .`: clean.
  - `uv run ty check`: clean.
  - `uv run pytest -q`: 21 passed in 0.20s.
- **Combined Test Count:** 215 passed across both repositories.
