# Agent Instructions

Instructions for anyone (human or coding agent) changing this repository.
What zipp does is in `docs/`, what comes next in `docs/PLAN.md`.

- Keep it simple. Prefer straightforward code that follows nearby patterns
  over abstractions, extra validation or options for theoretical cases.
  Build the smallest version that solves the actual need; every code path
  must be reachable today.
- The content model is code. Don't add ways to change it at runtime.
- Content stays plain JSON. Whatever has to be queried gets its own rows
  (`entry_values`, `entry_relations`), written on every save and rebuilt
  after migrations, never a second copy that can drift.
- `db.rs` holds the tables of zipp itself, changes of a site's content
  model are schema migrations (`migrate/`). While zipp is not live,
  `db.rs`' first migration and the example's `0001_initial` are edited in
  place instead of adding new ones; real migrations start with the first
  live site.
- Rust is formatted with `cargo fmt` (hard tabs, max width 80), checked
  with `cargo clippy --all-targets`. Svelte and TypeScript use prettier
  (tabs, single quotes) and `npm run check`.
- Use blank lines to separate logical steps inside functions.
- Tests are small, offline unit tests next to the code (`#[cfg(test)] mod
  tests`). Database tests use `Db::memory()`.
- Commits: a short imperative subject in plain words, then a body that
  says what changed and why.

## Layout

- `crates/zipp/src`: one folder per area (`schema`, `content`, `entries`,
  `drafts`, `graphql`, `migrate`, `assets`, `ai`, `craft` for the
  import), each with a `mod.rs` for its types and core functions, files
  for the parts, `routes.rs` for the HTTP handlers and `tests.rs` when
  the tests are long. Types that several files of a folder use stay in
  its `mod.rs`. `lib.rs` is the
  public api, `cli.rs` the commands, `server.rs` the router.
- `studio/src`: `api/` (one file per area, `index.ts` re-exports),
  `pages/` (`edit/` for the editor), `fields/` (one component per field
  kind), `lib/` (shared components and helpers). Shared styles are in
  `app.scss`.

## Decisions

What the owner decided, so it doesn't get asked again:

- The content model is code, also roles, SEO defaults, translations keys,
  offline sites and image widths. Editors change content, never the model;
  they see the model read only under Setup.
- Everything editors change has drafts, revisions and previews, also
  globals, translations and the SEO settings (they are singles).
- zipp replaces Craft behind crelte frontends. GraphQL answers in Craft's
  shape where it helps a port (arguments like `id: ["not", 1]`, `orderBy`,
  `seoJson`), but the type names don't have to match Craft's; small
  changes of a project's queries are fine.
- GraphQL is private like Craft's: frontends send a token, the studio a
  session. The rest api is the studio's.
- SEO is the plugin `crates/zipp-seo` (one line turns it on) and does
  what the Craft plugin meierlabs-seo does, so its npm package works
  unchanged. It is also the test of the plugin system: what it needs from
  zipp belongs into the `Plugin`/`FieldType` api, not into zipp's core.
- Admins see field handles; entry types can have tabs, Setup shows each
  type's editor as editors see it.
- The studio looks like the old Zipp (dark top bar, white side nav, grey
  page, white cards), its pages are paths under `/admin`.
- AI models run through soe-llms on OpenRouter, never a provider's api
  directly; the AI only writes drafts and can do what its user can.
  Spending limits are set on the key, not in zipp.
- GraphiQL only in development (`--dev`).
- Big JavaScript in the studio is fine, only editors load it, once.

## Before a commit

- `cargo fmt`, `cargo clippy --all-targets`, `cargo test`, and in
  `studio/` `npx prettier --write src` and `npx svelte-check`.
- A change of the studio: `npm run build` in `studio/` and commit the
  embedded build in `crates/zipp/studio`.
- A change of the example's schema: regenerate `0001_initial`, then
  `cargo run -- check --queries ../crelte/svelte/src/queries` in
  `example/cms`, it fails on what `serve` would refuse.
- New features get a line in `docs/` and, if the example can show them,
  in the example and `example/readme.md`.
