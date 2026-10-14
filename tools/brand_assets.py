#!/usr/bin/env python3
"""Generate Not Word's original brand artwork and portable application icons.

Requires Python 3 and Inkscape. Uses frozen vectors of the user-approved concept B.
Run from any directory: python3 tools/brand_assets.py
No network access or proprietary artwork/fonts are used.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import shutil
import struct
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ICON_DIR = ROOT / "assets/app-icon"
BRAND_DIR = ROOT / "docs/brand"
APP_ID = "io.github.itskorrah.NotWord"
SIZES = (16, 24, 32, 48, 64, 128, 256, 512)
SVG_HEAD = '<svg xmlns="http://www.w3.org/2000/svg"'


# ORIGINAL approved concept B, generated with Codex ImageGen; no font substitution.
# Frozen paths were traced with Potrace 1.16 (threshold 128; alphamax 1 for
# lettering and 1.334 for rounded symbol caps;
# opttolerance .2; upper lockup crop x=296,y=175,w=946,h=164).
# Preserve the chosen bold lettering, capsule lengths, caret and original spacing.
APPROVED_SOURCE_SHA256 = "bdfb11a114b74029f6963b19972bbbe4a8bb1a520d856607bc90bd08a8277b33"
APPROVED_SOURCE = BRAND_DIR / "not-word-approved-concept.png"
LOCKUP_PATH = 'M2340 1610 c-15 -16 -30 -44 -34 -62 -10 -58 -6 -1430 5 -1471 25 -95 176 -104 215 -12 20 50 21 1459 0 1508 -18 45 -56 67 -112 67 -38 0 -51 -5 -74 -30z M56 1459 c-57 -45 -73 -126 -36 -187 39 -64 1 -62 955 -62 942 0 911 -2 952 56 46 65 36 135 -26 187 -33 27 -33 27 -925 27 -891 0 -893 0 -920 -21z M74 941 c-104 -47 -96 -201 13 -247 26 -11 143 -14 613 -14 628 0 619 -1 670 55 21 22 25 37 25 85 0 48 -4 63 -25 85 -51 56 -41 55 -672 55 -546 0 -586 -2 -624 -19z M70 412 c-48 -23 -72 -73 -67 -136 4 -42 10 -55 41 -83 36 -33 36 -33 208 -34 368 -3 572 3 600 17 87 45 86 173 -1 231 -34 23 -36 23 -390 23 -319 0 -360 -2 -391 -18z M3270 819 l0 -540 98 5 97 5 0 370 c0 204 2 371 5 371 3 -1 106 -152 230 -337 124 -185 237 -353 251 -372 l26 -36 99 0 99 0 3 538 2 537 -100 0 -100 0 0 -361 c0 -198 -3 -358 -8 -355 -4 2 -115 164 -247 360 l-239 356 -108 0 -108 0 0 -541z M5870 1350 c0 -8 62 -239 203 -755 33 -121 64 -239 70 -262 12 -51 28 -56 151 -51 l79 3 13 40 c7 22 30 101 52 175 123 420 156 529 159 527 2 -2 33 -105 69 -228 36 -123 85 -289 108 -368 l43 -145 110 -3 110 -3 32 113 c83 299 173 628 211 772 22 88 43 168 46 178 5 15 -4 17 -99 17 l-103 0 -87 -327 c-48 -181 -92 -352 -98 -380 -7 -29 -16 -53 -20 -53 -4 0 -10 16 -14 36 -7 38 -91 335 -186 657 l-20 68 -102 -3 -102 -3 -21 -65 c-11 -36 -61 -205 -110 -377 -49 -171 -92 -309 -95 -305 -4 4 -13 36 -21 72 -24 103 -169 663 -174 672 -7 11 -204 10 -204 -2z M9270 1174 l0 -186 -37 32 c-66 58 -116 75 -223 75 -84 0 -102 -3 -157 -29 -78 -36 -144 -104 -185 -193 -43 -91 -51 -227 -20 -330 46 -151 153 -248 298 -271 110 -18 211 8 283 73 l41 37 0 -45 c0 -44 1 -46 33 -52 17 -3 59 -5 92 -3 l60 3 0 535 0 535 -92 3 -93 3 0 -187z m-135 -273 c153 -69 178 -314 42 -412 -137 -99 -311 -31 -348 138 -25 109 22 223 110 270 55 29 138 31 196 4z M5200 1185 l0 -105 -60 0 -60 0 0 -85 0 -85 60 0 60 0 0 -227 c0 -220 1 -230 24 -280 41 -88 126 -127 266 -121 l75 3 3 87 3 88 -65 0 c-48 0 -70 5 -87 19 -24 19 -24 23 -27 225 l-3 206 90 0 91 0 0 85 0 85 -90 0 -90 0 0 105 0 105 -95 0 -95 0 0 -105z M4533 1080 c-147 -48 -250 -173 -275 -335 -24 -156 37 -315 153 -401 85 -62 203 -90 313 -75 282 40 432 330 310 600 -36 81 -130 170 -211 201 -84 32 -209 36 -290 10z m210 -173 c89 -38 137 -116 137 -226 0 -140 -87 -236 -215 -236 -129 0 -215 95 -215 237 0 77 17 128 58 172 56 60 161 83 235 53z M7573 1090 c-103 -24 -212 -110 -259 -202 -61 -121 -63 -279 -5 -402 96 -200 345 -279 557 -176 273 133 298 541 45 719 -84 59 -233 86 -338 61z m193 -189 c157 -71 170 -333 21 -425 -85 -53 -198 -39 -265 32 -58 62 -78 170 -50 266 15 50 84 121 130 135 53 15 120 12 164 -8z M8471 1077 c-19 -6 -54 -30 -78 -52 l-43 -42 0 49 0 48 -95 0 -95 0 2 -397 3 -398 93 -3 92 -3 0 210 c0 247 8 298 57 352 41 45 89 63 161 58 l52 -4 0 98 0 97 -57 -1 c-32 -1 -73 -6 -92 -12z'
MARK_PATH = 'M2340 1610 c-15 -16 -30 -44 -34 -62 -10 -58 -6 -1430 5 -1471 25 -95 176 -104 215 -12 20 50 21 1459 0 1508 -18 45 -56 67 -112 67 -38 0 -51 -5 -74 -30z M56 1459 c-57 -45 -73 -126 -36 -187 39 -64 1 -62 955 -62 942 0 911 -2 952 56 46 65 36 135 -26 187 -33 27 -33 27 -925 27 -891 0 -893 0 -920 -21z M74 941 c-104 -47 -96 -201 13 -247 26 -11 143 -14 613 -14 628 0 619 -1 670 55 21 22 25 37 25 85 0 48 -4 63 -25 85 -51 56 -41 55 -672 55 -546 0 -586 -2 -624 -19z M70 412 c-48 -23 -72 -73 -67 -136 4 -42 10 -55 41 -83 36 -33 36 -33 208 -34 368 -3 572 3 600 17 87 45 86 173 -1 231 -34 23 -36 23 -390 23 -319 0 -360 -2 -391 -18z'


def icon_svg(view_box: str = "0 0 512 512") -> str:
    """The exact approved standalone mark, centered on a monochrome app tile."""
    return f'''{SVG_HEAD} width="512" height="512" viewBox="{view_box}" role="img" aria-labelledby="title desc">
  <title id="title">Not Word application icon</title>
  <desc id="desc">The approved Not Word mark: three writing capsules and a vertical insertion caret, in off-white on graphite.</desc>
  <rect width="512" height="512" rx="112" fill="#171717"/>
  <g transform="translate(84.55 366.7) scale(.135 -.135)" fill="#f5f5f5">
    <path d="{MARK_PATH}"/>
  </g>
</svg>
'''


def logo_svg(foreground: str, *, mark_only: bool = False) -> str:
    """Frozen outlines of original concept B, on a transparent background.

    Both colours share identical geometry. The symbol is 254 by 164 pixels;
    the original horizontal lockup is 946 by 164 pixels, plus a 2px safe margin.
    Letterforms come from the selected artwork, not an installed substitute font.
    """
    width = 258 if mark_only else 950
    path = MARK_PATH if mark_only else LOCKUP_PATH
    description = (
        "Three rounded writing capsules and a vertical insertion caret."
        if mark_only else
        "The original approved Not Word symbol and bold wordmark, with their chosen proportions and spacing."
    )
    return f'''{SVG_HEAD} width="{width}" height="168" viewBox="0 0 {width} 168" role="img" aria-labelledby="title desc">
  <title id="title">Not Word</title>
  <desc id="desc">{description}</desc>
  <metadata>AI-assisted original Not Word artwork. Approved source SHA-256: {APPROVED_SOURCE_SHA256}. MIT OR Apache-2.0.</metadata>
  <g transform="translate(2 166) scale(.1 -.1)" fill="{foreground}">
    <path d="{path}"/>
  </g>
</svg>
'''


def render(renderer: str, source: Path, size: int, destination: Path) -> None:
    destination.parent.mkdir(parents=True, exist_ok=True)
    result = subprocess.run(
        [renderer, str(source), "--export-type=png", f"--export-filename={destination}",
         f"--export-width={size}", f"--export-height={size}"],
        capture_output=True, text=True,
    )
    if result.returncode != 0:
        raise SystemExit(f"SVG rendering failed for {destination}:\n{result.stderr}")


def write_ico(destination: Path, pngs: list[tuple[int, bytes]]) -> None:
    """Windows Vista+ ICO directory containing lossless PNG images."""
    offset = 6 + 16 * len(pngs)
    directory = bytearray(struct.pack("<HHH", 0, 1, len(pngs)))
    images = bytearray()
    for size, data in pngs:
        directory.extend(struct.pack("<BBBBHHII", size % 256, size % 256, 0, 0, 1, 32, len(data), offset))
        images.extend(data)
        offset += len(data)
    destination.write_bytes(directory + images)


def write_icns(destination: Path, pngs: list[tuple[bytes, bytes]]) -> None:
    """Write Apple's documented PNG-backed icon elements, including Retina."""
    chunks = b"".join(kind + struct.pack(">I", len(data) + 8) + data for kind, data in pngs)
    destination.write_bytes(b"icns" + struct.pack(">I", len(chunks) + 8) + chunks)


