#!/bin/bash
# generate-migration.sh
# Usage: ./generate-migration.sh tsdb|redshift name_of_migration

set -e

TARGET=$1
NAME=$2

if [ -z "$TARGET" ] || [ -z "$NAME" ]; then
  echo "Usage: $0 tsdb|redshift name_of_migration"
  exit 1
fi

if [ "$TARGET" != "tsdb" ] && [ "$TARGET" != "redshift" ]; then
  echo "Invalid target: $TARGET (must be 'tsdb' or 'redshift')"
  exit 1
fi

DIR="./migrations/$TARGET"
mkdir -p $DIR

dbmate --migrations-dir $DIR new $NAME
