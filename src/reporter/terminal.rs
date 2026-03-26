//! Colored terminal reporter.

use std::fmt::Write;

use colored::Colorize;

use crate::{
    discovery::Scenario,
    evaluator::{SuiteResults, quality::Verdict},
};

/// Print suite results to terminal with colors.
pub fn report(results: &SuiteResults) {
    println!();

    for result in &results.results {
        let status = if result.passed {
            match result.verdict {
                Verdict::Verified => "PASS".green(),
                Verdict::Skeleton => "SKEL".yellow(),
                Verdict::Weak => "WEAK".yellow(),
            }
        } else {
            "FAIL".red()
        };

        println!(
            "{status} {} {} ({})",
            result.ac_id.cyan(),
            result.scenario_name,
            result.feature_file.dimmed()
        );

        if !result.passed {
            println!("    {}", result.message.dimmed());
        }

        for warning in &result.warnings {
            println!("    {} {}", "warn:".yellow(), warning.dimmed());
        }
    }

    let mut summary = format!(
        "Summary: {} passed, {} failed, {} total",
        results.passed_count(),
        results.failed_count(),
        results.total_count()
    );

    let skel = results.skeleton_count();
    let weak = results.weak_count();
    if skel > 0 || weak > 0 {
        let _ = write!(summary, " ({skel} skeleton, {weak} weak)");
    }

    println!("\n{}", summary.bold());
}

/// List discovered scenarios to terminal.
pub fn list_scenarios(scenarios: &[Scenario]) {
    println!("\n{}\n", "Discovered BDD scenarios:".bold());

    let mut current_feature = String::new();
    for scenario in scenarios {
        if scenario.feature_file != current_feature {
            current_feature.clone_from(&scenario.feature_file);
            println!("  {}", current_feature.bold().underline());
        }

        let has_eval = if scenario.eval.is_some() {
            "".green()
        } else {
            " (no eval)".yellow()
        };

        println!(
            "    {} {}{has_eval}",
            scenario.ac_id.cyan(),
            scenario.name.dimmed()
        );
    }

    if scenarios.is_empty() {
        println!("  {}", "No scenarios found".yellow());
    }

    println!(
        "\n  {}",
        format!(
            "Total: {} scenario{}",
            scenarios.len(),
            if scenarios.len() == 1 { "" } else { "s" }
        )
        .dimmed()
    );
}
