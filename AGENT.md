# rara-bdd — Agent Guidelines

## Purpose
BDD testing framework that parses Gherkin `.feature` files and evaluates acceptance criteria via a declarative YAML DSL (`.eval.yaml`), inspired by ralph-orchestrator's hooks BDD system.

## Architecture
```
src/
├── cli/mod.rs          # Clap CLI: run, list, validate, trace
├── discovery.rs        # Gherkin parser → Scenario structs
├── evaluator/
│   ├── mod.rs          # Suite runner + scenario dispatch
│   ├── loader.rs       # .eval.yaml serde models
│   ├── runtime.rs      # cargo test + command execution
│   └── source.rs       # File content assertions
├── harness.rs          # Bounded command execution + artifact capture
├── reporter/
│   ├── terminal.rs     # Colored output
│   ├── json.rs         # Machine-readable JSON
│   └── markdown.rs     # Agent-readable markdown
├── traceability.rs     # TRACEABILITY.md generation
├── error.rs            # RaraBddError (snafu)
└── main.rs             # Entry point
```

Data flow: `discover() → load eval YAML → run_suite() → report()`

## Critical Invariants
- `.feature` files are parsed by `cucumber::gherkin::Feature::parse` — do NOT write a custom Gherkin parser
- `.eval.yaml` files are the ONLY way to define assertions — evaluator logic must NOT be hardcoded in Rust
- AC IDs (`@AC-XX` tags) are the routing key between Gherkin scenarios and eval configs
- JSON on stdout, human text on stderr — never mix formats

## What NOT To Do
- Do NOT hardcode evaluator logic in Rust — all assertions come from `.eval.yaml` DSL
- Do NOT use `thiserror` — use `snafu` per org standard
- Do NOT write manual constructors for 3+ field structs — use `bon::Builder`
- Do NOT add interactive prompts — all parameters via CLI flags

## Dependencies
- `cucumber` 0.22 — Gherkin parsing only (not using its test runner)
- `serde_yaml` — eval DSL parsing
- Upstream: consumed by all rararulab Rust projects as a dev tool
- No runtime dependencies on target projects — communicates via `std::process::Command`

## Documentation

@docs/architecture.md
@docs/eval-dsl.md
@docs/cli.md
@docs/integration.md
