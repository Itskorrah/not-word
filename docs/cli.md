# Not Word command line

Build with `cargo build --locked --release -p wordcraft-cli --bin not-word-cli`.
Run `target/release/not-word-cli` directly or add `target/release` to `PATH`.
The `wordcraft-cli` binary name remains a compatibility alias.

## Read, inspect and export

```sh
not-word-cli info report.docx
not-word-cli text report.docx
not-word-cli inspect report.docx
not-word-cli convert report.docx report.pdf
not-word-cli render report.docx page.png --page 1 --scale 2
```

`info` returns document counts and properties as JSON; `inspect` returns structured document
content. Output format follows the destination extension. Legacy `.doc` is read-only; PDF and
PNG are export outputs. Conversion fidelity is bounded by the supported format features in
[file-format-parity.md](file-format-parity.md).

## Run document commands

```sh
not-word-cli commands --json
not-word-cli run --template sample \
  --cmd 'select.text={"text":"Membership"}' \
  --cmd format.bold --save formatted.docx
not-word-cli run --file report.docx --author "Editing assistant" \
  --cmd 'review.trackChanges={"value":true}' \
  --cmd 'text.insert={"text":"Reviewed. "}' --save reviewed.docx
```

Commands have stable IDs and JSON parameters. Quote JSON according to your shell; the examples
above use POSIX shell single quotes. Use `commands` to inspect parameter documentation and
availability rather than guessing syntax. Headless programmatic actions do not open dialogs.

## Integrations

```sh
not-word-cli mcp --automation-read-root /work/input --automation-write-root /work/output
not-word-cli mcp --connect 127.0.0.1:7981
not-word-cli parity --markdown
```

See [MCP](mcp.md) for tools and filesystem authority, [control](control-protocol.md) for app
authentication, and [Zotero](zotero.md) for local citation commands. `--connect` drives an
already-running app and cannot be combined with headless filesystem confinement flags.

`not-word-cli --help` lists the current usage; `--version` reports the workspace version.
New MCP resources use `not-word://document` and `not-word://commands`; legacy
`wordcraft://` resource aliases and internal Rust package names remain compatible.
