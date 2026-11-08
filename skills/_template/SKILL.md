---
name: SKILL_NAME
description: One or two sentences. What this skill helps an agent DO with raea, and WHEN to use it (the words a person might say). The agent reads only this line to decide whether to load the skill, so make it specific.
metadata:
  raea-tested-against: RAEA_VERSION
  raea-verbs: render check
---

# SKILL_NAME

<!-- Delete these comments as you fill each section. See docs/SOP.md. -->

## What this is for

<!-- The job, in a sentence or two. Name the outcome, not a look. -->

## Before you start

<!-- What the agent must read or have first: `raea context PROJECT`, the brief,
     the person's taste.ts. Taste comes from THERE, never from this file. -->

## Steps

<!-- Numbered. Each step names the raea command it uses and where it runs
     (HEADLESS / HOST / PAGE / PERSON, as raea's own skill does). -->

1.

## Choices to leave open

<!-- The decisions an editor would make differently every time (pace, palette,
     type, music feel). List them as choices with their trade-offs, so the agent
     asks or reads taste.ts instead of picking one for everyone. -->

## How to check the result

<!-- Objective checks only: `raea check`, safe areas, loudness, legibility,
     sync. Anything subjective is the person's call. -->

## Gotchas

<!-- Real failure modes you've seen as an editor. These are the most valuable
     lines in the skill. -->
