# CLI Reference

## `rara-bdd run`

Execute scenarios and evaluate acceptance criteria.

| Flag | Default | Description |
|------|---------|-------------|
| `--features-dir <PATH>` | `features` | Features directory path |
| `--filter <STRING>` | — | Filter by AC ID, tag, or name (case-insensitive substring) |
| `--report <FORMAT>` | `terminal` | `terminal` / `json` / `markdown` |
| `--mock` | `false` | CI-safe mode — skip external dependencies |

```bash
rara-bdd run
rara-bdd run --filter AC-01
rara-bdd run --features-dir ./features --report json
rara-bdd run --mock
```

Exit: `0` = all pass, `1` = failure or error.

JSON output:

```json
{
  "ok": true,
  "action": "bdd-run",
  "passed": 5, "failed": 0, "total": 5,
  "scenarios": [
    {
      "ac_id": "AC-01",
      "scenario": "AC-01 Valid credentials",
      "feature_file": "auth/login.feature",
      "passed": true,
      "message": "AC-01: acceptance criterion verified green"
    }
  ]
}
```

---

## `rara-bdd list`

List discovered scenarios without executing.

| Flag | Default | Description |
|------|---------|-------------|
| `--features-dir <PATH>` | `features` | Features directory path |
| `--filter <STRING>` | — | Filter by AC ID, tag, or name |

```bash
rara-bdd list
rara-bdd list --filter auth
```

---

## `rara-bdd validate`

Schema-check `.eval.yaml` files without running assertions.

| Flag | Default | Description |
|------|---------|-------------|
| `--features-dir <PATH>` | `features` | Features directory path |

```bash
rara-bdd validate
```

```json
{"ok": true, "action": "validate", "features": 3, "evals": 3, "errors": []}
```

---

## `rara-bdd trace`

Generate `TRACEABILITY.md` in features directory.

| Flag | Default | Description |
|------|---------|-------------|
| `--features-dir <PATH>` | `features` | Features directory path |

```bash
rara-bdd trace
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
