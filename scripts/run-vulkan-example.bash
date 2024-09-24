#!/usr/bin/env bash

set -euo pipefail

repo_dir=$(git rev-parse --show-toplevel)
readonly repo_dir

(
  cd "${repo_dir}"

  cargo run --bin vulkan-example
)
