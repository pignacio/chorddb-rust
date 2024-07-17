#!/bin/bash
set -e
set -u

git_hash=$(git rev-parse --short HEAD)
git_tag=$(git describe --tags --exact-match --abbrev=0 2>/dev/null || true)


echo "Building images with hash $git_hash"
docker build -t chorddb-api:$git_hash .
docker build -t chorddb-frontend:$git_hash frontend

if [ ! -z "$git_tag" ]; then
  echo "Tagging images with tag $git_tag"
  docker tag chorddb-api:$git_hash chorddb-api:$git_tag
  docker tag chorddb-frontend:$git_hash chorddb-frontend:$git_tag
fi
