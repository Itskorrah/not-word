#!/usr/bin/env bash
# Build a portable Not Word FreeBSD archive with the CLI, desktop metadata and licenses.
# Usage: packaging/freebsd/package.sh [--skip-build]
set -euo pipefail
# shellcheck source=../env.sh
. "$(dirname "${BASH_SOURCE[0]}")/../env.sh"
if [ "${1:-}" != --skip-build ]; then
  [ -z "${1:-}" ] || { echo "unknown argument: $1" >&2; exit 2; }
  (cd "$ROOT" && cargo build --release --locked -p wordcraft --bin not-word -p wordcraft-cli --bin not-word-cli)
fi
ARCH="$(uname -m)"
case "$ARCH" in amd64 | x86_64) ARCH=x86_64 ;; arm64 | aarch64) ARCH=aarch64 ;; *) echo "unsupported architecture: $ARCH" >&2; exit 2 ;; esac
NAME="not-word-$VERSION-freebsd-$ARCH"
WORK="$CARGO_TARGET_DIR/freebsd-package"
STAGE="$WORK/$NAME"
rm -rf "$WORK"
mkdir -p "$STAGE/bin" "$STAGE/share/applications" "$STAGE/share/icons" "$STAGE/share/doc/not-word"
cp "$CARGO_TARGET_DIR/release/not-word" "$CARGO_TARGET_DIR/release/not-word-cli" "$STAGE/bin/"
cp "$ROOT/packaging/linux/io.github.itskorrah.NotWord.desktop" "$STAGE/share/applications/"
cp -R "$ROOT/assets/app-icon/hicolor" "$STAGE/share/icons/"
copy_docs "$STAGE/share/doc/not-word"
"$STAGE/bin/not-word-cli" --version
tar -C "$WORK" -czf "$DIST/$NAME.tar.gz" "$NAME"
echo "wrote $DIST/$NAME.tar.gz"
