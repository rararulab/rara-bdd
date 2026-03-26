mod common;

use assert_cmd::Command;
use predicates::prelude::predicate;
use std::fs;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("rara-bdd").expect("binary should exist")
}

/// Create a temp dir with a minimal feature + eval setup.
fn setup_features(tmp: &TempDir) -> String {
    let features_dir = tmp.path().join("features");
    fs::create_dir_all(&features_dir).unwrap();

    fs::write(
        features_dir.join("example.feature"),
        r"@example
Feature: Example feature
  @AC-01
  Scenario: AC-01 Basic check
    Given a configured system
    When the check runs
    Then it passes
",
    )
    .unwrap();

    fs::write(
        features_dir.join("example.eval.yaml"),
        r#"AC-01:
  description: "Basic check"
  source_assertions:
    - file: Cargo.toml
      contains:
        - "rara-bdd"
      description: "Project name is rara-bdd"
"#,
    )
    .unwrap();

    features_dir.to_string_lossy().to_string()
}

#[test]
fn list_shows_scenarios() {
    let tmp = TempDir::new().unwrap();
    let features_dir = setup_features(&tmp);

    cmd()
        .args(["list", "--features-dir", &features_dir])
        .assert()
        .success()
        .stdout(predicate::str::contains("AC-01"));
}

#[test]
fn run_passes_with_valid_source_assertion() {
    let tmp = TempDir::new().unwrap();
    let features_dir = setup_features(&tmp);

    cmd()
        .args(["run", "--features-dir", &features_dir, "--report", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""ok":true"#));
}

#[test]
fn validate_checks_eval_files() {
    let tmp = TempDir::new().unwrap();
    let features_dir = setup_features(&tmp);

    cmd()
        .args(["validate", "--features-dir", &features_dir])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""ok":true"#));
}

#[test]
fn run_fails_with_missing_features_dir() {
    cmd()
        .args(["run", "--features-dir", "/nonexistent/path"])
        .assert()
        .failure()
        .stdout(predicate::str::contains(r#""ok":false"#));
}

#[test]
fn trace_generates_traceability_md() {
    let tmp = TempDir::new().unwrap();
    let features_dir = setup_features(&tmp);

    cmd()
        .args(["trace", "--features-dir", &features_dir])
        .assert()
        .success();

    let trace_path = tmp.path().join("features/TRACEABILITY.md");
    assert!(trace_path.exists());
    let content = fs::read_to_string(trace_path).unwrap();
    assert!(content.contains("AC-01"));
}
