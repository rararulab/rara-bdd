//! CLI command definitions.

use clap::{Parser, Subcommand};

/// rara-bdd — cucumber-rs scaffolding tool for AI agents.
///
/// Generates project skeletons and step definitions from Gherkin features.
#[derive(Parser)]
#[command(name = "rara-bdd", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// Available subcommands.
#[derive(Subcommand)]
pub enum Command {
    /// Set up cucumber-rs in the current project
    ///
    /// Creates features/ directory, modifies Cargo.toml, generates
    /// tests/bdd.rs and tests/steps/ for a working cucumber-rs setup.
    #[command(after_help = "\
EXAMPLES:
    rara-bdd setup
    rara-bdd setup --features-dir specs")]
    Setup {
        /// Path to features directory to create
        #[arg(long, default_value = "features")]
        features_dir: String,
    },

    /// Generate step definition skeletons from .feature files
    ///
    /// Parses Gherkin steps and generates `#[given]`/`#[when]`/`#[then]`
    /// async functions with todo!() bodies. Skips already-defined steps.
    #[command(after_help = "\
EXAMPLES:
    rara-bdd generate
    rara-bdd generate --dry-run
    rara-bdd generate --steps-dir tests/steps")]
    Generate {
        /// Path to features directory
        #[arg(long, default_value = "features")]
        features_dir: String,

        /// Path to step definitions directory
        #[arg(long, default_value = "tests/steps")]
        steps_dir: String,

        /// Print what would be generated without writing files
        #[arg(long)]
        dry_run: bool,
    },

    /// Check step definition coverage
    ///
    /// Reports which Gherkin steps lack matching step definitions.
    #[command(after_help = "\
EXAMPLES:
    rara-bdd coverage
    rara-bdd coverage --steps-dir tests/steps")]
    Coverage {
        /// Path to features directory
        #[arg(long, default_value = "features")]
        features_dir: String,

        /// Path to step definitions directory
        #[arg(long, default_value = "tests/steps")]
        steps_dir: String,
    },

    /// List features, scenarios, and steps
    #[command(after_help = "\
EXAMPLES:
    rara-bdd list
    rara-bdd list --filter login")]
    List {
        /// Path to features directory
        #[arg(long, default_value = "features")]
        features_dir: String,

        /// Filter by tag or scenario name
        #[arg(long)]
        filter: Option<String>,
    },
}
