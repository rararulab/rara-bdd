//! Gherkin feature file discovery and parsing.
//!
//! Scans a `features/` directory for `.feature` files, parses them
//! with the `cucumber` crate's Gherkin parser, and extracts scenarios
//! with their steps.

use std::{fs, path::Path};

use snafu::ResultExt;

use crate::error::{self, FeaturesNotFoundSnafu, IoSnafu};

/// Gherkin step keyword.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StepKeyword {
    Given,
    When,
    Then,
}

impl StepKeyword {
    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Given => "Given",
            Self::When => "When",
            Self::Then => "Then",
        }
    }
}

impl std::fmt::Display for StepKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str(self.label()) }
}

/// A parsed step from a `.feature` file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Step {
    /// Keyword: Given, When, Then.
    pub keyword: StepKeyword,
    /// Raw step text (e.g., `a valid user "alice"`).
    pub text:    String,
}

/// A discovered BDD scenario from a `.feature` file.
#[derive(Debug, Clone)]
pub struct Scenario {
    /// Scenario title from `Scenario:` line.
    pub name:         String,
    /// Feature file relative path (e.g., `auth/login.feature`).
    pub feature_file: String,
    /// Feature name from the `Feature:` line.
    pub feature_name: String,
    /// All tags on this scenario (without `@` prefix).
    pub tags:         Vec<String>,
    /// Ordered steps.
    pub steps:        Vec<Step>,
}

/// Discover all scenarios from `.feature` files in the given directory.
///
/// Optionally filters by tag or scenario name substring.
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
            s.name.to_lowercase().contains(&f_lower)
                || s.tags.iter().any(|t| t.to_lowercase().contains(&f_lower))
        });
    }

    Ok(scenarios)
}

/// Extract all unique steps across all scenarios.
pub fn unique_steps(scenarios: &[Scenario]) -> Vec<&Step> {
    let mut seen = std::collections::HashSet::new();
    scenarios
        .iter()
        .flat_map(|s| &s.steps)
        .filter(|step| seen.insert((&step.keyword, &step.text)))
        .collect()
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

    let feature_name = feature.name.clone();

    for scenario in &feature.scenarios {
        // Resolve And/But to the preceding Given/When/Then
        let mut last_keyword = StepKeyword::Given;
        let steps: Vec<Step> = scenario
            .steps
            .iter()
            .map(|s| {
                let keyword = match s.ty {
                    cucumber::gherkin::StepType::Given => StepKeyword::Given,
                    cucumber::gherkin::StepType::When => StepKeyword::When,
                    cucumber::gherkin::StepType::Then => StepKeyword::Then,
                };
                last_keyword = keyword;
                Step {
                    keyword: last_keyword,
                    text:    s.value.clone(),
                }
            })
            .collect();

        scenarios.push(Scenario {
            name: scenario.name.clone(),
            feature_file: feature_file.to_string(),
            feature_name: feature_name.clone(),
            tags: scenario.tags.clone(),
            steps,
        });
    }

    Ok(())
}
