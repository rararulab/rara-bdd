//! Agent-readable markdown reporter.

use crate::runner::SuiteResults;

/// Print suite results as markdown to stdout.
pub fn report(results: &SuiteResults) {
    println!("# BDD Suite Results\n");
    println!(
        "**{}** passed, **{}** failed, **{}** uncovered, **{}** total\n",
        results.passed_count(),
        results.failed_count(),
        results.uncovered_count(),
        results.total_count()
    );

    println!("| AC | Scenario | Feature | Status | Tests | Message |");
    println!("|---|---|---|---|---|---|");

    for r in &results.results {
        println!(
            "| {} | {} | {} | {} | {} | {} |",
            r.ac_id,
            r.scenario_name,
            r.feature_file,
            r.status.label(),
            r.tests.len(),
            r.message
        );
    }
}
