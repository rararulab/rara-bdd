//! CLI command definitions.

use clap::{Parser, Subcommand, ValueEnum};

/// rara-bdd — BDD testing framework for rararulab projects.
#[derive(Parser)]
#[command(name = "rara-bdd", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// Available subcommands.
#[derive(Subcommand)]
pub enum Command {
    /// Run BDD scenarios and evaluate acceptance criteria
    #[command(after_help = "\
EXAMPLES:
    rara-bdd run
    rara-bdd run --filter AC-01
    rara-bdd run --features-dir ./features --report json
    rara-bdd run --mock")]
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

        /// CI-safe mode (skip external dependencies)
        #[arg(long)]
        r#mock: bool,
    },

    /// List discovered BDD scenarios without running them
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
    },

    /// Validate .eval.yaml files (schema check, no execution)
    #[command(after_help = "\
EXAMPLES:
    rara-bdd validate
    rara-bdd validate --features-dir ./features")]
    Validate {
        /// Path to features directory
        #[arg(long, default_value = "features")]
        features_dir: String,
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
