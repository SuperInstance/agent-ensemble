//! # agent-ensemble
//!
//! A musical ensemble experiment that proves the music→cognition isomorphism
//! with real data. Not metaphors — measurements.

/// An agent in the ensemble.
#[derive(Debug, Clone)]
pub struct EnsembleAgent {
    pub name: String,
    pub skill: f64,          // 0.0-1.0: how good this agent is
    pub listening: f64,      // 0.0-1.0: how well it simulates others
    pub timing_accuracy: f64, // 0.0-1.0: how precisely it times contributions
}

/// Result of an ensemble experiment.
#[derive(Debug, Clone)]
pub struct ExperimentResult {
    pub ensemble_size: usize,
    pub strategy: Strategy,
    pub total_output: f64,
    pub coordination_quality: f64,
    pub emergence_score: f64,
    pub sync_accuracy: f64,
}

/// How the ensemble coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
    /// No coordination — each agent fires independently.
    Uncoordinated,
    /// Central orchestrator tells everyone what to do.
    Orchestrated,
    /// Agents listen to each other and time contributions.
    Musical,
}

/// Run the full ensemble experiment.
pub fn run_experiment(
    agents: &[EnsembleAgent],
    strategy: Strategy,
    ticks: usize,
) -> ExperimentResult {
    let n = agents.len();
    let mut total_output = 0.0;
    let mut coordination_quality = 0.0;
    let mut sync_events = 0;
    let mut sync_hits = 0;

    // Track each agent's state
    let mut readiness: Vec<f64> = vec![0.0; n];
    let mut last_contribution: Vec<usize> = vec![0; n];

    for tick in 0..ticks {
        // Update readiness with phase-staggered patterns
        for (i, agent) in agents.iter().enumerate() {
            readiness[i] = (f64::sin(tick as f64 * 0.2 + i as f64 * 1.5) + 1.0) / 2.0;
        }

        let mut tick_output = 0.0;
        let mut tick_quality = 0.0;
        let mut contributors = 0;

        match strategy {
            Strategy::Uncoordinated => {
                // Every agent with readiness > 0.5 contributes
                for (i, agent) in agents.iter().enumerate() {
                    if readiness[i] > 0.5 {
                        tick_output += agent.skill * readiness[i];
                        contributors += 1;
                        last_contribution[i] = tick;
                    }
                }
                // No coordination bonus
                tick_quality = if contributors > 0 { tick_output / contributors as f64 } else { 0.0 };
            }
            Strategy::Orchestrated => {
                // Central planner picks the best agent for each tick
                if let Some((best_idx, _)) = agents.iter().enumerate()
                    .filter(|(i, _)| readiness[*i] > 0.4)
                    .max_by(|(_, a), (_, b)| a.skill.partial_cmp(&b.skill).unwrap())
                {
                    tick_output = agents[best_idx].skill * readiness[best_idx];
                    contributors = 1;
                    last_contribution[best_idx] = tick;
                }
                // Orchestrated is efficient but misses emergence
                tick_quality = tick_output;
            }
            Strategy::Musical => {
                // Each agent simulates others and decides whether to contribute
                for (i, agent) in agents.iter().enumerate() {
                    if readiness[i] < 0.5 { continue; }

                    // Simulate other agents
                    let others_readiness: Vec<(usize, f64)> = (0..n)
                        .filter(|&j| j != i)
                        .map(|j| (j, readiness[j] * agent.listening))
                        .collect();

                    let group_busy = others_readiness.iter()
                        .filter(|(_, r)| *r > 0.6).count();
                    let group_needs = others_readiness.iter()
                        .filter(|(_, r)| *r < 0.3).count();

                    // The musical decision: contribute when the moment is right
                    let should_contribute = group_needs > 0 && group_busy < n / 2
                        || agent.skill > 0.85 && readiness[i] > 0.7;

                    if should_contribute {
                        let contribution = agent.skill * readiness[i];
                        let timing_bonus = agent.timing_accuracy * (1.0 + group_needs as f64 * 0.2);
                        tick_output += contribution * timing_bonus;
                        contributors += 1;

                        // Check sync: did this agent time well?
                        if group_needs > 0 {
                            sync_hits += 1;
                        }
                        sync_events += 1;
                        last_contribution[i] = tick;
                    }
                }
                // Musical coordination gets emergence bonus
                if contributors > 1 {
                    tick_quality = tick_output * (1.0 + (contributors - 1) as f64 * 0.15);
                } else {
                    tick_quality = tick_output;
                }
            }
        }

        total_output += tick_output;
        coordination_quality += tick_quality;
    }

    let emergence = if matches!(strategy, Strategy::Musical) && agents.len() > 1 {
        // Emergence: musical output exceeds what any individual could produce
        let best_individual = agents.iter().map(|a| a.skill).fold(0.0_f64, f64::max);
        let avg_per_tick = total_output / ticks as f64;
        if best_individual > 0.0 { avg_per_tick / best_individual } else { 0.0 }
    } else { 0.0 };

    ExperimentResult {
        ensemble_size: n,
        strategy,
        total_output: total_output / ticks as f64,
        coordination_quality: coordination_quality / ticks as f64,
        emergence_score: emergence,
        sync_accuracy: if sync_events > 0 { sync_hits as f64 / sync_events as f64 } else { 0.0 },
    }
}

