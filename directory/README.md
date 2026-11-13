# raea for Claude

raea is the Render Anything Engine. You write a video, a motion graphic, a game or a 3D scene as React and TypeScript,
and raea renders it on your computer's GPU with an engine built on Bevy. The same file renders to video, plays live as
a game, or renders single frames to check. This plugin teaches Claude to use raea well.

## What's in it

**The raea skill** (`skills/raea/`). It covers how to install raea, write a composition, render and check it, and what
each kind of work needs: footage cut by its words, motion graphics and type, sound and music, 3D scenes (Bevy
materials, lights, physics, Gaussian splats), games, app interfaces, generated media, and delivery. Its `reference/`
files are generated from raea's code, so they list the real verbs, flags and SDK imports.

This is the directory's edition of the skill, so it holds Markdown, text, JSON and images only. Two TypeScript kits
the guides copy into a project (`craft/checks` and `craft/match-cuts`) aren't in it; each of those folders has a README
saying where to get the kit.

## Install raea

The skill uses the `raea` command, which you install yourself:

- macOS (Apple Silicon) or Linux (x86_64): `curl -fsSL https://raea.io/install | sh`
- Windows 10 or 11, in PowerShell: `irm https://raea.io/install.ps1 | iex`

The skill tells Claude how to install it when you ask for something that needs it. raea's installer also copies the
whole skill, kits included, into `~/.claude/skills`.

raea has two hooks for Claude Code: before a shell command, media opened in the Mac's viewer goes to `raea show`
instead, and after a composition is saved, raea's quick check runs on it. This plugin doesn't add them. To add them,
run `raea skills install --agent claude`, or install the whole plugin from GitHub in Claude Code:

```
/plugin install raea --marketplace shinyobjectz/raea-skills
```

## Where it works

The skill loads in Claude Code, in Cowork and in chat on claude.ai. Rendering needs raea on a computer, so in chat the
skill can explain and write compositions but can't render them.

## What it runs and sends

The plugin itself runs nothing, sends nothing and stores nothing: it's a skill, text Claude reads.

The skill tells Claude to run the `raea` command, which works on your computer. A few of raea's verbs reach the
network when you ask for them: the installer downloads raea from raea.io, `raea gen` calls the generation providers you
set up with your own keys (and spends your money there, so the skill has Claude quote the cost first), and a review
link made in the raea app uploads the cut you choose to share. raea's privacy policy is at https://raea.io/privacy/.

## Source and license

Built from the `skills/` folder of [shinyobjectz/raea-skills](https://github.com/shinyobjectz/raea-skills) by
`scripts/directory-edition.sh`. Apache-2.0; the copied guides keep their notices (`skills/raea/3d/NOTICE.txt`,
`skills/raea/craft/NOTICE.txt`). Questions and bugs: [issues](https://github.com/shinyobjectz/raea-skills/issues).
