#!/bin/sh

LOCAL_MAN_DIR="./man"
CMD_MAN_DIR="/usr/share/man/man1"
TMP_MAN_DIR=$(mktemp -d)
PRIV=${PRIV:-}

trap 'rm -rf -- "$TMP_MAN_DIR"' EXIT

for page in "$LOCAL_MAN_DIR"/*; do
  gzip -c -- $page > "$TMP_MAN_DIR/$(basename $page).gz"
done

if [ "$(id -u)" -eq 0 ]; then
    PRIV=
elif [ -z "$PRIV" ] && command -v doas >/dev/null 2>&1; then
    PRIV=doas
elif [ -z "$PRIV" ] && command -v sudo >/dev/null 2>&1; then
    PRIV=sudo
fi

if [ -n "$PRIV" ]; then
 "$PRIV" install -m 644 "$TMP_MAN_DIR"/*.gz "$CMD_MAN_DIR"
else
  install -m 644 "$LOCAL_MAN_DIR"/*.gz "$CMD_MAN_DIR"
fi
