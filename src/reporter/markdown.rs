//! Agent-readable markdown reporter.

use crate::evaluator::{SuiteResults, quality::Verdict};

/// Print suite results as markdown to stdout.
pub fn report(results: &SuiteResults) {
    println!("# BDD Suite Results\n");
    println!(
        "**{}** passed, **{}** failed, **{}** skeleton, **{}** weak, **{}** total\n",
        results.passed_count(),
        results.failed_count(),
        results.skeleton_count(),
        results.weak_count(),
        results.total_count()
    );

    println!("| AC | Scenario | Feature | Verdict | Message |");
    println!("|---|---|---|---|---|");

    for r in &results.results {
        let verdict_label = if r.passed {
            match r.verdict {
                Verdict::Verified => "PASS",
                Verdict::Skeleton => "SKEL",
                Verdict::Weak => "WEAK",
            }
        } else {
            "FAIL"
        };
        println!(
            "| {} | {} | {} | {} | {} |",
            r.ac_id, r.scenario_name, r.feature_file, verdict_label, r.message
        );
    }
}
