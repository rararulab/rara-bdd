# rara-bdd

BDD testing framework for rararulab Rust projects. Parses Gherkin `.feature` files and evaluates acceptance criteria via a declarative YAML DSL (`.eval.yaml`).

Inspired by [ralph-orchestrator](https://github.com/rararulab/rara)'s hooks BDD system.

## Quick Start

```bash
# List discovered scenarios
rara-bdd list --features-dir features

# Run BDD suite
rara-bdd run --features-dir features

# JSON output for CI
rara-bdd run --features-dir features --report json

# Validate eval files (schema check only)
rara-bdd validate --features-dir features

# Generate traceability matrix
rara-bdd trace --features-dir features
```

## Per-Project Setup

Add a `features/` directory to your Rust project:

```
my-project/
├── features/
│   ├── auth/
│   │   ├── login.feature        # Gherkin scenarios
│   │   └── login.eval.yaml      # Evaluator DSL
│   └── TRACEABILITY.md          # Auto-generated
├── src/
├── tests/
└── Cargo.toml
```

### .feature file (Gherkin)

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

### .eval.yaml (Evaluator DSL)

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
  runtime_tests:
    - package: my-project
      filter: test_login_invalid_credentials
  command_assertions:
    - command: "cargo run -- login --user bad --pass wrong"
      exit_code: 1
      stdout_contains: ['"ok":false']
```

## Assertion Types

| Type | Purpose |
|------|---------|
| `runtime_tests` | Run `cargo test -p {package} -- {filter}` |
| `source_assertions` | Verify file contains/not_contains/matches patterns |
| `command_assertions` | Run shell command, check exit code + stdout/stderr |

## Development

```bash
just fmt      # Format
just clippy   # Lint
just test     # Run tests
just lint     # Full lint suite
```

## License

MIT
