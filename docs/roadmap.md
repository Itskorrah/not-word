# Not Word: milestones and next steps

Not Word stays **alpha** while document fidelity, recovery and platform integrations are
validated. This roadmap lists outcomes; it does not assign speculative readiness percentages
or promise a completion date. The [summary](../ROADMAP.md) and
[engineering log](engineering-log.md) describe scope and completed evidence.

## Current focus

1. **Protect documents.** Strengthen malformed-input handling, save failure behavior, backups,
   recovery and preference migrations. A crash or data-loss regression takes priority.
2. **Establish independent identity.** Original Not Word artwork and metadata; remove upstream
   trademarks from the active product while retaining legal notices and contributor history.
3. **Make everyday editing clearer.** Modern light/dark appearance, accessible contrasts,
   persistent workspace controls and keyboard/automation access to the same actions.
4. **Improve DOCX retention.** Preserve unsupported package content where feasible, warn about
   unsupported behavior and build repeated save/reopen tests for relationships and metadata.
5. **Measure slow workflows.** Use realistic long documents and repeated edits; improve proven
   bottlenecks without weakening correctness or adding caches that retain stale data.

## Milestones

| Milestone | Existing foundation | Next evidence / completion gate |
|---|---|---|
| Everyday writing | Rich editing, styles, lists, search, undo, tables, references and review commands | Long-session typing/undo checks; verify enabled behavior and discoverability |
| Document exchange | DOCX read/write, legacy DOC import, ODT/RTF/HTML/Markdown/LaTeX/text, PDF export | Representative corpus, retained-part inspections and repeated round trips |
| Product identity | Independent fork with preserved licenses and contributor data | App, browser, packaging and original asset consistency |
| Personal workspace | Theme, scaling, toolbar and editing preferences | Verified customisation commands, presets and migration across restarts |
| Native/browser integration | Rust desktop hosts, browser persistence/printing, local control and MCP | Actual platform tests for dialogs, clipboard, printer, IME and accessibility |
| Performance | Incremental layout and rendered-page caches | Repeatable large-document benchmarks with environment and limits recorded |
| Release delivery | Existing packaging workflows inherited from upstream | Fork metadata, artifact names, signing settings, CI and release smoke checks |

The detailed upstream [gap list](gaps.md) and [parity assessment](target-app-parity.md) contain
historical technical observations. Re-validate an item before treating it as currently open:
several features arrived after those assessments. Upstream issue/PR numbers are provenance,
not a live queue of requests in this fork.

## Alpha gate (core workflows)

For each relevant host, verify that users can complete a workflow and retain their documents:

| Workflow | Acceptance evidence |
|---|---|
| Write and format | Type, styles/lists, search, undo, save DOCX and reopen with intended formatting |
| Exchange a received document | Open, edit, save, reopen; retain supported content and opaque parts; inspect losses |
| Build structured content | Tables, pictures, headers/footers, notes, sections and page numbering remain consistent |
| Review | Comments/tracked edits retain author and content; accept/reject and undo behave predictably |
| Produce references | TOC, citations, captions and notes update and survive file exchange within documented limits |
| Share and print | PDF export validated; native/browser printing exercised where the backend is available |

Synthetic tests are an essential baseline. A beta claim also needs representative real-world
fixtures and actual platform evidence. Keep user documents private, and use synthetic or
appropriately licensed fixtures in the repository.

## Next opportunities

Prioritise format fidelity and reliable existing workflows over an unverified breadth count.
Useful follow-on work includes stronger document inspection, reusable automation, richer
Markdown exchange, template management and accessible keyboard flows. Full Word compatibility,
real-time co-authoring, VBA execution and cloud/AI integrations are separate large commitments;
optional integrations must preserve offline use and explicit document privacy.
