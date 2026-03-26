# rara-bdd — Agent Integration Guide

## What is rara-bdd?

A cucumber-rs scaffolding tool for Rust projects. It generates project skeletons and step definition code from Gherkin `.feature` files — so AI agents can set up and use BDD testing without manual boilerplate.

Test execution is handled entirely by [cucumber-rs](https://github.com/cucumber-rs/cucumber).

## Setup

```bash
cargo install --git https://github.com/rararulab/rara-bdd
rara-bdd setup
```

`rara-bdd setup` will:
- Create `features/` directory
- Add `cucumber` and `tokio` to `[dev-dependencies]` in `Cargo.toml`
- Add `[[test]] name = "bdd" harness = false` to `Cargo.toml`
- Generate `tests/bdd.rs` with World struct and entry point
- Generate `tests/steps/mod.rs`
- Add BDD workflow instructions to `CLAUDE.md`

## How to Use (for AI Agents)

### Step 1: Write .feature files

Create `features/<name>.feature` with Gherkin scenarios:

```gherkin
Feature: User authentication

  Scenario: Valid credentials return a session token
    Given a registered user with email "test@example.com"
    When the user submits correct credentials via POST /login
    Then the response status is 200
    And the response body contains a non-empty "token" field

  Scenario: Invalid password returns 401
    Given a registered user with email "test@example.com"
    When the user submits an incorrect password via POST /login
    Then the response status is 401
```

Rules:
- Steps should be concrete and verifiable, not vague
- Use `Given`/`When`/`Then` (and `And`/`But` for continuation)
- Quoted strings become `{string}` parameters, numbers become `{int}`/`{float}`

### Step 2: Generate step skeletons

```bash
rara-bdd generate
```

This creates `tests/steps/<feature>_steps.rs` with skeleton functions:

```rust
use cucumber::{given, when, then};
use crate::TestWorld;

#[given(expr = "a registered user with email {string}")]
async fn a_registered_user_with_email(world: &mut TestWorld, p0: String) {
    todo!("implement: a registered user with email \"test@example.com\"")
}

#[when("the user submits correct credentials via POST /login")]
async fn the_user_submits_correct_credentials_via_post_login(world: &mut TestWorld) {
    todo!("implement: the user submits correct credentials via POST /login")
}
```

### Step 3: Implement step definitions

Replace `todo!()` with actual test logic. Use the `TestWorld` struct (in `tests/bdd.rs`) to share state between steps.

### Step 4: Verify

```bash
rara-bdd coverage             # Ensure every step has a definition
cargo test --test bdd         # Run BDD tests via cucumber-rs
```

A task is **done** when:
- `rara-bdd coverage` → zero missing steps
- `cargo test --test bdd` → all scenarios pass

## Commands Reference

| Command | Purpose | Exit 0 |
|---------|---------|--------|
| `rara-bdd setup` | Scaffold cucumber-rs project | Always |
| `rara-bdd generate` | Generate step skeletons | Always |
| `rara-bdd generate --dry-run` | Preview generation | Always |
| `rara-bdd coverage` | Report missing step definitions | Full coverage |
| `rara-bdd list` | Show features, scenarios, steps | Always |
| `rara-bdd list --filter <tag>` | Filter by tag | Always |
| `cargo test --test bdd` | Run BDD tests | All pass |

All commands accept `--features-dir <path>` (default: `features`). `generate` and `coverage` also accept `--steps-dir <path>` (default: `tests/steps`).

## Project Layout After Setup

```
my-project/
├── features/
│   ├── auth.feature          # Gherkin scenarios
│   └── billing.feature
├── tests/
│   ├── bdd.rs                # World struct + cucumber-rs main
│   └── steps/
│       ├── mod.rs            # Module declarations
│       ├── auth_steps.rs     # #[given]/#[when]/#[then] for auth
│       └── billing_steps.rs  # #[given]/#[when]/#[then] for billing
├── CLAUDE.md                 # Contains BDD workflow section
└── Cargo.toml                # cucumber + tokio in [dev-dependencies]
```

## Writing Good .feature Files

Steps should be specific enough that an agent can derive test logic:

```gherkin
# BAD — too vague, agent can't derive assertions
Scenario: Login works
  Given a user
  When they login
  Then it works

# GOOD — specific, testable, maps to code
Scenario: Valid credentials return a session token
  Given a registered user with email "test@example.com"
  When the user submits correct credentials via POST /login
  Then the response status is 200
  And the response body contains a non-empty "token" field
```

## Agent Workflow

rara-bdd is designed as a contract between two agents:

### Phase 1: Design Agent creates .feature files

The design agent analyzes requirements and produces Gherkin scenarios with concrete, verifiable steps.

### Phase 2: Implementation Agent writes code + tests

The implementation agent:
1. Reads the `.feature` files
2. Runs `rara-bdd generate` to create step skeletons
3. Implements the feature code
4. Implements step definitions (replaces `todo!()`)
5. Verifies:

```bash
rara-bdd coverage             # No missing steps
cargo test --test bdd         # All scenarios pass
```

Both must succeed before the task is considered complete.

## Developing rara-bdd Itself

Architecture and internal conventions: see `docs/architecture.md`, `docs/cli.md`, `docs/integration.md`.

Critical invariants:
- Gherkin parsing: `cucumber::gherkin::Feature::parse` — no custom parser
- Error handling: `snafu`, not `thiserror`
- JSON to stdout, human text to stderr
