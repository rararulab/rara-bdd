//! YAML eval DSL loader.
//!
//! Parses `.eval.yaml` files that define how to verify each acceptance criterion.

use std::collections::HashMap;
use std::fs;

use serde::Deserialize;
use snafu::ResultExt;

use crate::error::{self, IoSnafu, YamlSnafu};

/// Evaluation config for a single acceptance criterion.
#[derive(Debug, Clone, Deserialize)]
pub struct AcEval {
    /// Human-readable description of what this AC verifies.
    pub description: Option<String>,

    /// Cargo tests that must pass.
    pub runtime_tests: Option<Vec<RuntimeTest>>,

    /// Source code patterns that must exist.
    pub source_assertions: Option<Vec<SourceAssertion>>,

    /// Commands to run and verify output.
    pub command_assertions: Option<Vec<CommandAssertion>>,
}

/// A cargo test to run.
#[derive(Debug, Clone, Deserialize)]
pub struct RuntimeTest {
    /// Cargo package name (the `-p` flag).
    pub package: String,
    /// Test name filter (the `-- {filter}` argument).
    pub filter: String,
}

/// A source code assertion.
#[derive(Debug, Clone, Deserialize)]
pub struct SourceAssertion {
    /// Relative file path to check.
    pub file: String,
    /// Strings that must be present in the file.
    pub contains: Option<Vec<String>>,
    /// Strings that must NOT be present.
    pub not_contains: Option<Vec<String>>,
    /// Regex pattern that must match.
    pub matches: Option<String>,
    /// Description of what this assertion verifies.
    pub description: Option<String>,
}

/// A command to run and verify.
#[derive(Debug, Clone, Deserialize)]
pub struct CommandAssertion {
    /// The command to execute (shell string).
    pub command: String,
    /// Expected exit code (default: 0).
    pub exit_code: Option<i32>,
    /// Strings that stdout must contain.
    pub stdout_contains: Option<Vec<String>>,
    /// Strings that stderr must contain.
    pub stderr_contains: Option<Vec<String>>,
    /// Description of what this assertion verifies.
    pub description: Option<String>,
}

/// Load and parse an `.eval.yaml` file into a map of AC ID → eval config.
pub fn load_eval_file(path: &str) -> error::Result<HashMap<String, AcEval>> {
    let content = fs::read_to_string(path).context(IoSnafu)?;
    let evals: HashMap<String, AcEval> =
        serde_yaml::from_str(&content).context(YamlSnafu { path })?;
    Ok(evals)
}
