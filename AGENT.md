# rara-bdd — Agent Guidelines

## Purpose

BDD testing framework for rararulab Rust projects.
Parses Gherkin `.feature` files, matches `@AC-XX` tags to `#[test] fn ac_XX_*()` functions by naming convention, executes tests, and reports results.

## Two-Agent Workflow

rara-bdd enforces a contract between **design** and **implementation**:

```
Design Agent                       Implementation Agent
─────────────                      ────────────────────
1. Analyze requirements             4. Read .feature from issue
2. Write .feature files             5. Write implementation code
3. Create issue (must include       6. Write #[test] fn ac_XX_*()
   .feature content)                7. rara-bdd run → all green to complete
```

### Design Agent Responsibilities

- Break down requirements into Gherkin scenarios, each tagged with `@AC-XX`
- `.feature` files are the acceptance criteria — must be included in the issue
- Steps must be specific and testable (avoid vague "should work correctly")
- One issue maps to one or more `.feature` files

### Implementation Agent Responsibilities

- Retrieve `.feature` files from the issue and place them in the target project's `features/` directory
- Read each scenario's Given/When/Then to understand what to verify
- Write implementation code to satisfy the described behavior
- Write `#[test] fn ac_XX_*()` test functions covering every AC
- Run `rara-bdd run --features-dir features` to confirm all pass
- Run `rara-bdd coverage` to confirm no AC is uncovered

## Architecture

```
src/
├── cli/mod.rs          # Clap CLI: run, list, coverage, trace
├── discovery.rs        # Gherkin parser → Scenario structs
├── matcher.rs          # AC tag → test function matching
├── runner.rs           # cargo test execution + result collection
├── reporter/
│   ├── terminal.rs     # Colored human-readable output
│   ├── json.rs         # Machine-readable JSON
│   └── markdown.rs     # Agent-readable markdown table
├── traceability.rs     # TRACEABILITY.md generation
├── error.rs            # RaraBddError (snafu)
└── main.rs             # Entry point
```

Data flow: `discover() → match_scenarios() → run_suite() → report()`

## .feature File Conventions

```gherkin
@auth
Feature: User authentication

  @AC-01
  Scenario: AC-01 Valid credentials return a session token
    Given a registered user with email "test@example.com"
    When the user submits correct credentials via POST /login
    Then the response status is 200
    And the response body contains a non-empty "token" field

  @AC-02
  Scenario: AC-02 Invalid password returns 401
    Given a registered user with email "test@example.com"
    When the user submits an incorrect password via POST /login
    Then the response status is 401
    And the response body contains error message "invalid credentials"
```

**Requirements**:
- Every scenario must have an `@AC-XX` tag (incrementing numbers)
- Scenario name should start with the AC ID
- Steps describe concrete behavior, not abstract intent
- Given = preconditions, When = action, Then = verifiable outcome

## Test Naming Convention

`@AC-XX` → `fn ac_XX_*()` — the prefix `ac_XX_` is the matching key.

```rust
#[test]
fn ac_01_valid_credentials_return_token() {
    // Matches AC-01: verify login returns token
}

#[test]
fn ac_01_token_is_valid_jwt() {
    // Same AC can have multiple tests
}

#[test]
fn ac_02_invalid_password_returns_401() {
    // Matches AC-02
}
```

## Issue Requirements

Every issue must include:
1. **Description** — what to implement and why
2. **`.feature` file content** — complete Gherkin scenarios as acceptance criteria
3. **Scope** — which modules/crates are affected

Implementation agent completion criteria:
- `rara-bdd run` — all PASS
- `rara-bdd coverage` — no uncovered ACs
- `rara-bdd trace` — traceability matrix is complete

## Critical Invariants

- Gherkin parsing uses `cucumber::gherkin::Feature::parse` — do NOT write a custom parser
- AC ID (`@AC-XX`) is the sole routing key between Gherkin and tests
- JSON output goes to stdout, human-readable text to stderr — never mix
- Use `snafu` for error handling, NOT `thiserror`
- Use `bon::Builder` for structs with 3+ fields
- No interactive prompts — all parameters via CLI flags

## Commands

```bash
rara-bdd run --features-dir features              # Execute test suite
rara-bdd run --features-dir features --report json # JSON output (for CI)
rara-bdd list --features-dir features              # List scenario ↔ test mapping
rara-bdd coverage --features-dir features          # Check for uncovered ACs
rara-bdd trace --features-dir features             # Generate TRACEABILITY.md
```

## Documentation

@docs/architecture.md
@docs/cli.md
@docs/integration.md
