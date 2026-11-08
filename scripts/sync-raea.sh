#!/usr/bin/env bash
# Snapshots the installed raea into upstream/raea/ so changes show up as a git diff.
#   upstream/raea/VERSION        raea --version
#   upstream/raea/help.txt       raea help (every verb and flag)
#   upstream/raea/skill/*.md     the skills raea ships, and their generated references
#   upstream/raea/guides.txt     the list of craft guides
set -euo pipefail

out="${1:-upstream/raea}"

if ! command -v raea >/dev/null 2>&1; then
  echo "raea is not installed. Install it: curl -fsSL https://raea.io/install | sh" >&2
  exit 1
fi

mkdir -p "$out/skill"
raea --version | tr -d '[:space:]' > "$out/VERSION"
echo >> "$out/VERSION"
raea help 2>&1 | sed "s#$HOME#~#g" > "$out/help.txt" || true
raea skill > "$out/skill/raea.md"
raea skill raea-bevy > "$out/skill/raea-bevy.md"
for ref in cli feed files sdk; do
  raea skill --reference "$ref" > "$out/skill/reference-$ref.md"
done
raea guide 2>&1 | sed "s#$HOME#~#g" > "$out/guides.txt" || true

echo "Snapshot of raea $(cat "$out/VERSION") written to $out/"
if git rev-parse --git-dir >/dev/null 2>&1; then
  git --no-pager diff --stat -- "$out" || true
fi
