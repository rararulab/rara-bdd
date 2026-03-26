//! Colored terminal reporter.

use colored::Colorize;

use crate::{discovery::Scenario, evaluator::SuiteResults};

/// Print suite results to terminal with colors.
pub fn report(results: &SuiteResults) {
    println!();

    for result in &results.results {
        let status = if result.passed {
            "PASS".green()
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
    }

    println!(
        "\n{}",
        format!(
            "Summary: {} passed, {} failed, {} total",
            results.passed_count(),
            results.failed_count(),
            results.total_count()
        )
        .bold()
    );
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
