use std::fs;

use assert_cmd::Command;
use predicates::prelude::predicate;
use tempfile::TempDir;

fn cmd() -> Command { Command::cargo_bin("rara-bdd").expect("binary should exist") }

/// Create a temp project with Cargo.toml and a .feature file.
fn setup_project(tmp: &TempDir) -> String {
    let project_dir = tmp.path();

    // Minimal Cargo.toml
    fs::write(
        project_dir.join("Cargo.toml"),
        r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    // Create src/lib.rs
    fs::create_dir_all(project_dir.join("src")).unwrap();
    fs::write(project_dir.join("src/lib.rs"), "").unwrap();

    // Create features directory with a .feature file
    let features_dir = project_dir.join("features");
    fs::create_dir_all(&features_dir).unwrap();
    fs::write(
        features_dir.join("login.feature"),
        r#"Feature: User login

  Scenario: Valid credentials
    Given a registered user "alice"
    When she logs in
    Then she sees the dashboard
"#,
    )
    .unwrap();

    project_dir.to_string_lossy().to_string()
}

#[test]
fn list_shows_scenarios_and_steps() {
    let tmp = TempDir::new().unwrap();
    let project_dir = setup_project(&tmp);
    let features_dir = format!("{project_dir}/features");

    cmd()
        .args(["list", "--features-dir", &features_dir])
        .assert()
        .success()
        .stdout(predicate::str::contains("Valid credentials"))
        .stdout(predicate::str::contains("Given"))
        .stdout(predicate::str::contains("logs in"));
}

#[test]
fn generate_creates_step_files() {
    let tmp = TempDir::new().unwrap();
    let project_dir = setup_project(&tmp);
    let features_dir = format!("{project_dir}/features");
    let steps_dir = format!("{project_dir}/tests/steps");

    cmd()
        .args([
            "generate",
            "--features-dir",
            &features_dir,
            "--steps-dir",
            &steps_dir,
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""steps_generated""#));

    // Verify step file was created
    let step_file = format!("{steps_dir}/login_steps.rs");
    assert!(
        std::path::Path::new(&step_file).exists(),
        "login_steps.rs should be created"
    );

    let content = fs::read_to_string(&step_file).unwrap();
    assert!(content.contains("#[given("));
    assert!(content.contains("#[when("));
    assert!(content.contains("#[then("));
    assert!(content.contains("todo!"));
}

#[test]
fn generate_dry_run_does_not_write() {
    let tmp = TempDir::new().unwrap();
    let project_dir = setup_project(&tmp);
    let features_dir = format!("{project_dir}/features");
    let steps_dir = format!("{project_dir}/tests/steps");

    cmd()
        .args([
            "generate",
            "--features-dir",
            &features_dir,
            "--steps-dir",
            &steps_dir,
            "--dry-run",
        ])
        .assert()
        .success();

    assert!(
        !std::path::Path::new(&format!("{steps_dir}/login_steps.rs")).exists(),
        "dry-run should not create files"
    );
}

#[test]
fn coverage_reports_missing_steps() {
    let tmp = TempDir::new().unwrap();
    let project_dir = setup_project(&tmp);
    let features_dir = format!("{project_dir}/features");
    let steps_dir = format!("{project_dir}/tests/steps");

    // No step definitions exist yet
    cmd()
        .args([
            "coverage",
            "--features-dir",
            &features_dir,
            "--steps-dir",
            &steps_dir,
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains(r#""missing_steps":3"#));
}

#[test]
fn coverage_passes_when_all_defined() {
    let tmp = TempDir::new().unwrap();
    let project_dir = setup_project(&tmp);
    let features_dir = format!("{project_dir}/features");
    let steps_dir = format!("{project_dir}/tests/steps");

    // Generate step definitions first
    cmd()
        .args([
            "generate",
            "--features-dir",
            &features_dir,
            "--steps-dir",
            &steps_dir,
        ])
        .assert()
        .success();

    // Now coverage should pass
    cmd()
        .args([
            "coverage",
            "--features-dir",
            &features_dir,
            "--steps-dir",
            &steps_dir,
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""missing_steps":0"#));
}

#[test]
fn setup_creates_project_skeleton() {
    let tmp = TempDir::new().unwrap();
    let project_dir = tmp.path();

    // Create minimal Cargo.toml
    fs::write(
        project_dir.join("Cargo.toml"),
        r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();
    fs::create_dir_all(project_dir.join("src")).unwrap();
    fs::write(project_dir.join("src/lib.rs"), "").unwrap();

    cmd()
        .current_dir(project_dir)
        .args(["setup"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""ok":true"#));

    // Verify files were created
    assert!(project_dir.join("features").is_dir());
    assert!(project_dir.join("tests/bdd.rs").exists());
    assert!(project_dir.join("tests/steps/mod.rs").exists());
    assert!(project_dir.join("CLAUDE.md").exists());

    // Verify Cargo.toml was modified
    let cargo_content = fs::read_to_string(project_dir.join("Cargo.toml")).unwrap();
    assert!(cargo_content.contains("cucumber"));
    assert!(cargo_content.contains("tokio"));
    assert!(cargo_content.contains("harness = false"));

    // Verify CLAUDE.md content
    let claude_content = fs::read_to_string(project_dir.join("CLAUDE.md")).unwrap();
    assert!(claude_content.contains("cucumber-rs"));
    assert!(claude_content.contains("rara-bdd generate"));
}

#[test]
fn setup_is_idempotent() {
    let tmp = TempDir::new().unwrap();
    let project_dir = tmp.path();

    fs::write(
        project_dir.join("Cargo.toml"),
        r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();
    fs::create_dir_all(project_dir.join("src")).unwrap();
    fs::write(project_dir.join("src/lib.rs"), "").unwrap();

    // Run setup twice
    cmd()
        .current_dir(project_dir)
        .args(["setup"])
        .assert()
        .success();
    cmd()
        .current_dir(project_dir)
        .args(["setup"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""cargo_toml":"already_present"#));
}

#[test]
fn list_fails_with_missing_features_dir() {
    cmd()
        .args(["list", "--features-dir", "/nonexistent/path"])
        .assert()
        .failure()
        .stdout(predicate::str::contains(r#""ok":false"#));
}
