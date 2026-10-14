#!/usr/bin/env bash
# Regenerate approved original-B icons, outlined lockups and the export manifest.
# Requires Python 3 and Inkscape. Outputs are committed; packaging does not
# need these tools. ICO and ICNS are generated on every supported development host.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
exec python3 "$ROOT/tools/brand_assets.py" "$@"
