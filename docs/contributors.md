# Contributors and provenance

Not Word is maintained by [Itskorrah](https://github.com/Itskorrah) and derives from WordCraft.
The About window retains upstream contributor and model credits. Product maintenance and
original authorship are different facts: rebranding does not transfer authorship of existing work.

## Credit data

Two files are compiled into the UI binary by `crates/ui-egui/build.rs`, including browser builds;
no network request is needed to display them:

- [`contributors/contributors.json`](../contributors/contributors.json) contains generated
  historical commit statistics by GitHub username. Do not hand-edit or replace it with an
  invented contributor list.
- [`contributors/people.toml`](../contributors/people.toml) contains names contributors chose
  to share. A contributor adds their own entry in their own change; never infer names from
  author emails or copy someone else's profile without consent.

About offers Contributors and Models tabs, flowing names or a table, sorting and display-name
choices. Missing optional names fall back to GitHub usernames. `ui.dialog` with
`{"name":"contributors"}` or `{"name":"models"}` opens the relevant tab.

## Add your chosen name

Every field is optional:

```toml
[people.your-github-username]
real_name = "Your Name"
display_name = "Nickname"
sync_github_name = true
```

Add only your own entry and commit it under your own identity. Removing your entry removes
those optional names from future refreshes; your historical contribution credit remains.
Never fabricate consent or modify another contributor's entry during a product rebrand.

## Generated statistics

The current dataset and its provenance came from the upstream contribution tooling in
[`storytold/craftrules`](https://github.com/storytold/craftrules). Its
[`scripts/contributors.py`](https://github.com/storytold/craftrules/blob/main/scripts/contributors.py)
can refresh/verify data when explicitly available and configured for this fork; it is not a
required sibling checkout for building Not Word. Review the tool's identity/consent checks and
repository scope before running it. Preserve inherited historical data during ordinary coding
work and keep the source of any future refresh explicit.

Required legal and asset credit remain in [NOTICE](../NOTICE),
[ATTRIBUTION.md](../ATTRIBUTION.md), [LICENSE-MIT](../LICENSE-MIT) and
[LICENSE-APACHE](../LICENSE-APACHE).
