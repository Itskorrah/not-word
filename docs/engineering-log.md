# Not Word engineering log

This log records concrete fork improvements and their validation. Keep it short, source-backed
and useful to the next contributor. Documented or planned work is distinct from verified code;
GitHub delivery requires remote evidence.

## 14 October 2026 — approved original-B identity

- Replaced the earlier N/teal artwork with the owner's selected original concept B: three
  rounded writing strokes and a vertical insertion caret, followed once by the original bold
  Not Word lettering. Frozen vector outlines preserve the selected proportions and spacing;
  the approved source PNG and its SHA-256 are retained in `docs/brand/`.
- Exported transparent graphite/off-white SVG and PNG lockups, standalone marks, and the
  existing Windows ICO, macOS ICNS and Linux hicolor sizes. `docs/brand/exports.json` inventories
  all 20 production exports with hashes. Repeated generation is deterministic; all ICO/ICNS
  entries decode and small/large icons were visually inspected. Existing font/upstream credits
  remain intact, with AI-assisted artwork provenance recorded in `ATTRIBUTION.md`.
- Backstage and About embed the canonical lockup, cache its light/dark textures, and expose
  accessible alternative text. Affected documentation screenshots, README badge colours,
  AppStream brand colours and the macOS installer background now match the monochrome identity.
- Verification: all six `cargo xtask ci` gates passed, with 1,960 workspace tests passed,
  zero failed and five ignored. Native/CLI/offscreen builds and the Trunk WASM build passed.
  CLI/DOCX/PDF/PNG/MCP and authenticated native editing smoke passed. Four final native logo
  captures were inspected; packaging metadata/script/XML checks and local documentation links
  passed. Chromium WebGL2 start/About captures confirmed both appearances, no page errors and
  a favicon byte hash equal to the current original-B ICO. Linux tar staging retained the new icon bytes exactly; this used debug binaries,
  not a production installer build. Windows/macOS installation and signing remain unverified.

## 14 October 2026 — independent fork documentation

- Rebuilt the README around Not Word, maintained by Itskorrah, with upstream provenance and
  licensing preserved. Removed upstream promotional links and claims that upstream signed
  downloads constitute releases of this fork.
- Consolidated agent guidance in `AGENTS.md` and made `CLAUDE.md` point to it. Added development,
  CLI and customisation guides and a current crate/layer architecture summary.
- Retained dated upstream parity assessments and progress history as engineering context;
  readiness percentages and effort estimates are not new measurements for Not Word.
- Documentation checks: all local Markdown and README image links resolve in the checkout.
  Original logo assets and actual light/dark/settings/focus screenshots are present and linked.
  Active legacy screenshot paths were also replaced with fresh branded captures; attribution
  explicitly retains the original WordCraft sample-text provenance.

## 14 October 2026 — reliability and workspace implementation

The following changes are implemented in the working tree. Focused tests have passed as
noted below; final workspace gates and GitHub delivery are recorded separately when complete.

| Area | Change | Focused evidence |
|---|---|---|
| DOCX integrity | Reject unreadable/CRC-invalid ZIP entries before returning an editable document; reject ambiguous part names; retain foreign XML namespace names/payloads in existing opaque controls/embeds/math storage | DOCX unit suite: 86 passed including package/namespace regressions; edited three-cycle control integrations passed in the final gate |
| Legacy DOC | Decode UTF-16 pairs across non-contiguous pieces without consuming an adjacent story; bound inline-image copies/decompression by the remaining budget and cache failed locations | `surrogate_pair_split_between_non_contiguous_pieces_is_preserved`, `incomplete_surrogate_never_consumes_the_next_story_or_compressed_piece` and media-budget regressions; legacy DOC suite: 72 passed |
| Text shaping | Reorder RTL clusters with stable in-place sorting, removing per-cluster temporary vectors while retaining glyph/mark order | Two RTL order regressions passed; the ignored benchmark ran with the exact old ordering reference in the same binary |
| Raster handling | Check pixel indices/dimensions and hostile export buffers; bound decode/conversion storage; reuse existing RGBA storage rather than cloning it | Seven render/cache regression tests passed; image decode timing does not establish a speed gain |
| UI and workspace | Teal/blue/violet/rose/amber accents, high contrast, six workspace presets, retained actual layout across restarts and focus restoration | UI library: 371 passed, two existing ignored; theme-switch/command-palette, tolerant preferences and workspace tests; visual theme/settings/About checks |

Foreign XML retention does not implement new evaluators or certify general OOXML fidelity.
Supported markup-compatibility prefix/QName lists and inherited ignorable namespace intent
are retained with declared bindings; arbitrary custom QName-valued attribute semantics remain
limited. Namespace metadata is bounded (4 KiB per URI, 8 MiB aggregate, 128 scoped list entries).
Native printer, screen-reader and cross-platform behavior require their relevant hosts/hardware.

## Measured performance

These are focused workloads, not overall application speed claims. Linux cloud measurements
used Rust 1.99, a development/test profile with `opt-level=1`, dependencies at level 2 and four
Cargo jobs. The table benchmark used debug info and incremental compilation disabled. Medians
use seven samples; the RTL comparison alternates old/new implementations in the same binary.

