//! Application-level error types.

use snafu::Snafu;

/// Top-level error for rara-bdd operations.
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum RaraBddError {
    /// IO error (file reads, process spawning).
    #[snafu(display("IO error: {source}"))]
    Io { source: std::io::Error },

    /// JSON serialization/deserialization error.
    #[snafu(display("JSON error: {source}"))]
    Json { source: serde_json::Error },

    /// YAML parsing error (eval DSL files).
    #[snafu(display("YAML parse error in {path}: {source}"))]
    Yaml {
        path:   String,
        source: serde_yaml::Error,
    },

    /// Gherkin parsing error.
    #[snafu(display("Gherkin parse error in {path}: {reason}"))]
    Gherkin { path: String, reason: String },

    /// Feature directory not found.
    #[snafu(display("features directory not found: {path}"))]
    FeaturesNotFound { path: String },

    /// Eval file not found for a feature.
    #[snafu(display("eval file not found: {path}"))]
    EvalNotFound { path: String },

    /// Runtime test execution failed.
    #[snafu(display("runtime test failed: {message}"))]
    RuntimeTest { message: String },

    /// Source assertion failed.
    #[snafu(display("source assertion failed: {message}"))]
    SourceAssertion { message: String },

    /// Command assertion failed.
    #[snafu(display("command assertion failed: {message}"))]
    CommandAssertion { message: String },
}

/// Convenience alias for `Result<T, RaraBddError>`.
pub type Result<T> = std::result::Result<T, RaraBddError>;
