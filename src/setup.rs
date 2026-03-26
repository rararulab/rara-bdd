//! Project scaffolding for cucumber-rs BDD testing.
//!
//! Creates `features/` directory, modifies `Cargo.toml`, and generates
//! `tests/bdd.rs` + `tests/steps/mod.rs` for a working cucumber-rs setup.

use std::{fs, path::Path};

use snafu::ResultExt;

use crate::error::{self, IoSnafu};

/// What happened to `Cargo.toml` during setup.
#[derive(Debug, Clone, Copy)]
pub enum CargoTomlAction {
    Modified,
    AlreadyPresent,
    NotFound,
}

impl CargoTomlAction {
    const fn label(self) -> &'static str {
        match self {
            Self::Modified => "modified",
            Self::AlreadyPresent => "already_present",
            Self::NotFound => "not_found",
        }
    }
}

impl std::fmt::Display for CargoTomlAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str(self.label()) }
}

/// What happened to `CLAUDE.md` during setup.
#[derive(Debug, Clone, Copy)]
pub enum ClaudeMdAction {
    Created,
    Appended,
    AlreadyPresent,
}

impl ClaudeMdAction {
    const fn label(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Appended => "appended",
            Self::AlreadyPresent => "already_present",
        }
    }
}

impl std::fmt::Display for ClaudeMdAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str(self.label()) }
}

/// Summary of setup actions taken.
#[derive(Debug)]
pub struct SetupSummary {
    pub created_features_dir: bool,
    pub features_dir:         String,
    pub cargo_toml:           CargoTomlAction,
    pub created_bdd_rs:       bool,
    pub created_steps_mod:    bool,
    pub claude_md:            ClaudeMdAction,
}

/// Run the full project setup.
pub fn run_setup(features_dir: &str) -> error::Result<SetupSummary> {
    let created_features_dir = create_features_dir(features_dir)?;
    let cargo_toml = modify_cargo_toml()?;
    let created_bdd_rs = create_bdd_rs()?;
    let created_steps_mod = create_steps_mod()?;
    let claude_md = update_claude_md()?;

    Ok(SetupSummary {
        created_features_dir,
        features_dir: features_dir.to_string(),
        cargo_toml,
        created_bdd_rs,
        created_steps_mod,
        claude_md,
    })
}

/// Create `features/` directory if it doesn't exist.
fn create_features_dir(features_dir: &str) -> error::Result<bool> {
    let path = Path::new(features_dir);
    if path.is_dir() {
        return Ok(false);
    }
    fs::create_dir_all(path).context(IoSnafu)?;
    Ok(true)
}

/// Modify `Cargo.toml` to add cucumber-rs dev-dependencies and test target.
fn modify_cargo_toml() -> error::Result<CargoTomlAction> {
    let path = Path::new("Cargo.toml");
    if !path.exists() {
        return Ok(CargoTomlAction::NotFound);
    }

    let content = fs::read_to_string(path).context(IoSnafu)?;
    let mut doc = content.parse::<toml_edit::DocumentMut>().map_err(|e| {
        error::RaraBddError::CargoTomlParse {
            path:   "Cargo.toml".to_string(),
            reason: e.to_string(),
        }
    })?;

    let mut modified = false;

    // Ensure [dev-dependencies] section exists
    if !doc.contains_key("dev-dependencies") {
        doc["dev-dependencies"] = toml_edit::Item::Table(toml_edit::Table::new());
    }

    let dev_deps = doc["dev-dependencies"].as_table_mut().ok_or_else(|| {
        error::RaraBddError::CargoTomlParse {
            path:   "Cargo.toml".to_string(),
            reason: "[dev-dependencies] is not a table".to_string(),
        }
    })?;

    // Add cucumber dependency
    if !dev_deps.contains_key("cucumber") {
        let mut cucumber = toml_edit::InlineTable::new();
        cucumber.insert("version", "0.22".into());
        dev_deps["cucumber"] = toml_edit::value(cucumber);
        modified = true;
    }

    // Add tokio dependency
    if !dev_deps.contains_key("tokio") {
        let mut tokio = toml_edit::InlineTable::new();
        tokio.insert("version", "1".into());
        let mut features = toml_edit::Array::new();
        features.push("macros");
        features.push("rt-multi-thread");
        tokio.insert("features", toml_edit::Value::Array(features));
        dev_deps["tokio"] = toml_edit::value(tokio);
        modified = true;
    }

    // Add [[test]] section for bdd
    let has_bdd_test = doc
        .as_table()
        .get("test")
        .and_then(|v| v.as_array_of_tables())
        .is_some_and(|arr| {
            arr.iter()
                .any(|t| t.get("name").and_then(|n| n.as_str()) == Some("bdd"))
        });

    if !has_bdd_test {
        let mut test_table = toml_edit::Table::new();
        test_table.insert("name", toml_edit::value("bdd"));
        test_table.insert("harness", toml_edit::value(false));

        if let Some(arr) = doc
            .as_table_mut()
            .get_mut("test")
            .and_then(|v| v.as_array_of_tables_mut())
        {
            arr.push(test_table);
        } else {
            let mut arr = toml_edit::ArrayOfTables::new();
            arr.push(test_table);
            doc.as_table_mut()
                .insert("test", toml_edit::Item::ArrayOfTables(arr));
        }
        modified = true;
    }

    if !modified {
        return Ok(CargoTomlAction::AlreadyPresent);
    }

    fs::write(path, doc.to_string()).map_err(|e| error::RaraBddError::CargoTomlWrite {
        path:   "Cargo.toml".to_string(),
        source: e,
    })?;

    Ok(CargoTomlAction::Modified)
}

