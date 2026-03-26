---
name: bdd-design
description: "Use when designing BDD features, writing Gherkin acceptance criteria, decomposing requirements into testable scenarios, creating GitHub issues with .feature specs, or specifying acceptance criteria for the implementation agent. Keywords: BDD design, Gherkin, acceptance criteria, feature spec, scenario design, requirement decomposition."
---

# BDD Design Skill

**IRON LAW: Every step must be specific enough that an agent can derive test assertions from the step text alone.** If a step says "it works" or "the result is correct", it is too vague — rewrite it with concrete, verifiable expectations.

You are Agent 1 (the design agent). Produce a GitHub issue containing Gherkin acceptance criteria and a design spec that Agent 2 (the implementation agent) will use to deliver working code with passing BDD tests.

## Workflow Checklist

Copy and track progress:

```
- [ ] ⚠️ 1. Understand the codebase (read AGENT.md, CLAUDE.md, src/, features/)
- [ ] ⚠️ 2. Check existing step definitions (rara-bdd list)
- [ ] ⚠️ 3. Write Gherkin .feature content (≥3 scenarios)
- [ ] 4. Write design spec (files, signatures, constraints)
- [ ] ⚠️ 5. Self-check (all checklist items pass)
- [ ] ⛔ 6. Confirm with user before creating issue
- [ ] 7. Create the GitHub issue
```

### 1. Understand the Codebase ⚠️

Read the project context before writing anything:

- Read `AGENT.md` — project conventions and architecture
- Read `CLAUDE.md` — dev workflow and commands
- Browse `src/` — source structure
- Browse `features/` — existing .feature files (if any)

### 2. Check Existing Step Definitions ⚠️

Reuse existing steps — do NOT create duplicates.

```bash
rara-bdd list
```

If an existing step matches your intent (even partially), reuse its exact wording.

### 3. Write Gherkin .feature Content ⚠️

Create complete Gherkin content with **at least 3 scenarios**: one happy path, one error case, and one edge case.

#### Rules

- Tag the feature with `@module-name` (e.g., `@auth`, `@billing`)
- Tag each scenario with `@AC-XX` (e.g., `@AC-01`, `@AC-02`)
- Steps must be concrete and verifiable — can an agent derive test logic from the step text alone?
- Use `"quoted strings"` for string parameters, bare integers for number parameters
- Reuse existing step definitions discovered via `rara-bdd list`
- Use `Given` for preconditions, `When` for actions, `Then` for assertions
- Use `And` / `But` for continuation within the same keyword group

#### Good vs Bad Examples

```gherkin
# BAD — too vague, agent cannot derive assertions
@AC-01
Scenario: User logs in
  Given a user exists
  When they log in
  Then it works

# BAD — implementation detail in steps (leaks internal API)
@AC-01
Scenario: User logs in
  Given a row in the users table with id 1
  When POST /api/v1/sessions with JSON body {"user_id": 1}
  Then the sessions table has a new row

# GOOD — specific, testable, no implementation leak
@AC-01
Scenario: Valid credentials return a session token
  Given a registered user with email "alice@example.com"
  When the user logs in with correct credentials
  Then the response status is 200
  And the response body contains a non-empty "token" field

# GOOD — error case with clear expected behavior
@AC-02
Scenario: Invalid password returns 401
  Given a registered user with email "alice@example.com"
  When the user logs in with password "wrong-password"
  Then the response status is 401
  And the response body contains error "invalid credentials"

# GOOD — edge case
@AC-03
Scenario: Login with non-existent email returns 404
  Given no user is registered with email "ghost@example.com"
  When the user logs in with email "ghost@example.com"
  Then the response status is 404
```

### 4. Write the Design Spec

Provide a concise design spec that tells the implementation agent exactly what to build:

- **Files to create or modify** — list every file path
- **Interface signatures** — public function/struct/trait signatures the implementation must expose
- **Constraints** — invariants, error handling strategy, performance requirements
- **Dependencies** — any new crates or services required

### 5. Self-Check ⚠️

Before presenting to the user, verify:

- [ ] Valid Gherkin syntax (Feature/Scenario/Given/When/Then structure)
- [ ] At least 3 scenarios (happy path + error + edge case)
- [ ] Steps are concrete and verifiable (not vague)
- [ ] Existing steps reused where applicable (checked via `rara-bdd list`)
- [ ] `"quoted strings"` for string params, bare integers for numbers
- [ ] Each scenario tagged with `@AC-XX`
- [ ] Feature tagged with `@module-name`
- [ ] Design spec lists all files to create/modify with signatures

### 6. Confirm with User ⛔

Present the complete Gherkin content and design spec to the user. Wait for explicit approval before creating the GitHub issue. Do NOT create the issue without confirmation.

### 7. Create the GitHub Issue

Use the `bdd_task.yml` issue template:

```bash
gh issue create --template bdd_task.yml \
  --title "feat(scope): short description" \
  --body "$(cat <<'EOF'
### Description

What should be implemented and why.

### .feature (Acceptance Criteria)

```gherkin
@module-name
Feature: Feature title

  @AC-01
  Scenario: Happy path description
    Given ...
    When ...
    Then ...

  @AC-02
  Scenario: Error case description
    Given ...
    When ...
    Then ...

  @AC-03
  Scenario: Edge case description
    Given ...
    When ...
    Then ...
```

### Scope

- `src/module.rs` — create: new module with XxxTrait
- `src/lib.rs` — modify: add `pub mod module;`
- `features/module.feature` — create: copy from above

### Additional Context

Design constraints, dependencies, notes.
EOF
)" --label "agent:claude" --label "enhancement" --label "core"
```

**Required labels:**
- Agent label: `agent:claude` (or whichever agent is performing this)
- Type label: auto-applied by template (`enhancement`)
- Component label: one of `core`, `backend`, `frontend`, `cli`, `ci`, `docs`

## Anti-Patterns

- Writing vague steps like "it works", "result is correct", "operation succeeds"
- Leaking implementation details into steps (SQL, HTTP methods, table names)
- Ignoring existing step definitions and creating duplicates
- Writing only the happy path — always include error + edge cases
- Creating the issue without user confirmation
