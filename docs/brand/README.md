# Not Word brand assets

This is the **original, user-approved concept B**: three rounded writing capsules and a vertical
insertion caret, followed once by the bold **Not Word** wordmark. Its selected proportions,
letterforms and symbol-to-text spacing are preserved as frozen vector outlines. There is no
substitute font, additional mark, colour accent or redesigned refinement.

Use `not-word-logo.svg` on light backgrounds and `not-word-logo-white.svg` on dark backgrounds.
Both have a transparent background and a `950 × 168` viewBox. Matching transparent PNGs are
`1900 × 336` pixels for native UI embedding. Standalone transparent marks are `not-word-mark.svg`
and `not-word-mark-white.svg`, with a `258 × 168` viewBox. Foregrounds are graphite `#171717` and
off-white `#F5F5F5`; every treatment uses exactly the same outlines.

Application icons place the selected standalone symbol on a flat graphite tile with transparent
rounded corners. There are no gradients, shadows, navy/teal elements, folded corners or letters
inside the mark. The platform tile is a container for the approved mark, not a new logo.

## Approved source and provenance

`not-word-approved-concept.png` is the unchanged original concept B produced with Codex
ImageGen and selected by the project owner. Its SHA-256 is:

```text
bdfb11a114b74029f6963b19972bbbe4a8bb1a520d856607bc90bd08a8277b33
```

The original generated filename was `exec-01ff5c3d-2024-4e80-944d-469791e20fcf.png`.
The rejected later refinement is not a source for these assets. The approved PNG retains its
original two-field presentation for provenance; use the transparent exports in the application.

The upper lockup's foreground was isolated at luminance `< 128`, cropping `x=296, y=175,
width=946, height=164`. The symbol crop uses the same origin and height, with width `254`.
Potrace **1.16** traced these masks with `--svg --flat --turdsize 0 --opttolerance 0.2`, using
`--alphamax 1` for the lettering and `--alphamax 1.334` for the symbol to preserve its rounded caps.
Only the resulting paths were frozen into `tools/brand_assets.py`; no tracing library or
implementation code is bundled. A two-pixel transparent export margin prevents clipping.

## Regenerate

From the repository root, run:

```sh
python3 tools/brand_assets.py
```

This requires **Python 3 and Inkscape**. It does not download files, use ImageGen, invoke a Rust
build, require installed fonts or retrace the artwork. An alternative executable can be supplied
with `--renderer /path/to/inkscape`. The generator verifies the approved source hash before
exporting, so a different concept cannot silently replace it.

`tools/brand_assets.py` holds the canonical frozen paths and regenerates all SVGs, the transparent
UI PNGs, and every committed application PNG, Windows ICO, macOS ICNS and Linux hicolor image.
The macOS tile uses Apple's 824/1024 grid with a transparent margin; ICNS includes standard and
Retina PNG elements through 1024 pixels. ICO images cover 16, 20, 24, 32, 40, 48, 64, 128 and
256 pixels. Linux icons use the identifier `io.github.itskorrah.NotWord`.

The generator checks application PNG signatures and dimensions. After regeneration, inspect the
16px and 1024px icons and both wordmarks, then run `cargo xtask assets`. Rendering is deterministic
with the same Inkscape version; renderer updates may change PNG bytes while leaving vector
geometry intact. Committed outputs mean application builds need none of these tooling dependencies.

The generated [export manifest](exports.json) records every production filename, format,
byte count and SHA-256, plus PNG dimensions and the approved source hash. It contains no
machine paths or generation timestamps.

The AI-assisted original Not Word artwork and its vector/raster exports are available under
**MIT OR Apache-2.0**, like the code. See `ATTRIBUTION.md`, `LICENSE-MIT` and `LICENSE-APACHE`.
Existing upstream, third-party and OFL font credits remain preserved; the new wordmark does not
incorporate the previously used bundled Inter glyph outlines.
