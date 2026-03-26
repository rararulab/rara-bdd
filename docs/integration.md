# Integration Guide

## Adding rara-bdd to a Project

1. Create `features/` at project root
2. Write `.feature` files with `@AC-XX` tagged scenarios
3. Write matching `.eval.yaml` files (same stem name)
4. `rara-bdd validate` — check for schema errors
5. `rara-bdd run` — execute the suite
6. `rara-bdd trace` — generate traceability matrix

## .feature File Conventions

```gherkin
@auth @login
Feature: User login
  @AC-01
  Scenario: AC-01 Valid credentials accepted
    Given a user with valid credentials
    When login is attempted
    Then access is granted
```

- Each scenario MUST have an `@AC-XX` tag (numeric)
- Scenario name SHOULD start with the AC ID
- Without `@AC-XX`, gets auto-ID `UNTAGGED-scenario-name`

## Directory Organization

Subdirectories are scanned recursively:

```
features/
├── api/
│   ├── auth.feature
│   └── auth.eval.yaml
├── cli/
│   ├── commands.feature
│   └── commands.eval.yaml
└── TRACEABILITY.md
```

## CI Integration (GitHub Actions)

```yaml
name: BDD
on: [push, pull_request]
jobs:
  bdd:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install rara-bdd
        run: cargo install --git https://github.com/rararulab/rara-bdd
      - run: rara-bdd validate --features-dir features
      - run: rara-bdd run --features-dir features --report json
      - run: rara-bdd trace --features-dir features
```

## Best Practices

**Start with source assertions** — fast, deterministic, no compilation:

```yaml
AC-01:
  source_assertions:
    - file: src/config.rs
      contains: ["#[derive(bon::Builder)]", "pub struct Config"]
      not_contains: ["fn new("]
```

**Add runtime tests** for behavior verification:

```yaml
AC-02:
  runtime_tests:
    - package: my-parser
      filter: test_empty_input
```

**Use command assertions** for end-to-end CLI checks:

```yaml
AC-03:
  command_assertions:
    - command: "cargo run -p my-cli -- list"
      exit_code: 0
      stdout_contains: ['"ok":true']
```

## Error Messages

```
AC-01: source assertion failed — Login function exists: 'src/auth.rs' missing expected pattern: pub fn login
AC-02: runtime test failed — cargo test -p auth-service -- test_login_valid failed (exit 101)
AC-03: command assertion failed — stdout of 'cargo run -- list' missing expected pattern: "ok":true
AC-05: no eval config found
```

JSON errors include `suggestion` for agent self-correction:

```json
{"ok": false, "error": "features directory not found: /bad/path", "suggestion": "check --help for usage"}
```
