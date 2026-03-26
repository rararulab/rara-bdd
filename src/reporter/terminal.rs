//! Colored terminal reporter.

use colored::Colorize;

use crate::{
    matcher::MatchedAc,
    runner::{AcStatus, SuiteResults},
};

/// Print suite results to terminal with colors.
pub fn report(results: &SuiteResults) {
    println!();

    for result in &results.results {
        let status = match result.status {
            AcStatus::Passed => "PASS".green(),
            AcStatus::Failed => "FAIL".red(),
            AcStatus::Uncovered => "NONE".yellow(),
        };

        println!(
            "{status} {} {} ({})",
            result.ac_id.cyan(),
            result.scenario_name,
            result.feature_file.dimmed()
        );

        if result.status == AcStatus::Failed || result.status == AcStatus::Uncovered {
            println!("    {}", result.message.dimmed());
        }

        for test in &result.tests {
            println!("    {} {test}", "test:".dimmed());
        }
    }

    println!(
        "\n{}",
        format!(
            "Summary: {} passed, {} failed, {} uncovered, {} total",
            results.passed_count(),
            results.failed_count(),
            results.uncovered_count(),
            results.total_count()
        )
        .bold()
    );
}

/// List discovered ACs and their matched tests.
pub fn list_matched(matched: &[MatchedAc]) {
    println!("\n{}\n", "BDD Scenarios → Tests:".bold());

    let mut current_feature = String::new();
    for m in matched {
        if m.scenario.feature_file != current_feature {
            current_feature.clone_from(&m.scenario.feature_file);
            println!("  {}", current_feature.bold().underline());
        }

        let coverage = if m.is_covered() {
            format!(
                " ({} test{})",
                m.tests.len(),
                if m.tests.len() == 1 { "" } else { "s" }
            )
            .green()
        } else {
            " (uncovered)".yellow()
        };

        println!(
            "    {} {}{coverage}",
            m.scenario.ac_id.cyan(),
            m.scenario.name.dimmed()
        );

        for test in &m.tests {
            println!("      {} {test}", "→".dimmed());
        }
    }

    if matched.is_empty() {
        println!("  {}", "No scenarios found".yellow());
    }

    let covered = matched.iter().filter(|m| m.is_covered()).count();
    println!(
        "\n  {}",
        format!(
            "Total: {} ACs, {} covered, {} uncovered",
            matched.len(),
            covered,
            matched.len() - covered
        )
        .dimmed()
    );
}