def validate_png(path: Path, size: int, height: int | None = None) -> None:
    data = path.read_bytes()
    expected = (size, size if height is None else height)
    if data[:8] != b"\x89PNG\r\n\x1a\n" or struct.unpack(">II", data[16:24]) != expected:
        raise SystemExit(f"Invalid {expected[0]}x{expected[1]} PNG: {path}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--renderer", default="inkscape", help="Inkscape executable (default: inkscape)")
    args = parser.parse_args()
    renderer = shutil.which(args.renderer)
    if renderer is None:
        raise SystemExit(f"SVG renderer not found: {args.renderer}")
    ICON_DIR.mkdir(parents=True, exist_ok=True)
    BRAND_DIR.mkdir(parents=True, exist_ok=True)
    if not APPROVED_SOURCE.is_file() or hashlib.sha256(APPROVED_SOURCE.read_bytes()).hexdigest() != APPROVED_SOURCE_SHA256:
        raise SystemExit("Approved original concept B source is missing or has changed; do not substitute the rejected refinement.")
    source = ICON_DIR / "not-word.svg"
    source.write_text(icon_svg(), encoding="utf-8")
    (BRAND_DIR / "not-word-mark.svg").write_text(logo_svg("#171717", mark_only=True), encoding="utf-8")
    (BRAND_DIR / "not-word-mark-white.svg").write_text(logo_svg("#f5f5f5", mark_only=True), encoding="utf-8")
    (BRAND_DIR / "not-word-logo.svg").write_text(logo_svg("#171717"), encoding="utf-8")
    (BRAND_DIR / "not-word-logo-white.svg").write_text(logo_svg("#f5f5f5"), encoding="utf-8")

    # UI consumers use exactly these transparent lockups at 2x native dimensions.
    for name in ("not-word-logo", "not-word-logo-white"):
        result = subprocess.run(
            [renderer, str(BRAND_DIR / f"{name}.svg"), "--export-type=png",
             f"--export-filename={BRAND_DIR / f'{name}.png'}",
             "--export-width=1900", "--export-height=336"],
            capture_output=True, text=True,
        )
        if result.returncode != 0:
            raise SystemExit(f"Wordmark rendering failed: {result.stderr}")
        validate_png(BRAND_DIR / f"{name}.png", 1900, 336)
    render(renderer, source, 1024, ICON_DIR / "not-word-1024.png")
    for size in SIZES:
        path = ICON_DIR / f"hicolor/{size}x{size}/apps/{APP_ID}.png"
        render(renderer, source, size, path)
        validate_png(path, size)
    scalable = ICON_DIR / f"hicolor/scalable/apps/{APP_ID}.svg"
    scalable.parent.mkdir(parents=True, exist_ok=True)
    scalable.write_text(icon_svg(), encoding="utf-8")

    with tempfile.TemporaryDirectory(prefix="not-word-icons-") as directory:
        temporary = Path(directory)
        mac = temporary / "mac.svg"
        # Apple's icon grid places an 824/1024 tile within the transparent canvas.
        mac.write_text(icon_svg("-62.136 -62.136 636.272 636.272"), encoding="utf-8")
        mac_images = {}
        for size in (16, 32, 64, 128, 256, 512, 1024):
            path = temporary / f"mac-{size}.png"
            render(renderer, mac, size, path)
            validate_png(path, size)
            mac_images[size] = path.read_bytes()
        (ICON_DIR / "not-word-macos-512.png").write_bytes(mac_images[512])
        # icp4/5/6 are 16/32/64px; ic07/8/9 and ic10 are 128/256/512/1024px.
        # ic11-14 duplicate standard images with the explicit Retina scale.
        elements = ((b"icp4", 16), (b"icp5", 32), (b"icp6", 64), (b"ic07", 128),
                    (b"ic08", 256), (b"ic09", 512), (b"ic10", 1024),
                    (b"ic11", 32), (b"ic12", 64), (b"ic13", 256), (b"ic14", 512))
        write_icns(ICON_DIR / "not-word.icns", [(kind, mac_images[size]) for kind, size in elements])
        pngs = []
        for size in (16, 20, 24, 32, 40, 48, 64, 128, 256):
            path = temporary / f"windows-{size}.png"
            render(renderer, source, size, path)
            validate_png(path, size)
            pngs.append((size, path.read_bytes()))
        write_ico(ICON_DIR / "not-word.ico", pngs)
    validate_png(ICON_DIR / "not-word-1024.png", 1024)
    validate_png(ICON_DIR / "not-word-macos-512.png", 512)
    # A stable inventory lets packaging and reviewers verify the final export pack.
    exports = []
    paths = list(ICON_DIR.rglob("*")) + list(BRAND_DIR.glob("not-word-*"))
    mime = {".svg": "image/svg+xml", ".png": "image/png", ".ico": "image/vnd.microsoft.icon", ".icns": "image/x-icns"}
    for path in sorted(paths):
        if not path.is_file() or path.suffix not in mime or path == APPROVED_SOURCE:
            continue
        data = path.read_bytes()
        item = {"path": path.relative_to(ROOT).as_posix(), "mediaType": mime[path.suffix],
                "bytes": len(data), "sha256": hashlib.sha256(data).hexdigest()}
        if path.suffix == ".png":
            item["width"], item["height"] = struct.unpack(">II", data[16:24])
        exports.append(item)
    manifest = {"version": 1, "approvedSource": APPROVED_SOURCE.relative_to(ROOT).as_posix(),
                "approvedSourceSha256": APPROVED_SOURCE_SHA256,
                "foregroundColours": {"lightBackground": "#171717", "darkBackground": "#F5F5F5"},
                "exports": exports}
    (BRAND_DIR / "exports.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    print("Generated original Not Word SVG wordmarks, PNG, ICO, ICNS and Linux hicolor icons.")


if __name__ == "__main__":
    main()
