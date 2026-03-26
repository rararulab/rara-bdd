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

    /// Gherkin parsing error.
    #[snafu(display("Gherkin parse error in {path}: {reason}"))]
    Gherkin { path: String, reason: String },

    /// Feature directory not found.
    #[snafu(display("features directory not found: {path}"))]
    FeaturesNotFound { path: String },

    /// Test discovery failed (`cargo test -- --list`).
    #[snafu(display("test discovery failed: {message}"))]
    TestDiscovery { message: String },

    /// Test execution failed.
    #[snafu(display("test execution failed: {message}"))]
    TestExecution { message: String },
}

/// Convenience alias for `Result<T, RaraBddError>`.
pub type Result<T> = std::result::Result<T, RaraBddError>;
