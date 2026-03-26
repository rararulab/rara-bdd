//! Source code assertion checker.
//!
//! Reads source files and verifies they contain (or don't contain) expected patterns.

use std::fs;

use crate::error;
use crate::evaluator::loader::SourceAssertion;

/// Check a single source assertion against the file system.
pub fn check_source_assertion(assertion: &SourceAssertion) -> error::Result<()> {
    let content = fs::read_to_string(&assertion.file).map_err(|e| {
        error::RaraBddError::SourceAssertion {
            message: format!("cannot read '{}': {e}", assertion.file),
        }
    })?;

    // Check `contains` patterns
    if let Some(patterns) = &assertion.contains {
        for pattern in patterns {
            if !content.contains(pattern) {
                let desc = assertion
                    .description
                    .as_deref()
                    .unwrap_or("source assertion");
                return Err(error::RaraBddError::SourceAssertion {
                    message: format!(
                        "{desc}: '{}' missing expected pattern: {pattern}",
                        assertion.file
                    ),
                });
            }
        }
    }

    // Check `not_contains` patterns
    if let Some(patterns) = &assertion.not_contains {
        for pattern in patterns {
            if content.contains(pattern) {
                let desc = assertion
                    .description
                    .as_deref()
                    .unwrap_or("source assertion");
                return Err(error::RaraBddError::SourceAssertion {
                    message: format!(
                        "{desc}: '{}' contains forbidden pattern: {pattern}",
                        assertion.file
                    ),
                });
            }
        }
    }

    // Check `matches` regex
    if let Some(regex_str) = &assertion.matches {
        let re = regex::Regex::new(regex_str).map_err(|e| {
            error::RaraBddError::SourceAssertion {
                message: format!("invalid regex '{regex_str}': {e}"),
            }
        })?;

        if !re.is_match(&content) {
            let desc = assertion
                .description
                .as_deref()
                .unwrap_or("source assertion");
            return Err(error::RaraBddError::SourceAssertion {
                message: format!(
                    "{desc}: '{}' does not match regex: {regex_str}",
                    assertion.file
                ),
            });
        }
    }

    Ok(())
}
