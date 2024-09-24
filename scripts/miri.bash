#!/usr/bin/env bash

# This script runs the cargo 'miri' command to test the project.
# This expects that miri is installed with 'rustup +nightly component add miri'.

set -euo pipefail

repo_dir="$(git rev-parse --show-toplevel)"
readonly repo_dir

(
  cd "${repo_dir}"

  cargo +nightly miri test
)
