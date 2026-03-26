//! Traceability matrix generation.
//!
//! Generates `TRACEABILITY.md` from discovered scenarios and their eval
//! configs, mapping every AC to its feature file, evaluator assertions, and
//! quality verdict.

use std::{fmt::Write, fs, path::Path};

use snafu::ResultExt;

use crate::{
    discovery::Scenario,
    error::{self, IoSnafu},
    evaluator::quality,
};

/// Generate `TRACEABILITY.md` in the features directory.
pub fn generate(features_dir: &str, scenarios: &[Scenario]) -> error::Result<()> {
    let mut output = String::new();

    output.push_str("# BDD AC Traceability Matrix\n\n");
    output.push_str(
        "| AC ID | Description | Feature File | Runtime | Source | Commands | Quality |\n",
    );
    output.push_str("|---|---|---|---|---|---|---|\n");

    for scenario in scenarios {
        let (rt, sa, ca, verdict) =
            scenario
                .eval
                .as_ref()
                .map_or((0, 0, 0, "missing eval"), |eval| {
                    let rt = eval.runtime_tests.as_ref().map_or(0, Vec::len);
                    let sa = eval.source_assertions.as_ref().map_or(0, Vec::len);
                    let ca = eval.command_assertions.as_ref().map_or(0, Vec::len);
                    let (verdict, _) = quality::analyze(eval);
                    (rt, sa, ca, verdict.label())
                });

        let _ = writeln!(
            output,
            "| {} | {} | {} | {} | {} | {} | {} |",
            scenario.ac_id, scenario.name, scenario.feature_file, rt, sa, ca, verdict
        );
    }

    let verified = scenarios
        .iter()
        .filter(|s| {
            s.eval
                .as_ref()
                .is_some_and(|e| quality::analyze(e).0 == quality::Verdict::Verified)
        })
        .count();

    let _ = write!(
        output,
        "\n## Summary\n\n- **Total ACs**: {}\n- **Verified**: {}\n- **Skeleton/Weak**: {}\n- \
         **Missing eval**: {}\n",
        scenarios.len(),
        verified,
        scenarios.len() - verified - scenarios.iter().filter(|s| s.eval.is_none()).count(),
        scenarios.iter().filter(|s| s.eval.is_none()).count(),
    );

    let path = Path::new(features_dir).join("TRACEABILITY.md");
    fs::write(&path, output).context(IoSnafu)?;

    eprintln!("Wrote {}", path.display());
    Ok(())
}
