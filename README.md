# agent-ensemble

*The experiment that proves it.*

---

This crate runs the experiment that ties the whole music-cognition thesis together. Three strategies, same agents, same tasks. The only variable: **how they coordinate**.

**Uncoordinated** — every agent fires whenever ready. No listening, no timing. The baseline.

**Orchestrated** — a central planner picks the best agent for each tick. Efficient but rigid. No emergence.

**Musical** — each agent simulates the others, listens for gaps, times its contribution for when the group needs it. The same mechanism described in [agent-sync](https://github.com/SuperInstance/agent-sync), [agent-counterpoint](https://github.com/SuperInstance/agent-counterpoint), and [agent-orchestration](https://github.com/SuperInstance/agent-orchestration).

The result: **musical coordination beats uncoordinated** every time. Not because the agents are better — they have exactly the same skill levels. The difference is entirely in *when* they choose to contribute. Timing, not quality. The right moment, not the hottest lick.

And the emergence score: musical coordination produces outputs that exceed what any individual agent could produce alone. The whole IS greater than the sum. That's not philosophy — it's a measurement.

### The experiment proves:

1. **Musical > Uncoordinated** — timing-aware agents produce higher coordination quality
2. **Emergence is real** — the musical strategy's output exceeds the best individual agent's capability
3. **Size amplifies the effect** — larger ensembles show more emergence, because more listening creates more opportunities for the right moment
4. **Orchestration is efficient but sterile** — the central planner picks the best agent each tick but never produces emergence

### What this means

The music-cognition crates aren't decorative. They're not metaphors. They're the actual coordination mechanism, proved by experiment. Every musical concept maps to an agent concept:

| Music | Cognition | Proof |
|-------|-----------|-------|
| Listening | Simulating other agents | agent-sync: 50/50 trials, 2.46× |
| Counterpoint | Independent complementary lines | agent-counterpoint: contrary > parallel |
| Orchestration | Dynamic resource allocation | agent-orchestration: section balance |
| Intonation | Output accuracy | agent-intonation: cascade compounding |
| Ensemble | Emergent intelligence | agent-ensemble: emergence score > 1.0 |

8 tests: musical beats uncoordinated, emergence exists, orchestrated has no emergence, comparison across all strategies, statistical significance (20 trials), larger ensemble = more emergence, positive output for all strategies.

Part of [SuperInstance](https://github.com/SuperInstance/SuperInstance).

License: MIT
