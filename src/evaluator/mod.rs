//! Evaluator dispatch and YAML DSL interpreter.
//!
//! Routes AC IDs to their evaluation logic defined in `.eval.yaml` files.

pub mod loader;
pub mod runtime;
pub mod source;

use crate::discovery::Scenario;
use crate::error;

/// Result of running the full BDD suite.
#[derive(Debug)]
pub struct SuiteResults {
    pub results: Vec<ScenarioResult>,
}

impl SuiteResults {
    /// Whether all scenarios passed.
    pub fn all_passed(&self) -> bool {
        self.results.iter().all(|r| r.passed)
    }

    pub fn passed_count(&self) -> usize {
        self.results.iter().filter(|r| r.passed).count()
    }

    pub fn failed_count(&self) -> usize {
        self.results.iter().filter(|r| !r.passed).count()
    }

    pub const fn total_count(&self) -> usize {
        self.results.len()
    }
}

/// Result of evaluating a single scenario.
#[derive(Debug)]
pub struct ScenarioResult {
    pub ac_id: String,
    pub scenario_name: String,
    pub feature_file: String,
    pub passed: bool,
    pub message: String,
}

/// Run the full BDD evaluation suite.
pub fn run_suite(scenarios: &[Scenario], ci_safe_mode: bool) -> error::Result<SuiteResults> {
    let results = scenarios
        .iter()
        .map(|scenario| evaluate_scenario(scenario, ci_safe_mode))
        .collect();

    Ok(SuiteResults { results })
}

/// Evaluate a single scenario against its `.eval.yaml` assertions.
fn evaluate_scenario(scenario: &Scenario, _ci_safe_mode: bool) -> ScenarioResult {
    let Some(eval) = &scenario.eval else {
        return ScenarioResult {
            ac_id: scenario.ac_id.clone(),
            scenario_name: scenario.name.clone(),
            feature_file: scenario.feature_file.clone(),
            passed: false,
            message: format!("{}: no eval config found", scenario.ac_id),
        };
    };

    // Run runtime tests
    if let Some(tests) = &eval.runtime_tests {
        for test in tests {
            if let Err(e) = runtime::run_cargo_test(&test.package, &test.filter) {
                return ScenarioResult {
                    ac_id: scenario.ac_id.clone(),
                    scenario_name: scenario.name.clone(),
                    feature_file: scenario.feature_file.clone(),
                    passed: false,
                    message: format!("{}: runtime test failed — {e}", scenario.ac_id),
                };
            }
        }
    }

    // Run source assertions
    if let Some(assertions) = &eval.source_assertions {
        for assertion in assertions {
            if let Err(e) = source::check_source_assertion(assertion) {
                return ScenarioResult {
                    ac_id: scenario.ac_id.clone(),
                    scenario_name: scenario.name.clone(),
                    feature_file: scenario.feature_file.clone(),
                    passed: false,
                    message: format!("{}: source assertion failed — {e}", scenario.ac_id),
                };
            }
        }
    }

    // Run command assertions
    if let Some(commands) = &eval.command_assertions {
        for cmd in commands {
            if let Err(e) = runtime::run_command_assertion(cmd) {
                return ScenarioResult {
                    ac_id: scenario.ac_id.clone(),
                    scenario_name: scenario.name.clone(),
                    feature_file: scenario.feature_file.clone(),
                    passed: false,
                    message: format!("{}: command assertion failed — {e}", scenario.ac_id),
                };
            }
        }
    }

    ScenarioResult {
        ac_id: scenario.ac_id.clone(),
        scenario_name: scenario.name.clone(),
        feature_file: scenario.feature_file.clone(),
        passed: true,
        message: format!("{}: acceptance criterion verified green", scenario.ac_id),
    }
}

/// Validation result for `.eval.yaml` files.
#[derive(Debug)]
pub struct ValidationResult {
    pub feature_count: usize,
    pub eval_count: usize,
    pub errors: Vec<String>,
}

/// Validate all `.eval.yaml` files in the features directory.
pub fn validate(features_dir: &str) -> error::Result<ValidationResult> {
    use std::path::Path;

    let dir = Path::new(features_dir);
    if !dir.is_dir() {
        return error::FeaturesNotFoundSnafu {
            path: features_dir.to_string(),
        }
        .fail();
    }

    let mut feature_count = 0;
    let mut eval_count = 0;
    let mut errors = Vec::new();

    validate_recursive(dir, &mut feature_count, &mut eval_count, &mut errors);

    Ok(ValidationResult {
        feature_count,
        eval_count,
        errors,
    })
}

fn validate_recursive(
    dir: &std::path::Path,
    feature_count: &mut usize,
    eval_count: &mut usize,
    errors: &mut Vec<String>,
) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            validate_recursive(&path, feature_count, eval_count, errors);
        } else if path.extension().is_some_and(|e| e == "feature") {
            *feature_count += 1;
        } else if path
            .to_string_lossy()
            .ends_with(".eval.yaml")
        {
            *eval_count += 1;
            if let Err(e) = loader::load_eval_file(&path.to_string_lossy()) {
                errors.push(format!("{}: {e}", path.display()));
            }
        }
    }
}
