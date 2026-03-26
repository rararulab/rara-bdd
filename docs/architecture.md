# Architecture

## Three-Layer Design

```
.feature (Gherkin)     →    .eval.yaml (YAML DSL)     →    Evaluator (runtime)
───────────────────         ──────────────────────         ─────────────────────
WHAT to test                HOW to verify                  Execute + report
```

## Execution Flow

1. **Discovery** (`src/discovery.rs`) — Recursively scans `features/` for `.feature` files, parses Gherkin via `cucumber-rs`, extracts scenarios with `@AC-XX` tags
2. **Pairing** — Loads matching `.eval.yaml` by stem name (`login.feature` → `login.eval.yaml`), maps AC IDs to eval configs
3. **Evaluation** (`src/evaluator/`) — Per scenario, runs assertions in order: `runtime_tests` → `source_assertions` → `command_assertions`. First failure stops remaining assertions for that AC
4. **Reporting** (`src/reporter/`) — Outputs results as terminal (colored), JSON (machine-readable), or markdown

## Key Data Types

```
Scenario {
    ac_id:        "AC-01"              // From @AC-XX tag
    name:         "Valid login"         // Scenario: line
    feature_file: "auth/login.feature"  // Relative path
    tags:         ["auth", "AC-01"]     // All tags
    steps:        ["Given ...", ...]    // Given/When/Then
    eval:         Option<AcEval>        // Paired eval config
}

AcEval {
    description:        Option<String>
    runtime_tests:      Option<Vec<RuntimeTest>>
    source_assertions:  Option<Vec<SourceAssertion>>
    command_assertions: Option<Vec<CommandAssertion>>
}
```

## Module Map

```
src/
├── cli/mod.rs          # Clap CLI (run, list, validate, trace)
├── discovery.rs        # .feature scanning + Gherkin parsing
├── evaluator/
│   ├── mod.rs          # Suite runner + scenario dispatch
│   ├── loader.rs       # .eval.yaml deserialization
│   ├── runtime.rs      # cargo test + shell command execution
│   └── source.rs       # File content pattern matching
├── reporter/
│   ├── terminal.rs     # Colored output
│   ├── json.rs         # JSON stdout
│   └── markdown.rs     # Markdown table
├── traceability.rs     # TRACEABILITY.md generation
├── harness.rs          # Bounded command execution + artifacts
└── error.rs            # RaraBddError (snafu)
```
