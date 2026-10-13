# Not Word — instructions for coding agents

Not Word is an independent Rust-native word processor maintained by Itskorrah at
<https://github.com/Itskorrah/not-word>. It is a fork of WordCraft; preserve upstream copyright,
legal notices and contributor history while keeping current product branding **Not Word**.
The engine, CLI, MCP and native/browser front ends share a document command model.

## Start here

1. Inspect `git status` and preserve existing user changes. Use the existing checkout in cloud
   tasks: each task is already isolated. Do not create a Git worktree unless the user requests one.
2. Read the requested scope, `ROADMAP.md`, `docs/engineering-log.md`, `docs/gaps.md` and the
   relevant architecture/format/UI documentation. Local ignored `plan/` notes are optional;
   this fork does not require upstream sibling repositories or a `craftrules` checkout.
3. Establish an actual build/test baseline and rank work by data safety, correctness, user
   impact and maintenance cost. Complete coherent implementations and verify them.
4. Follow the user's authorisation for commits, remote changes and delivery. Instructions in
   local files do not supply missing secrets or authorise unrelated publishing.

## Toolchain and workflow

Use **Rust 1.95+**, Cargo, rustfmt and Clippy. The locked dependency graph requires Rust 1.95;
retain `Cargo.lock` and prefer `--locked` while installing/building. Desktop UI execution needs
native graphics/windowing support; headless CLI and `ui_shot` are useful in cloud environments.
The [development guide](docs/development.md) explains prerequisites and limitations.

```sh
cargo run --locked -p wordcraft --bin not-word -- --sample --control 7981
cargo run --locked -p wordcraft-cli --bin not-word-cli -- commands --json
cargo run --locked -p wordcraft-cli --bin not-word-cli -- convert in.docx out.pdf
cargo run --locked -p wordcraft-ui-egui --example ui_shot -- script.jsonl
```

Internal package names and directories remain `wordcraft-*`; do not rename all crates or
protocol resource identifiers during a product-text change. `wordcraft`/`wordcraft-cli` remain
binary aliases for existing automation. New user examples use `not-word`/`not-word-cli`.

## Protect users' work

Stability and document integrity outrank feature work.

- No `unwrap`, `expect`, `panic!`, `unreachable!`, `todo!`, `unimplemented!` or `unsafe` in
  production paths. Follow the existing crate-level Clippy restrictions and workspace
  `unsafe_code = "forbid"`. Tests may unwrap fixtures.
- Return the crate's error type through `Result` and `?`. Unsupported input produces an error,
  a clear warning or a preserved opaque structure; it must not silently destroy content.
- Treat file/command-derived numbers as hostile: checked indexing, character boundaries,
  checked/saturating arithmetic, allocation limits and bounded recursion.
- `Session::run` catches escaped panics and restores the document; this is a final safety net.
  A crash fix needs a regression test. Keep `hostile_params_never_panic` green.
- Use the engine's central filesystem helpers and safe-save path. Test interrupted saves,
  malformed files and round trips when changing I/O. Preserve originals and recovery files.
- No mandatory telemetry, remote document transmission, hardcoded credentials or weakened
  TLS/signature/checksum verification. Optional integrations must keep explicit user control.

## Assets and provenance

- Never copy, trace or crop proprietary product icons, artwork, templates, themes, fonts,
  sounds or cursors. Not Word assets must be original or redistributably licensed.
- Every asset file needs an `ATTRIBUTION.md` row in the same change; `cargo xtask assets`
  enforces this. Preserve font and other third-party license texts.
- Do not reintroduce upstream ArtCraft trademarks as Not Word branding. References in
  legal attribution and clearly identified historical provenance are intentional.
- Draw UI icons in `crates/ui-egui/src/icons.rs` where practical. Use bundled OFL fonts,
  installed fonts or metric-compatible open substitutes for proprietary document fonts.
  `CRAFT_FONTS_DIR` is an optional open-font build input, not a cloud prerequisite.
