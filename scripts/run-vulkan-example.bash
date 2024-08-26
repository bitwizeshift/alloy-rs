#!/usr/bin/env bash

readonly repo_dir=$(git rev-parse --show-toplevel)

(
  cd "${repo_dir}"

  cargo run --bin vulkan-example
)
