//! Step definition skeleton generator.
//!
//! Reads `.feature` files and generates `#[given]`/`#[when]`/`#[then]`
//! async function skeletons with `todo!()` bodies.

use std::{collections::HashSet, fmt::Write, fs, path::Path};

use snafu::ResultExt;

use crate::{
    discovery::{Scenario, Step},
    error::{self, IoSnafu},
    step_coverage::{self, DefinedStep},
};

/// Summary of what was generated.
#[derive(Debug)]
pub struct GenerateSummary {
    /// Files that were created.
    pub files_created:   Vec<String>,
    /// Number of step functions generated.
    pub steps_generated: usize,
    /// Number of steps skipped (already defined).
    pub steps_skipped:   usize,
}

/// Generate step definition skeletons from discovered scenarios.
///
/// Groups steps by feature file, generates one `<feature>_steps.rs` per
/// feature, and updates `tests/steps/mod.rs` with module declarations.
pub fn generate_steps(
    scenarios: &[Scenario],
    steps_dir: &str,
    defined: &[DefinedStep],
    dry_run: bool,
) -> error::Result<GenerateSummary> {
    let steps_path = Path::new(steps_dir);

    // Group scenarios by feature file stem
    let mut by_feature: std::collections::BTreeMap<String, Vec<&Step>> =
        std::collections::BTreeMap::new();

    let mut seen = HashSet::new();
    for scenario in scenarios {
        let stem = feature_file_stem(&scenario.feature_file);
        for step in &scenario.steps {
            let key = (step.keyword, step.text.clone());
            if seen.insert(key) {
                by_feature.entry(stem.clone()).or_default().push(step);
            }
        }
    }

    let mut summary = GenerateSummary {
        files_created:   Vec::new(),
        steps_generated: 0,
        steps_skipped:   0,
    };

    let mut new_modules = Vec::new();

    for (stem, steps) in &by_feature {
        let module_name = format!("{stem}_steps");
        let file_name = format!("{module_name}.rs");
        let file_path = steps_path.join(&file_name);

        // Filter out already-defined steps
        let new_steps: Vec<&&Step> = steps
            .iter()
            .filter(|s| !step_coverage::is_step_defined(s, defined))
            .collect();

        summary.steps_skipped += steps.len() - new_steps.len();

        if new_steps.is_empty() {
            continue;
        }

        let content = render_step_file(&new_steps);
        summary.steps_generated += new_steps.len();

        if dry_run {
            eprintln!("Would create: {}", file_path.display());
            for step in &new_steps {
                eprintln!("  {} {}", step.keyword, step.text);
            }
        } else {
            fs::create_dir_all(steps_path).context(IoSnafu)?;

            if file_path.exists() {
                // Append new steps to existing file
                let existing = fs::read_to_string(&file_path).context(IoSnafu)?;
                let appended = format!("{existing}\n{content}");
                fs::write(&file_path, appended).context(IoSnafu)?;
            } else {
                let header = format!(
                    "use cucumber::{{given, when, then}};\nuse crate::TestWorld;\n\n{content}"
                );
                fs::write(&file_path, header).context(IoSnafu)?;
                new_modules.push(module_name);
            }
        }

        summary
            .files_created
            .push(file_path.to_string_lossy().to_string());
    }

    // Update mod.rs with new module declarations
    if !dry_run && !new_modules.is_empty() {
        update_steps_mod(steps_path, &new_modules)?;
    }

    Ok(summary)
}

/// Render step functions for a single feature.
fn render_step_file(steps: &[&&Step]) -> String {
    let mut output = String::new();

    for step in steps {
        let expression = step_coverage::normalize_step_text(&step.text);
        let fn_name = step_text_to_fn_name(&step.text);
        let keyword = step.keyword.label().to_lowercase();
        let params = extract_params(&expression);

        // Choose between literal match and expr match
        let attr = if expression.contains('{') {
            format!(r#"#[{keyword}(expr = "{expression}")]"#)
        } else {
            format!(r#"#[{keyword}("{expression}")]"#)
        };

        let param_list = if params.is_empty() {
            "world: &mut TestWorld".to_string()
        } else {
            let extra: Vec<String> = params
                .iter()
                .enumerate()
                .map(|(i, p)| format!("p{i}: {p}"))
                .collect();
            format!("world: &mut TestWorld, {}", extra.join(", "))
        };

        let _ = write!(
            output,
            "{attr}\nasync fn {fn_name}({param_list}) {{\n    todo!(\"implement: {}\")\n}}\n\n",
            step.text
        );
    }

    output
}

/// Convert step text to a valid Rust function name.
fn step_text_to_fn_name(text: &str) -> String {
    let name: String = text
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect();
    let name = name.to_lowercase();

    // Collapse multiple underscores and trim
    let mut result = String::new();
    let mut prev_underscore = true; // skip leading underscores
    for ch in name.chars() {
        if ch == '_' {
            if !prev_underscore {
                result.push('_');
            }
            prev_underscore = true;
        } else {
            result.push(ch);
            prev_underscore = false;
        }
    }

    result.trim_end_matches('_').to_string()
}

/// Extract parameter types from a cucumber expression.
fn extract_params(expression: &str) -> Vec<&str> {
    let mut params = Vec::new();
    for cap in regex::Regex::new(r"\{(\w+)\}")
        .expect("valid regex")
        .captures_iter(expression)
    {
        match &cap[1] {
            "int" => params.push("i32"),
            "float" => params.push("f64"),
            _ => params.push("String"),
        }
    }
    params
}

/// Extract feature file stem (e.g., `auth/login.feature` → `login`).
fn feature_file_stem(feature_file: &str) -> String {
    Path::new(feature_file).file_stem().map_or_else(
        || "unknown".to_string(),
        |s| s.to_string_lossy().to_string(),
    )
}

/// Add module declarations to `tests/steps/mod.rs`.
fn update_steps_mod(steps_dir: &Path, new_modules: &[String]) -> error::Result<()> {
    let mod_path = steps_dir.join("mod.rs");
    let existing = if mod_path.exists() {
        fs::read_to_string(&mod_path).context(IoSnafu)?
    } else {
        String::new()
    };

    let mut additions = String::new();
    for module in new_modules {
        let decl = format!("mod {module};");
        if !existing.contains(&decl) {
            use std::fmt::Write;
            let _ = writeln!(additions, "{decl}");
        }
    }

    if !additions.is_empty() {
        let updated = if existing.is_empty() {
            additions
        } else {
            format!("{existing}\n{additions}")
        };
        fs::write(&mod_path, updated).context(IoSnafu)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_name_from_step_text() {
        assert_eq!(step_text_to_fn_name("a hungry cat"), "a_hungry_cat");
        assert_eq!(
            step_text_to_fn_name(r#"a user named "alice""#),
            "a_user_named_alice"
        );
        assert_eq!(
            step_text_to_fn_name("she eats 3 cucumbers"),
            "she_eats_3_cucumbers"
        );
    }

    #[test]
    fn extract_params_from_expression() {
        assert_eq!(extract_params("a user named {string}"), vec!["String"]);
        assert_eq!(extract_params("she eats {int} cucumbers"), vec!["i32"]);
        assert_eq!(
            extract_params("{string} buys {int} items for {float}"),
            vec!["String", "i32", "f64"]
        );
        assert!(extract_params("a simple step").is_empty());
    }

    #[test]
    fn feature_stem_extraction() {
        assert_eq!(feature_file_stem("auth/login.feature"), "login");
        assert_eq!(feature_file_stem("simple.feature"), "simple");
    }
}
