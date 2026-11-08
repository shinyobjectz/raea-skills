Feature: Skills keep up with raea as it changes
  raea is young and changes often. Each skill records the raea version it was
  tested against and the raea commands it relies on. `just sync-raea` snapshots
  the installed raea into upstream/raea/, and this check compares the two.

  Background:
    Given raea "0.1.0-canary.4" is the current upstream snapshot
    And the upstream raea help says:
      """
      usage: raea render EDIT.tsx -o OUT.mp4
             raea cut TAKE "line" ...
             raea check VIDEO.mp4
      """

  Scenario: A skill tested on an older raea is flagged as stale
    Given a skill folder "captions" with this SKILL.md:
      """
      ---
      name: captions
      description: Adds captions.
      metadata:
        raea-tested-against: 0.1.0-canary.3
      ---
      """
    When I run the skill check
    Then it passes
    And a warning mentions "stale: tested against raea 0.1.0-canary.3"

  Scenario: A skill must say which raea it was tested on
    Given a skill folder "captions" with this SKILL.md:
      """
      ---
      name: captions
      description: Adds captions.
      ---
      """
    When I run the skill check
    Then it fails
    And an error mentions "raea-tested-against"

  Scenario: A skill that relies on a removed raea command fails
    Given a skill folder "rough-cut" with this SKILL.md:
      """
      ---
      name: rough-cut
      description: Builds a rough cut.
      metadata:
        raea-tested-against: 0.1.0-canary.4
        raea-verbs: cut render frame
      ---
      """
    When I run the skill check
    Then it fails
    And an error mentions "uses `raea frame`"

  Scenario: A skill whose commands all exist passes
    Given a skill folder "rough-cut" with this SKILL.md:
      """
      ---
      name: rough-cut
      description: Builds a rough cut.
      metadata:
        raea-tested-against: 0.1.0-canary.4
        raea-verbs: cut render check
      ---
      """
    When I run the skill check
    Then it passes with no warnings
