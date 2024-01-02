set -u


cd $(dirname "$0")/../frontend

last_sha=""

while true; do
  new_sha=$(find . -name "*.ts" -exec sha256sum {} + | sha256sum)
  if [ "z${new_sha}" != "z${last_sha}" ]; then
    echo "$(date): SHA changed from '${last_sha}' to '${new_sha}'. Rebuilding..."
    bun build chorddb.ts --outdir ../dist --target browser
    last_sha=${new_sha}
  fi
  sleep 1s;
done
