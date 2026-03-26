//! Assertion quality analysis.
//!
//! Detects skeleton and weak assertions so the tool refuses to report
//! false confidence.

use super::loader::AcEval;

/// Minimum length for a `contains` pattern to not be flagged as weak.
const MIN_PATTERN_LEN: usize = 8;

/// Quality verdict for a scenario's eval config.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// Has `runtime_tests` or `command_assertions` — real behavioral
    /// verification.
    Verified,
    /// Only `source_assertions`, no behavioral tests — structural check only.
    Skeleton,
    /// Has weak patterns (too short) that match almost anything.
    Weak,
}

impl Verdict {
    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Skeleton => "skeleton",
            Self::Weak => "weak",
        }
    }
}

/// Analyze the quality of an eval config.
pub fn analyze(eval: &AcEval) -> (Verdict, Vec<String>) {
    let mut warnings: Vec<String> = Vec::new();

    let has_runtime = eval.runtime_tests.as_ref().is_some_and(|v| !v.is_empty());
    let has_command = eval
        .command_assertions
        .as_ref()
        .is_some_and(|v| !v.is_empty());
    let has_source = eval
        .source_assertions
        .as_ref()
        .is_some_and(|v| !v.is_empty());

    // Check for weak contains patterns
    let mut has_weak_pattern = false;
    if let Some(assertions) = &eval.source_assertions {
        for assertion in assertions {
            if let Some(patterns) = &assertion.contains {
                for pattern in patterns {
                    if pattern.len() < MIN_PATTERN_LEN {
                        has_weak_pattern = true;
                        warnings.push(format!(
                            "contains pattern '{}' is too short ({} chars < {MIN_PATTERN_LEN}) — \
                             matches almost anything",
                            pattern,
                            pattern.len()
                        ));
                    }
                }
            }
        }
    }

    if has_weak_pattern {
        return (Verdict::Weak, warnings);
    }

    if !has_runtime && !has_command {
        if has_source {
            warnings.push(
                "only source_assertions — add runtime_tests or command_assertions for behavioral \
                 verification"
                    .to_string(),
            );
        }
        return (Verdict::Skeleton, warnings);
    }

    (Verdict::Verified, warnings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evaluator::loader::{AcEval, CommandAssertion, RuntimeTest, SourceAssertion};

    #[test]
    fn verified_with_runtime_test() {
        let eval = AcEval {
            description:        None,
            runtime_tests:      Some(vec![RuntimeTest {
                package: "my-crate".into(),
                filter:  "test_fn".into(),
            }]),
            source_assertions:  None,
            command_assertions: None,
        };
        let (verdict, warnings) = analyze(&eval);
        assert_eq!(verdict, Verdict::Verified);
        assert!(warnings.is_empty());
    }

    #[test]
    fn skeleton_with_only_source() {
        let eval = AcEval {
            description:        None,
            runtime_tests:      None,
            source_assertions:  Some(vec![SourceAssertion {
                file:         "src/lib.rs".into(),
                contains:     Some(vec!["pub struct Config".into()]),
                not_contains: None,
                matches:      None,
                description:  None,
            }]),
            command_assertions: None,
        };
        let (verdict, _) = analyze(&eval);
        assert_eq!(verdict, Verdict::Skeleton);
    }

    #[test]
    fn weak_with_short_pattern() {
        let eval = AcEval {
            description:        None,
            runtime_tests:      None,
            source_assertions:  Some(vec![SourceAssertion {
                file:         "src/lib.rs".into(),
                contains:     Some(vec!["Exit".into()]),
                not_contains: None,
                matches:      None,
                description:  None,
            }]),
            command_assertions: None,
        };
        let (verdict, warnings) = analyze(&eval);
        assert_eq!(verdict, Verdict::Weak);
        assert!(!warnings.is_empty());
    }

    #[test]
    fn verified_with_command_assertion() {
        let eval = AcEval {
            description:        None,
            runtime_tests:      None,
            source_assertions:  None,
            command_assertions: Some(vec![CommandAssertion {
                command:         "cargo run -- list".into(),
                exit_code:       Some(0),
                stdout_contains: None,
                stderr_contains: None,
                description:     None,
            }]),
        };
        let (verdict, _) = analyze(&eval);
        assert_eq!(verdict, Verdict::Verified);
    }
}
