//! Gherkin feature file discovery and parsing.
//!
//! Scans a `features/` directory for `.feature` files, parses them
//! with the `cucumber` crate's Gherkin parser, and extracts scenarios
//! with their AC tags.

use std::{fs, path::Path};

use snafu::ResultExt;

use crate::error::{self, FeaturesNotFoundSnafu, IoSnafu};

/// A discovered BDD scenario from a `.feature` file.
#[derive(Debug, Clone)]
pub struct Scenario {
    /// Stable AC ID tag (e.g., `AC-01`).
    pub ac_id:        String,
    /// Scenario title from `Scenario:` line.
    pub name:         String,
    /// Feature file relative path (e.g., `auth/login.feature`).
    pub feature_file: String,
    /// All tags on this scenario (without `@` prefix).
    pub tags:         Vec<String>,
    /// Given/When/Then steps as raw strings.
    pub steps:        Vec<String>,
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
        });
    }

    Ok(())
}

/// Check if a tag matches the AC-XX pattern.
fn is_ac_tag(tag: &str) -> bool {
    tag.starts_with("AC-") && tag.len() > 3 && tag[3..].chars().all(|c| c.is_ascii_digit())
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
}
