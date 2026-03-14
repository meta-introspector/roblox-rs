#!/usr/bin/env bash
# gist-bridge — watches pastebin spool, forwards pastes tagged "share-to-gist" to GitHub Gists
set -euo pipefail

SPOOL="/mnt/data1/spool/uucp/pastebin"
GIST_LOG="$SPOOL/.gist-bridge.log"
GIST_SENT="$SPOOL/.gist-sent"
PASTEBIN="https://solana.solfunmeme.com/pastebin"
TRIGGER_KEYWORD="share-to-gist"
POLL_INTERVAL=10

touch "$GIST_SENT" "$GIST_LOG"

log() { echo "$(date -u +%Y%m%dT%H%M%SZ) $*" | tee -a "$GIST_LOG"; }

send_to_gist() {
    local file="$1"
    local bn
    bn=$(basename "$file" .txt)
    local title
    title=$(sed -n 's/^Title: //p' "$file" | head -1)
    local cid
    cid=$(sed -n 's/^CID: //p' "$file" | head -1)
    local content
    content=$(sed '1,/^$/d' "$file")

    if [ -z "$content" ]; then
        log "SKIP $bn — empty content"
        return 1
    fi

    local desc="${title:-$bn}"
    local tmpfile
    tmpfile=$(mktemp --suffix=.md)
    echo "$content" > "$tmpfile"

    local gist_url
    gist_url=$(gh gist create --public -d "$desc" "$tmpfile" 2>&1 | grep "https://" || true)
    rm -f "$tmpfile"

    if [ -n "$gist_url" ]; then
        log "SENT $bn → $gist_url"
        echo "$bn $gist_url" >> "$GIST_SENT"
        curl -s -X POST "$PASTEBIN/paste" \
            -H 'Content-Type: application/json' \
            --max-time 10 \
            -d "$(jq -n \
                --arg content "Shared to GitHub Gist: $gist_url
Original: $bn
CID: $cid" \
                --arg title "gist-bridge: $desc" \
                --argjson keywords '["gist-bridge","gist","shared"]' \
                '{title: $title, content: $content, keywords: $keywords}')" \
            >/dev/null 2>&1 || true
        echo "$gist_url"
    else
        log "FAIL $bn — gh gist create failed"
        return 1
    fi
}

already_sent() { grep -q "^$1 " "$GIST_SENT" 2>/dev/null; }

CMD="${1:-help}"
shift || true

case "$CMD" in
    send)
        PASTE_ID="${1:?usage: gist-bridge send <paste-id>}"
        FILE=$(find "$SPOOL" -maxdepth 1 -name "${PASTE_ID}*" -print -quit)
        if [ -z "$FILE" ]; then echo "Not found: $PASTE_ID" >&2; exit 1; fi
        send_to_gist "$FILE"
        ;;
    send-file)
        send_to_gist "${1:?usage: gist-bridge send-file <spool-file>}"
        ;;
    watch)
        log "gist-bridge started, watching $SPOOL for '$TRIGGER_KEYWORD'"
        while true; do
            for file in "$SPOOL"/*.txt; do
                [ -f "$file" ] || continue
                local_bn=$(basename "$file" .txt)
                already_sent "$local_bn" && continue
                kw=$(sed -n 's/^Keywords: //p' "$file" | head -1)
                if echo "$kw" | grep -qi "$TRIGGER_KEYWORD"; then
                    log "FOUND $local_bn"
                    send_to_gist "$file" || true
                fi
            done
            sleep "$POLL_INTERVAL"
        done
        ;;
    status)
        echo "=== Recent gist posts ==="
        tail -20 "$GIST_LOG" 2>/dev/null
        echo ""
        echo "=== Sent count: $(wc -l < "$GIST_SENT") ==="
        tail -5 "$GIST_SENT" 2>/dev/null
        ;;
    *)
        echo "gist-bridge {watch|send <id>|send-file <path>|status}"
        ;;
esac
