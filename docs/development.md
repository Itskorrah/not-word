# Developing Not Word

Use the existing `/workspace/not-word` checkout in a managed cloud task. Each task already
runs in an isolated environment; do not create a Git worktree unless requested. Inspect
`git status` before work and preserve unrelated changes.

## Prerequisites

- Rust **1.95 or newer**, Cargo, rustfmt and Clippy. Use the version declared in the workspace
  and its locked dependencies; older upstream toolchain requirements are insufficient.
- A C linker and platform SDK/build tools. Linux desktop execution needs X11 or Wayland,
  keyboard libraries and a working Vulkan/OpenGL backend. Native file dialogs generally use
  the XDG desktop portal. A displayless cloud machine can still run the CLI and offscreen UI.
- `wasm32-unknown-unknown` for shared portability checks. Trunk is needed only when serving or
  packaging the web app.
- Network access during dependency acquisition to crates.io, Rust distribution servers and
  the pinned `storytold/craft-libs` Git repository. Normal document editing is local.

```sh
rustup component add rustfmt clippy
rustup target add wasm32-unknown-unknown
cargo fetch --locked
```

Do not print secrets or credential files to debug a fetch. Managed cloud Git access may use
an injected HTTPS proxy rather than `GH_TOKEN`. Keep TLS and artifact verification enabled.
Optional fonts can be supplied through `CRAFT_FONTS_DIR`; a sibling fonts checkout is not
required for the standard source build.

## Build and run

```sh
cargo build --locked -p wordcraft --bin not-word
cargo build --locked -p wordcraft-cli --bin not-word-cli
cargo run --locked -p wordcraft --bin not-word -- --sample
cargo run --locked -p wordcraft-cli --bin not-word-cli -- info report.docx
```

Add `--release` for an optimised build. For memory-limited machines use a bounded
`CARGO_BUILD_JOBS` value and coordinate workspace builds across agents; reuse valid target
outputs instead of creating many independent copies. Package names remain `wordcraft` and
`wordcraft-cli`; their primary binaries are `not-word` and `not-word-cli`.

## Useful headless validation

CLI conversion exercises parsing, layout and export without opening a window:

```sh
cargo run --locked -p wordcraft-cli --bin not-word-cli -- run --template sample --save /tmp/not-word-sample.docx
cargo run --locked -p wordcraft-cli --bin not-word-cli -- inspect /tmp/not-word-sample.docx
cargo run --locked -p wordcraft-cli --bin not-word-cli -- convert /tmp/not-word-sample.docx /tmp/not-word-sample.pdf
```

For UI evidence, save a JSON-lines script outside the checkout, for example
`/tmp/not-word-shot.jsonl`:

```json
{"method":"ui.theme","params":{"value":"light"}}
{"steps":6}
{"shot":"/tmp/not-word-light.png"}
```

```sh
cargo run --locked -p wordcraft-ui-egui --example ui_shot -- /tmp/not-word-shot.jsonl
```

`ui_shot` uses the app's actual offscreen egui/wgpu interface and starts with the sample
document. It returns a failure exit status for malformed scripts, control errors or failed
renders. Screenshot sizes are bounded to 320–4096 per axis; script steps and waits are capped. It needs a supported graphics adapter; a correctly installed software Vulkan
adapter can be useful on Linux. Inspect the resulting PNG, and distinguish rendering evidence
from a native-window interaction test. See the example header and
[control protocol](control-protocol.md) for script requests.

## Quality gates

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo xtask assets
cargo xtask layers
cargo xtask wasm
```

`cargo xtask ci` runs all six in sequence, stopping at the first error. Run relevant focused
regression tests while implementing, then complete applicable gates before delivery. Do not
weaken checks or suppress failures. The crate/package selector remains `wordcraft-*`, for
example `cargo test --locked -p wordcraft-docx`.

`cargo xtask parity` rewrites the generated checklist; review that diff separately from
implementation. `cargo xtask stats --exact` asks test harnesses for current counts. Synthetic
or openly licensed fixtures belong in test coverage; private user documents do not.

## Current managed-cloud setup

The validated Linux setup uses a workspace-local Rust/Cargo installation and verified Debian
13 native-library packages extracted into a local sysroot. It keeps package-signature and
artifact-checksum verification enabled and does not need a system-wide installation. The
session helper is `/workspace/.wordcraft-setup/env.sh`; when it is present, load it before
commands:

```sh
source /workspace/.wordcraft-setup/env.sh
cd /workspace/not-word
```

The helper sets workspace-local tool/runtime paths, a software Vulkan adapter, four Cargo jobs,
and development/test debug info and incremental compilation disabled to stay within disk
limits. The setup used Rust 1.99 and Trunk 0.21.14. Internal helper-directory names are retained
for the existing environment. These measured settings are not platform requirements or a
substitute for the general prerequisites above. Reuse the cloud environment's saved installer
and startup instructions where supplied.

## Logs

Native logs go to standard error and `<settings>/logs/not-word.log`; the previous two launches
are retained as `not-word.1.log` and `not-word.2.log`, with a 16 MiB file limit. The settings
folders are listed in [customization.md](customization.md). `NOT_WORD_NO_PREFS` (or legacy
`WORDCRAFT_NO_PREFS`) makes isolated test runs log to standard error without preference files.
`RUST_LOG` accepts directives such as `warn,wordcraft*=debug`; internal Rust log targets retain
compatible names. Inspect a log before reporting a file/driver issue, and avoid sharing private
paths or document metadata unnecessarily.

## Browser

Install Trunk through its official instructions, then:

```sh
cd apps/wordcraft-web
trunk serve
```

The configured address is <http://127.0.0.1:8771/?sample>. Production builds use
`trunk build --release` and write to `dist/web`. A Trunk browser build and smoke test are
separate from `cargo xtask wasm`, which checks the shared layered crates. The actual Chromium/Playwright smoke used WebGL2, loaded and changed
appearance through the command palette, and verified preferences after reload. Add `webgl`
to the query to request that fallback explicitly, for example `?sample&webgl`. On the tested
software browser backend, SwiftShader and unsupported viewport title/theme command warnings
were observed without page errors; the app's canvas and selected theme rendered correctly.
This does not certify every browser/GPU or native integration.

## Platform troubleshooting

- A Linux cloud environment without a display should use CLI tests and `ui_shot`; native UI
  startup needs an available X11/Wayland display.
- Windows graphics selection can be overridden with `WGPU_BACKEND` when diagnosing driver
  failures. Keep troubleshooting local and document the actual adapter/backend tested.
- macOS document-open events, native printing and packaging need a macOS host. A Linux build
  cannot verify those integrations.
- Check logs and report exact errors before installing new dependencies. Inspect environment
  variable names/presence, never values. Missing native runtime services are distinct from a
  Rust compilation failure.

Keep screenshots, temporary scripts and logs outside tracked source files unless they are
intentional documented assets. Record actual verification and remaining platform limits in
[engineering-log.md](engineering-log.md).
