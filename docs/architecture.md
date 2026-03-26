# Architecture

## Two-Layer Design

```
.feature (Gherkin)  -->  #[test] fn ac_XX_*() (Rust)
--------------------     --------------------------
WHAT to test             HOW to verify
```

Gherkin scenarios declare acceptance criteria with `@AC-XX` tags. Rust `#[test]` functions verify them. rara-bdd connects the two by naming convention: `@AC-01` matches `fn ac_01_*()`.

## Execution Flow

1. **Discovery** (`src/discovery.rs`) -- Recursively scans `features/` for `.feature` files, parses Gherkin, extracts scenarios with `@AC-XX` tags
2. **Matching** (`src/matcher.rs`) -- Runs `cargo test -- --list` to discover test functions, matches each `@AC-XX` tag to tests with `ac_XX_` prefix
3. **Running** (`src/runner.rs`) -- Executes matched tests via `cargo test -- {name} --exact`, collects pass/fail results
4. **Reporting** (`src/reporter/`) -- Outputs results as terminal (colored), JSON (machine-readable), or markdown

## Key Data Types

```
Scenario {
    ac_id:        "AC-01"              // From @AC-XX tag
    name:         "Valid login"         // Scenario: line
    feature_file: "auth/login.feature"  // Relative path
    tags:         ["auth", "AC-01"]     // All tags
    steps:        ["Given ...", ...]    // Given/When/Then
}

MatchedAc {
    ac_id:        "AC-01"
    scenario:     Scenario
    test_names:   ["ac_01_valid_login", "ac_01_returns_token"]
}

AcResult {
    ac_id:        "AC-01"
    status:       AcStatus             // Passed | Failed | Uncovered
    test_results: Vec<TestResult>
}

AcStatus = Passed | Failed | Uncovered
```

- **Passed** -- All matched tests passed
- **Failed** -- At least one matched test failed
- **Uncovered** -- No test function matches the `ac_XX_` prefix

## Module Map

```
src/
├── cli/mod.rs          # Clap CLI (run, list, coverage, trace)
├── discovery.rs        # .feature scanning + Gherkin parsing
├── matcher.rs          # Test discovery + AC-to-test matching
├── runner.rs           # cargo test execution
├── reporter/
│   ├── terminal.rs     # Colored output
│   ├── json.rs         # JSON stdout
│   └── markdown.rs     # Markdown table
├── traceability.rs     # TRACEABILITY.md generation
└── error.rs            # RaraBddError (snafu)
```