| Workload | Before | After | Change |
|---|---:|---:|---:|
| Marked Arabic total shaping, Noto Naskh Arabic Regular, 13,184 bytes / 7,040 glyphs | 1,210.0 µs | 1,054.4 µs | 12.86% lower |
| Mixed Arabic/Hebrew/Latin total shaping, 8,448 bytes / 4,992 glyphs | 1,854.1 µs | 1,657.1 µs | 10.62% lower |
| Cold layout, 1,000 × 8 styled TableGrid table, proofing disabled | 130.31 ms | 112.24 ms | 13.87% lower |
| Warm incremental layout, same table | 47.43 ms | 27.80 ms | 41.39% lower |

The table change uses a private, bounded 64-entry per-pass text-formatting digest memo. Two
memo regressions passed. Outputs stayed at 88 pages, 50,066 display items and 8,002 laid-out
paragraphs. A second unchanged baseline measured 131.79/49.57 ms (cold/warm), showing workload
variation below the reported gain. The 2,097,152-pixel image decode probe measured
PNG 23.28 → 23.81 ms and BMP 6.83 → 7.28 ms: the resource preflight improves safety with a
small measured cost, and no image speed improvement is claimed. Reproduce the successful
workloads with:

```sh
cargo test -p wordcraft-fonts --lib rtl_shaping_benchmark -- --ignored --nocapture
cargo test -p wordcraft-layout --lib styled_table_layout_benchmark -- --ignored --nocapture
```

## Configuration and packaging

- Native preferences/window geometry and custom dictionary read legacy files only when the
  Not Word file is absent; writes use the new settings folder and atomic replacement. Added
  fonts load from both locations without moving/deleting user files.
- Browser preferences use bounded tolerant encoding under `not-word.ui.v1`, save changes
  every five seconds; they do not save/recover document content. Two codec tests passed.
- New desktop/CLI names call shared library entry points; legacy binary, environment and MCP
  resource aliases remain accepted. `not-word://` resources are primary for new MCP clients.
- Independent release orchestration, Not Word package identity/artifacts, mandatory production
  signing, explicit unsigned-test artifacts, checksums and original DMG artwork are configured.
  Static metadata checks, actionlint, shellcheck, Microsoft PowerShell 7.5 parser validation,
  XML/plist/Desktop/AppStream checks and reproducible DMG generation passed. AppStream emitted
  one pedantic notice. Linux tar staging with primary debug binaries passed metadata, IDs,
  icons, legal notice, CLI version and Unicode TXT → DOCX → TXT round-trip checks. This is
  a debug staging smoke, not a production release build or native installer verification.
  Release signing and publication remain unverified.

## Integration status

The final integration run passed `cargo xtask ci` with all six gates: formatting, Clippy with warnings denied, workspace tests,
asset attribution, layering and shared-crate WebAssembly checks. The workspace test harnesses
reported **1,960 passed, zero failed and five ignored** (two existing UI ignores plus three
manual benchmarks). The baseline was 1,922 passed and two ignored. The final gate includes
DOCX markup-compatibility/edited-save regressions, legacy DOC, UI theme/command-palette tests
and browser preference codec checks. Final review restored the exact legacy settings
folder names and retained Windows dictionary search compatibility; five control-key tests,
101 proofing tests and Clippy passed before the successful final six-gate run.

The environment installer completed locked dependency fetch, native/CLI/offscreen UI builds
and a real WASM app build. Runtime smoke checks passed CLI editing, DOCX save/reopen, PDF
export, PNG rendering, command discovery, MCP initialisation/tool discovery, and native desktop
startup with authenticated control and live sample editing. A synthetic native migration smoke
loaded legacy dark/violet/high-contrast/Academic preferences and a custom-dictionary word,
saved a Rose accent into the new settings folder, verified unchanged legacy bytes, and
confirmed the new preference after restart. An additional `ui_shot` harness regression passed.
Chromium/Playwright WebGL2 smoke passed sample startup and full preference serialisation.
Dark/Violet/high-contrast/Academic preferences survived reload. Real command-palette clicks,
typing and Enter changed the theme to Light, accent to Rose and high contrast off; the changed
preferences saved after five seconds and survived a further reload. No page errors occurred.
Software SwiftShader and unsupported browser viewport title/theme command warnings were
observed; the application canvas and selected appearance rendered correctly.

Static docs checks found zero missing local links and no whitespace errors. Native macOS,
Windows/FreeBSD runtime, physical printing and real screen-reader checks were not performed
on this Linux host. GitHub delivery is recorded separately after remote confirmation.
Reusable cloud installation/startup instructions were saved as a configuration
draft after the actual installer passed; draft publication is a separate platform step.

## Recording future work

For each meaningful change state the user-visible behavior, relevant source/tests, commands
run, pass/fail outcome and remaining limits. Performance entries need input size, environment
and before/after results. Keep commit/PR/merge links only after those operations succeed.
