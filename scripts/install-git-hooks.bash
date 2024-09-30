#!/usr/bin/env bash

# This script installs a git pre-submit hook that runs the tests before
# allowing a commit to be made.

set -euo pipefail

scripts_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly scripts_dir

repo_dir="$(git rev-parse --show-toplevel)"
readonly repo_dir

hook_file="${repo_dir}/.git/hooks/pre-commit"
readonly hook_file

echo "Installing pre-commit hook"
cp "${scripts_dir}/git/pre-commit" "${hook_file}"
