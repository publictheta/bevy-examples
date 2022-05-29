#!/usr/bin/env bash

set -eu

function deploy {
    local PACKAGE_NAME="$1"
    cd "$(dirname "$BASH_SOURCE")/examples/$PACKAGE_NAME"
    netlify deploy --dir=dist --prod
}

deploy "$1"
