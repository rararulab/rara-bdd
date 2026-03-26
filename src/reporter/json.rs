//! Machine-readable JSON reporter.

use crate::evaluator::SuiteResults;

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
                "passed": r.passed,
                "verdict": r.verdict.label(),
                "warnings": r.warnings,
                "message": r.message,
            })
        })
        .collect();

    println!(
        "{}",
        serde_json::json!({
            "ok": results.all_verified(),
            "action": "bdd-run",
            "passed": results.passed_count(),
            "failed": results.failed_count(),
            "skeleton": results.skeleton_count(),
            "weak": results.weak_count(),
            "total": results.total_count(),
            "scenarios": scenarios,
        })
    );
}
