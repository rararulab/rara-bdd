//! Gherkin feature file discovery and parsing.
//!
//! Scans a `features/` directory for `.feature` files, parses them
//! with the `cucumber` crate's Gherkin parser, and extracts scenarios
//! with their AC tags.

use std::{fs, path::Path};

use snafu::ResultExt;

use crate::{
    error::{self, FeaturesNotFoundSnafu, IoSnafu},
    evaluator::loader,
};

/// A discovered BDD scenario from a `.feature` file.
#[derive(Debug, Clone)]
pub struct Scenario {
    /// Stable AC ID tag when present (e.g. `AC-01`).
    pub ac_id:        String,
    /// Scenario title from `Scenario:` line.
    pub name:         String,
    /// Feature file relative path (e.g. `auth/login.feature`).
    pub feature_file: String,
    /// All tags on this scenario (without `@` prefix).
    pub tags:         Vec<String>,
    /// Given/When/Then steps as raw strings.
    pub steps:        Vec<String>,
    /// Paired eval config (loaded from .eval.yaml if present).
    pub eval:         Option<loader::AcEval>,
}

/// Discover all scenarios from `.feature` files in the given directory.
///
/// Optionally filters by AC ID, tag, or scenario name substring.
pub fn discover(features_dir: &str, filter: Option<&str>) -> error::Result<Vec<Scenario>> {
    let dir = Path::new(features_dir);
    if !dir.is_dir() {
        return FeaturesNotFoundSnafu {
            path: features_dir.to_string(),
        }
        .fail();
    }

    let mut scenarios = Vec::new();
    discover_recursive(dir, dir, &mut scenarios)?;

    // Load paired eval configs
    for scenario in &mut scenarios {
        let eval_path = eval_path_for_feature(features_dir, &scenario.feature_file);
        if let Ok(evals) = loader::load_eval_file(&eval_path) {
            scenario.eval = evals.get(&scenario.ac_id).cloned();
        }
    }

    // Apply filter
    if let Some(f) = filter {
        let f_lower = f.to_lowercase();
        scenarios.retain(|s| {
            s.ac_id.to_lowercase().contains(&f_lower)
                || s.name.to_lowercase().contains(&f_lower)
                || s.tags.iter().any(|t| t.to_lowercase().contains(&f_lower))
        });
    }

    Ok(scenarios)
}

/// Recursively scan directory for `.feature` files.
fn discover_recursive(base: &Path, dir: &Path, scenarios: &mut Vec<Scenario>) -> error::Result<()> {
    let entries = fs::read_dir(dir).context(IoSnafu)?;

    for entry in entries {
        let entry = entry.context(IoSnafu)?;
        let path = entry.path();

        if path.is_dir() {
            discover_recursive(base, &path, scenarios)?;
        } else if path.extension().is_some_and(|e| e == "feature") {
            let content = fs::read_to_string(&path).context(IoSnafu)?;
            let relative = path
                .strip_prefix(base)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();
            parse_feature(&content, &relative, scenarios)?;
        }
    }

    Ok(())
}

/// Parse a single `.feature` file content into scenarios.
fn parse_feature(
    content: &str,
    feature_file: &str,
    scenarios: &mut Vec<Scenario>,
) -> error::Result<()> {
    let feature =
        cucumber::gherkin::Feature::parse(content, cucumber::gherkin::GherkinEnv::default())
            .map_err(|e| error::RaraBddError::Gherkin {
                path:   feature_file.to_string(),
                reason: e.to_string(),
            })?;

    for scenario in &feature.scenarios {
        let tags = scenario.tags.clone();

        // Extract AC ID from tags (first tag matching AC-XX pattern)
        let ac_id = tags
            .iter()
            .find(|t| is_ac_tag(t))
            .cloned()
            .unwrap_or_else(|| format!("UNTAGGED-{}", scenario.name.replace(' ', "-")));

        let steps: Vec<String> = scenario
            .steps
            .iter()
            .map(|s| format!("{} {}", s.keyword.trim(), s.value))
            .collect();

        scenarios.push(Scenario {
            ac_id,
            name: scenario.name.clone(),
            feature_file: feature_file.to_string(),
            tags,
            steps,
            eval: None,
        });
    }

    Ok(())
}

/// Check if a tag matches the AC-XX pattern.
fn is_ac_tag(tag: &str) -> bool {
    tag.starts_with("AC-") && tag.len() > 3 && tag[3..].chars().all(|c| c.is_ascii_digit())
}

/// Derive the `.eval.yaml` path from a `.feature` file path.
fn eval_path_for_feature(features_dir: &str, feature_file: &str) -> String {
    let stem = feature_file.trim_end_matches(".feature");
    format!("{features_dir}/{stem}.eval.yaml")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ac_tag() {
        assert!(is_ac_tag("AC-01"));
        assert!(is_ac_tag("AC-123"));
        assert!(!is_ac_tag("AC-"));
        assert!(!is_ac_tag("ac-01"));
        assert!(!is_ac_tag("hooks"));
    }

    #[test]
    fn test_eval_path_for_feature() {
        assert_eq!(
            eval_path_for_feature("features", "auth/login.feature"),
            "features/auth/login.eval.yaml"
        );
    }
}
