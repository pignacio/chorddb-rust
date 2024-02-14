#!/bin/bash
set -u


cd $(dirname "$0")/../frontend

last_sha=""

while true; do
  new_sha=$(find . ../templates ../styles -not -path "./node_modules/*" \( -name "*.js" -o -name "*.html" -o -name "*.css" \) -exec sha256sum {} + | sha256sum)
  if [ "z${new_sha}" != "z${last_sha}" ]; then
    echo "$(date): SHA changed from '${last_sha}' to '${new_sha}'. Rebuilding..."
    bun build chorddb.ts --outdir ../dist --target browser
    bun run tailwindcss -i ../styles/input.css -o ../dist/chorddb.css
    last_sha=${new_sha}
  fi
  sleep 1s;
done
