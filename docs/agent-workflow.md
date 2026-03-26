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

## Agents

| Agent | Skill | Role |
|-------|-------|------|
| Agent 1 (Design) | `/bdd-design` | Decompose requirement → GitHub issue with Gherkin + design spec |
| Agent 2 (Implement) | `/bdd-implement` | Pick up issue → deliver working code with passing BDD tests |

**Contract:** The `.feature` file is Agent 1's output and Agent 2's input. Agent 2 never modifies it.

## Quick Start

```
# Agent 1 designs a feature
/bdd-design
> I need a feature that does X, Y, Z.

# Agent 2 implements the issued feature
/bdd-implement
> Implement issue #1
```

Skills are maintained in [rara-skills](https://github.com/rararulab/rara-skills). See each skill's SKILL.md for the full workflow, checklist, and failure handling.

## Setup Guide

The pipeline depends on three repos. Follow the steps below to set it up in any Rust project.

### What comes from where

| Repo | Provides |
|------|----------|
| [rara-bdd](https://github.com/rararulab/rara-bdd) | CLI tool (`setup`, `generate`, `coverage`, `list`) |
| [rara-skills](https://github.com/rararulab/rara-skills) | `/bdd-design` and `/bdd-implement` Claude Code skills |
| [.github](https://github.com/rararulab/.github) | `bdd_task.yml` issue template + `bdd.yml` reusable CI workflow |

### Step 1. Install the CLI

```bash
cargo install --git https://github.com/rararulab/rara-bdd
```

### Step 2. Scaffold cucumber-rs in your project

```bash
rara-bdd setup
```

Creates `features/`, `tests/bdd.rs`, `tests/steps/mod.rs`, and adds cucumber dependencies to `Cargo.toml`.

### Step 3. Add the skills

Copy from [rara-skills](https://github.com/rararulab/rara-skills) into your project's `.claude/skills/`:

```bash
mkdir -p .claude/skills
cp path/to/rara-skills/skills/bdd-design/SKILL.md .claude/skills/bdd-design.md
cp path/to/rara-skills/skills/bdd-implement/SKILL.md .claude/skills/bdd-implement.md
```

This gives Claude Code the `/bdd-design` and `/bdd-implement` commands.

### Step 4. Add the issue template (org repos get this for free)

For rararulab repos, the `bdd_task.yml` template is inherited from the [.github](https://github.com/rararulab/.github) repo automatically.

For external repos, copy it:

```bash
mkdir -p .github/ISSUE_TEMPLATE
cp path/to/.github/ISSUE_TEMPLATE/bdd_task.yml .github/ISSUE_TEMPLATE/
```

### Step 5. Add CI

For rararulab repos, use the org reusable workflow:

```yaml
# .github/workflows/bdd.yml
name: BDD
on:
  pull_request:
    paths: ["features/**", "tests/**", "src/**"]

jobs:
  bdd:
    uses: rararulab/.github/workflows/bdd.yml@main
```

For external repos, add the steps directly:

```yaml
# .github/workflows/bdd.yml
name: BDD
on:
  pull_request:
    paths: ["features/**", "tests/**", "src/**"]

jobs:
  bdd:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install --git https://github.com/rararulab/rara-bdd
      - run: rara-bdd coverage
      - run: cargo test --test bdd
```

### Step 6. Run the pipeline

```
/bdd-design          # → creates a GitHub issue with Gherkin + design spec
/bdd-implement       # → implements the issue, pushes PR, waits for CI green
```
