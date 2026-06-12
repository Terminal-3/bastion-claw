#!/usr/bin/env bash
# Standalone build script for the optional WeChat-voice SILK decoder helper.
#
# This crate is intentionally excluded from the T3Claw workspace so the
# main `cargo build` does not require libclang. It is built separately:
#
#     ./crates/t3claw_silk_decoder/build.sh
#
# After building, the binary lands in `target/release/t3claw-silk-decoder`
# (relative to this crate). Install it next to your `t3claw` binary, on
# `$PATH`, or point the `T3CLAW_SILK_DECODER` environment variable at it.

set -euo pipefail

cd "$(dirname "$0")"

if ! command -v cargo >/dev/null 2>&1; then
    echo "Error: cargo not found on PATH" >&2
    exit 1
fi

echo "Building t3claw-silk-decoder (requires libclang + a C toolchain)..."
cargo build --release

OUT_BIN="target/release/t3claw-silk-decoder"

if [ ! -f "$OUT_BIN" ]; then
    echo "Error: build did not produce $OUT_BIN" >&2
    exit 1
fi

echo ""
echo "Built: $OUT_BIN ($(du -h "$OUT_BIN" | cut -f1))"
echo ""
echo "To install (one of the following):"
echo "  cp $OUT_BIN \"\$(dirname \"\$(command -v t3claw)\")/\"   # sibling install"
echo "  cp $OUT_BIN /usr/local/bin/                                  # system PATH"
echo "  export T3CLAW_SILK_DECODER=\"\$(pwd)/$OUT_BIN\"          # explicit path"
