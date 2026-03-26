//! Step definition coverage analysis.
//!
//! Scans Rust source files for `#[given]`, `#[when]`, `#[then]`
//! attribute macros and compares against steps in `.feature` files.

use std::{fs, path::Path};

use regex::Regex;
use snafu::ResultExt;

use crate::{
    discovery::{Scenario, Step, StepKeyword},
    error::{self, IoSnafu},
};

/// A step definition found in Rust source.
#[derive(Debug, Clone)]
pub struct DefinedStep {
    /// Keyword: given/when/then.
    pub keyword:    StepKeyword,
    /// The cucumber expression or literal string.
    pub expression: String,
    /// Source file path.
    pub file:       String,
}

/// Coverage report.
#[derive(Debug)]
pub struct CoverageReport {
    /// Total unique steps in .feature files.
    pub total_steps:   usize,
    /// Steps with matching definitions.
    pub covered_steps: usize,
    /// Steps without matching definitions.
    pub missing_steps: Vec<MissingStep>,
}

/// A step that lacks a definition.
#[derive(Debug)]
pub struct MissingStep {
    pub keyword:       StepKeyword,
    pub text:          String,
    pub feature_file:  String,
    pub scenario_name: String,
}

/// Scan Rust source files for step definition annotations.
pub fn find_defined_steps(steps_dir: &Path) -> error::Result<Vec<DefinedStep>> {
    if !steps_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut defined = Vec::new();
    scan_dir_recursive(steps_dir, &mut defined)?;
    Ok(defined)
}

/// Recursively scan `.rs` files for step annotations.
fn scan_dir_recursive(dir: &Path, defined: &mut Vec<DefinedStep>) -> error::Result<()> {
    let entries = fs::read_dir(dir).context(IoSnafu)?;

    for entry in entries {
        let entry = entry.context(IoSnafu)?;
        let path = entry.path();

        if path.is_dir() {
            scan_dir_recursive(&path, defined)?;
        } else if path.extension().is_some_and(|e| e == "rs") {
            let content = fs::read_to_string(&path).context(IoSnafu)?;
            let file_str = path.to_string_lossy().to_string();
            parse_step_annotations(&content, &file_str, defined);
        }
    }

    Ok(())
}

/// Parse step annotations from Rust source content.
fn parse_step_annotations(content: &str, file: &str, defined: &mut Vec<DefinedStep>) {
    let re = Regex::new(r#"#\[(given|when|then)\(\s*(?:expr\s*=\s*)?"([^"]+)"\s*\)\]"#)
        .expect("valid regex");

    for cap in re.captures_iter(content) {
        let keyword = match &cap[1] {
            "given" => StepKeyword::Given,
            "when" => StepKeyword::When,
            "then" => StepKeyword::Then,
            _ => continue,
        };

        defined.push(DefinedStep {
            keyword,
            expression: cap[2].to_string(),
            file: file.to_string(),
        });
    }
}

/// Normalize step text for comparison: replace quoted strings and numbers
/// with cucumber expression placeholders.
pub fn normalize_step_text(text: &str) -> String {
    let result = regex::Regex::new(r#""[^"]*""#)
        .expect("valid regex")
        .replace_all(text, "{string}");
    let result = regex::Regex::new(r"\b\d+\.\d+\b")
        .expect("valid regex")
        .replace_all(&result, "{float}");
    regex::Regex::new(r"\b\d+\b")
        .expect("valid regex")
        .replace_all(&result, "{int}")
        .to_string()
}

/// Check which feature steps have matching step definitions.
pub fn check_coverage(scenarios: &[Scenario], defined: &[DefinedStep]) -> CoverageReport {
    let normalized_defs: Vec<(StepKeyword, String)> = defined
        .iter()
        .map(|d| (d.keyword, d.expression.clone()))
        .collect();

    let mut seen = std::collections::HashSet::new();
    let mut missing_steps = Vec::new();
    let mut total = 0;
    let mut covered = 0;

    for scenario in scenarios {
        for step in &scenario.steps {
            let key = (step.keyword, step.text.clone());
            if !seen.insert(key) {
                continue;
            }

            total += 1;
            let normalized = normalize_step_text(&step.text);

            let is_covered = normalized_defs
                .iter()
                .any(|(kw, expr)| *kw == step.keyword && *expr == normalized);

            if is_covered {
                covered += 1;
            } else {
                missing_steps.push(MissingStep {
                    keyword:       step.keyword,
                    text:          step.text.clone(),
                    feature_file:  scenario.feature_file.clone(),
                    scenario_name: scenario.name.clone(),
                });
            }
        }
    }

    CoverageReport {
        total_steps: total,
        covered_steps: covered,
        missing_steps,
    }
}

/// Check if a step is already defined.
pub fn is_step_defined(step: &Step, defined: &[DefinedStep]) -> bool {
    let normalized = normalize_step_text(&step.text);
    defined
        .iter()
        .any(|d| d.keyword == step.keyword && d.expression == normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_replaces_quoted_strings() {
        assert_eq!(
            normalize_step_text(r#"a user named "alice""#),
            "a user named {string}"
        );
    }

    #[test]
    fn normalize_replaces_integers() {
        assert_eq!(
            normalize_step_text("she eats 3 cucumbers"),
            "she eats {int} cucumbers"
        );
    }

    #[test]
    fn normalize_replaces_floats() {
        assert_eq!(
            normalize_step_text("the price is 9.99"),
            "the price is {float}"
        );
    }

    #[test]
    fn parse_annotations_finds_steps() {
        let src = r#"
            #[given("a hungry cat")]
            async fn hungry(w: &mut World) {}

            #[when(expr = "she eats {int} cucumbers")]
            async fn eat(w: &mut World, n: i32) {}

            #[then("she is full")]
            async fn full(w: &mut World) {}
        "#;

        let mut defined = Vec::new();
        parse_step_annotations(src, "test.rs", &mut defined);
        assert_eq!(defined.len(), 3);
        assert_eq!(defined[0].expression, "a hungry cat");
        assert_eq!(defined[1].expression, "she eats {int} cucumbers");
        assert_eq!(defined[2].expression, "she is full");
    }
}
