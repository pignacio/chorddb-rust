#!/bin/bash
set -u
set -e

if [[ ${DATABASE_URL} == sqlite://* ]]; then
    db_file="${DATABASE_URL#sqlite://}"
    if [ ! -f $db_file ]; then
      echo "Touching database..."
      mkdir -p $(dirname $db_file)
      touch $db_file
    fi
fi

./migration
./chorddb
