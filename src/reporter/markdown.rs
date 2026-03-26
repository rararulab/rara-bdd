//! Agent-readable markdown reporter.

use crate::evaluator::SuiteResults;

/// Print suite results as markdown to stdout.
pub fn report(results: &SuiteResults) {
    println!("# BDD Suite Results\n");
    println!(
        "**{}** passed, **{}** failed, **{}** total\n",
        results.passed_count(),
        results.failed_count(),
        results.total_count()
    );

    println!("| AC | Scenario | Feature | Status | Message |");
    println!("|---|---|---|---|---|");

    for r in &results.results {
        let status = if r.passed { "PASS" } else { "FAIL" };
        println!(
            "| {} | {} | {} | {} | {} |",
            r.ac_id, r.scenario_name, r.feature_file, status, r.message
        );
    }
}
