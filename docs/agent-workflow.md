# Agent-Driven BDD Pipeline

## Overview

rara-bdd enables a two-agent pipeline where requirements flow from design to implementation through Gherkin `.feature` files as a contract.

```
 Requirement
     |
     v
+------------------+     GitHub Issue      +---------------------+
| Agent 1 (Design) | ------------------->  | Agent 2 (Implement) |
|                  |   .feature content    |                     |
|  /bdd-design     |   + design spec      |  /bdd-implement     |
+------------------+                       +---------------------+
                                                    |
                                           +--------+--------+
                                           |        |        |
                                           v        v        v
                                        types    logic    steps
                                           |        |        |
                                           +--------+--------+
                                                    |
                                                    v
                                           rara-bdd coverage
                                           cargo test --test bdd
                                                    |
                                                    v
                                                Push + PR
```

## Agent 1: Design Agent (`/bdd-design`)

**Role:** Analyze a requirement and produce a GitHub issue with Gherkin acceptance criteria and a design spec.

**Invocation:**

```
/bdd-design

User: I need a user registration feature that validates email format,
      rejects duplicate emails, and sends a welcome email.
```

**What it does:**

1. Reads `AGENT.md` and source code to understand the codebase
2. Runs `rara-bdd list` to find reusable existing step definitions
3. Writes Gherkin scenarios (minimum 3: happy path, error case, edge case)
4. Writes a design spec (files, signatures, constraints)
5. Creates a GitHub issue using the `feature.yml` template with proper labels

**Output:** A GitHub issue with:
- `.feature` content (the contract)
- Design spec (implementation guidance)
- Proper labels (`agent:claude` + type + component)

## Agent 2: Implementation Agent (`/bdd-implement`)

**Role:** Pick up a BDD-specced issue and deliver working code with all BDD tests passing.

**Invocation:**

```
/bdd-implement

User: Implement issue #42
```

**What it does:**

1. Reads the issue (`gh issue view <N>`)
2. Creates a worktree per org workflow
3. Runs `rara-bdd setup` (idempotent)
4. Writes the `.feature` file from the issue content (verbatim, no modifications)
5. Runs `rara-bdd generate` to create step skeletons
6. Runs `rara-bdd coverage` to verify all steps have definitions
7. Implements: data types, core logic, integration, step definitions
8. Verifies: `rara-bdd coverage` exit 0 + `cargo test --test bdd` pass + clippy clean
9. Pushes and creates a PR
10. Waits for CI green (`gh pr checks --watch`)

**Critical rule:** Agent 2 never modifies the `.feature` file. It is Agent 1's contract.

## Failure Recovery

| Failure | Agent | Action | Max Retries |
|---------|-------|--------|-------------|
| Compile error | Agent 2 | Read error, fix code, rebuild | 3 |
| Missing step definition | Agent 2 | Run `rara-bdd generate`, implement | 1 |
| Assertion failure | Agent 2 | Analyze expected vs actual, fix logic | 3 |
| Clippy warning | Agent 2 | Fix the warning | 3 |
| Retries exhausted | Agent 2 | Add `needs-human` label + analysis comment | -- |
| Ambiguous requirement | Agent 1 | Ask user for clarification before creating issue | -- |
| Conflicting steps | Agent 1 | Reconcile with existing steps via `rara-bdd list` | -- |

When Agent 2 exhausts retries, it adds the `needs-human` label to the issue and posts a detailed comment with the failure type, attempts made, last error output, and analysis of what was tried.

## CI Integration

Add BDD verification to your GitHub Actions workflow:

```yaml
name: BDD
on:
  pull_request:
    paths:
      - "features/**"
      - "tests/**"
      - "src/**"

jobs:
  bdd:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install rara-bdd
        run: cargo install --git https://github.com/rararulab/rara-bdd

      - name: Check step coverage
        run: rara-bdd coverage

      - name: Run BDD tests
        run: cargo test --test bdd
```

For rararulab repos, you can use the org reusable workflow if available:

```yaml
jobs:
  bdd:
    uses: rararulab/.github/workflows/bdd.yml@main
```

## Bootstrapping a New Project

To add the agent-driven BDD pipeline to a new Rust project:

### 1. Install rara-bdd

```bash
cargo install --git https://github.com/rararulab/rara-bdd
```

### 2. Scaffold the project

```bash
rara-bdd setup
```

This creates `features/`, `tests/bdd.rs`, `tests/steps/mod.rs`, updates `Cargo.toml`, and adds BDD workflow instructions to `CLAUDE.md`.

### 3. Add Claude Code skills

Copy the skill files into your project:

```bash
mkdir -p .claude/skills
# Copy bdd-design.md and bdd-implement.md from rara-bdd repo
cp path/to/rara-bdd/.claude/skills/bdd-design.md .claude/skills/
cp path/to/rara-bdd/.claude/skills/bdd-implement.md .claude/skills/
```

### 4. Add CI workflow

Add the BDD GitHub Actions workflow shown above to `.github/workflows/bdd.yml`.

### 5. Start using the pipeline

```
# Agent 1 designs a feature
/bdd-design
> I need a feature that does X, Y, Z.

# Agent 2 implements the issued feature
/bdd-implement
> Implement issue #1
```

## TestWorld Pattern

The `TestWorld` struct is the shared state container between BDD steps. Each step type has a specific role:

| Step Type | Role | TestWorld Usage |
|-----------|------|-----------------|
| `Given` | Setup preconditions | Initialize fields |
| `When` | Execute the action under test | Call production code, store results |
| `Then` | Assert expected outcomes | Read fields, assert values |

Keep `TestWorld` minimal. Only add fields that must be shared between steps. Use local variables within step functions for intermediate state.
