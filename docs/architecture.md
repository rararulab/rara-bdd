# Architecture

## Design

rara-bdd is a **scaffolding tool** for cucumber-rs. It does not execute tests — that is cucumber-rs's job.

```
.feature (Gherkin)  -->  rara-bdd generate  -->  #[given]/#[when]/#[then] skeletons
                                                  ↓
                                             cargo test --test bdd (cucumber-rs)
```

## What rara-bdd Does

1. **Setup** — scaffolds a complete cucumber-rs project (Cargo.toml, bdd.rs, steps/)
2. **Generate** — reads `.feature` files and generates step definition skeletons
3. **Coverage** — compares feature steps against defined step annotations
4. **List** — displays all discovered scenarios and steps

## What cucumber-rs Does

- Parses `.feature` files at runtime
- Matches steps to `#[given]`/`#[when]`/`#[then]` functions
- Executes scenarios with World state management
- Reports pass/fail results

## Execution Flow

### Generate

1. **Discovery** (`src/discovery.rs`) — scans `features/` for `.feature` files, parses Gherkin, extracts scenarios with steps
2. **Coverage check** (`src/step_coverage.rs`) — scans `tests/steps/` for existing `#[given]`/`#[when]`/`#[then]` annotations
3. **Generation** (`src/generate.rs`) — creates skeleton functions for undefined steps, writes to `tests/steps/<feature>_steps.rs`

### Coverage

1. **Discovery** — same as above
2. **Annotation scan** (`src/step_coverage.rs`) — regex-parses Rust source for step annotations
3. **Comparison** — normalizes step text (quoted strings → `{string}`, numbers → `{int}`/`{float}`) and matches against annotations

## Key Data Types

```
StepKeyword = Given | When | Then

Step {
    keyword: StepKeyword,
    text:    "a registered user with email \"alice\""
}

Scenario {
    name:         "Valid credentials"
    feature_file: "auth/login.feature"
    feature_name: "User login"
    tags:         ["auth"]
    steps:        Vec<Step>
}

DefinedStep {
    keyword:    StepKeyword,
    expression: "a registered user with email {string}"
    file:       "tests/steps/auth_steps.rs"
}
```

## Module Map

```
src/
├── cli/mod.rs          # Clap CLI (setup, generate, coverage, list)
├── discovery.rs        # .feature scanning + Gherkin parsing
├── generate.rs         # Step definition skeleton generation
├── step_coverage.rs    # Step annotation scanning + coverage analysis
├── setup.rs            # Project scaffolding (Cargo.toml, bdd.rs, steps/)
├── error.rs            # RaraBddError (snafu)
├── main.rs             # CLI entry point
└── lib.rs              # Module re-exports
```
