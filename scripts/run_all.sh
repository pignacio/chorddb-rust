#!/bin/bash

set -u

function kill_children() {
  children=$(pgrep -P $1)
  echo "Terminating children of process ($1): '${children}'"
  kill ${children}
  echo "Terminated!"
}


cd $(dirname "$0")/..

watch=""
bun_opts=""

for opt in "$@"; do
  case "${opt}" in
    --watch)
      echo "Enabling watch mode for cargo"
      watch="true"
      ;;
    --open)
      echo "Will use 0.0.0.0 instead of 127.0.0.1 for the frontend"
      bun_opts="${bun_opts} --host 0.0.0.0"
      ;;
    -h|--help)
      echo "Usage: $0 [--watch] [--open]"
      exit 0
      ;;
    *)
      echo "Unknown option: $opt"
      exit 1
      ;;
  esac
done

echo "Starting frontend application"
pushd frontend
bun install
bun run dev ${bun_opts} &
frontend_pid=$!
trap "trap - SIGTERM && kill_children ${frontend_pid}" SIGINT SIGTERM EXIT
popd
echo "Frontend application started in background. PID=${frontend_pid}"

echo "Starting backend application"
if [ -z "${watch}" ]; then
  cargo run
else
  cargo watch --ignore "frontend/**" -x run
fi
