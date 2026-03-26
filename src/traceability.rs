//! Traceability matrix generation.
//!
//! Generates `TRACEABILITY.md` mapping every AC to its feature file
//! and matched test functions.

use std::{fmt::Write, fs, path::Path};

use snafu::ResultExt;

use crate::{
    error::{self, IoSnafu},
    matcher::MatchedAc,
};

/// Generate `TRACEABILITY.md` in the features directory.
pub fn generate(features_dir: &str, matched: &[MatchedAc]) -> error::Result<()> {
    let mut output = String::new();

    output.push_str("# BDD AC Traceability Matrix\n\n");
    output.push_str("| AC ID | Scenario | Feature File | Tests | Coverage |\n");
    output.push_str("|---|---|---|---|---|\n");

    for m in matched {
        let tests_display = if m.tests.is_empty() {
            "\u{2014}".to_string()
        } else {
            m.tests
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        };

        let coverage = if m.is_covered() {
            "covered"
        } else {
            "uncovered"
        };

        let _ = writeln!(
            output,
            "| {} | {} | {} | {} | {} |",
            m.scenario.ac_id, m.scenario.name, m.scenario.feature_file, tests_display, coverage
        );
    }

    let covered = matched.iter().filter(|m| m.is_covered()).count();
    let uncovered = matched.len() - covered;

    let _ = write!(
        output,
        "\n## Summary\n\n- **Total ACs**: {}\n- **Covered**: {}\n- **Uncovered**: {}\n",
        matched.len(),
        covered,
        uncovered,
    );

    let path = Path::new(features_dir).join("TRACEABILITY.md");
    fs::write(&path, output).context(IoSnafu)?;

    eprintln!("Wrote {}", path.display());
    Ok(())
}
