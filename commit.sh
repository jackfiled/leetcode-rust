#!/bin/sh

set -e

time=$(date "+%Y%m%d")
message="$time Finished"

git add -A
git commit -m "$message"

git push
