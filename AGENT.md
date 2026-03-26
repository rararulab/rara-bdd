# rara-bdd — Agent Integration Guide

## What is rara-bdd?

A BDD framework for Rust projects. It maps Gherkin `@AC-XX` tags in `.feature` files to `#[test] fn ac_XX_*()` functions by naming convention — no macros, no runtime, just naming.

## Setup

```bash
cargo install --git https://github.com/rararulab/rara-bdd
rara-bdd setup
```

`rara-bdd setup` will:
- Create `features/` directory
- Add BDD workflow instructions to your project's `CLAUDE.md`

## How to Use (for AI Agents)

### Step 1: Write .feature files

Create `features/<name>.feature` with `@AC-XX` tags:

```gherkin
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
```

Rules:
- Every scenario needs an `@AC-XX` tag (incrementing)
- Scenario name starts with the AC ID
- Steps are concrete and verifiable, not vague

### Step 2: Write matching tests

`@AC-XX` → `fn ac_XX_*()`. The `ac_XX_` prefix is the matching key.

```rust
#[test]
fn ac_01_valid_credentials_return_token() {
    // Verifies AC-01
}

#[test]
fn ac_02_invalid_password_returns_401() {
    // Verifies AC-02
}
```

One AC can have multiple test functions (all must share the `ac_XX_` prefix).

### Step 3: Verify

```bash
rara-bdd run          # Execute all matched tests
rara-bdd coverage     # Ensure every AC has tests
rara-bdd trace        # Generate TRACEABILITY.md
```

A task is **done** when:
- `rara-bdd run` → all PASS
- `rara-bdd coverage` → zero uncovered ACs

## Commands Reference

| Command | Purpose | Exit 0 |
|---------|---------|--------|
| `rara-bdd run` | Run matched tests | All pass |
| `rara-bdd run --filter AC-01` | Run specific AC | Filtered tests pass |
| `rara-bdd run --report json` | JSON output (CI) | All pass |
| `rara-bdd list` | Show AC ↔ test mapping | Always |
| `rara-bdd coverage` | Report uncovered ACs | Full coverage |
| `rara-bdd trace` | Generate TRACEABILITY.md | Always |
| `rara-bdd setup` | Scaffold project | Always |

All commands accept `--features-dir <path>` (default: `features`) and `--package <crate>` (for workspaces).

## Project Layout After Setup

```
my-project/
├── features/
│   ├── auth.feature        # @AC-01, @AC-02, ...
│   ├── billing.feature     # @AC-03, @AC-04, ...
│   └── TRACEABILITY.md     # Auto-generated
├── tests/
│   ├── auth.rs             # fn ac_01_*(), fn ac_02_*()
│   └── billing.rs          # fn ac_03_*(), fn ac_04_*()
├── CLAUDE.md               # Contains BDD workflow section
└── Cargo.toml
```

## Developing rara-bdd Itself

Architecture and internal conventions: see `@docs/architecture.md`, `@docs/cli.md`, `@docs/integration.md`.

Critical invariants:
- Gherkin parsing: `cucumber::gherkin::Feature::parse` — no custom parser
- Error handling: `snafu`, not `thiserror`
- Structs with 3+ fields: `bon::Builder`
- JSON to stdout, human text to stderr
