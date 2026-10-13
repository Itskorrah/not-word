#!/usr/bin/env python3
"""Write Not Word draft-release instructions from the assembled artifact inventory."""
import re
import sys
from pathlib import Path


def main() -> int:
    if len(sys.argv) != 2 or not re.fullmatch(r"\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?", sys.argv[1]):
        print("usage: release_notes.py X.Y.Z[-prerelease]", file=sys.stderr)
        return 2
    version = sys.argv[1]
    root = Path(__file__).resolve().parents[1]
    files = sorted((root / "dist/release").glob("not-word-*"))
    if not files:
        print("error: release artifacts have not been assembled", file=sys.stderr)
        return 1
    print(f"Not Word {version} is an independently maintained, open-source Rust word processor by Itskorrah. It remains alpha software: keep backups of important documents and review DOCX round trips before sharing them.")
    print("\nChoose the package for your operating system and architecture.\n")
    print("| Package | Installation |\n| --- | --- |")
    for artifact in files:
        name = artifact.name
        if name.endswith(".zsync"):
            continue
        if name.endswith(".dmg"):
            guidance = "Open the signed and notarized image; drag Not Word.app to Applications."
        elif name.endswith(".msi"):
            guidance = "Run the signed installer; the application appears in the Start menu."
        elif "portable.zip" in name:
            guidance = "Extract and run not-word.exe; not-word-cli.exe provides automation."
        elif name.endswith(".flatpak"):
            guidance = f"`flatpak install --user {name}`; run `flatpak run io.github.itskorrah.NotWord`."
        elif name.endswith(".deb"):
            guidance = f"`sudo apt install ./{name}`"
        elif name.endswith(".rpm"):
            guidance = f"`sudo dnf install ./{name}`"
        elif name.endswith(".AppImage"):
            guidance = f"`chmod +x {name}` and run it; no installer required."
        elif "-web-" in name:
            guidance = "Extract and host the static files over HTTPS; see HOSTING.md inside."
        else:
            guidance = "Extract; see the included README and license files."
        print(f"| [{name}]({name}) | {guidance} |")
    print("\nVerify downloaded artifacts with `sha256sum --ignore-missing -c SHA256SUMS` (or the platform's SHA-256 tool). Only selected files need to be present locally.")
    print("\nNot Word works offline. Its CLI and MCP interfaces are optional and do not require an AI provider. Report problems at https://github.com/Itskorrah/not-word/issues.")
    print("\nLicensed under MIT OR Apache-2.0. Upstream WordCraft authors and all third-party assets remain credited in the included NOTICE and ATTRIBUTION.md.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
