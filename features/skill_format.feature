Feature: A skill is shaped so an agent can load it
  Every skill lives in skills/NAME/SKILL.md and starts with a frontmatter block.
  The agent reads `name` and `description` to decide when to use the skill.

  Background:
    Given raea "0.1.0-canary.3" is the current upstream snapshot

  Scenario: A well-formed skill passes
    Given a skill folder "cut-by-words" with this SKILL.md:
      """
      ---
      name: cut-by-words
      description: Cut talking-head footage at the words a person chooses, with raea cut.
      metadata:
        raea-tested-against: 0.1.0-canary.3
      ---
      # Cut by words
      """
    When I run the skill check
    Then it passes with no warnings

  Scenario: The name must match the folder
    Given a skill folder "captions" with this SKILL.md:
      """
      ---
      name: subtitles
      description: Adds captions.
      metadata:
        raea-tested-against: 0.1.0-canary.3
      ---
      """
    When I run the skill check
    Then it fails
    And an error mentions "must match its folder name"

  Scenario: A description is required
    Given a skill folder "captions" with this SKILL.md:
      """
      ---
      name: captions
      metadata:
        raea-tested-against: 0.1.0-canary.3
      ---
      """
    When I run the skill check
    Then it fails
    And an error mentions "missing `description`"

  Scenario: A folder without SKILL.md fails
    Given an empty skill folder "half-done"
    When I run the skill check
    Then it fails
    And an error mentions "no SKILL.md"

  Scenario: The template folder is not checked
    Given an empty skill folder "_template"
    When I run the skill check
    Then it passes
    And 0 skills were checked
