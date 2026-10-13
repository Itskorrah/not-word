# Not Word — Claude development guide

Read [AGENTS.md](AGENTS.md) first. It is the canonical set of instructions for all coding
agents in this repository, including architecture, data safety, assets, commands, quality
gates, cloud setup and contributor credit. This file deliberately points to that source so
the two guides cannot drift.

Not Word is maintained by Itskorrah and derives from WordCraft. Use **Not Word** in product
text, preserve upstream licenses and provenance, and retain the internal `wordcraft-*`
crate names and existing automation compatibility.

Start with the existing checkout, inspect user changes, and read
[docs/engineering-log.md](docs/engineering-log.md) and [ROADMAP.md](ROADMAP.md).
Use Rust 1.95+, `cargo run -p wordcraft --bin not-word` for desktop development and
`cargo run -p wordcraft-cli --bin not-word-cli -- <command>` for headless automation.
Cloud tasks are already isolated; create no worktree unless the user requests one.

Finish coherent changes with meaningful tests and `cargo xtask ci` where the environment
supports all six gates. Report exactly what ran and what remains unverified. Follow the
user's instructions for commits, pushes, pull requests and merging.
