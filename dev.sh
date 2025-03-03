#!/usr/bin/env bash
set -e

TAILWIND_CLI=$(command -v tailwindcss || echo "/usr/local/bin/tailwindcss")

if [[ ! -x "$TAILWIND_CLI" ]]; then
    echo "Error: tailwindcss not found. Install it and try again."
    exit 1
fi

INPUT_CSS="./style/main.css"
OUTPUT_CSS="./style/output.css"

$TAILWIND_CLI -i $INPUT_CSS -o $OUTPUT_CSS --watch &
TAILWIND_PID=$!

# Start cargo leptos watch
cargo leptos watch --hot-reload

# Kill Tailwind on exit
trap "kill $TAILWIND_PID" EXIT
