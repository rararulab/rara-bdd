use std::path::Path;

use clap::Parser;
use colored::Colorize;
use rara_bdd::{
    cli::{Cli, Command},
    discovery, error, generate, setup, step_coverage,
};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        println!(
            "{}",
            serde_json::json!({"ok": false, "error": e.to_string()})
        );
        std::process::exit(1);
    }
}

#[allow(clippy::too_many_lines)]
fn run() -> error::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Setup { features_dir } => {
            let summary = setup::run_setup(&features_dir)?;
            println!(
                "{}",
                serde_json::json!({
                    "ok": true,
                    "action": "setup",
                    "created_features_dir": summary.created_features_dir,
                    "features_dir": summary.features_dir,
                    "cargo_toml": summary.cargo_toml.to_string(),
                    "created_bdd_rs": summary.created_bdd_rs,
                    "created_steps_mod": summary.created_steps_mod,
                    "claude_md": summary.claude_md.to_string(),
                })
            );
        }
        Command::Generate {
            features_dir,
            steps_dir,
            dry_run,
        } => {
            let scenarios = discovery::discover(&features_dir, None)?;
            let defined = step_coverage::find_defined_steps(Path::new(&steps_dir))?;
            let summary = generate::generate_steps(&scenarios, &steps_dir, &defined, dry_run)?;
            println!(
                "{}",
                serde_json::json!({
                    "ok": true,
                    "action": "generate",
                    "files_created": summary.files_created,
                    "steps_generated": summary.steps_generated,
                    "steps_skipped": summary.steps_skipped,
                })
            );
        }
        Command::Coverage {
            features_dir,
            steps_dir,
        } => {
            let scenarios = discovery::discover(&features_dir, None)?;
            let defined = step_coverage::find_defined_steps(Path::new(&steps_dir))?;
            let report = step_coverage::check_coverage(&scenarios, &defined);

            if report.missing_steps.is_empty() {
                println!(
                    "{}",
                    format!("All {} steps covered", report.total_steps)
                        .green()
                        .bold()
                );
            } else {
                eprintln!(
                    "{}\n",
                    format!(
                        "{} of {} steps missing definitions:",
                        report.missing_steps.len(),
                        report.total_steps
                    )
                    .yellow()
                    .bold()
                );
                for step in &report.missing_steps {
                    eprintln!(
                        "  {} {} {} ({})",
                        "MISSING".red(),
                        step.keyword.label().cyan(),
                        step.text,
                        step.feature_file.dimmed()
                    );
                }
            }

            println!(
                "{}",
                serde_json::json!({
                    "ok": report.missing_steps.is_empty(),
                    "action": "coverage",
                    "total_steps": report.total_steps,
                    "covered_steps": report.covered_steps,
                    "missing_steps": report.missing_steps.len(),
                })
            );

            if !report.missing_steps.is_empty() {
                std::process::exit(1);
            }
        }
        Command::List {
            features_dir,
            filter,
        } => {
            let scenarios = discovery::discover(&features_dir, filter.as_deref())?;

            for scenario in &scenarios {
                println!(
                    "\n{} {} ({})",
                    "Scenario:".bold(),
                    scenario.name,
                    scenario.feature_file.dimmed()
                );
                if !scenario.tags.is_empty() {
                    println!(
                        "  {}",
                        scenario
                            .tags
                            .iter()
                            .map(|t| format!("@{t}"))
                            .collect::<Vec<_>>()
                            .join(" ")
                            .cyan()
                    );
                }
                for step in &scenario.steps {
                    println!("    {} {}", step.keyword.label().green(), step.text);
                }
            }

            println!(
                "\n{}",
                format!(
                    "Total: {} scenarios, {} steps",
                    scenarios.len(),
                    scenarios.iter().map(|s| s.steps.len()).sum::<usize>()
                )
                .dimmed()
            );
        }
    }

    Ok(())
}
