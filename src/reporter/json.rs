//! Machine-readable JSON reporter.

use crate::runner::SuiteResults;

/// Print suite results as JSON to stdout.
pub fn report(results: &SuiteResults) {
    let scenarios: Vec<serde_json::Value> = results
        .results
        .iter()
        .map(|r| {
            serde_json::json!({
                "ac_id": r.ac_id,
                "scenario": r.scenario_name,
                "feature_file": r.feature_file,
                "status": r.status.label(),
                "tests": r.tests,
                "message": r.message,
            })
        })
        .collect();

    println!(
        "{}",
        serde_json::json!({
            "ok": results.all_passed(),
            "action": "bdd-run",
            "passed": results.passed_count(),
            "failed": results.failed_count(),
            "uncovered": results.uncovered_count(),
            "total": results.total_count(),
            "scenarios": scenarios,
        })
    );
}
