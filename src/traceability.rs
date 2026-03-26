//! Traceability matrix generation.
//!
//! Generates `TRACEABILITY.md` from discovered scenarios and their eval configs,
//! mapping every AC to its feature file, evaluator assertions, and CI status.

use std::fmt::Write;
use std::fs;
use std::path::Path;

use snafu::ResultExt;

use crate::discovery::Scenario;
use crate::error::{self, IoSnafu};

/// Generate `TRACEABILITY.md` in the features directory.
pub fn generate(features_dir: &str, scenarios: &[Scenario]) -> error::Result<()> {
    let mut output = String::new();

    output.push_str("# BDD AC Traceability Matrix\n\n");
    output.push_str("| AC ID | Description | Feature File | Runtime Tests | Source Assertions | Status |\n");
    output.push_str("|---|---|---|---|---|---|\n");

    for scenario in scenarios {
        let (runtime_count, source_count, status) =
            scenario.eval.as_ref().map_or((0, 0, "missing eval"), |eval| {
                let rt = eval.runtime_tests.as_ref().map_or(0, Vec::len);
                let sa = eval.source_assertions.as_ref().map_or(0, Vec::len);
                let status = if rt > 0 || sa > 0 { "configured" } else { "skeleton" };
                (rt, sa, status)
            });

        let _ = writeln!(
            output,
            "| {} | {} | {} | {} | {} | {} |",
            scenario.ac_id, scenario.name, scenario.feature_file, runtime_count, source_count, status
        );
    }

    let _ = write!(
        output,
        "\n## Summary\n\n- **Total ACs**: {}\n- **With eval**: {}\n- **Missing eval**: {}\n",
        scenarios.len(),
        scenarios.iter().filter(|s| s.eval.is_some()).count(),
        scenarios.iter().filter(|s| s.eval.is_none()).count(),
    );

    let path = Path::new(features_dir).join("TRACEABILITY.md");
    fs::write(&path, output).context(IoSnafu)?;

    eprintln!("Wrote {}", path.display());
    Ok(())
}
