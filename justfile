# raea-skills — run `just` to see every recipe.

set shell := ["bash", "-uc"]

# List recipes
default:
    @just --list --unsorted

# ── Setup ──────────────────────────────────────────────

# Check that every tool this repo needs is installed
doctor:
    #!/usr/bin/env bash
    ok=1
    for t in git cargo just raea uv gh; do
      if command -v "$t" >/dev/null 2>&1; then
        printf "  ok   %-6s %s\n" "$t" "$("$t" --version 2>&1 | head -1)"
      else
        printf "  MISSING %s\n" "$t"; ok=0
      fi
    done
    [ "$ok" = 1 ] && echo "All tools present." || { echo "Install the missing tools (see README)."; exit 1; }

# Build the Rust checker (first run downloads crates)
setup:
    cargo build --workspace

# ── Daily work ─────────────────────────────────────────

# Lint every skill: format, raea drift, opinion creep
check:
    cargo run -q -p skillcheck --

# Same, but warnings fail too (use before a release)
check-strict:
    cargo run -q -p skillcheck -- --strict

# Run the Gherkin behaviour specs in features/ (plus unit tests)
test:
    cargo test --workspace

# Format and lint the Rust code
fmt:
    cargo fmt --all
    cargo clippy --workspace --all-targets -- -D warnings

# Everything CI runs
ci: test check
    cargo fmt --all -- --check

# ── Skills ─────────────────────────────────────────────

# Start a new skill from the template: just new cut-by-words
new name:
    #!/usr/bin/env bash
    set -euo pipefail
    [[ "{{name}}" =~ ^[a-z0-9]+(-[a-z0-9]+)*$ ]] || { echo "Name must be lowercase-with-hyphens"; exit 1; }
    [ -e "skills/{{name}}" ] && { echo "skills/{{name}} already exists"; exit 1; }
    cp -R skills/_template "skills/{{name}}"
    version="$(cat upstream/raea/VERSION 2>/dev/null || echo unknown)"
    sed -i '' -e "s/SKILL_NAME/{{name}}/g" -e "s/RAEA_VERSION/${version}/g" "skills/{{name}}/SKILL.md"
    echo "Created skills/{{name}}/SKILL.md (tested-against: ${version})"

# Install this repo's skills into ~/.claude/skills as links, so edits apply live
link:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p ~/.claude/skills
    for d in skills/*/; do
      n="$(basename "$d")"; [[ "$n" == _* ]] && continue
      [ -f "skills/$n/SKILL.md" ] || continue  # the plugin's hooks/ isn't a skill
      ln -sfn "$PWD/skills/$n" ~/.claude/skills/"$n"
      echo "linked $n"
    done

# Remove the links `just link` made
unlink:
    #!/usr/bin/env bash
    for d in skills/*/; do
      n="$(basename "$d")"
      [ -L ~/.claude/skills/"$n" ] && rm ~/.claude/skills/"$n" && echo "unlinked $n"
    done; true

# ── The Claude plugin ──────────────────────────────────

# Check the plugin (skills/) and its marketplace with Claude Code's own validator
plugin:
    claude plugin validate skills
    claude plugin validate .

# Try the plugin in one Claude Code session, without installing it
plugin-try:
    claude --plugin-dir skills

# ── Tracking raea ──────────────────────────────────────

# Snapshot the installed raea into upstream/raea/ (run after every raea update)
sync-raea:
    bash scripts/sync-raea.sh

# After updating raea (re-run its installer): re-snapshot and show which skills went stale
drift:
    raea --version
    just sync-raea
    -just check

# ── Vocabulary (montology) ─────────────────────────────

# Check a word before naming anything: just word cut
word name:
    uvx --from "git+https://github.com/shinyobjectz/montology#subdirectory=.monty/cli" monty onto check {{name}}

# Run the montology vocabulary gate
vocab:
    uvx --from "git+https://github.com/shinyobjectz/montology#subdirectory=.monty/cli" monty lint
