#!/usr/bin/env bash
# Builds directory/skills/raea, the raea skill as Claude's plugin directory takes it from an organization Anthropic
# hasn't verified: Markdown, text, JSON, YAML and images only, and no hooks. So the skill's TypeScript kits
# (craft/checks, craft/match-cuts) stay out, each folder gets a README saying where the kit is, and each NOTICE is
# copied as NOTICE.txt. skills/ stays the whole plugin (kits and hooks), which
# `/plugin install raea --marketplace shinyobjectz/raea-skills` installs.
#
#   scripts/directory-edition.sh           rebuild directory/skills/raea from skills/raea
#   scripts/directory-edition.sh --check   exit 1 when directory/skills/raea isn't what a rebuild would make
set -euo pipefail
cd "$(dirname "$0")/.."

build() {
  local out=$1/raea
  mkdir -p "$out"
  (cd skills/raea && find . -type f | LC_ALL=C sort) | while IFS= read -r f; do
    f=${f#./}
    case "$f" in
      *.ts | *.tsx) continue ;;
      NOTICE | */NOTICE) dst="$out/$f.txt" ;;
      *) dst="$out/$f" ;;
    esac
    mkdir -p "$(dirname "$dst")"
    cp "skills/raea/$f" "$dst"
  done
  for kit in craft/checks craft/match-cuts; do
    mkdir -p "$out/$kit"
    cat >"$out/$kit/README.md" <<EOF
# ${kit#craft/}: a kit of TypeScript files

This edition of the raea skill comes from Claude's plugin directory, which carries Markdown, text, JSON, YAML and
images only, so the kit's TypeScript files aren't here. Where a guide says to copy them from this folder into a
project, take them from either of these instead:

- the raea skill that raea's installer puts in \`~/.claude/skills/raea/$kit/\` (and keeps in
  \`~/.raea/share/raea/skills/raea/$kit/\`),
- or GitHub: https://github.com/shinyobjectz/raea-skills/tree/main/skills/raea/$kit
EOF
  done
}

if [ "${1:-}" = --check ]; then
  tmp=$(mktemp -d)
  trap 'rm -rf "${tmp:?}"' EXIT
  build "$tmp"
  if diff -r -q "$tmp/raea" directory/skills/raea >/dev/null 2>&1; then
    echo "directory/skills/raea matches skills/raea"
  else
    diff -r -q "$tmp/raea" directory/skills/raea 2>&1 | sed "s|$tmp/raea|(rebuilt)|" | head -20
    echo "directory/skills/raea is stale: run just directory"
    exit 1
  fi
else
  rm -rf "directory/skills/raea"
  build directory/skills
  echo "built directory/skills/raea from skills/raea"
fi
