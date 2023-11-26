#!/bin/bash

# Downloads inputs for all the days a rust solution file exists for.
# If the input file already exists it skips the download, and sleeps a little bit in between.

set -e

START_YEAR=2021
END_YEAR=2023

if [[ -z "${AOC_SESSION_COOKIE}" ]]; then
  echo "Missing env variable AOC_SESSION_COOKIE"
  exit 1
fi

for YEAR in $(seq $START_YEAR $END_YEAR); do
  echo "Processing year $YEAR"
  for file in aoc-solver/src/y$YEAR/day*.rs; do
    if [[ $file =~ day([0-9]+)\.rs ]]; then
      DAY=${BASH_REMATCH[1]}
      DAY_WITHOUT_LEADING_ZERO=$((10#$DAY))

      INPUT_FILE="inputs/$YEAR/day$DAY.txt"

      if [ ! -f "$INPUT_FILE" ]; then
        echo "Downloading input for Year $YEAR, Day $DAY"

        HTTP_STATUS=$(curl -s -o "$INPUT_FILE" -w "%{http_code}" -H "Cookie: session=$AOC_SESSION_COOKIE" "https://adventofcode.com/$YEAR/day/$DAY_WITHOUT_LEADING_ZERO/input")

        if [ "$HTTP_STATUS" -ne 200 ]; then
          echo "Error: Failed to download input for Year $YEAR, Day $DAY (HTTP Status: $HTTP_STATUS)"
          rm $INPUT_FILE;
          exit 1
        fi

        # Let's be polite and wait a little bit.
        sleep 1
      else
        echo "Input for Year $YEAR, Day $DAY already downloaded"
      fi
    fi
  done
done
