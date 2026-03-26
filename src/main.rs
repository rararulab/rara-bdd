use clap::Parser;
use rara_bdd::{
    cli::{Cli, Command},
    discovery, error, evaluator, reporter, traceability,
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
            r#mock,
            strict,
        } => {
            let scenarios = discovery::discover(&features_dir, filter.as_deref())?;
            let results = evaluator::run_suite(&scenarios, r#mock)?;
            reporter::report(&results, report);

            if !results.all_passed() {
                std::process::exit(1);
            }

            if strict && !results.all_verified() {
                eprintln!(
                    "Strict mode: {} skeleton, {} weak — all ACs must have behavioral tests",
                    results.skeleton_count(),
                    results.weak_count()
                );
                std::process::exit(1);
            }
        }
        Command::List {
            features_dir,
            filter,
        } => {
            let scenarios = discovery::discover(&features_dir, filter.as_deref())?;
            reporter::list_scenarios(&scenarios);
        }
        Command::Validate { features_dir } => {
            let result = evaluator::validate(&features_dir)?;
            println!(
                "{}",
                serde_json::json!({
                    "ok": result.errors.is_empty(),
                    "action": "validate",
                    "features": result.feature_count,
                    "evals": result.eval_count,
                    "errors": result.errors,
                    "warnings": result.warnings,
                })
            );
        }
        Command::Trace { features_dir } => {
            let scenarios = discovery::discover(&features_dir, None)?;
            traceability::generate(&features_dir, &scenarios)?;
            println!(
                "{}",
                serde_json::json!({
                    "ok": true,
                    "action": "trace",
                    "scenarios": scenarios.len(),
                })
            );
        }
    }

    Ok(())
}
