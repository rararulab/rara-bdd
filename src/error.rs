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

    /// Cargo.toml parse error.
    #[snafu(display("failed to parse Cargo.toml at {path}: {reason}"))]
    CargoTomlParse { path: String, reason: String },

    /// Cargo.toml write error.
    #[snafu(display("failed to write Cargo.toml at {path}: {source}"))]
    CargoTomlWrite {
        path:   String,
        source: std::io::Error,
    },

    /// Step definition generation error.
    #[snafu(display("step generation failed: {message}"))]
    StepGeneration { message: String },

    /// Step definition parsing error.
    #[snafu(display("failed to parse step definitions in {path}: {reason}"))]
    StepParse { path: String, reason: String },
}

/// Convenience alias for `Result<T, RaraBddError>`.
pub type Result<T> = std::result::Result<T, RaraBddError>;
