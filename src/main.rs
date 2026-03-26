use clap::Parser;
use rara_bdd::{
    cli::{Cli, Command},
    discovery, error, matcher, reporter, runner, traceability,
};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::WARN.into()),
        )
        .init();

    if let Err(e) = run() {
        eprintln!("Error: {e}");
        println!(
            "{}",
            serde_json::json!({"ok": false, "error": e.to_string(), "suggestion": "check --help for usage"})
        );
        std::process::exit(1);
    }
}

fn run() -> error::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Run {
            features_dir,
            filter,
            report,
            package,
        } => {
            let scenarios = discovery::discover(&features_dir, filter.as_deref())?;
            let test_names = matcher::discover_tests(&package)?;
            let matched = matcher::match_scenarios(&scenarios, &test_names);
            let results = runner::run_suite(&matched)?;
            reporter::report(&results, report);

            if !results.all_passed() {
                std::process::exit(1);
            }
        }
        Command::List {
            features_dir,
            filter,
            package,
        } => {
            let scenarios = discovery::discover(&features_dir, filter.as_deref())?;
            let test_names = matcher::discover_tests(&package)?;
            let matched = matcher::match_scenarios(&scenarios, &test_names);
            reporter::list_matched(&matched);
        }
        Command::Coverage {
            features_dir,
            package,
        } => {
            let scenarios = discovery::discover(&features_dir, None)?;
            let test_names = matcher::discover_tests(&package)?;
            let matched = matcher::match_scenarios(&scenarios, &test_names);
            let summary = matcher::coverage_summary(&matched);

            let uncovered_ids: Vec<&str> = matched
                .iter()
                .filter(|m| !m.is_covered())
                .map(|m| m.scenario.ac_id.as_str())
                .collect();

            println!(
                "{}",
                serde_json::json!({
                    "ok": summary.uncovered == 0,
                    "action": "coverage",
                    "total": summary.total,
                    "covered": summary.covered,
                    "uncovered": summary.uncovered,
                    "uncovered_ids": uncovered_ids,
                })
            );

            if summary.uncovered > 0 {
                std::process::exit(1);
            }
        }
        Command::Trace {
            features_dir,
            package,
        } => {
            let scenarios = discovery::discover(&features_dir, None)?;
            let test_names = matcher::discover_tests(&package)?;
            let matched = matcher::match_scenarios(&scenarios, &test_names);
            let summary = matcher::coverage_summary(&matched);
            traceability::generate(&features_dir, &matched)?;
            println!(
                "{}",
                serde_json::json!({
                    "ok": true,
                    "action": "trace",
                    "total": summary.total,
                    "covered": summary.covered,
                    "uncovered": summary.uncovered,
                })
            );
        }
    }

    Ok(())
}
