Feature: The skills folder is a Claude plugin
  skills/ is also raea's plugin for Claude: skills/.claude-plugin/plugin.json names it, and Claude Code, Cowork and
  claude.ai load the skills its `skills` lists. A skill the manifest leaves out never reaches the plugin's users, so
  every skill folder must be listed, and every entry must be a skill. The plugin's own folders (hooks/) aren't skills.

  Background:
    Given raea "0.1.0-canary.3" is the current upstream snapshot

  Scenario: Every skill listed passes
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
    And the plugin manifest lists the skills "./cut-by-words"
    When I run the skill check
    Then it passes with no warnings

  Scenario: A skill the manifest leaves out fails
    Given a skill folder "cut-by-words" with this SKILL.md:
      """
      ---
      name: cut-by-words
      description: Cut talking-head footage at the words a person chooses, with raea cut.
      metadata:
        raea-tested-against: 0.1.0-canary.3
      ---
      """
    And a skill folder "captions" with this SKILL.md:
      """
      ---
      name: captions
      description: Put captions on a cut from its words, with raea.
      metadata:
        raea-tested-against: 0.1.0-canary.3
      ---
      """
    And the plugin manifest lists the skills "./cut-by-words"
    When I run the skill check
    Then it fails
    And an error mentions "skills/captions isn't in the plugin's skills"

  Scenario: A manifest entry with no skill fails
    Given a skill folder "cut-by-words" with this SKILL.md:
      """
      ---
      name: cut-by-words
      description: Cut talking-head footage at the words a person chooses, with raea cut.
      metadata:
        raea-tested-against: 0.1.0-canary.3
      ---
      """
    And the plugin manifest lists the skills "./cut-by-words ./gone"
    When I run the skill check
    Then it fails
    And an error mentions "holds no SKILL.md"

  Scenario: The plugin's hooks folder is not a skill
    Given an empty skill folder "hooks"
    And the plugin manifest lists the skills ""
    When I run the skill check
    Then it passes
    And 0 skills were checked

  Scenario: A manifest that doesn't parse fails
    Given the plugin manifest is "{ not json"
    When I run the skill check
    Then it fails
    And an error mentions "doesn't parse"
