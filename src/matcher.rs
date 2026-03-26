//! AC-to-test matching via naming convention.
//!
//! Discovers `#[test]` functions by running `cargo test -- --list`,
//! then matches them to AC IDs using the `ac_XX_*` naming convention.
//!
//! Convention: `@AC-01` in Gherkin maps to `fn ac_01_*()` in Rust tests.

use std::process::Command;

use crate::{discovery::Scenario, error};

/// A matched AC with its discovered test functions.
#[derive(Debug, Clone)]
pub struct MatchedAc {
    /// The scenario from the `.feature` file.
    pub scenario: Scenario,
    /// Test function names that matched this AC (may be empty = uncovered).
    pub tests:    Vec<String>,
}

impl MatchedAc {
    /// Whether this AC has at least one matching test.
    pub const fn is_covered(&self) -> bool { !self.tests.is_empty() }
}

/// Discover all test function names via `cargo test -- --list`.
pub fn discover_tests(packages: &[String]) -> error::Result<Vec<String>> {
    let mut all_tests = Vec::new();

    if packages.is_empty() {
        all_tests.extend(list_tests_for_package(None)?);
    } else {
        for pkg in packages {
            all_tests.extend(list_tests_for_package(Some(pkg))?);
        }
    }

    Ok(all_tests)
}

/// Run `cargo test` with `--list` to get test names from one package (or all).
fn list_tests_for_package(package: Option<&str>) -> error::Result<Vec<String>> {
    let mut cmd = Command::new("cargo");
    cmd.args(["test"]);

    if let Some(pkg) = package {
        cmd.args(["-p", pkg]);
    }

    cmd.args(["--", "--list"]);

    let output = cmd
        .output()
        .map_err(|e| error::RaraBddError::TestDiscovery {
            message: format!("failed to run cargo test --list: {e}"),
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Each line looks like: "module::test_name: test" or "test_name: test"
    let tests = stdout
        .lines()
        .filter_map(|line| {
            let name = line.strip_suffix(": test")?;
            Some(name.to_string())
        })
        .collect();

    Ok(tests)
}

/// Match discovered scenarios to test functions by naming convention.
///
/// `@AC-01` matches test functions containing `ac_01_` (case-insensitive).
pub fn match_scenarios(scenarios: &[Scenario], test_names: &[String]) -> Vec<MatchedAc> {
    scenarios
        .iter()
        .map(|scenario| {
            let prefix = ac_id_to_test_prefix(&scenario.ac_id);
            let matched: Vec<String> = test_names
                .iter()
                .filter(|t| {
                    let lower = t.to_lowercase();
                    // Match "ac_01_" anywhere in the test path (handles module::ac_01_xxx)
                    lower.contains(&prefix)
                })
                .cloned()
                .collect();

            MatchedAc {
                scenario: scenario.clone(),
                tests:    matched,
            }
        })
        .collect()
}

/// Convert `AC-01` to `ac_01_` for test name matching.
fn ac_id_to_test_prefix(ac_id: &str) -> String {
    let mut prefix = ac_id.to_lowercase().replace('-', "_");
    prefix.push('_');
    prefix
}

/// Summary of coverage analysis.
#[derive(Debug)]
pub struct CoverageSummary {
    /// Total number of ACs.
    pub total:     usize,
    /// ACs with at least one matching test.
    pub covered:   usize,
    /// ACs without any matching test.
    pub uncovered: usize,
}

/// Compute coverage summary from matched ACs.
pub fn coverage_summary(matched: &[MatchedAc]) -> CoverageSummary {
    let covered = matched.iter().filter(|m| m.is_covered()).count();
    CoverageSummary {
        total: matched.len(),
        covered,
        uncovered: matched.len() - covered,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ac_id_to_prefix() {
        assert_eq!(ac_id_to_test_prefix("AC-01"), "ac_01_");
        assert_eq!(ac_id_to_test_prefix("AC-123"), "ac_123_");
    }

    #[test]
    fn matching_finds_tests() {
        let scenarios = vec![Scenario {
            ac_id:        "AC-01".into(),
            name:         "Test scenario".into(),
            feature_file: "test.feature".into(),
            tags:         vec!["AC-01".into()],
            steps:        vec![],
        }];

        let test_names = vec![
            "tests::ac_01_basic_check".into(),
            "tests::ac_01_edge_case".into(),
            "tests::ac_02_other".into(),
            "tests::unrelated".into(),
        ];

        let matched = match_scenarios(&scenarios, &test_names);
        assert_eq!(matched.len(), 1);
        assert_eq!(matched[0].tests.len(), 2);
        assert!(matched[0].is_covered());
    }

    #[test]
    fn unmatched_is_uncovered() {
        let scenarios = vec![Scenario {
            ac_id:        "AC-99".into(),
            name:         "No tests".into(),
            feature_file: "test.feature".into(),
            tags:         vec!["AC-99".into()],
            steps:        vec![],
        }];

        let test_names = vec!["tests::ac_01_something".into()];

        let matched = match_scenarios(&scenarios, &test_names);
        assert!(!matched[0].is_covered());
    }
}
