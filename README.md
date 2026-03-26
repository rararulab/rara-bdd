# rara-bdd

BDD testing framework for rararulab Rust projects. Parses Gherkin `.feature` files and evaluates acceptance criteria via a declarative YAML DSL (`.eval.yaml`).

Inspired by [ralph-orchestrator](https://github.com/rararulab/rara)'s hooks BDD system.

## How It Works

rara-bdd uses a three-layer architecture:

```
.feature (Gherkin)     →    .eval.yaml (YAML DSL)     →    Evaluator (runtime)
───────────────────         ──────────────────────         ─────────────────────
Describes WHAT to test      Declares HOW to verify         Executes assertions
in human-readable BDD       each AC with declarative       and reports results
scenarios                   assertion configs
```

**Execution flow:**

1. **Discovery** — Recursively scans `features/` for `.feature` files, parses Gherkin via `cucumber-rs`, extracts scenarios with `@AC-XX` tags
2. **Pairing** — For each `.feature` file, loads the matching `.eval.yaml` (same name, e.g., `login.feature` → `login.eval.yaml`)
3. **Evaluation** — For each scenario's AC ID, runs the assertion pipeline in order: `runtime_tests` → `source_assertions` → `command_assertions`
4. **Reporting** — Outputs results in the chosen format (terminal, JSON, or markdown)

If any assertion in the pipeline fails, the scenario is marked as failed and the remaining assertions for that AC are skipped.

## Installation

```bash
cargo install --git https://github.com/rararulab/rara-bdd
```

Or build from source:

```bash
git clone https://github.com/rararulab/rara-bdd
cd rara-bdd
cargo build --release
```

## Quick Start

### 1. Create a features directory

```
my-project/
├── features/
│   ├── auth/
│   │   ├── login.feature        # Gherkin scenarios
│   │   └── login.eval.yaml      # Evaluator config
│   └── TRACEABILITY.md          # Auto-generated
├── src/
└── Cargo.toml
```

### 2. Write a `.feature` file

```gherkin
@auth @login
Feature: User login
  @AC-01
  Scenario: AC-01 Valid credentials accepted
    Given a user with valid credentials
    When login is attempted
    Then access is granted

  @AC-02
  Scenario: AC-02 Invalid credentials rejected
    Given a user with invalid credentials
    When login is attempted
    Then access is denied with error message
```

**Conventions:**
- Each scenario MUST have an `@AC-XX` tag (numeric, e.g., `@AC-01`, `@AC-123`)
- The scenario name SHOULD start with the AC ID for readability
- Feature-level tags (e.g., `@auth`) are inherited by all scenarios in the feature
- Scenarios without `@AC-XX` tags get an auto-generated ID like `UNTAGGED-scenario-name`

### 3. Write a `.eval.yaml` file

The eval file MUST have the same stem as the feature file (`login.feature` → `login.eval.yaml`). Each top-level key is an AC ID mapping to its assertion config:

```yaml
AC-01:
  description: "Valid credentials accepted"
  runtime_tests:
    - package: my-project
      filter: test_login_valid_credentials
  source_assertions:
    - file: src/auth.rs
      contains:
        - "pub fn login"
      description: "Login function exists"

AC-02:
  description: "Invalid credentials rejected"
  command_assertions:
    - command: "cargo run -- login --user bad --pass wrong"
      exit_code: 1
      stdout_contains:
        - '"ok":false'
      description: "CLI rejects invalid credentials"
```

### 4. Run the suite

```bash
# Colored terminal output (default)
rara-bdd run --features-dir features

# JSON for CI pipelines
rara-bdd run --features-dir features --report json

# Markdown for agent consumption
rara-bdd run --features-dir features --report markdown
```

## CLI Reference

### `rara-bdd run`

Execute BDD scenarios and evaluate acceptance criteria.

```bash
rara-bdd run [OPTIONS]
```

| Flag | Default | Description |
|------|---------|-------------|
| `--features-dir <PATH>` | `features` | Path to the features directory |
| `--filter <STRING>` | *(none)* | Filter by AC ID, tag, or scenario name (case-insensitive substring match) |
| `--report <FORMAT>` | `terminal` | Output format: `terminal`, `json`, `markdown` |
| `--mock` | `false` | CI-safe mode — skip external dependencies |

**Examples:**

```bash
# Run all scenarios
rara-bdd run

# Run only auth-related scenarios
rara-bdd run --filter auth

# Run a specific AC
rara-bdd run --filter AC-01

# JSON output for CI
rara-bdd run --features-dir ./features --report json

# CI-safe mode (skip runtime tests that need external services)
rara-bdd run --mock
```

**JSON output shape:**

```json
{
  "ok": true,
  "action": "bdd-run",
  "passed": 5,
  "failed": 0,
  "total": 5,
  "scenarios": [
    {
      "ac_id": "AC-01",
      "scenario": "AC-01 Valid credentials accepted",
      "feature_file": "auth/login.feature",
      "passed": true,
      "message": "AC-01: acceptance criterion verified green"
    }
  ]
}
```

**Exit codes:**
- `0` — all scenarios passed
- `1` — one or more scenarios failed, or an error occurred

### `rara-bdd list`

List discovered scenarios without running them.

```bash
rara-bdd list [OPTIONS]
```

| Flag | Default | Description |
|------|---------|-------------|
| `--features-dir <PATH>` | `features` | Path to the features directory |
| `--filter <STRING>` | *(none)* | Filter by AC ID, tag, or scenario name |

**Examples:**

```bash
rara-bdd list
rara-bdd list --features-dir ./features
rara-bdd list --filter auth
```

### `rara-bdd validate`

Check `.eval.yaml` files for schema errors without executing any assertions. Useful for catching typos before a full run.

```bash
rara-bdd validate [OPTIONS]
```

| Flag | Default | Description |
|------|---------|-------------|
| `--features-dir <PATH>` | `features` | Path to the features directory |

**Examples:**

```bash
rara-bdd validate
rara-bdd validate --features-dir ./features
```

**JSON output:**

```json
{
  "ok": true,
  "action": "validate",
  "features": 3,
  "evals": 3,
  "errors": []
}
```

### `rara-bdd trace`

Generate a `TRACEABILITY.md` matrix in the features directory, mapping every AC to its feature file, evaluator assertions, and status.

```bash
rara-bdd trace [OPTIONS]
```

| Flag | Default | Description |
|------|---------|-------------|
| `--features-dir <PATH>` | `features` | Path to the features directory |

**Examples:**

```bash
rara-bdd trace
rara-bdd trace --features-dir ./features
```

**Generated matrix example:**

```markdown
# BDD AC Traceability Matrix

| AC ID | Description | Feature File | Runtime Tests | Source Assertions | Status |
|---|---|---|---|---|---|
| AC-01 | Valid credentials accepted | auth/login.feature | 1 | 1 | configured |
| AC-02 | Invalid credentials rejected | auth/login.feature | 1 | 0 | configured |

## Summary

- **Total ACs**: 2
- **With eval**: 2
- **Missing eval**: 0
```

## YAML Eval DSL Reference

The `.eval.yaml` file is a YAML dictionary where each key is an AC ID and each value defines how to verify that acceptance criterion.

### Top-Level Structure

```yaml
AC-XX:
  description: "Human-readable description (optional)"
  runtime_tests:       # Run cargo test commands
    - ...
  source_assertions:   # Check file contents
    - ...
  command_assertions:  # Run shell commands and check output
    - ...
```

All three assertion types are optional. An AC with no assertions will still pass (skeleton state). The evaluator runs them in order: `runtime_tests` → `source_assertions` → `command_assertions`. On first failure, remaining assertions are skipped.

### `runtime_tests`

Run `cargo test` and assert it passes.

```yaml
runtime_tests:
  - package: my-crate        # Required: cargo package name (-p flag)
    filter: test_func_name    # Required: test name filter (-- {filter} --exact)
```

**What happens:** Executes `cargo test -p {package} -- {filter} --exact`. Passes if exit code is 0.

**Example:**

```yaml
AC-01:
  description: "Login function handles valid credentials"
  runtime_tests:
    - package: auth-service
      filter: test_login_valid
    - package: auth-service
      filter: test_login_returns_token
```

### `source_assertions`

Read a file and check its contents against patterns.

```yaml
source_assertions:
  - file: path/to/file.rs     # Required: relative path from project root
    contains:                  # Optional: strings that MUST be present
      - "pub fn login"
      - "impl AuthService"
    not_contains:              # Optional: strings that MUST NOT be present
      - "unsafe"
      - "unwrap()"
    matches: "fn login\\(.*\\)"  # Optional: regex that must match somewhere
    description: "Describe what this checks"  # Optional
```

**Fields:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `file` | string | yes | Relative file path from project root |
| `contains` | string[] | no | Substrings that must appear in the file |
| `not_contains` | string[] | no | Substrings that must NOT appear |
| `matches` | string | no | Regex pattern that must match (uses Rust `regex` crate syntax) |
| `description` | string | no | Human-readable description for error messages |

**What happens:** Reads the file, then checks each assertion in order: `contains` → `not_contains` → `matches`. First failure stops evaluation.

**Example with all fields:**

```yaml
AC-05:
  description: "Auth module uses snafu, not thiserror"
  source_assertions:
    - file: src/error.rs
      contains:
        - "use snafu"
        - "#[derive(Debug, Snafu)]"
      not_contains:
        - "thiserror"
      matches: "pub enum \\w+Error"
      description: "Error types use snafu derive pattern"
```

### `command_assertions`

Run a shell command and verify its exit code and output.

```yaml
command_assertions:
  - command: "cargo run -- list"     # Required: shell command string
    exit_code: 0                     # Optional: expected exit code (default: 0)
    stdout_contains:                 # Optional: patterns in stdout
      - '"ok":true'
    stderr_contains:                 # Optional: patterns in stderr
      - "Loaded config"
    description: "List command works"  # Optional
```

**Fields:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `command` | string | yes | Shell command (executed via `sh -c`) |
| `exit_code` | integer | no | Expected exit code (default: `0`) |
| `stdout_contains` | string[] | no | Substrings that must appear in stdout |
| `stderr_contains` | string[] | no | Substrings that must appear in stderr |
| `description` | string | no | Human-readable description |

**What happens:** Runs the command via `sh -c "{command}"`, checks exit code, then checks stdout and stderr patterns.

**Example — testing a CLI error path:**

```yaml
AC-12:
  description: "CLI rejects missing required flags"
  command_assertions:
    - command: "cargo run -p my-cli -- deploy"
      exit_code: 1
      stdout_contains:
        - '"ok":false'
        - '"suggestion"'
      description: "Deploy without --env flag fails with suggestion"
```

## Filtering

The `--filter` flag performs case-insensitive substring matching against:
- AC ID (e.g., `AC-01`)
- Scenario name (e.g., `Valid credentials accepted`)
- Tags (e.g., `auth`, `login`)

```bash
# Match AC ID
rara-bdd run --filter AC-01

# Match tag
rara-bdd run --filter auth

# Match scenario name
rara-bdd run --filter "credentials"
```

Multiple scenarios can match a single filter. For example, `--filter auth` matches any scenario with `auth` in its AC ID, name, or tags.

## Integration Guide

### Adding rara-bdd to an Existing Project

1. Create a `features/` directory at the project root
2. Write `.feature` files with `@AC-XX` tagged scenarios
3. Write matching `.eval.yaml` files with assertion configs
4. Run `rara-bdd validate` to check for schema errors
5. Run `rara-bdd run` to execute the suite
6. Run `rara-bdd trace` to generate the traceability matrix

### CI Integration (GitHub Actions)

```yaml
# .github/workflows/bdd.yml
name: BDD
on: [push, pull_request]

jobs:
  bdd:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install rara-bdd
        run: cargo install --git https://github.com/rararulab/rara-bdd

      - name: Validate eval files
        run: rara-bdd validate --features-dir features

      - name: Run BDD suite
        run: rara-bdd run --features-dir features --report json

      - name: Update traceability matrix
        run: rara-bdd trace --features-dir features
```

### Organizing Features

For larger projects, organize features in subdirectories. rara-bdd scans recursively:

```
features/
├── api/
│   ├── auth.feature
│   ├── auth.eval.yaml
│   ├── users.feature
│   └── users.eval.yaml
├── cli/
│   ├── commands.feature
│   └── commands.eval.yaml
├── core/
│   ├── engine.feature
│   └── engine.eval.yaml
└── TRACEABILITY.md
```

### Writing Effective Assertions

**Start with source assertions** — they're fast, deterministic, and don't require compilation:

```yaml
AC-01:
  description: "Config struct uses bon builder"
  source_assertions:
    - file: src/config.rs
      contains:
        - "#[derive(bon::Builder)]"
        - "pub struct Config"
      not_contains:
        - "fn new("
      description: "Config uses builder pattern, not manual constructor"
```

**Add runtime tests** when you need to verify behavior:

```yaml
AC-02:
  description: "Parser handles edge cases"
  runtime_tests:
    - package: my-parser
      filter: test_empty_input
    - package: my-parser
      filter: test_unicode_input
```

**Use command assertions** for end-to-end CLI verification:

```yaml
AC-03:
  description: "CLI outputs valid JSON"
  command_assertions:
    - command: "cargo run -p my-cli -- list"
      exit_code: 0
      stdout_contains:
        - '"ok":true'
```

**Combine all three** for thorough coverage:

```yaml
AC-10:
  description: "Deploy command works end-to-end"
  source_assertions:
    - file: src/cmd/deploy.rs
      contains:
        - "pub fn deploy"
      description: "Deploy function exists"
  runtime_tests:
    - package: my-cli
      filter: test_deploy_dry_run
  command_assertions:
    - command: "cargo run -p my-cli -- deploy --env staging --dry-run"
      exit_code: 0
      stdout_contains:
        - '"dry_run":true'
      description: "Dry run outputs plan without executing"
```

## Error Messages

When an assertion fails, the error message includes the AC ID and a description of what went wrong:

```
AC-01: source assertion failed — Login function exists: 'src/auth.rs' missing expected pattern: pub fn login
AC-02: runtime test failed — cargo test -p auth-service -- test_login_valid failed (exit 101)
AC-03: command assertion failed — stdout of 'cargo run -- list' missing expected pattern: "ok":true
AC-05: no eval config found
```

JSON error output includes the `suggestion` field for agent self-correction:

```json
{"ok": false, "error": "features directory not found: /bad/path", "suggestion": "check --help for usage"}
```

## Development

```bash
just fmt      # Format code
just clippy   # Run clippy lints
just test     # Run tests (unit + integration)
just lint     # Full lint suite (fmt + clippy + deny)
```

## License

MIT
