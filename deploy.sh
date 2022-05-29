#!/usr/bin/env bash

set -eu

function deploy {
    local PACKAGE_NAME="$1"

    local PACKAGE_DIR="$(dirname "$BASH_SOURCE")/crates/$PACKAGE_NAME"

    if [ ! -d "$PACKAGE_DIR" ]; then
        echo "ERROR: $PACKAGE_DIR is not a directory." >&2
        exit 1
    fi

    local PACKAGE_HTML="$PACKAGE_DIR/index.html"

    if [ ! -f "$PACKAGE_HTML" ]; then
        echo "ERROR: $PACKAGE_HTML is not a file." >&2
        exit 1
    fi

    trunk build -- "$PACKAGE_HTML"

    cd "$PACKAGE_DIR"
    netlify deploy --dir=dist --prod
}

deploy "$1"
