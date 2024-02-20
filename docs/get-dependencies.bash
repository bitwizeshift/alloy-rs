#!/usr/bin/env bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
OUT_DIR="${SCRIPT_DIR}"

PERMALINK="https://raw.githubusercontent.com/badboy/mdbook-mermaid/ca18754266fc1da8270bb5e7a9a9dcace3dfc944/src/bin/assets/"

if [[ ! -f "${OUT_DIR}/mermaid.min.js" ]]; then
  wget "${PERMALINK}/mermaid.min.js" -O "${OUT_DIR}/mermaid.min.js"
fi
if [[ ! -f "${OUT_DIR}/mermaid-init.js" ]]; then
  wget "${PERMALINK}/mermaid-init.js" -O "${OUT_DIR}/mermaid-init.js"
fi
