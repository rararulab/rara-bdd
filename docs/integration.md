# Integration Guide

## Adding rara-bdd to a Project

```bash
cargo install --git https://github.com/rararulab/rara-bdd
rara-bdd setup
```

This gives you a complete cucumber-rs setup. Then:

1. Write `.feature` files in `features/`
2. `rara-bdd generate` — create step definition skeletons
3. Implement step definitions (replace `todo!()` with real logic)
4. `rara-bdd coverage` — check for missing step definitions
5. `cargo test --test bdd` — run the BDD suite

## .feature File Conventions

```gherkin
@auth @login
Feature: User login

  Scenario: Valid credentials accepted
    Given a user with valid credentials
    When login is attempted
    Then access is granted

  Scenario: Invalid password rejected
    Given a user with valid credentials
    When login is attempted with wrong password
    Then access is denied with status 401
```

- Use tags (`@auth`, `@login`) for categorization
- Steps should be concrete and verifiable
- Quoted strings are auto-detected as `{string}` parameters
- Numbers are auto-detected as `{int}` or `{float}` parameters

## Generated Step Definitions

`rara-bdd generate` creates one file per feature:

```rust
// tests/steps/login_steps.rs
use cucumber::{given, when, then};
use crate::TestWorld;

#[given("a user with valid credentials")]
async fn a_user_with_valid_credentials(world: &mut TestWorld) {
    todo!("implement: a user with valid credentials")
}

#[when("login is attempted")]
async fn login_is_attempted(world: &mut TestWorld) {
    todo!("implement: login is attempted")
}

#[then("access is granted")]
async fn access_is_granted(world: &mut TestWorld) {
    todo!("implement: access is granted")
}
```

Steps with parameters use `expr` syntax:

```rust
#[given(expr = "a user with email {string}")]
async fn a_user_with_email(world: &mut TestWorld, p0: String) {
    todo!("implement: a user with email \"test@example.com\"")
}
```

## World Struct

Shared state between steps lives in `tests/bdd.rs`:

```rust
#[derive(Debug, Default, World)]
pub struct TestWorld {
    // Add your test state fields here
    user_email: Option<String>,
    response_status: Option<u16>,
}
```

## Directory Organization

```
my-project/
├── features/
│   ├── api/
│   │   └── auth.feature
│   └── cli/
│       └── commands.feature
├── tests/
│   ├── bdd.rs                # World struct + main
│   └── steps/
│       ├── mod.rs            # mod auth_steps; mod commands_steps;
│       ├── auth_steps.rs     # #[given]/#[when]/#[then] for auth
│       └── commands_steps.rs # #[given]/#[when]/#[then] for commands
└── Cargo.toml
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
      - run: rara-bdd coverage
      - run: cargo test --test bdd
```

## Best Practices

**One step, one definition** — each Gherkin step maps to exactly one `#[given]`/`#[when]`/`#[then]` function. cucumber-rs reuses definitions across scenarios automatically.

**Keep World minimal** — only add fields that need to be shared between steps. Prefer local variables within step functions for intermediate state.

**Use `rara-bdd generate` iteratively** — it's idempotent. When you add new scenarios, run `generate` again and it will only create skeletons for new steps.

**Check coverage before running** — `rara-bdd coverage` is fast and catches missing definitions before you wait for test compilation.

## Adding rara-bdd to a Target Project's CLAUDE.md

`rara-bdd setup` automatically adds a BDD section to `CLAUDE.md`. If you need to add it manually:

```markdown
## BDD Testing (cucumber-rs)

This project uses cucumber-rs for BDD acceptance testing,
scaffolded by rara-bdd.

### Workflow

1. Write Gherkin scenarios in `features/*.feature`
2. Generate step skeletons: `rara-bdd generate`
3. Implement step definitions in `tests/steps/`
4. Run tests: `cargo test --test bdd`

### Completion criteria

A task is done when:
- `rara-bdd coverage` — zero missing steps
- `cargo test --test bdd` — all scenarios pass
```
