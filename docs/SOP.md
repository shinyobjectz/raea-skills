# SOP: Adding or Changing a Skill

Follow these in order. Each step is small.

## A. New skill (about 30–60 min)

1. **Name it.** Check the word first: `just word <name>`. Lowercase-with-hyphens.
2. **Scaffold it.** `just new <name>` → creates `skills/<name>/SKILL.md`.
3. **Write the description first.** One or two sentences: what it does, and the
   words a person would say that should trigger it.
4. **Fill the sections** in the template. Apply the floor-vs-taste test
   (docs/RULES.md §1) to every sentence.
5. **List the raea commands** it uses in `metadata.raea-verbs`.
6. **Write a behaviour spec** in `features/` if the skill adds a rule worth
   checking (Gherkin: Given / When / Then).
7. **Check it.** `just check` then `just test`. Fix every FAIL. Read every warn.
8. **Try it live.** `just link`, open a new Claude Code session, ask for the task
   in plain words. Does the skill load? Does it work?
9. **Open a pull request.** Fill in the checklist below.

## B. raea updated (about 10 min + re-testing)

1. Update raea: re-run its installer.
2. `just drift` — snapshots raea and lists stale skills.
3. Read the diff: `git diff upstream/` shows exactly what changed in raea.
4. For each stale skill: re-test it (step A8), fix what broke, bump
   `raea-tested-against`.
5. Commit the snapshot and the fixes together: `raea X.Y.Z: <summary>`.

## C. Review checklist (paste into every PR)

- [ ] `just ci` passes
- [ ] Description says what AND when
- [ ] No taste in the skill (fonts, colors, pace, "cinematic") — or it's under "Choices to leave open"
- [ ] Every `opinion-ok` has a reason a reviewer agrees with
- [ ] `raea-tested-against` matches `upstream/raea/VERSION`
- [ ] Tried live in a fresh session

## D. Roles

- **Kelly (lead / creative director):** what the skill is for, gotchas, checks,
  the floor-vs-taste call, final review.
- **Claude (agent):** structure, wording, raea command accuracy, tests,
  flagging when a line reads as taste.
- **shinyobjectz (raea author):** questions about how raea itself is meant to work.
