#!/bin/bash

set -u

function kill_children() {
  children=$(pgrep -P $1)
  echo "Terminating children of process ($1): '${children}'"
  kill ${children}
  echo "Terminated!"
}


cd $(dirname "$0")/..

if [ "${1:-}" = "--watch" ]; then
  echo "Enabling watch mode for cargo"
  watch="true";
else
  watch=""
fi

echo "Starting frontend application"
pushd frontend
bun install
bun run dev &
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
