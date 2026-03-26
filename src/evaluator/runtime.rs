//! Runtime test executor.
//!
//! Runs `cargo test` commands and command assertions via `std::process::Command`.

use std::process::Command;

use crate::error;
use crate::evaluator::loader::CommandAssertion;

/// Run a single `cargo test -p {package} -- {filter}` and assert it passes.
pub fn run_cargo_test(package: &str, filter: &str) -> error::Result<()> {
    let output = Command::new("cargo")
        .args(["test", "-p", package, "--", filter, "--exact"])
        .output()
        .map_err(|e| error::RaraBddError::RuntimeTest {
            message: format!("failed to spawn cargo test: {e}"),
        })?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(error::RaraBddError::RuntimeTest {
            message: format!(
                "cargo test -p {package} -- {filter} failed (exit {})\n{stderr}",
                output.status.code().unwrap_or(-1)
            ),
        })
    }
}

/// Run a command assertion and verify exit code + output patterns.
pub fn run_command_assertion(assertion: &CommandAssertion) -> error::Result<()> {
    let output = Command::new("sh")
        .args(["-c", &assertion.command])
        .output()
        .map_err(|e| error::RaraBddError::CommandAssertion {
            message: format!("failed to spawn command '{}': {e}", assertion.command),
        })?;

    let expected_exit = assertion.exit_code.unwrap_or(0);
    let actual_exit = output.status.code().unwrap_or(-1);

    if actual_exit != expected_exit {
        return Err(error::RaraBddError::CommandAssertion {
            message: format!(
                "command '{}' exited with {actual_exit}, expected {expected_exit}",
                assertion.command
            ),
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if let Some(patterns) = &assertion.stdout_contains {
        for pattern in patterns {
            if !stdout.contains(pattern) {
                return Err(error::RaraBddError::CommandAssertion {
                    message: format!(
                        "stdout of '{}' missing expected pattern: {pattern}",
                        assertion.command
                    ),
                });
            }
        }
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    if let Some(patterns) = &assertion.stderr_contains {
        for pattern in patterns {
            if !stderr.contains(pattern) {
                return Err(error::RaraBddError::CommandAssertion {
                    message: format!(
                        "stderr of '{}' missing expected pattern: {pattern}",
                        assertion.command
                    ),
                });
            }
        }
    }

    Ok(())
}
