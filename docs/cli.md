# CLI Reference

## `rara-bdd run`

Execute matched tests and report results.

| Flag | Default | Description |
|------|---------|-------------|
| `--features-dir <PATH>` | `features` | Features directory path |
| `--filter <STRING>` | -- | Filter by AC ID, tag, or name (case-insensitive substring) |
| `--report <FORMAT>` | `terminal` | `terminal` / `json` / `markdown` |
| `--package <NAME>` | -- | Scope test discovery to a specific crate in a workspace |

```bash
rara-bdd run
rara-bdd run --filter AC-01
rara-bdd run --features-dir ./features --report json
rara-bdd run --package my-crate
```

Exit: `0` = all pass, `1` = failure or uncovered ACs.

JSON output:

```json
{
  "ok": true,
  "action": "bdd-run",
  "passed": 5, "failed": 0, "uncovered": 0, "total": 5,
  "scenarios": [
    {
      "ac_id": "AC-01",
      "scenario": "AC-01 Valid credentials",
      "feature_file": "auth/login.feature",
      "status": "passed",
      "tests": ["ac_01_valid_credentials", "ac_01_returns_token"]
    }
  ]
}
```

---

## `rara-bdd list`

List scenarios with their matched test functions.

| Flag | Default | Description |
|------|---------|-------------|
| `--features-dir <PATH>` | `features` | Features directory path |
| `--filter <STRING>` | -- | Filter by AC ID, tag, or name |
| `--package <NAME>` | -- | Scope test discovery to a specific crate |

```bash
rara-bdd list
rara-bdd list --filter auth
rara-bdd list --package my-crate
```

---

## `rara-bdd coverage`

Report coverage gaps -- which ACs have no matching test.

| Flag | Default | Description |
|------|---------|-------------|
| `--features-dir <PATH>` | `features` | Features directory path |
| `--package <NAME>` | -- | Scope test discovery to a specific crate |

```bash
rara-bdd coverage
rara-bdd coverage --package my-crate
```

```json
{
  "ok": true,
  "action": "coverage",
  "total": 10,
  "covered": 8,
  "uncovered": 2,
  "uncovered_ids": ["AC-04", "AC-09"]
}
```

Exit: `0` = full coverage, `1` = uncovered ACs exist.

---

## `rara-bdd trace`

Generate `TRACEABILITY.md` in features directory.

| Flag | Default | Description |
|------|---------|-------------|
| `--features-dir <PATH>` | `features` | Features directory path |
| `--package <NAME>` | -- | Scope test discovery to a specific crate |

```bash
rara-bdd trace
rara-bdd trace --package my-crate
```

```json
{"ok": true, "action": "trace", "scenarios": 15}
```

---

## Filtering

`--filter` matches case-insensitively against AC ID, scenario name, and tags:

```bash
--filter AC-01       # exact AC
--filter auth        # tag or name containing "auth"
--filter "credentials"  # scenario name substring
```
