---
name: bdd-implement
description: "Use when implementing a BDD-specced GitHub issue, writing step definitions, delivering code with passing cucumber tests, or picking up an agent-designed feature task. Keywords: BDD implement, step definition, cucumber test, implement issue, feature implementation, agent task."
---

# BDD Implement Skill

**IRON LAW: NEVER modify the .feature file.** The .feature content is Agent 1's contract. If a scenario seems wrong, add a `needs-design-review` label to the issue and comment — do not change the Gherkin.

You are Agent 2 (the implementation agent). Take a GitHub issue created by Agent 1 and deliver working code with all BDD tests passing.

## Workflow Checklist

Copy and track progress:

```
- [ ] ⚠️ 1. Read the issue (extract Gherkin + design spec)
- [ ] 2. Create worktree
- [ ] 3. Run rara-bdd setup (idempotent)
- [ ] ⛔ 4. Write .feature file (VERBATIM from issue — never modify)
- [ ] 5. Run rara-bdd generate
- [ ] ⚠️ 6. Run rara-bdd coverage (must exit 0)
- [ ] 7. Implement: types → logic → integration → steps
- [ ] ⚠️ 8. Verify: coverage + test + clippy (ALL three must pass)
- [ ] ⛔ 9. Confirm with user before push & PR
- [ ] 10. Push & create PR
- [ ] ⚠️ 11. Wait for CI green
```

### 1. Read the Issue ⚠️

```bash
gh issue view <N>
```

Extract:
- The `.feature` Gherkin content (acceptance criteria)
- The design spec (files to create/modify, interface signatures, constraints)
- The component label (to know which area you are working in)

### 2. Create Worktree

Follow the standard org workflow:

```bash
git worktree add .worktrees/issue-<N>-<short-name> -b issue-<N>-<short-name>
cd .worktrees/issue-<N>-<short-name>
```

All work happens inside the worktree. Never edit files in the main checkout.

### 3. Run Setup (Idempotent)

```bash
rara-bdd setup
```

Safe to run even if already set up. Ensures `features/`, `tests/bdd.rs`, `tests/steps/mod.rs`, and Cargo.toml dependencies are in place.

### 4. Write the .feature File ⛔

Copy the Gherkin content from the issue **verbatim** into `features/<name>.feature`. Do NOT modify, reword, or reorder the scenarios.

### 5. Generate Step Skeletons

```bash
rara-bdd generate
```

Creates `tests/steps/<name>_steps.rs` with `todo!()` bodies for each step.

### 6. Check Coverage ⚠️

```bash
rara-bdd coverage
```

All steps from the .feature file must have skeleton definitions. If any are missing, run `rara-bdd generate` again.

### 7. Implement

Follow this order to minimize compile errors:

1. **Data types** — structs, enums, error types referenced by the design spec
2. **Core logic** — the main business logic (functions, trait implementations)
3. **Integration** — wiring (module declarations, re-exports, dependency injection)
4. **Step definitions** — replace every `todo!()` in `tests/steps/<name>_steps.rs`

#### TestWorld Usage

The `TestWorld` struct in `tests/bdd.rs` carries state between steps:

- **Given steps** (setup): Initialize state in `TestWorld` fields
- **When steps** (action): Execute the operation, store results in `TestWorld`
- **Then steps** (assert): Read from `TestWorld` and assert expected outcomes

```rust
#[given(expr = "a registered user with email {string}")]
async fn a_registered_user_with_email(world: &mut TestWorld, email: String) {
    world.user_email = Some(email);
}

#[when("the user logs in with correct credentials")]
async fn the_user_logs_in(world: &mut TestWorld) {
    let email = world.user_email.as_ref().expect("email set in Given step");
    world.response = Some(login(email, "correct-password").await);
}

#[then(expr = "the response status is {int}")]
async fn the_response_status_is(world: &mut TestWorld, expected: i32) {
    let response = world.response.as_ref().expect("response set in When step");
    assert_eq!(response.status, expected as u16);
}
```

Add new fields to `TestWorld` as needed. Keep it minimal — only fields shared between steps.

### 8. Verify ⚠️

All three checks must pass before proceeding:

```bash
rara-bdd coverage                # Exit 0 = all steps covered
cargo test --test bdd            # All scenarios pass
cargo clippy --workspace --all-targets --all-features --no-deps -- -D warnings
```

### 9. Confirm with User ⛔

Present a summary of what was implemented (files changed, scenarios passing) and ask for confirmation before pushing. Do NOT push without approval.

### 10. Push & Create PR

```bash
git add -A
git commit -m "$(cat <<'EOF'
feat(scope): implement feature description (#N)

Implement Gherkin acceptance criteria from issue #N.
All BDD scenarios pass.

Closes #N

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>
EOF
)"

git push -u origin issue-<N>-<short-name>

gh pr create --title "feat(scope): implement feature description (#N)" --body "$(cat <<'PR_EOF'
## Summary

Implement acceptance criteria from #N.

## Type of change

| Type | Label |
|------|-------|
| New feature | `enhancement` |

## Component

`core`

## Closes

Closes #N

## Test plan

- [x] `rara-bdd coverage` — zero missing steps
- [x] `cargo test --test bdd` — all scenarios pass
- [x] `cargo clippy` — no warnings
PR_EOF
)" --label "enhancement" --label "core"
```

### 11. Wait for CI Green ⚠️

```bash
gh pr checks <PR-number> --watch
```

Do NOT report completion until all CI checks pass. If a check fails, investigate and fix in the worktree, push again, and re-verify.

## Failure Handling

| Failure | Action | Max Retries |
|---------|--------|-------------|
| Compile error | Read error output, fix the code, rebuild | 3 |
| Missing step definition | Run `rara-bdd generate`, then implement | 1 |
| Assertion failure in BDD test | Analyze expected vs actual, fix logic | 3 |
| Clippy warning | Fix the warning | 3 |
| Retries exhausted | Add `needs-human` label + post analysis comment on the issue | -- |

When retries are exhausted:

```bash
gh issue edit <N> --add-label "needs-human"
gh issue comment <N> --body "$(cat <<'EOF'
## Agent stuck — needs human review

**Failure type:** <compile error | assertion failure | ...>
**Attempts:** 3/3
**Last error:**
```
<paste last error output>
```

**Analysis:**
<what the agent tried and why it did not work>
EOF
)"
```

## Anti-Patterns

- Modifying the .feature file for any reason (add `needs-design-review` label instead)
- Implementing steps before data types and core logic (causes compile cascade)
- Pushing without all three verification checks passing
- Silently giving up when retries are exhausted (must add `needs-human` label)
- Adding unnecessary fields to `TestWorld` (keep it minimal)
- Pushing or creating PRs without user confirmation
