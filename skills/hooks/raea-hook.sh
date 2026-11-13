#!/bin/sh
# The raea plugin's hooks. Claude Code hands this script the tool call on stdin, before a Bash command and after a
# file is saved, and `raea hook claude` answers it: media opened in the Mac's viewer is pointed at `raea show` once,
# and a saved composition gets raea's quick check, run locally.
#
# It does nothing when raea isn't installed, and nothing when `raea skills install` has already put the same hooks
# in ~/.claude/settings.json, so they never run twice.
command -v raea >/dev/null 2>&1 || exit 0
grep -qs 'raea hook' ~/.claude/settings.json && exit 0
exec raea hook claude
