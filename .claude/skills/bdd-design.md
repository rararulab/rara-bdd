---
name: bdd-design
description: Decompose a requirement into a GitHub issue with Gherkin acceptance criteria and a design spec for the implementation agent.
---

# BDD Design Skill

You are Agent 1 (the design agent). Your job is to analyze a requirement and produce a GitHub issue containing Gherkin acceptance criteria and a design spec that Agent 2 (the implementation agent) will use to deliver working code with passing BDD tests.

## Workflow

### 1. Understand the Codebase

Before writing anything, read the project context:

```bash
cat AGENT.md                    # Project conventions and architecture
cat CLAUDE.md                   # Dev workflow and commands
ls src/                         # Source structure
ls features/ 2>/dev/null        # Existing .feature files
```

### 2. Check Existing Step Definitions

Reuse existing steps whenever possible — do NOT create duplicates.

```bash
rara-bdd list                   # Show all features, scenarios, and steps
```

Review the output. If an existing step matches your intent (even partially), reuse its exact wording in your new scenarios.

### 3. Write Gherkin .feature Content

Create complete Gherkin content with **at least 3 scenarios**: one happy path, one error case, and one edge case.

#### Rules

- Tag the feature with `@module-name` (e.g., `@auth`, `@billing`)
- Tag each scenario with `@AC-XX` (e.g., `@AC-01`, `@AC-02`)
- Steps must be concrete and verifiable — an agent must be able to derive test logic from the step text alone
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

### 5. Create the GitHub Issue

Use the `feature.yml` issue template. The issue must include the Gherkin content and the design spec.

```bash
gh issue create --template feature.yml \
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

### 6. Self-Check

Before submitting, verify:

- [ ] Valid Gherkin syntax (Feature/Scenario/Given/When/Then structure)
- [ ] At least 3 scenarios (happy path + error + edge case)
- [ ] Steps are concrete and verifiable (not vague)
- [ ] Existing steps reused where applicable (checked via `rara-bdd list`)
- [ ] `"quoted strings"` for string params, bare integers for numbers
- [ ] Each scenario tagged with `@AC-XX`
- [ ] Feature tagged with `@module-name`
- [ ] Issue has agent + type + component labels
- [ ] Design spec lists all files to create/modify with signatures
