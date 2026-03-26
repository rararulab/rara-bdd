//! Project scaffolding for rara-bdd integration.
//!
//! Creates the `features/` directory and appends BDD workflow
//! instructions to the target project's `CLAUDE.md`.

use std::{fs, path::Path};

use snafu::ResultExt;

use crate::error::{self, IoSnafu};

/// Instructions appended to the target project's `CLAUDE.md`.
const CLAUDE_MD_SECTION: &str = r"
## BDD Testing (rara-bdd)

This project uses [rara-bdd](https://github.com/rararulab/rara-bdd) for acceptance testing.

### Workflow

1. Write Gherkin scenarios in `features/*.feature` with `@AC-XX` tags
2. Write `#[test] fn ac_XX_*()` functions matching each AC
3. Verify with `rara-bdd run`

### .feature file format

```gherkin
Feature: Describe the feature

  @AC-01
  Scenario: AC-01 Short description of expected behavior
    Given some precondition
    When an action is performed
    Then a verifiable outcome occurs
```

### Test naming convention

`@AC-XX` tag maps to test functions prefixed with `ac_XX_`:

```rust
#[test]
fn ac_01_short_description() {
    // Verifies AC-01
}
```

### Verification commands

```bash
rara-bdd run                  # Run all BDD scenarios
rara-bdd run --filter AC-01   # Run specific AC
rara-bdd coverage             # Check for uncovered ACs
rara-bdd list                 # List AC ↔ test mapping
rara-bdd trace                # Generate TRACEABILITY.md
```

### Completion criteria

A task is done when:
- `rara-bdd run` — all PASS
- `rara-bdd coverage` — zero uncovered ACs
";

/// Marker used to detect whether the BDD section already exists.
const MARKER: &str = "## BDD Testing (rara-bdd)";

/// Run the setup scaffolding in the current directory.
///
/// Returns a JSON-serializable summary of actions taken.
pub fn run_setup(features_dir: &str) -> error::Result<SetupSummary> {
    let features_path = Path::new(features_dir);
    let claude_md_path = Path::new("CLAUDE.md");

    let created_features = if features_path.exists() {
        false
    } else {
        fs::create_dir_all(features_path).context(IoSnafu)?;
        true
    };

    let claude_md_action = if claude_md_path.exists() {
        let content = fs::read_to_string(claude_md_path).context(IoSnafu)?;
        if content.contains(MARKER) {
            ClaudeMdAction::AlreadyPresent
        } else {
            let mut new_content = content;
            new_content.push_str(CLAUDE_MD_SECTION);
            fs::write(claude_md_path, new_content).context(IoSnafu)?;
            ClaudeMdAction::Appended
        }
    } else {
        fs::write(claude_md_path, format!("# CLAUDE.md\n{CLAUDE_MD_SECTION}")).context(IoSnafu)?;
        ClaudeMdAction::Created
    };

    Ok(SetupSummary {
        created_features_dir: created_features,
        features_dir:         features_dir.to_string(),
        claude_md:            claude_md_action,
    })
}

/// What happened to `CLAUDE.md` during setup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaudeMdAction {
    /// Created a new `CLAUDE.md`.
    Created,
    /// Appended BDD section to existing `CLAUDE.md`.
    Appended,
    /// BDD section already present — no changes.
    AlreadyPresent,
}

/// Summary of setup actions, serializable to JSON.
#[derive(Debug, serde::Serialize)]
pub struct SetupSummary {
    /// Whether the `features/` directory was newly created.
    pub created_features_dir: bool,
    /// Path to the features directory.
    pub features_dir:         String,
    /// What happened to `CLAUDE.md`.
    pub claude_md:            ClaudeMdAction,
}
