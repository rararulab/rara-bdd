# CLI Reference

## `rara-bdd setup`

Scaffold cucumber-rs in the current project.

| Flag | Default | Description |
|------|---------|-------------|
| `--features-dir <PATH>` | `features` | Features directory to create |

```bash
rara-bdd setup
rara-bdd setup --features-dir specs
```

Creates:
- `features/` directory
- `tests/bdd.rs` — World struct + cucumber-rs entry point
- `tests/steps/mod.rs` — step module declarations
- Modifies `Cargo.toml` — adds cucumber + tokio dev-dependencies, `[[test]]` section
- Updates `CLAUDE.md` — adds BDD workflow instructions

All operations are idempotent (safe to run multiple times).

```json
{
  "ok": true,
  "action": "setup",
  "created_features_dir": true,
  "cargo_toml": "modified",
  "created_bdd_rs": true,
  "created_steps_mod": true,
  "claude_md": "created"
}
```

---

## `rara-bdd generate`

Generate step definition skeletons from `.feature` files.

| Flag | Default | Description |
|------|---------|-------------|
| `--features-dir <PATH>` | `features` | Features directory path |
| `--steps-dir <PATH>` | `tests/steps` | Step definitions directory |
| `--dry-run` | false | Preview without writing files |

```bash
rara-bdd generate
rara-bdd generate --dry-run
rara-bdd generate --steps-dir tests/steps
```

For each `.feature` file, creates `tests/steps/<feature>_steps.rs` with `#[given]`/`#[when]`/`#[then]` async functions containing `todo!()` bodies. Skips steps that already have definitions.

```json
{
  "ok": true,
  "action": "generate",
  "files_created": ["tests/steps/auth_steps.rs"],
  "steps_generated": 5,
  "steps_skipped": 0
}
```

---

## `rara-bdd coverage`

Report which feature steps lack step definitions.

| Flag | Default | Description |
|------|---------|-------------|
| `--features-dir <PATH>` | `features` | Features directory path |
| `--steps-dir <PATH>` | `tests/steps` | Step definitions directory |

```bash
rara-bdd coverage
rara-bdd coverage --features-dir specs --steps-dir tests/steps
```

Exit: `0` = all steps covered, `1` = missing steps exist.

```json
{
  "ok": true,
  "action": "coverage",
  "total_steps": 10,
  "covered_steps": 10,
  "missing_steps": 0
}
```

---

## `rara-bdd list`

List all discovered scenarios and steps.

| Flag | Default | Description |
|------|---------|-------------|
| `--features-dir <PATH>` | `features` | Features directory path |
| `--filter <STRING>` | -- | Filter by tag (case-insensitive) |

```bash
rara-bdd list
rara-bdd list --filter auth
rara-bdd list --features-dir specs
```

---

## Running Tests

Test execution is handled by cucumber-rs, not rara-bdd:

```bash
cargo test --test bdd           # Run all BDD scenarios
cargo test --test bdd -- login  # Filter by scenario name
```
