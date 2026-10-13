# Releasing Not Word

This fork inherits a multi-platform packaging pipeline from WordCraft. Existing upstream
releases, signing credentials and download links do not constitute releases of Not Word.
Review the actual workflow and generated packages before publishing a fork release.

## Prepare a release

1. Verify `main`, user changes, the intended release scope and repository/branch permissions.
2. Set the workspace version with `cargo xtask version set <version>`. Review its diff; the
   workspace package version is shared by the apps and shown in About/`--version`.
3. Run `cargo xtask ci`, relevant packaging checks and native/browser smoke tests. Record
   unavailable checks rather than treating one host as proof for every platform.
4. Review `.github/workflows/release.yml` and local packaging scripts for Not Word names,
   repository URLs, bundle/package IDs, update endpoints and retained legal notices.
5. Check the fork's GitHub environment and branch protection before invoking workflows.
   Signing and notarisation require the fork maintainer's own configured credentials.
6. Verify checksums, inspect artifacts and draft release notes from actual completed changes.
   Publish only after the requested release process and required checks permit it.

The fork now has independent workflow orchestration for macOS, Windows, Linux, FreeBSD and
WebAssembly, with a `cargo xtask ci` gate, pinned actions and target-specific signing and
cross-compilation steps. Production signing is mandatory; explicit unsigned-test dispatches
produce test artifacts rather than a production release. Release tags must be fresh so the
rebrand is not associated with an old upstream commit. A configured pipeline is not proof
that artifacts exist or that they have been signed. Do not copy upstream signing identities.

## Local packaging entry points

| Target | Existing script |
|---|---|
| macOS | `packaging/macos/package.sh --arch universal` |
| Windows | `packaging/windows/package.ps1` |
| Linux | `packaging/linux/package.sh`; `packaging/linux/flatpak-bundle.sh` |
| Browser | `packaging/web/package.sh` |
| App icons | `packaging/icons.sh` |

Scripts and internal filenames may retain compatibility names while generating Not Word
metadata. Read each script's arguments and target requirements before running a costly build.
Keep artifact checksums, TLS verification and package signature verification enabled.

## Artifact identity

The package ID is `io.github.itskorrah.NotWord`. A versioned release pipeline is configured to
produce these public names; this list does not claim that a release has run:

| Target | Configured artifact name |
|---|---|
| macOS | `not-word-<version>-macos-universal.dmg` with `Not Word.app`; `not-word-cli-<version>-macos-universal.zip` |
| Windows | `not-word-<version>-windows-<x64,x86,arm64>.msi` and `-portable.zip` |
| Linux | `not-word-<version>-linux-<arch>` with `.AppImage`, `.AppImage.zsync`, `.deb`, `.rpm`, `.tar.gz` or `.flatpak` as supported per target |
| FreeBSD | `not-word-<version>-freebsd-x86_64.tar.gz` |
| Browser | `not-word-web-<version>.zip` |

Run `python3 packaging/check_metadata.py` (requires PyYAML) for consistency checks. The
packaging lint workflow also validates actions, shell/PowerShell syntax, XML/plists, Desktop
and AppStream metadata. The DMG artwork is original Not Word artwork and reproducibly
generated. Native installers and signing still need their actual platform/release runners.

## Release evidence

A release report should link the fork's workflow run and release, list generated platform
artifacts, record verified signing/checksum state and identify smoke tests that ran. Do not
advertise unavailable installers or imply that an unsigned build is signed/notarised.
The [engineering log](engineering-log.md) records implementation and validation; a release
should refer to completed commits and pull requests, rather than planned capabilities.
