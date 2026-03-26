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

### 1. Install rara-bdd

```bash
cargo install --git https://github.com/rararulab/rara-bdd
```

### 2. Scaffold the project

```bash
rara-bdd setup
```

### 3. Add Claude Code skills

Install from [rara-skills](https://github.com/rararulab/rara-skills):

```bash
mkdir -p .claude/skills
cp path/to/rara-skills/skills/bdd-design/SKILL.md .claude/skills/bdd-design.md
cp path/to/rara-skills/skills/bdd-implement/SKILL.md .claude/skills/bdd-implement.md
```

### 4. Add CI workflow

Add the GitHub Actions workflow shown above to `.github/workflows/bdd.yml`.
