# raea-skills

The official skills library for [raea](https://raea.io), the Render Anything
Engine — agent skills for video editing and media work.

Skills teach an agent **how to use raea well** without deciding **what the
result should look like**. raea's variety is the point; see
[docs/RULES.md](docs/RULES.md).

## Quick start

```bash
just doctor   # are the tools installed?
just setup    # build the checker
just ci       # run every check
```

## Needs

| Tool | Why | Install |
|---|---|---|
| raea | the engine | `curl -fsSL https://raea.io/install \| sh` |
| Rust (cargo) | the checker + Gherkin tests | `curl https://sh.rustup.rs -sSf \| sh` |
| just | runs the recipes | `brew install just` |
| uv | runs montology (vocabulary) | `brew install uv` |
| gh | GitHub | `brew install gh` |

## Everyday recipes

| Command | Does |
|---|---|
| `just new cut-by-words` | start a skill from the template |
| `just check` | lint skills: format, raea drift, opinion creep |
| `just test` | run the Gherkin behaviour specs |
| `just link` | install this repo's skills into Claude Code, live |
| `just drift` | after a raea update: re-snapshot, list stale skills |
| `just word cut` | check a term with montology before using it |
| `just plugin` | check the Claude plugin (`skills/`, `directory/`) with Claude Code's own validator |
| `just directory` | rebuild `directory/`, the plugin directory's edition, after changing `skills/raea` |

## Use it as a Claude plugin

`skills/` is raea's plugin for Claude: the skills its manifest lists, plus hooks that run raea's quick check
after a composition is saved and send media to `raea show`. Install it in Claude Code with
`/plugin install raea --marketplace shinyobjectz/raea-skills`; [skills/README.md](skills/README.md) says what it runs.
A new skill goes in `skills/.claude-plugin/plugin.json`'s `skills` too, or `just check` fails.

`directory/` is the edition Claude's plugin directory takes: the same skill with no hooks and without its TypeScript
kits, which the directory accepts only from verified organizations. `scripts/directory-edition.sh` builds it from
`skills/raea`, so run `just directory` after changing the skill and commit the result; `just ci` warns when it's stale.

## Layout

```
skills/         one folder per skill (SKILL.md); the folder is also raea's Claude plugin
directory/      the plugin as Claude's plugin directory takes it, built by `just directory` (don't edit by hand)
features/       behaviour specs in Gherkin (Given / When / Then)
crates/         skillcheck — the Rust checker and test runner
upstream/raea/  snapshot of the raea these skills are checked against
docs/           RULES · SOP · HOW-SKILLS-WORK · TREE (how the raea skill is laid out, covered and deepened)
```

## Contributing

Follow [docs/SOP.md](docs/SOP.md). New to skills? Start with
[docs/HOW-SKILLS-WORK.md](docs/HOW-SKILLS-WORK.md).
