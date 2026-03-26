//! Test runner — executes matched tests and collects results.

use std::process::Command;

use crate::{error, matcher::MatchedAc};

/// Result of running the BDD suite.
#[derive(Debug)]
pub struct SuiteResults {
    pub results: Vec<AcResult>,
}

impl SuiteResults {
    /// Whether every AC is covered and all tests passed.
    pub fn all_passed(&self) -> bool { self.results.iter().all(|r| r.status == AcStatus::Passed) }

    pub fn passed_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.status == AcStatus::Passed)
            .count()
    }

    pub fn failed_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.status == AcStatus::Failed)
            .count()
    }

    pub fn uncovered_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.status == AcStatus::Uncovered)
            .count()
    }

    pub const fn total_count(&self) -> usize { self.results.len() }
}

/// Status of a single AC after evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcStatus {
    /// Has tests and all passed.
    Passed,
    /// Has tests but some failed.
    Failed,
    /// No matching tests found.
    Uncovered,
}

impl AcStatus {
    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Passed => "pass",
            Self::Failed => "fail",
            Self::Uncovered => "uncovered",
        }
    }
}

/// Result of evaluating a single AC.
#[derive(Debug)]
pub struct AcResult {
    pub ac_id:         String,
    pub scenario_name: String,
    pub feature_file:  String,
    pub status:        AcStatus,
    pub tests:         Vec<String>,
    pub message:       String,
}

/// Run matched tests and produce suite results.
pub fn run_suite(matched: &[MatchedAc]) -> error::Result<SuiteResults> {
    let results = matched.iter().map(run_ac).collect();
    Ok(SuiteResults { results })
}

/// Run tests for a single AC.
fn run_ac(matched: &MatchedAc) -> AcResult {
    let ac_id = &matched.scenario.ac_id;

    if !matched.is_covered() {
        return AcResult {
            ac_id:         ac_id.clone(),
            scenario_name: matched.scenario.name.clone(),
            feature_file:  matched.scenario.feature_file.clone(),
            status:        AcStatus::Uncovered,
            tests:         vec![],
            message:       format!(
                "{ac_id}: no matching test functions (expected fn ac_{}_*())",
                ac_id.strip_prefix("AC-").unwrap_or(ac_id).to_lowercase()
            ),
        };
    }

    // Run cargo test with exact filters for matched test names
    for test_name in &matched.tests {
        match run_single_test(test_name) {
            Ok(()) => {}
            Err(e) => {
                return AcResult {
                    ac_id:         ac_id.clone(),
                    scenario_name: matched.scenario.name.clone(),
                    feature_file:  matched.scenario.feature_file.clone(),
                    status:        AcStatus::Failed,
                    tests:         matched.tests.clone(),
                    message:       format!("{ac_id}: test failed — {e}"),
                };
            }
        }
    }

    AcResult {
        ac_id:         ac_id.clone(),
        scenario_name: matched.scenario.name.clone(),
        feature_file:  matched.scenario.feature_file.clone(),
        status:        AcStatus::Passed,
        tests:         matched.tests.clone(),
        message:       format!(
            "{ac_id}: {} test{} passed",
            matched.tests.len(),
            if matched.tests.len() == 1 { "" } else { "s" }
        ),
    }
}

/// Run a single test by exact name.
fn run_single_test(test_name: &str) -> error::Result<()> {
    let output = Command::new("cargo")
        .args(["test", "--", test_name, "--exact"])
        .output()
        .map_err(|e| error::RaraBddError::TestExecution {
            message: format!("failed to spawn cargo test: {e}"),
        })?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(error::RaraBddError::TestExecution {
            message: format!(
                "{test_name} failed (exit {})\n{stderr}",
                output.status.code().unwrap_or(-1)
            ),
        })
    }
}
