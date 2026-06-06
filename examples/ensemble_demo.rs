//! 4 agents playing together — the Musical strategy produces emergence
//! that no individual agent could achieve alone.

use agent_ensemble::{run_experiment, EnsembleAgent, Strategy};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          ENSEMBLE DEMO — 4 Agents, 3 Strategies            ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let agents = vec![
        EnsembleAgent { name: "synth".into(),   skill: 0.85, listening: 0.9,  timing_accuracy: 0.88 },
        EnsembleAgent { name: "bass".into(),    skill: 0.75, listening: 0.95, timing_accuracy: 0.92 },
        EnsembleAgent { name: "drums".into(),   skill: 0.90, listening: 0.7,  timing_accuracy: 0.95 },
        EnsembleAgent { name: "melody".into(),  skill: 0.80, listening: 0.85, timing_accuracy: 0.80 },
    ];

    println!("Ensemble members:");
    for a in &agents {
        println!("  🎵 {} — skill: {:.2}, listening: {:.2}, timing: {:.2}",
            a.name, a.skill, a.listening, a.timing_accuracy);
    }
    println!();

    let ticks = 500;
    let strategies = [
        (Strategy::Uncoordinated, "Uncoordinated", "Every agent fires whenever ready"),
        (Strategy::Orchestrated,  "Orchestrated",  "Central planner picks the best agent"),
        (Strategy::Musical,       "Musical",       "Agents listen and find their moment"),
    ];

    println!("Running {} ticks per strategy...\n", ticks);
    println!("{:<18} {:>14} {:>14} {:>14} {:>14}",
        "Strategy", "Output", "Coordination", "Emergence", "Sync");
    println!("{}", "─".repeat(76));

    let mut results = Vec::new();
    for (strategy, name, _desc) in &strategies {
        let r = run_experiment(&agents, *strategy, ticks);
        println!("{:<18} {:>14.3} {:>14.3} {:>14.3} {:>14.3}",
            name, r.total_output, r.coordination_quality, r.emergence_score, r.sync_accuracy);
        results.push((*name, r));
    }

    println!();

    // Prove emergence
    let musical = &results.iter().find(|(n, _)| *n == "Musical").unwrap().1;
    let best_individual = agents.iter().map(|a| a.skill).fold(0.0_f64, f64::max);
    println!("━━━ Emergence Analysis ━━━");
    println!("  Best individual skill:  {:.3}", best_individual);
    println!("  Musical avg output/tick: {:.3}", musical.total_output);
    println!("  Musical emergence score: {:.3}", musical.emergence_score);
    println!();

    if musical.emergence_score > 1.0 {
        println!("  ✅ EMERGENCE DETECTED! The ensemble ({:.1}x) outperforms the best individual.",
            musical.emergence_score);
    } else {
        println!("  ⚠️  Emergence score: {:.3}x individual best", musical.emergence_score);
    }

    println!();

    // Statistical significance
    println!("━━━ Statistical Validation (50 trials, 4 agents) ━━━");
    let (wins, median_ratio) = agent_ensemble::statistical_test(50, 4, 200);
    println!("  Musical wins:          {} / 50 ({:.0}%)", wins, wins as f64 / 50.0 * 100.0);
    println!("  Median quality ratio:  {:.2}x", median_ratio);
    if wins > 40 {
        println!("  ✅ Musical coordination is statistically superior");
    }
}
