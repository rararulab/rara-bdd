# CLAUDE.md — rara-bdd

## Communication
- 用中文与用户交流

## Project
rara-bdd is a cucumber-rs scaffolding tool for AI agents.
It generates project skeletons and step definition code from
Gherkin `.feature` files, enabling quick BDD testing setup.

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
cargo run -- setup                # Set up cucumber-rs skeleton
cargo run -- generate             # Generate step definitions
cargo run -- coverage             # Check step coverage
cargo run -- list                 # List scenarios
```
