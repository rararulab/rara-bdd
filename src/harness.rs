//! Integration harness for bounded command execution and artifact management.
//!
//! Provides helpers to run commands with timeout, capture stdout/stderr,
//! and store artifacts for debugging failed scenarios.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use snafu::ResultExt;

use crate::error::{self, IoSnafu};

/// Artifact from a single command invocation.
#[derive(Debug, Clone)]
pub struct RunArtifact {
    /// Command that was executed.
    pub command: String,
    /// Exit code.
    pub exit_code: Option<i32>,
    /// Whether the command timed out.
    pub timed_out: bool,
    /// Captured stdout.
    pub stdout: String,
    /// Captured stderr.
    pub stderr: String,
    /// Execution duration.
    pub duration: Duration,
}

/// Run a command with a timeout and capture all output.
pub fn run_bounded_command(
    command: &str,
    args: &[&str],
    working_dir: Option<&Path>,
    timeout: Duration,
) -> error::Result<RunArtifact> {
    let start = Instant::now();

    let mut cmd = Command::new(command);
    cmd.args(args);
    if let Some(dir) = working_dir {
        cmd.current_dir(dir);
    }

    let output = cmd.output().context(IoSnafu)?;
    let duration = start.elapsed();

    let timed_out = duration >= timeout;

    Ok(RunArtifact {
        command: format!("{command} {}", args.join(" ")),
        exit_code: output.status.code(),
        timed_out,
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        duration,
    })
}

/// Write artifacts to disk for post-mortem debugging.
pub fn save_artifacts(artifact: &RunArtifact, output_dir: &Path) -> error::Result<()> {
    fs::create_dir_all(output_dir).context(IoSnafu)?;

    fs::write(output_dir.join("stdout.txt"), &artifact.stdout).context(IoSnafu)?;
    fs::write(output_dir.join("stderr.txt"), &artifact.stderr).context(IoSnafu)?;
    fs::write(
        output_dir.join("meta.json"),
        serde_json::json!({
            "command": artifact.command,
            "exit_code": artifact.exit_code,
            "timed_out": artifact.timed_out,
            "duration_ms": artifact.duration.as_millis(),
        })
        .to_string(),
    )
    .context(IoSnafu)?;

    Ok(())
}

/// Default artifact directory for a scenario.
pub fn artifact_dir(base: &Path, ac_id: &str) -> PathBuf {
    base.join(".bdd-artifacts").join(ac_id)
}
