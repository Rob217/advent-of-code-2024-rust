# To get your session cookie, follow the instructions at:
# https://github.com/wimglenn/advent-of-code-wim/issues/1
#
# Then set the AOC_SESSION environment variable to the output of that command.
# Save this in ~/.config/aoc/token to avoid having to do this every time.
#
# Acknowledgements:
# This script was taken from https://github.com/AxlLind/AdventOfCode2022/blob/main/fetch.sh


#!/usr/bin/env bash

set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Error: You must provide a single argument in the format 'dayXX'."
    exit 1
fi

day=$1
day_number=$(echo $day | sed 's/day0*//')

if [[ ! $day =~ ^day[0-9]{2}$ ]]; then
    echo "Error: The argument must be in the format 'dayXX', where XX are digits."
    exit 1
fi

# # download input data
# AOC_SESSION=$(<~/.config/aoc/token)
# if [[ -z "${AOC_SESSION-""}" ]]; then
#   echo \$AOC_SESSION not set
#   exit 1
# fi

# SCRIPT_DIR=$(realpath "$(dirname "$0")")
# mkdir -p "$SCRIPT_DIR/../inputs"

# curl -s "https://adventofcode.com/2024/day/${day_number}/input" \
#     --cookie "session=$AOC_SESSION" \
#     -A "Bash script at $(git remote -v | awk 'NR==1{print $2}')" \
#     | tee "$SCRIPT_DIR/inputs/$day.in"

# make a new directory for the day using the template dayXX
mkdir -p $day
cp -r dayXX/* $day

# update the package name in the $day/Cargo.toml file
sed -i '' "s/name = \"dayXX\"/name = \"$day\"/" $day/Cargo.toml

# add $day to the workspace in the root Cargo.toml
sed -i '' "s/\"dayXX\"/\"$day\",\n\t\"dayXX\"/" Cargo.toml

# add 'use $day;' to run_solution/src/main.rs
sed -i '' "s/use dayXX;/use $day;\nuse dayXX;/" run_solution/src/main.rs

# add match statement for $day in run_solution/src/main.rs
sed -i '' "s/        \"dayXX\" => run_day(dayXX::part1, dayXX::part2, \"dayXX\", \&input)/        \"$day\" => run_day($day::part1, $day::part2, \"$day\", \&input),\n        \"dayXX\" => run_day(dayXX::part1, dayXX::part2, \"dayXX\", \&input)/" run_solution/src/main.rs

# add $day = { path = "../$day" } to end of run_solution/Cargo.toml
echo "$day = { path = \"../$day\" }" >> run_solution/Cargo.toml