#!/usr/bin/env python3
"""Generate Not Word's original brand artwork and portable application icons.

Requires Python 3, fontTools (for bundled OFL Inter outlines), and Inkscape.
Run from any directory: python3 tools/brand_assets.py
No network access or proprietary artwork/fonts are used.
"""

from __future__ import annotations

import argparse
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


def icon_svg(view_box: str = "0 0 512 512") -> str:
    """A geometric N, with a folded-page corner and a line of teal ink.

    All coordinates were drawn for this project. The diagonal's pen-like taper
    and the detached folded corner make the mark readable without small detail.
    """
    return f'''{SVG_HEAD} width="512" height="512" viewBox="{view_box}" role="img" aria-labelledby="title desc">
  <title id="title">Not Word application icon</title>
  <desc id="desc">An original white N with a teal ink diagonal and folded page corner on a dark slate tile.</desc>
  <defs>
    <linearGradient id="tile" x1="0" y1="0" x2="1" y2="1">
      <stop stop-color="#213b50"/>
      <stop offset="1" stop-color="#101d2d"/>
    </linearGradient>
    <linearGradient id="ink" x1="0" y1="0" x2="1" y2="1">
      <stop stop-color="#65ead4"/>
      <stop offset="1" stop-color="#3eacd9"/>
    </linearGradient>
  </defs>
  <rect width="512" height="512" rx="112" fill="url(#tile)"/>
  <rect x="1" y="1" width="510" height="510" rx="111" fill="none" stroke="#7293a5" stroke-opacity=".2" stroke-width="2"/>
  <path d="M130 368V144h58l136 155V144h58v224h-58L188 213v155z" fill="#f1f7fa"/>
  <path d="M188 144l136 155v69L188 213z" fill="url(#ink)"/>
  <path d="M343 105h39v39z" fill="#65ead4"/>
</svg>
'''


def logo_svg(foreground: str) -> str:
    """Use the repository's redistributable Inter font, converted to paths.

    Outlining the bundled font makes the committed SVG independent of the
    viewer's installed fonts. The typeface itself retains its OFL attribution.
    """
    try:
        from fontTools.pens.svgPathPen import SVGPathPen
        from fontTools.ttLib import TTFont
    except ImportError as exc:
        raise SystemExit("Wordmark generation requires fontTools: install python3-fonttools or pip install fonttools") from exc

    with TTFont(ROOT / "assets/fonts/Inter-SemiBold.ttf") as font:
        glyphs = font.getGlyphSet()
        cmap = font.getBestCmap()
        scale = 116 / font["head"].unitsPerEm
        x = 237.0
        lettering = []
        for char in "Not Word":
            name = cmap[ord(char)]
            pen = SVGPathPen(glyphs)
            glyphs[name].draw(pen)
            if pen.getCommands():
                lettering.append(
                    f'  <path d="{pen.getCommands()}" transform="translate({x:.3f} 142) scale({scale:.8f} {-scale:.8f})"/>'
                )
            x += (font["hmtx"].metrics[name][0] + 5) * scale
    width = round(x + 20)
    # The embedded mark has its own title; document-level text gives the name.
    mark = icon_svg().split("  <defs>", 1)[1].rsplit("</svg>", 1)[0]
    return f'''{SVG_HEAD} width="{width}" height="200" viewBox="0 0 {width} 200" role="img" aria-labelledby="title desc">
  <title id="title">Not Word</title>
  <desc id="desc">Original Not Word mark and wordmark, with outlined open-source Inter lettering.</desc>
  <g transform="translate(8 8) scale(.359375)">
  <defs>{mark}
  </g>
  <g fill="{foreground}">
{chr(10).join(lettering)}
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


def validate_png(path: Path, size: int) -> None:
    data = path.read_bytes()
    if data[:8] != b"\x89PNG\r\n\x1a\n" or struct.unpack(">II", data[16:24]) != (size, size):
        raise SystemExit(f"Invalid {size}px PNG: {path}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--renderer", default="inkscape", help="Inkscape executable (default: inkscape)")
    args = parser.parse_args()
    renderer = shutil.which(args.renderer)
    if renderer is None:
        raise SystemExit(f"SVG renderer not found: {args.renderer}")
    ICON_DIR.mkdir(parents=True, exist_ok=True)
    BRAND_DIR.mkdir(parents=True, exist_ok=True)
    source = ICON_DIR / "not-word.svg"
    source.write_text(icon_svg(), encoding="utf-8")
    (BRAND_DIR / "not-word-mark.svg").write_text(icon_svg(), encoding="utf-8")
    (BRAND_DIR / "not-word-logo.svg").write_text(logo_svg("#192f41"), encoding="utf-8")
    (BRAND_DIR / "not-word-logo-white.svg").write_text(logo_svg("#f1f7fa"), encoding="utf-8")

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
    print("Generated original Not Word SVG wordmarks, PNG, ICO, ICNS and Linux hicolor icons.")


if __name__ == "__main__":
    main()
