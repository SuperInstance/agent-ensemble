//! Measure emergence at different ensemble sizes. Shows the emergence curve —
//! how adding agents creates nonlinear gains under Musical coordination.

use agent_ensemble::{run_experiment, EnsembleAgent, Strategy};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║        EMERGENCE CURVE — Ensemble Size vs. Emergence        ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let sizes = [1, 2, 3, 4, 5, 6, 8, 10, 12, 16];
    let ticks = 300;

    println!("Generating homogeneous agents (skill=0.7, listening=0.7, timing=0.8)...\n");
    println!("{:<6} {:>10} {:>12} {:>12} {:>12}  {}", 
        "Size", "Output", "Coord.", "Emergence", "Sync", "Bar");
    println!("{}", "─".repeat(78));

    let mut max_emergence: f64 = 0.0;
    let mut results = Vec::new();

    for &n in &sizes {
        let agents: Vec<EnsembleAgent> = (0..n)
            .map(|i| EnsembleAgent {
                name: format!("a{}", i),
                skill: 0.7,
                listening: 0.7,
                timing_accuracy: 0.8,
            })
            .collect();

        let r = run_experiment(&agents, Strategy::Musical, ticks);
        max_emergence = max_emergence.max(r.emergence_score);
        results.push((n, r));
    }

    for (n, r) in &results {
        let bar_len = if max_emergence > 0.0 {
            ((r.emergence_score / max_emergence) * 30.0) as usize
        } else {
            0
        };
        let bar: String = "█".repeat(bar_len);
        println!("{:<6} {:>10.3} {:>12.3} {:>12.3} {:>12.3}  {}",
            n, r.total_output, r.coordination_quality, r.emergence_score, r.sync_accuracy, bar);
    }

    println!();

    // Also show the comparison for one size
    println!("━━━ Strategy Comparison at Size 8 ━━━");
    let agents8: Vec<EnsembleAgent> = (0..8)
        .map(|i| EnsembleAgent {
            name: format!("a{}", i),
            skill: 0.5 + (i as f64 * 0.05),
            listening: 0.6 + (i as f64 * 0.04),
            timing_accuracy: 0.7 + (i as f64 * 0.03),
        })
        .collect();

    for (strategy, label) in [(Strategy::Uncoordinated, "Uncoordinated"), (Strategy::Orchestrated, "Orchestrated"), (Strategy::Musical, "Musical")] {
        let r = run_experiment(&agents8, strategy, ticks);
        let em = if r.emergence_score > 0.0 { format!("{:.3}", r.emergence_score) } else { "—".to_string() };
        println!("  {:<15} output={:.3}  coord={:.3}  emergence={}", label, r.total_output, r.coordination_quality, em);
    }

    println!();

    // Key insight
    if max_emergence > 0.0 {
        println!("💡 Key insight: Emergence is a property of the Musical strategy,");
        println!("   not just having more agents. More agents = more potential for");
        println!("   coordination, but only Musical strategy realizes it.");
    }
}
