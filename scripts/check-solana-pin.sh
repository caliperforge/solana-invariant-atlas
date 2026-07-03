#!/usr/bin/env bash
# Verify the active solana-cli matches the pinned rail (2.1.21).
# Build spec section 3: host defaults commonly float (the spike host
# defaults to 4.0.1); every local build must run against the pin.
set -euo pipefail

PINNED="2.1.21"
PIN_BIN="$HOME/.local/share/solana/install/releases/${PINNED}/solana-release/bin"

if ! command -v solana >/dev/null 2>&1; then
  echo "check-solana-pin: no solana on PATH." >&2
  echo "Install the pinned release, then prefix your PATH:" >&2
  echo "  export PATH=${PIN_BIN}:\$PATH" >&2
  exit 1
fi

ACTIVE="$(solana --version)"
if [[ "$ACTIVE" == *"solana-cli ${PINNED}"* ]]; then
  echo "check-solana-pin: OK (${ACTIVE})"
  exit 0
fi

echo "check-solana-pin: active solana-cli is not the pinned ${PINNED}:" >&2
echo "  active: ${ACTIVE}" >&2
echo "Prefix your PATH with the pinned release before any cargo build-sbf:" >&2
echo "  export PATH=${PIN_BIN}:\$PATH" >&2
exit 1
