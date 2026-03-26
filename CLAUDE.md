# CLAUDE.md — rara-bdd

## Communication
- 用中文与用户交流

## Project
rara-bdd is a BDD testing framework for rararulab Rust projects.
It parses Gherkin `.feature` files and evaluates acceptance criteria
via a declarative YAML DSL (`.eval.yaml`).

## Development Workflow
Follows rararulab org standards: issue → worktree → PR → merge.
See https://github.com/rararulab/.github/blob/main/docs/workflow.md

## Code Quality
- Error handling: `snafu` (see `src/error.rs`)
- Builder pattern: `bon` for structs with 3+ fields
- Functional style: iterator chains over imperative loops
- English doc comments on all `pub` items
- No wildcard imports

## Commands
```bash
just fmt          # Format
just clippy       # Lint
just test         # Run tests
just lint         # Full lint suite
cargo run -- list --features-dir features  # List scenarios
cargo run -- run --features-dir features   # Run BDD suite
```