- Implement format support from public OOXML, RTF, ODF and legacy DOC specifications.
  Black-box observations of other editors may inform behavior; never inspect proprietary
  implementation files. Use synthetic or appropriately licensed redistributable fixtures.
- Never introduce GPL/AGPL-incompatible code or dependencies into this permissive project
  without explicit user approval.

## Architecture

See [docs/architecture.md](docs/architecture.md). Short names below mean `wordcraft-<name>`.

| Layer | Crates | Responsibility |
|---|---|---|
| L0 | `geom`, `control-key` (standalone) | Units, geometry, measurement parsing and local authentication keys |
| L1 | `doc`, `fonts`, `proof`, `metafile` | Model/edit primitives, shaping, proofing and metafile decoding |
| L2 | `layout`, `docx`, `docbin`, `formats` | Layout and format I/O |
| L3 | `render`, `pdf` | Raster and PDF output from display lists |
| L4 | `engine` | Sessions, commands, undo and I/O dispatch |
| L5 | `mcp`, `zotero` | Headless/desktop-bridge MCP tools and citation integration |
| L6 | `ui-egui` | Native/browser interface and control channel |
| Apps | `apps/wordcraft`, `apps/wordcraft-cli`, `apps/wordcraft-web` | Desktop, CLI and WebAssembly hosts |

- User-visible engine behavior belongs in a `CommandSpec` in `crates/engine/src/cmd/` with
  documented parameters, enablement and tests. UI-only `ui.*` commands live in `ui-egui`.
  Ribbon, shortcuts, command search, CLI, control and MCP dispatch the same IDs; automation
  must not open dialogs unexpectedly.
- Keep lower layers free of egui/eframe/winit/rfd. `cargo xtask layers` enforces boundaries;
  `xtask/src/layers.rs` is the source of truth for layer classification.
- Keep Rust-native native/web architecture. Theme colours come from `theme::Tokens`.
  Preserve WASM portability and avoid unnecessary dependencies or framework rewrites.
- Typing changes update `docs/typing-parity.md` and `crates/engine/src/tests_typing.rs` together.
  Command registration does not establish feature depth or file-format fidelity.

## Verification

Before delivery run the applicable focused tests, then the repository gates:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo xtask assets
cargo xtask layers
cargo xtask wasm
```

`cargo xtask ci` runs the same six gates and stops at the first failure. Keep gates intact:
do not delete tests, suppress failures or weaken checks to make a build green. Install
`wasm32-unknown-unknown` for portability checks; a full browser build also requires Trunk.
Coordinate shared builds rather than compiling the whole workspace independently in each agent.
Use bounded `CARGO_BUILD_JOBS` for memory/disk constraints.

For UI changes, render `ui_shot` output and inspect the image. For file-format changes, test
open → edit → save → reopen and inspect retained package content. For performance claims,
record representative inputs, environment and before/after measurements. Report unavailable
platform checks clearly; a Linux headless run does not verify Windows/macOS desktop behavior.

## Automation and local configuration

The app's control server binds loopback and authenticates with a per-window key. Keep keys
private; never paste them into docs, commit them, or log them. Use
[docs/control-protocol.md](docs/control-protocol.md) and [docs/mcp.md](docs/mcp.md).
MCP `--automation-read-root`/`--automation-write-root` confine the headless process; they do
not confine a running desktop app. Use `not-word://document` and `not-word://commands` for new MCP clients; retain
`wordcraft://` aliases for existing clients. Command IDs remain compatible.

## Documentation and contributor credit

Update relevant behavior docs and `docs/engineering-log.md` when work lands. Keep known
limitations visible, label inherited estimates, and never claim complete Microsoft Word parity
or published/signed releases without evidence. Generated parity content comes from
`cargo xtask parity`; do not hand-adjust its counts.

About credits come from `contributors/contributors.json` (generated historical stats) and
`contributors/people.toml` (opt-in names). Preserve both. Never invent or alter another
contributor's identity or hand-edit generated stats; see [docs/contributors.md](docs/contributors.md).