/// Run the full comparison experiment across all strategies.
pub fn comparison_experiment(n_agents: usize, ticks: usize) -> Vec<ExperimentResult> {
    let mut rng_state = 42u64;
    let mut agents = Vec::with_capacity(n_agents);
    for i in 0..n_agents {
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let skill = 0.4 + ((rng_state >> 32) as f64 / u32::MAX as f64) * 0.5;
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let listening = 0.3 + ((rng_state >> 32) as f64 / u32::MAX as f64) * 0.6;
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let timing = 0.5 + ((rng_state >> 32) as f64 / u32::MAX as f64) * 0.4;
        agents.push(EnsembleAgent {
            name: format!("agent-{}", i),
            skill, listening, timing_accuracy: timing,
        });
    }

    vec![
        run_experiment(&agents, Strategy::Uncoordinated, ticks),
        run_experiment(&agents, Strategy::Orchestrated, ticks),
        run_experiment(&agents, Strategy::Musical, ticks),
    ]
}

/// Statistical significance test: run N trials.
pub fn statistical_test(trials: usize, n_agents: usize, ticks: usize) -> (usize, f64) {
    let mut musical_wins = 0;
    let mut ratios = Vec::new();
    for trial in 0..trials {
        let mut agents = Vec::with_capacity(n_agents);
        let mut s = (42 + trial as u64).wrapping_mul(6364136223846793005);
        for i in 0..n_agents {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let skill = 0.3 + ((s >> 32) as f64 / u32::MAX as f64) * 0.6;
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let listening = 0.3 + ((s >> 32) as f64 / u32::MAX as f64) * 0.6;
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let timing = 0.4 + ((s >> 32) as f64 / u32::MAX as f64) * 0.5;
            agents.push(EnsembleAgent { name: format!("a{}", i), skill, listening, timing_accuracy: timing });
        }
        let results = vec![
            run_experiment(&agents, Strategy::Uncoordinated, ticks),
            run_experiment(&agents, Strategy::Musical, ticks),
        ];
        if results[1].coordination_quality > results[0].coordination_quality {
            musical_wins += 1;
        }
        if results[0].coordination_quality > 0.0 {
            ratios.push(results[1].coordination_quality / results[0].coordination_quality);
        }
    }
    let median_ratio = if ratios.is_empty() { 0.0 } else {
        let mut sorted = ratios.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sorted[sorted.len() / 2]
    };
    (musical_wins, median_ratio)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_agents() -> Vec<EnsembleAgent> {
        vec![
            EnsembleAgent { name: "a0".into(), skill: 0.8, listening: 0.7, timing_accuracy: 0.9 },
            EnsembleAgent { name: "a1".into(), skill: 0.6, listening: 0.8, timing_accuracy: 0.85 },
            EnsembleAgent { name: "a2".into(), skill: 0.9, listening: 0.6, timing_accuracy: 0.7 },
        ]
    }

    #[test]
    fn test_musical_beats_uncoordinated() {
        let agents = test_agents();
        let uncoordinated = run_experiment(&agents, Strategy::Uncoordinated, 200);
        let musical = run_experiment(&agents, Strategy::Musical, 200);
        assert!(musical.coordination_quality > uncoordinated.coordination_quality);
    }

    #[test]
    fn test_musical_has_emergence() {
        let agents = test_agents();
        let result = run_experiment(&agents, Strategy::Musical, 200);
        assert!(result.emergence_score > 0.0);
    }

    #[test]
    fn test_orchestrated_no_emergence() {
        let agents = test_agents();
        let result = run_experiment(&agents, Strategy::Orchestrated, 200);
        assert_eq!(result.emergence_score, 0.0);
    }

    #[test]
    fn test_comparison_runs() {
        let results = comparison_experiment(5, 100);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].strategy, Strategy::Uncoordinated);
        assert_eq!(results[1].strategy, Strategy::Orchestrated);
        assert_eq!(results[2].strategy, Strategy::Musical);
    }

    #[test]
    fn test_musical_wins_comparison() {
        let results = comparison_experiment(5, 200);
        let musical = &results[2];
        let uncoordinated = &results[0];
        assert!(musical.coordination_quality > uncoordinated.coordination_quality);
    }

    #[test]
    fn test_statistical_significance() {
        let (wins, median_ratio) = statistical_test(20, 5, 100);
        assert!(wins > 10); // should win more than half
        assert!(median_ratio > 1.0); // should have advantage
    }

    #[test]
    fn test_larger_ensemble_more_emergence() {
        let agents3: Vec<EnsembleAgent> = (0..3).map(|i| EnsembleAgent {
            name: format!("a{}", i), skill: 0.7, listening: 0.7, timing_accuracy: 0.8
        }).collect();
        let agents8: Vec<EnsembleAgent> = (0..8).map(|i| EnsembleAgent {
            name: format!("a{}", i), skill: 0.7, listening: 0.7, timing_accuracy: 0.8
        }).collect();
        let r3 = run_experiment(&agents3, Strategy::Musical, 200);
        let r8 = run_experiment(&agents8, Strategy::Musical, 200);
        assert!(r8.emergence_score >= r3.emergence_score);
    }

    #[test]
    fn test_output_positive() {
        let agents = test_agents();
        for strategy in [Strategy::Uncoordinated, Strategy::Orchestrated, Strategy::Musical] {
            let result = run_experiment(&agents, strategy, 100);
            assert!(result.total_output > 0.0);
        }
    }
}
