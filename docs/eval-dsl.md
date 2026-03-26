# YAML Eval DSL Reference

`.eval.yaml` files define how to verify each acceptance criterion. Must share the same stem as the `.feature` file.

## Structure

```yaml
AC-XX:
  description: "What this AC verifies"   # optional
  runtime_tests:                          # optional — cargo test
    - ...
  source_assertions:                      # optional — file content checks
    - ...
  command_assertions:                     # optional — shell commands
    - ...
```

Assertions run in order: `runtime_tests` → `source_assertions` → `command_assertions`. First failure stops remaining assertions for that AC.

---

## `runtime_tests`

Run `cargo test -p {package} -- {filter} --exact`.

```yaml
runtime_tests:
  - package: my-crate       # required — cargo -p flag
    filter: test_func_name   # required — test name filter
```

Example:

```yaml
AC-01:
  runtime_tests:
    - package: auth-service
      filter: test_login_valid
    - package: auth-service
      filter: test_login_returns_token
```

---

## `source_assertions`

Read a file and check contents.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `file` | string | yes | Relative path from project root |
| `contains` | string[] | no | Substrings that must be present |
| `not_contains` | string[] | no | Substrings that must NOT be present |
| `matches` | string | no | Regex (Rust `regex` syntax) that must match |
| `description` | string | no | Label for error messages |

Example:

```yaml
AC-05:
  source_assertions:
    - file: src/error.rs
      contains:
        - "use snafu"
        - "#[derive(Debug, Snafu)]"
      not_contains:
        - "thiserror"
      matches: "pub enum \\w+Error"
      description: "Error types use snafu"
```

---

## `command_assertions`

Run a shell command via `sh -c` and verify output.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `command` | string | yes | Shell command |
| `exit_code` | integer | no | Expected exit code (default: `0`) |
| `stdout_contains` | string[] | no | Patterns in stdout |
| `stderr_contains` | string[] | no | Patterns in stderr |
| `description` | string | no | Label for error messages |

Example:

```yaml
AC-12:
  command_assertions:
    - command: "cargo run -p my-cli -- deploy"
      exit_code: 1
      stdout_contains:
        - '"ok":false'
        - '"suggestion"'
      description: "Deploy without --env fails with suggestion"
```

---

## Combined Example

```yaml
AC-10:
  description: "Deploy command works end-to-end"
  source_assertions:
    - file: src/cmd/deploy.rs
      contains: ["pub fn deploy"]
  runtime_tests:
    - package: my-cli
      filter: test_deploy_dry_run
  command_assertions:
    - command: "cargo run -p my-cli -- deploy --env staging --dry-run"
      exit_code: 0
      stdout_contains: ['"dry_run":true']
```
