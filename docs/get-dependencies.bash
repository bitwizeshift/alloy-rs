#!/usr/bin/env bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
OUT_DIR="${SCRIPT_DIR}"

PERMALINK="https://raw.githubusercontent.com/badboy/mdbook-mermaid/5f87002c29d28fdfaad454024a80859e5d51c972/src/bin/assets/"

if [[ ! -f "${OUT_DIR}/mermaid.min.js" ]]; then
  wget "${PERMALINK}/mermaid.min.js" -O "${OUT_DIR}/mermaid.min.js"
fi
if [[ ! -f "${OUT_DIR}/mermaid-init.js" ]]; then
  wget "${PERMALINK}/mermaid-init.js" -O "${OUT_DIR}/mermaid-init.js"
fi
