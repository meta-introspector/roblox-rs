#!/usr/bin/env bash
# gist.sh — post to / read from GitHub Gists via gh CLI
# Usage:
#   ./scripts/gist.sh post <file> [description]   — create public gist
#   ./scripts/gist.sh update <gist-id> <file>      — update existing gist
#   ./scripts/gist.sh read <gist-id>               — read gist content
#   ./scripts/gist.sh list [limit]                  — list recent gists
#   ./scripts/gist.sh report                        — post CI analysis as gist
set -euo pipefail

CMD="${1:-help}"
shift || true

case "$CMD" in
  post)
    FILE="${1:?usage: gist.sh post <file> [description]}"
    DESC="${2:-Monster Gyroscope report $(date -u +%Y%m%d_%H%M%S)}"
    gh gist create --public -d "$DESC" "$FILE"
    ;;
  update)
    GID="${1:?usage: gist.sh update <gist-id> <file>}"
    FILE="${2:?usage: gist.sh update <gist-id> <file>}"
    gh gist edit "$GID" -a "$FILE"
    ;;
  read)
    GID="${1:?usage: gist.sh read <gist-id>}"
    gh gist view "$GID" --raw
    ;;
  list)
    LIMIT="${1:-10}"
    gh gist list --limit "$LIMIT"
    ;;
  report)
    # Generate fresh analysis and post as gist
    OUTDIR="$(mktemp -d)"
    trap "rm -rf $OUTDIR" EXIT
    REPORT="$OUTDIR/monster_gyroscope_report.md"
    echo "# Monster Gyroscope CI Report — $(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$REPORT"
    echo "" >> "$REPORT"
    echo "## Analysis Output" >> "$REPORT"
    echo '```' >> "$REPORT"
    if command -v lune &>/dev/null; then
      lune run scripts/monster_gyroscope >> "$REPORT" 2>&1
    elif [ -f output/analysis.txt ]; then
      cat output/analysis.txt >> "$REPORT"
    else
      echo "No analysis available — run: lune run scripts/monster_gyroscope" >> "$REPORT"
    fi
    echo '```' >> "$REPORT"
    echo "" >> "$REPORT"
    echo "## Build" >> "$REPORT"
    if [ -f output/roblox/MonsterGyroscope.rbxl ]; then
      echo "- MonsterGyroscope.rbxl: $(wc -c < output/roblox/MonsterGyroscope.rbxl) bytes" >> "$REPORT"
    fi
    if [ -d output/site ]; then
      echo "- Static site: $(ls output/site/*.html 2>/dev/null | wc -l) pages" >> "$REPORT"
      for f in output/site/*.html; do
        echo "  - $(basename $f): $(wc -c < $f) bytes" >> "$REPORT"
      done
    fi
    echo "" >> "$REPORT"
    echo "## Repo: https://github.com/meta-introspector/roblox-rs" >> "$REPORT"
    echo "## Commit: $(git rev-parse --short HEAD)" >> "$REPORT"
    gh gist create --public -d "Monster Gyroscope CI Report $(date -u +%Y%m%d_%H%M%S)" "$REPORT"
    ;;
  help|*)
    echo "Usage: gist.sh {post|update|read|list|report} [args]"
    echo "  post <file> [desc]    — create public gist"
    echo "  update <id> <file>    — update existing gist"
    echo "  read <id>             — print gist content"
    echo "  list [limit]          — list recent gists"
    echo "  report                — generate + post CI report"
    ;;
esac
