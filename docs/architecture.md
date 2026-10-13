# Not Word architecture

Not Word retains the Rust-native architecture inherited from WordCraft. The product and
binary names are Not Word; workspace package names and directories stay `wordcraft-*` for
compatibility. This document describes the sources rather than claiming a tested performance
or platform certification. See [AGENTS.md](../AGENTS.md) for contribution rules.

## Layers

`cargo xtask layers` checks the classification in `xtask/src/layers.rs`: lower layers do not
depend on higher layers, and UI toolkit dependencies stay at L6 or in host applications.
A few explicitly ordered dependencies inside a layer and dev/build dependencies have their
own rules in that checker.

| Layer | Package (prefix `wordcraft-` unless noted) | Responsibility |
|---|---|---|
| L0 | `geom` | Points, twips, EMU, rectangles and measurement parsing |
| Standalone | `control-key` | Settings-directory discovery and local control authentication keys; no workspace dependencies |
| L1 | `doc` | Document stories, blocks, runs, sections, styles, numbering, fields, notes, comments, revisions and math; edit primitives |
| L1 | `fonts` | Bundled/system font database, HarfRust shaping, fallback and metric-compatible substitution, OpenType math |
| L1 | `proof` | Per-run language routing, spelling/grammar rules, user dictionary and hyphenation |
| L1 | `metafile` | Pure-Rust EMF/WMF handling for imported drawings |
| L2 | `layout` | Shaping, line breaking, tables, pagination, notes, floating objects, fields and hit testing; display lists |
| L2 | `docx` | OOXML package import/export, relationships, styles, math and preserved opaque content |
| L2 | `docbin` | Word 97–2003 DOC import through compound-file containers |
| L2 | `formats` | Plain text, Markdown, HTML, RTF, ODT and LaTeX import/export |
| L3 | `render`, `pdf` | vello_cpu page rasterisation; krilla PDF with text, links and outlines |
| L4 | `engine` | Session, command registry, undo, selection, layout caches, samples and central file dispatch |
| L5 | `mcp`, `zotero` | MCP tools and authenticated desktop bridge; local Zotero integration protocol |
| L6 | `ui-egui` | Ribbon, canvas, rulers, panes, dialogs, themes, translations, control channel and offscreen UI harness |
| Hosts | `wordcraft`, `wordcraft-cli`, `wordcraft-web` | Desktop `not-word`, headless `not-word-cli` and browser runner |
| Tooling | `xtask` | Formatting/test gates, attribution, layering, version and parity tools |

The desktop and browser use egui/eframe with wgpu. There is no Electron or Tauri host.
Shared `craft-ui` and native `craft-print` dependencies are pinned to a specific commit of
`storytold/craft-libs`; those engineering dependencies do not imply upstream product ownership.

## Document lifecycle

1. **Open.** `engine::io` selects `docx`, `docbin` or `formats` by extension and returns a
   `doc::Document`. The readers retain supported content and specific opaque structures;
   see [file-format parity](file-format-parity.md) for retention and loss boundaries.
2. **Edit.** A registered `CommandSpec` runs through `Session::run`. Ribbon, keyboard,
   command search, CLI, MCP and control use the same IDs. UI-only commands live at L6.
3. **Undo and recovery.** Session snapshots use shared block storage. The command guard
   restores the document if a panic escapes; production code must still return errors.
4. **Layout.** `layout` shapes and positions text and objects in page display lists.
   Incremental caches reuse unaffected work; compatibility with another editor's pagination
   depends on fonts and supported layout rules.
5. **Draw/export.** The UI caches rendered pages; PDF export reads the same display-list
   structure. Sharing that representation keeps layout and export behavior aligned.
6. **Save.** The central I/O dispatcher selects a writer. Native filesystem access goes
   through engine helpers so automation boundaries and safe writes apply consistently.

## Host responsibilities

The desktop host supplies native file dialogs, clipboard formats, printing, window geometry,
preferences, log files and control-port services. Desktop and CLI implementations live in
`apps/wordcraft/src/lib.rs` and `apps/wordcraft-cli/src/lib.rs`; thin primary and legacy binary
wrappers call the same entry points, avoiding duplicate implementation. The browser supplies browser file APIs,
UI preference storage, IndexedDB font storage, web clipboard support and browser printing.
Its UI preference record does not persist document content. Those integrations need
platform-specific checks even when the shared engine's tests pass.

Appearance and workspace state live in `UiState`; editing preferences belong to `Session`.
`WordApp::prefs` and `apply_prefs` bridge the persisted representation. Theme tokens centralise
colours and spacing. Settings migrations must preserve existing user choices and safely handle
missing or invalid data; see [customisation](customization.md).

## Validation

`cargo xtask ci` runs formatting, Clippy with warnings denied, workspace tests, asset attribution,
layering and shared-crate WASM checks. Focused tests cover document edits, hostile command
parameters, DOCX round trips, legacy formats, layout, typing, UI interaction and MCP flows.
Test counts change; use `cargo xtask stats --exact` when a current count is needed.

Use the offscreen `ui_shot` example to inspect UI changes. A shared-crate WASM check does not
build a distributable browser app; use Trunk and a browser smoke test for host changes. Native
printer, IME and accessibility validation require the relevant platform or hardware.

Catalog coverage generated by `cargo xtask parity` measures registered command presence. It
must not be presented as overall compatibility or depth. Dated upstream estimates remain in
[target-app-parity.md](target-app-parity.md); current work and evidence belong in
[engineering-log.md](engineering-log.md).
