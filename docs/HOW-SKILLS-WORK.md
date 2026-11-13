# How Skills Work (the 5-minute version)

## 1. A skill is a folder with one Markdown file

```
skills/cut-by-words/
  SKILL.md          ← required
  reference/…       ← optional extra files the skill can point to
```

## 2. The top of SKILL.md is the "label"

```yaml
---
name: cut-by-words
description: Cut talking-head footage at chosen words with raea cut. Use when…
---
```

The agent **always** sees every skill's `name` + `description` — and nothing
else. It's like reading the spine of every book on a shelf.

## 3. The rest loads only when needed

When your request matches a description, the agent opens that book and reads
the body. So:

- **Description** = when to use it. Must be specific.
- **Body** = how to do it. Can be long; it only costs anything when used.
- **reference/ files** = deeper detail, read only if the body points there.

This is called *progressive disclosure*: small label, bigger body, deepest detail.

## 4. Where skills live on your machine

| Place | Who gets it |
|---|---|
| `~/.claude/skills/` | You, in every project |
| `<project>/.claude/skills/` | Anyone working in that project |
| A plugin (like i-have-adhd) | Whoever installs the plugin |

This repo's `skills/` folder is itself a plugin, `raea`, listed in Claude's plugin
directory: whoever installs it gets the skills its manifest
(`skills/.claude-plugin/plugin.json`) lists, plus raea's hooks.

`just link` puts this repo's skills into `~/.claude/skills/` as links, so your
edits apply the next session — no copying.

## 5. How to call one

- **Automatic:** just ask in plain words; the agent picks the skill.
- **By name:** type `/cut-by-words`.

## 6. What makes a good skill

- Description names the job AND the trigger words.
- Steps are numbered and name the exact command.
- Gotchas come from real experience.
- It teaches a tool, not a taste (see docs/RULES.md).

## 7. Skills you already have

- `raea` — raea's core skill, in skills/ here, 3D included (`raea/3d.md` and `raea/3d/`). raea's installer and `raea skills install` ship it, at the commit raea pins. Its generated parts (`raea/reference/`, and the 3D topic files in `raea/3d/`, copied from raea's craft guides; all but `rows.md` and `plugins.md`) come from raea's `tools/skills.sh gen`: change those in raea, not here.
- `montology`, `words`, `intake` — vocabulary tools.
- `i-have-adhd` — shapes how Claude talks to you (always on).
