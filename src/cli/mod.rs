//! CLI command definitions.

use clap::{Parser, Subcommand, ValueEnum};

/// rara-bdd — BDD testing framework for rararulab projects.
///
/// Maps Gherkin @AC-XX tags to `#[test] fn ac_XX_*()` by naming convention.
#[derive(Parser)]
#[command(name = "rara-bdd", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// Available subcommands.
#[derive(Subcommand)]
pub enum Command {
    /// Run BDD scenarios — discover ACs, match to tests, execute, report
    #[command(after_help = "\
EXAMPLES:
    rara-bdd run
    rara-bdd run --filter AC-01
    rara-bdd run --features-dir ./features --report json
    rara-bdd run --package my-crate")]
    Run {
        /// Path to features directory
        #[arg(long, default_value = "features")]
        features_dir: String,

        /// Filter scenarios by AC ID, tag, or name
        #[arg(long)]
        filter: Option<String>,

        /// Report format
        #[arg(long, value_enum, default_value_t = ReportFormat::Terminal)]
        report: ReportFormat,

        /// Cargo package(s) to scan for tests (repeatable, default: all)
        #[arg(long)]
        package: Vec<String>,
    },

    /// List discovered ACs and their matched tests
    #[command(after_help = "\
EXAMPLES:
    rara-bdd list
    rara-bdd list --features-dir ./features
    rara-bdd list --filter auth")]
    List {
        /// Path to features directory
        #[arg(long, default_value = "features")]
        features_dir: String,

        /// Filter scenarios by AC ID, tag, or name
        #[arg(long)]
        filter: Option<String>,

        /// Cargo package(s) to scan for tests (repeatable, default: all)
        #[arg(long)]
        package: Vec<String>,
    },

    /// Check coverage — report ACs without matching tests
    #[command(after_help = "\
EXAMPLES:
    rara-bdd coverage
    rara-bdd coverage --features-dir ./features")]
    Coverage {
        /// Path to features directory
        #[arg(long, default_value = "features")]
        features_dir: String,

        /// Cargo package(s) to scan for tests (repeatable, default: all)
        #[arg(long)]
        package: Vec<String>,
    },

    /// Generate or update the TRACEABILITY.md matrix
    #[command(after_help = "\
EXAMPLES:
    rara-bdd trace
    rara-bdd trace --features-dir ./features")]
    Trace {
        /// Path to features directory
        #[arg(long, default_value = "features")]
        features_dir: String,

        /// Cargo package(s) to scan for tests (repeatable, default: all)
        #[arg(long)]
        package: Vec<String>,
    },

    /// Set up rara-bdd in the current project
    ///
    /// Creates the features/ directory and adds BDD workflow instructions
    /// to CLAUDE.md so AI agents know how to use the framework.
    #[command(after_help = "\
EXAMPLES:
    rara-bdd setup
    rara-bdd setup --features-dir specs")]
    Setup {
        /// Path to features directory to create
        #[arg(long, default_value = "features")]
        features_dir: String,
    },
}

/// Report output format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum ReportFormat {
    /// Colored terminal output
    #[default]
    Terminal,
    /// Machine-readable JSON
    Json,
    /// Agent-readable markdown
    Markdown,
}
