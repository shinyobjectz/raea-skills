Feature: Skills describe tools, not taste
  raea is non-deterministic on purpose: two runs of the same brief can make two
  different, valid pieces. A skill that hard-codes a look (a fixed color, "always
  use X") narrows that. These checks warn; a human decides. See docs/RULES.md.

  Background:
    Given raea "0.1.0-canary.3" is the current upstream snapshot

  Scenario: A fixed hex color is flagged
    Given a skill folder "lower-thirds" with this SKILL.md:
      """
      ---
      name: lower-thirds
      description: Lower-third name titles.
      metadata:
        raea-tested-against: 0.1.0-canary.3
      ---
      Set the bar to #ff3366 so it pops.
      """
    When I run the skill check
    Then it passes
    And a warning mentions "fixed color #ff3366"

  Scenario: Prescriptive phrasing is flagged
    Given a skill folder "pacing" with this SKILL.md:
      """
      ---
      name: pacing
      description: Pacing a cut.
      metadata:
        raea-tested-against: 0.1.0-canary.3
      ---
      Always use a J-cut between speakers.
      """
    When I run the skill check
    Then a warning mentions "prescribes taste"

  Scenario: A line marked opinion-ok is allowed
    Given a skill folder "safe-areas" with this SKILL.md:
      """
      ---
      name: safe-areas
      description: Keep text inside platform safe areas.
      metadata:
        raea-tested-against: 0.1.0-canary.3
      ---
      Always use the platform's safe area for text. <!-- opinion-ok: platform spec, not taste -->
      """
    When I run the skill check
    Then it passes with no warnings
