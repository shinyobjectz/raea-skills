# raea for Claude

raea is the Render Anything Engine. You write a video, a motion graphic, a game or a 3D scene as React and TypeScript,
and raea renders it on your computer's GPU with an engine built on Bevy. The same file renders to video, plays live as
a game, or renders single frames to check. This plugin teaches Claude to use raea well and connects Claude Code to it.

## What's in it

- **The raea skill** (`raea/`). It covers how to install raea, write a composition, render and check it, and what each
  kind of work needs: footage cut by its words, motion graphics and type, sound and music, 3D scenes (Bevy materials,
  lights, physics, Gaussian splats), games, app interfaces, generated media, and delivery. Its `reference/` files are
  generated from raea's code, so they list the real verbs, flags and SDK imports.
- **Two hooks** (`hooks/`), for Claude Code and Cowork. Before Claude runs a shell command, a command that opens a
  still, a video or a sound in the Mac's viewer is stopped once and Claude is told to use `raea show`, which puts it in
  the raea app where you can answer it; the same command again goes through. This happens only while the raea app is
  installed. After Claude saves a composition (a `.tsx` file that calls `registerRoot`), `raea check --quick` runs on
  it and Claude gets what it finds.

## Install the plugin

Install it from this repository in Claude Code (2.1.275 or later):

```
/plugin install raea --marketplace shinyobjectz/raea-skills
```

Claude's plugin directory carries an edition of this plugin without the hooks and the skill's TypeScript kits
(`directory/` in the repository), since it takes neither from an organization Anthropic hasn't verified. Once it's
listed, you can add that edition from **Customize > Plugins** on claude.ai, or with `/plugin directory` in Claude Code.

## Install raea

The plugin uses the `raea` command, which you install yourself:

- macOS (Apple Silicon) or Linux (x86_64): `curl -fsSL https://raea.io/install | sh`
- Windows 10 or 11, in PowerShell: `irm https://raea.io/install.ps1 | iex`

Until `raea` is on your PATH the hooks do nothing, and the skill tells Claude how to install it when you ask for
something that needs it. raea's installer also copies the skill into `~/.claude/skills`; with the plugin you can set
`RAEA_NO_SKILLS=1` when you install raea to skip that copy.

If you already ran `raea skills install --agent claude`, the same two hooks are in `~/.claude/settings.json`, and the
plugin's hook script steps aside so they never run twice.

## Where it works

| | Skill | Hooks |
|---|---|---|
| Claude Code | yes | yes |
| Cowork | yes | yes, when raea is installed where the session runs |
| Chat on claude.ai | yes | no (chat doesn't run hooks) |

Rendering needs raea on a computer, so in chat the skill can explain and write compositions but can't render them.

## What it runs and sends

The plugin itself sends nothing anywhere and stores nothing. Its hook script (`hooks/raea-hook.sh`) runs
`raea hook claude` on your computer, which reads the tool call Claude Code hands it, may run raea's quick check on the
saved file, and keeps a small marker under `~/.raea/hooks` so a command it stopped goes through when tried again. It
reads `~/.claude/settings.json` only to see whether raea's hooks are already there.

The skill tells Claude to run the `raea` command, which works on your computer. A few of raea's verbs reach the
network when you ask for them: the installer downloads raea from raea.io, `raea gen` calls the generation providers you
set up with your own keys (and spends your money there, so the skill has Claude quote the cost first), and a review
link made in the raea app uploads the cut you choose to share. raea's privacy policy is at https://raea.io/privacy/.

## Source and license

This folder is the `skills/` folder of
[shinyobjectz/raea-skills](https://github.com/shinyobjectz/raea-skills), where the skill is written and checked.
Apache-2.0; the copied guides keep their notices (`raea/3d/NOTICE`, `raea/craft/NOTICE`). Questions and bugs:
[issues](https://github.com/shinyobjectz/raea-skills/issues).