/// Create `tests/bdd.rs` with World struct and main entry point.
fn create_bdd_rs() -> error::Result<bool> {
    let path = Path::new("tests/bdd.rs");
    if path.exists() {
        return Ok(false);
    }

    fs::create_dir_all("tests").context(IoSnafu)?;
    fs::write(
        path,
        r#"use cucumber::World;

mod steps;

#[derive(Debug, Default, World)]
pub struct TestWorld {
    // Add your test state fields here
}

#[tokio::main]
async fn main() {
    TestWorld::run("features").await;
}
"#,
    )
    .context(IoSnafu)?;

    Ok(true)
}

/// Create `tests/steps/mod.rs` skeleton.
fn create_steps_mod() -> error::Result<bool> {
    let path = Path::new("tests/steps/mod.rs");
    if path.exists() {
        return Ok(false);
    }

    fs::create_dir_all("tests/steps").context(IoSnafu)?;
    fs::write(
        path,
        "// Step definitions — auto-discovered by cucumber-rs.\n// Add new step modules here, or \
         run `rara-bdd generate`.\n",
    )
    .context(IoSnafu)?;

    Ok(true)
}

const CLAUDE_MD_MARKER: &str = "## BDD Testing (cucumber-rs)";

const CLAUDE_MD_SECTION: &str = r#"## BDD Testing (cucumber-rs)

This project uses [cucumber-rs](https://github.com/cucumber-rs/cucumber) for BDD acceptance testing,
scaffolded by [rara-bdd](https://github.com/rararulab/rara-bdd).

### Workflow

1. Write Gherkin scenarios in `features/*.feature`
2. Generate step skeletons: `rara-bdd generate`
3. Implement step definitions in `tests/steps/`
4. Run tests: `cargo test --test bdd`

### .feature file format

```gherkin
Feature: Describe the feature

  Scenario: Short description
    Given some precondition
    When an action is performed
    Then a verifiable outcome occurs
```

### Step definitions

Step definitions live in `tests/steps/<feature>_steps.rs`:

```rust
#[given("some precondition")]
async fn some_precondition(world: &mut TestWorld) {
    // setup code
}
```

### Commands

```bash
rara-bdd setup                # Set up cucumber-rs project skeleton
rara-bdd generate             # Generate step definition skeletons
rara-bdd generate --dry-run   # Preview without writing files
rara-bdd coverage             # Check for missing step definitions
rara-bdd list                 # List features and scenarios
cargo test --test bdd         # Run BDD tests
```

### Completion criteria

A task is done when:
- `rara-bdd coverage` — zero missing steps
- `cargo test --test bdd` — all scenarios pass
"#;

/// Update `CLAUDE.md` with BDD workflow instructions.
fn update_claude_md() -> error::Result<ClaudeMdAction> {
    let path = Path::new("CLAUDE.md");

    if !path.exists() {
        fs::write(path, format!("# CLAUDE.md\n\n{CLAUDE_MD_SECTION}")).context(IoSnafu)?;
        return Ok(ClaudeMdAction::Created);
    }

    let content = fs::read_to_string(path).context(IoSnafu)?;

    if content.contains(CLAUDE_MD_MARKER) {
        return Ok(ClaudeMdAction::AlreadyPresent);
    }

    // Remove old rara-bdd section if present
    let content = if content.contains("## BDD Testing (rara-bdd)") {
        remove_old_bdd_section(&content)
    } else {
        content
    };

    let updated = format!("{content}\n{CLAUDE_MD_SECTION}");
    fs::write(path, updated).context(IoSnafu)?;

    Ok(ClaudeMdAction::Appended)
}

/// Remove the old rara-bdd BDD section from CLAUDE.md content.
fn remove_old_bdd_section(content: &str) -> String {
    let marker = "## BDD Testing (rara-bdd)";
    let Some(start) = content.find(marker) else {
        return content.to_string();
    };

    let before = &content[..start];
    let rest = &content[start + marker.len()..];
    let end = rest
        .find("\n## ")
        .map_or(content.len(), |pos| start + marker.len() + pos);

    let after = &content[end..];
    format!("{}{}", before.trim_end(), after)
}
