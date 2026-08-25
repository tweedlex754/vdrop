# Contributing

## Getting it running

```bash
npm install          # npm workspaces: root + frontend
npm run tauri:dev    # a real window, with live reload
```

`npm run dev` opens only the interface in a browser against a fake IPC layer.
It is much faster for styling work, but nothing actually downloads.

## Checks

```bash
npm test             # cargo test --workspace
npm run test:front   # vitest
npm run lint         # cargo clippy -D warnings
npm run test:live    # needs network; hits real servers
```

Live tests are marked `#[ignore]` so they stay out of the normal run. They
are the ones that catch "the parser is right but the wrong quality lands on
disk", so run them when you touch resolving or downloading.

## What the codebase expects

**Business logic lives in the crates, not in `src-tauri`.** The desktop shell
is wiring: it routes IPC commands to crates that compile and test without
Tauri. That is why the test suite runs in seconds.

**Comments explain why, not what.** The code says what it does. A comment
earns its place by recording the reason a choice was made, or the failure that
forced it. `docs/DURUM.md` keeps the longer form of that history.

**Tests should fail for a reason you can name.** Prefer a test that pins the
behaviour someone might innocently undo. A few in here exist purely to make a
tradeoff visible - for example one asserting that search folds diacritics
together, so nobody "fixes" it back into an exact match.

**Do not synchronise with wall clocks.** `sleep(120ms)` looked fine and broke
under load; timing-sensitive tests wait on an observable event instead, and
rate-limit tests use tokio's virtual clock.

**Look at the interface when you change it.** Several defects in this project
passed every test and were still visibly wrong: a toggle group that assumed
exactly two buttons, an error note that was never rendered, a dictionary with
no Turkish diacritics. Screenshots catch what assertions do not.

## Translations

Every language file is checked against the `Dictionary` type derived from
`frontend/src/i18n/tr.ts`, so a missing key stops the build rather than
shipping an empty label. A test also compares key sets across all languages.

To add a language: copy `en.ts`, translate the values, register it in
`i18n/index.tsx` with its name **in that language** and the correct `dir`.
Right-to-left works; the stylesheet uses logical properties rather than left
and right.

## Commits

Explain the change and the reason it was needed. If a wrong turn cost real
time, say so - the next person is going to consider the same turn.
