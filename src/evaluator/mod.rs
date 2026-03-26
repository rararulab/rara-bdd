//! Evaluator dispatch and YAML DSL interpreter.
//!
//! Routes AC IDs to their evaluation logic defined in `.eval.yaml` files.

pub mod loader;
pub mod quality;
pub mod runtime;
pub mod source;

use quality::Verdict;

use crate::{discovery::Scenario, error};

/// Result of running the full BDD suite.
#[derive(Debug)]
pub struct SuiteResults {
    pub results: Vec<ScenarioResult>,
}

impl SuiteResults {
    /// Whether all scenarios are verified green (skeleton/weak do NOT count).
    pub fn all_verified(&self) -> bool {
        self.results
            .iter()
            .all(|r| r.passed && r.verdict == Verdict::Verified)
    }

    /// Whether all assertions that ran actually passed (ignoring verdict
    /// quality).
    pub fn all_passed(&self) -> bool { self.results.iter().all(|r| r.passed) }

    pub fn passed_count(&self) -> usize { self.results.iter().filter(|r| r.passed).count() }

    pub fn failed_count(&self) -> usize { self.results.iter().filter(|r| !r.passed).count() }

    /// Count of skeleton scenarios (only `source_assertions`, no behavioral
    /// tests).
    pub fn skeleton_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.verdict == Verdict::Skeleton)
            .count()
    }

    /// Count of weak scenarios (trivially short patterns).
    pub fn weak_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.verdict == Verdict::Weak)
            .count()
    }

    pub const fn total_count(&self) -> usize { self.results.len() }
}

/// Result of evaluating a single scenario.
#[derive(Debug)]
pub struct ScenarioResult {
    pub ac_id:         String,
    pub scenario_name: String,
    pub feature_file:  String,
    pub passed:        bool,
    pub verdict:       Verdict,
    pub warnings:      Vec<String>,
    pub message:       String,
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
            ac_id:         scenario.ac_id.clone(),
            scenario_name: scenario.name.clone(),
            feature_file:  scenario.feature_file.clone(),
            passed:        false,
            verdict:       Verdict::Skeleton,
            warnings:      vec!["no eval config found".into()],
            message:       format!("{}: no eval config found", scenario.ac_id),
        };
    };

    // Analyze quality before running
    let (verdict, warnings) = quality::analyze(eval);

    // Run runtime tests
    if let Some(tests) = &eval.runtime_tests {
        for test in tests {
            if let Err(e) = runtime::run_cargo_test(&test.package, &test.filter) {
                return ScenarioResult {
                    ac_id: scenario.ac_id.clone(),
                    scenario_name: scenario.name.clone(),
                    feature_file: scenario.feature_file.clone(),
                    passed: false,
                    verdict,
                    warnings,
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
                    verdict,
                    warnings,
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
                    verdict,
                    warnings,
                    message: format!("{}: command assertion failed — {e}", scenario.ac_id),
                };
            }
        }
    }

    let message = match verdict {
        Verdict::Verified => format!("{}: acceptance criterion verified green", scenario.ac_id),
        Verdict::Skeleton => format!(
            "{}: assertions passed but SKELETON — only source_assertions, no behavioral tests",
            scenario.ac_id
        ),
        Verdict::Weak => format!(
            "{}: assertions passed but WEAK — contains trivially short patterns",
            scenario.ac_id
        ),
    };

    ScenarioResult {
        ac_id: scenario.ac_id.clone(),
        scenario_name: scenario.name.clone(),
        feature_file: scenario.feature_file.clone(),
        passed: true,
        verdict,
        warnings,
        message,
    }
}

/// Validation result for `.eval.yaml` files.
#[derive(Debug)]
pub struct ValidationResult {
    pub feature_count: usize,
    pub eval_count:    usize,
    pub errors:        Vec<String>,
    pub warnings:      Vec<String>,
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
    let mut warnings = Vec::new();

    validate_recursive(
        dir,
        &mut feature_count,
        &mut eval_count,
        &mut errors,
        &mut warnings,
    );

    Ok(ValidationResult {
        feature_count,
        eval_count,
        errors,
        warnings,
    })
}

fn validate_recursive(
    dir: &std::path::Path,
    feature_count: &mut usize,
    eval_count: &mut usize,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            validate_recursive(&path, feature_count, eval_count, errors, warnings);
        } else if path.extension().is_some_and(|e| e == "feature") {
            *feature_count += 1;
        } else if path.to_string_lossy().ends_with(".eval.yaml") {
            *eval_count += 1;
            match loader::load_eval_file(&path.to_string_lossy()) {
                Ok(evals) => {
                    for (ac_id, eval) in &evals {
                        let (verdict, ac_warnings) = quality::analyze(eval);
                        for w in ac_warnings {
                            warnings.push(format!("{} ({}): {w}", ac_id, path.display()));
                        }
                        if verdict != Verdict::Verified {
                            warnings.push(format!(
                                "{} ({}): quality={} — needs runtime_tests or command_assertions",
                                ac_id,
                                path.display(),
                                verdict.label()
                            ));
                        }
                    }
                }
                Err(e) => errors.push(format!("{}: {e}", path.display())),
            }
        }
    }
}
