# Not Word brand assets

The Not Word mark is original artwork: a white geometric **N**, a teal-to-blue ink diagonal,
and a small folded-page corner on a dark slate tile. It identifies the independent Not Word
project maintained by Itskorrah. It does not use ArtCraft trademarks or another editor's artwork.

Use `not-word-logo.svg` on light backgrounds, `not-word-logo-white.svg` on dark backgrounds,
and `not-word-mark.svg` when only the square mark is needed. The SVG wordmarks contain paths,
so they render without installed fonts. Their lettering comes from the repository's bundled
Inter Semibold, released under the SIL Open Font License; its attribution and full licence remain
in `ATTRIBUTION.md` and `assets/fonts/OFL-Inter.txt`.

## Regenerate

From the repository root, run:

```sh
python3 tools/brand_assets.py
```

This requires Python 3, Inkscape and Python's `fontTools` module. It does not download anything
or invoke a Rust build. On Debian/Ubuntu, the tool packages are `inkscape` and `python3-fonttools`.
An alternative executable path can be supplied with `--renderer /path/to/inkscape`.

`tools/brand_assets.py` is the editable source of the mark's geometry and wordmark composition.
It regenerates these SVGs and `assets/app-icon/not-word.svg`, plus every committed PNG, Windows
ICO, macOS ICNS and Linux hicolor image. PNG images have transparent corners. The macOS tile
uses Apple's 824/1024 grid with a transparent margin; the ICNS includes standard and Retina
PNG elements through 1024 pixels. ICO images cover 16, 20, 24, 32, 40, 48, 64, 128 and 256 pixels.
Linux icons use the desktop identifier `io.github.itskorrah.NotWord`.

The generator checks PNG signatures and dimensions. After changing geometry, inspect the 16px
and 1024px results and both wordmarks, then run `cargo xtask assets`. Rendering is deterministic
with the same Inkscape and fontTools versions; a renderer update can change PNG bytes without
changing the vector geometry. Committed outputs mean application builds need none of these tools.

The mark and its composition are available under **MIT OR Apache-2.0**, like the code. Inter's
outlined letterforms retain **OFL-1.1** licensing. See `ATTRIBUTION.md`, `LICENSE-MIT` and
`LICENSE-APACHE`. Preserve upstream code and third-party credits when reusing the project.
