# Agent Instructions

Instructions for anyone (human or coding agent) changing this repository.
What zipp does is in `docs/`, what comes next in `docs/PLAN.md`.

- Keep it simple. Prefer straightforward code that follows nearby patterns
  over abstractions, extra validation or options for theoretical cases.
  Build the smallest version that solves the actual need; every code path
  must be reachable today.
- The content model is code. Don't add ways to change it at runtime.
  Writing a site's schema: follow "Designing a content model" in
  `docs/content-model.md` (groups for fields that belong together, tabs,
  widths, instructions).
- Content stays plain JSON. Whatever has to be queried gets its own rows
  (`entry_values`, `entry_relations`), written on every save and rebuilt
  after migrations, never a second copy that can drift.
- `db.rs` holds the tables of zipp itself, changes of a site's content
  model are schema migrations (`migrate/`). Since 0.1.0 sites run
  `db.rs`' migrations: they are frozen (`db_tests.rs` pins their hash),
  a change of zipp's tables is a new entry at the end. The example isn't
  live, its `0001_initial` is still regenerated instead of adding new
  ones.
- The serde shape of a migration file (`Migration`, `Op` and the schema
  in it: `Schema`, `Section`, `Component`, `Field`, ...) is a format
  sites keep in git; the studio reads the same shape from `/schema`.
  Only add fields with a default; a rename or a change needs a new
  `migrate::FORMAT` that still reads the old files. The files in
  `migrate/fixtures/` are as released and are never regenerated.
- Rust is formatted with `cargo fmt` (hard tabs, max width 80), checked
  with `cargo clippy --all-targets`. Svelte and TypeScript use prettier
  (tabs, single quotes) and `npm run check`.
- Use blank lines to separate logical steps inside functions.
- Tests are small and offline: unit tests of an area's functions in its
  `tests.rs` (database tests use `Db::memory()`), and every request of
  the api in its `api_tests.rs` through `testing::TestServer`, with each
  role that may and may not (`Error::Forbidden`). A bigger part of an
  area can have its own (`migrate/apply_tests.rs`,
  `assets/archive_tests.rs`). The test site and what several areas'
  tests share are in `fixtures.rs`, files in `tempfile::tempdir()`.
  Tests run on the real SQL, no mocks.
- The public api is what sites and plugins use (`lib.rs` says where to
  start), everything else is `pub(crate)`. Public data structs and enums
  are `#[non_exhaustive]`, made with constructors or builders, so zipp
  can add a field without a major version; crates of the api are
  re-exported (`zipp::axum`, `zipp::rusqlite`, ...).
- Permissions are in a handler's signature: `Admin`, `EntryAccess<Publish>`,
  `AssetAccess<Edit>`, `AiAccess`; `Authed::check` only where the section
  comes with the request (creating, lists).
- Commits: a short imperative subject in plain words, then a body that
  says what changed and why.

## Layout

- `crates/zipp/src`: one folder per area (`schema`, `content`, `entries`,
  `drafts`, `graphql`, `migrate`, `assets`, `auth`, `users`, `studio`,
  `ai`, `craft` for the import). An area with an api has the same files:
  - `mod.rs`: its types and its functions on `&Connection` (the SQL),
    files for bigger parts (`entries/save.rs`, `assets/manage.rs`);
    types that several files of a folder use stay in `mod.rs`;
  - `api.rs`: the requests and responses, each request `request!(Req,
    METHOD "/path/{id}" => Resp)` (see `api.rs` and docs/extending.md);
  - `routes.rs`: the handlers: who may (an extractor of `auth`), the
    request (`Req<..>`), a call of the functions, the answer; no SQL;
    added with `.api::<Req, _>(handler)`;
  - `tests.rs` and `api_tests.rs`, see above.
  `lib.rs` is the public api, `cli.rs` the commands, `server.rs` the
  router, `testing.rs` the test server. Plugin crates have the same
  files.
- `studio/src`: one folder per area like the server: `entries/` (lists,
  the editor in `edit/`, drafts), `assets/` (the library, the asset
  panel, the image editor in `image/`), `fields/` (one component per
  field kind, `registry.ts` maps the kinds, `rich/` the rich text),
  `setup/`, `ai/`, `users/`, `graphql/` (the tokens), `auth/`. An area
  has its `api.ts` (the requests of its server area), its pages and
  components and their logic in `.ts`. An area that shows a second
  server area or plugin has its requests in a file named after it:
  `entries/drafts.ts` (`drafts`), `ai/agent.ts` (`zipp-agent`). What several areas use:
  `zipp-ui` (`studio/ui`, the
  components without an area and the icons, a package that plugins use
  too), `core/` (`client.ts` sends requests and reads
  streams, `schema.ts` the schema's types and lookups, `router` the
  pages, the path and the site, `paths` the paths of the pages, `values`
  field values (`clean`), `access`, `tree` (depths and parents),
  `format`, `list`, `busy`, `latest` (the newest of overlapping loads,
  searches as you type), `flyout`, `synced`), `shell/`
  (the app with its pages, the sidebar). An area imports from `zipp-ui`,
  `core/` and other areas' `api.ts` and components; `core/` imports no
  area, `zipp-ui` nothing of the studio (plugins bundle it). Tokens and
  shared classes are in `sass/main.scss`, breakpoints and mixins in
  `ui/vars.scss` (every style block of the studio has them, like zandi;
  a component of zipp-ui uses them with `@use '../vars' as *`).

## The studio

- One component per piece of UI, one look. Before building something,
  use what `zipp-ui` and the areas have:
  - overlays: `Flyout` (the panel from the right, also for pickers),
    `Popover` (menus, pickers under their button; Escape and a click
    outside close it, `layers.ts` closes only the top one), `Menu` (⋯),
    `SearchList` in a popover, `.popup-item` for its lines;
  - pages: `PageTop` (crumbs or title, state, actions), `.with-aside`
    and `.cards` for the layout, `Table` for every table (users, Setup;
    `href` opens a row), `.empty` for nothing and Loading…, `.error` for
    errors;
  - lists: every list that has an order moves with `DragHandle` and
    `sortable` (`zipp-ui/sortable`: rows with `data-sort` and a depth,
    drag, its menu and the arrow keys; trees move a level up and down),
    `SortLine` shows where;
  - fields: `ItemCard` (blocks, embeds, nav items), `FieldGroup` and
    `FieldRow` (the card of a group, rows with the label on the left,
    for the fields of plugins), `ItemActions`
    (duplicate, remove in every list), `EntryChip` and `AssetChip` (a
    chosen entry or file), `AddButton`, `UploadButton`, `SearchInput`,
    `Segmented` (tabs and short choices), `Switch` (yes or no),
    `Checkbox` (one option, never a native one), `CheckGroup` (some
    of a few choices, the same tick), `entries/EntryPicker`,
    `entries/SiteBadges`,
    `assets/AssetPicker`;
  - text: `.muted`, `.note`, `.truncate`, `label.stacked`.
- Pages: chuchi's router (`core/router.svelte.ts`, like hostdinghy).
  Each area registers its pages in its `routes.ts` with a lazy import,
  in a frame (`inLayout(Shell, [[path, () => import(...)]])`). A page
  module exports in `<script module>`: `loadProps(props, lp)` for its
  data, so it shows loaded (the bar at the top shows the loading), and
  `isPublic` when no login is needed. The frame's `loadProps` gives
  every page the `schema`; an error of a page's `loadProps` shows in the
  frame. Links to `/admin/...` open without a reload, `go(path)` too.
- Who is logged in: `session` (`core/session.svelte.ts`), a class with
  the token, its end and the user, loaded at the start through `GET
  /auth/session` (which renews a session that ends within 10 days);
  `lp.session` in `loadProps`. The schema has no user.
- Errors of the api are `ApiError` (`core/client.ts`): `type` and
  `detail` like the server's `Error`, `fields` for INVALID.
- A component renders; what it does goes into a `.ts` (or `.svelte.ts`
  for state) next to it when it is more than a few lines: geometry,
  loading and saving, streams, trees, lookups. Classes with `$state`
  fields work well for a page's state (`entryEditor.svelte.ts`,
  `assetForm.svelte.ts`, `chat.svelte.ts`); `Busy` runs an action with
  its busy state and error.
- CSS uses the tokens of `sass/main.scss` (colors, `--fs-*`, `--radius*`,
  shadows, z-index layers), no hex colors or pixel font sizes in
  components; a component styles only its own layout. Sizes are rem
  (16px = 1rem), px only for hairlines up to 3px and shadows.
- Media queries only through the mixins of `ui/vars.scss`
  (`@include max-tablet`, `max-desktop`, ...); `:hover` only inside
  `@include hover { ... }`, so touch screens get no sticky hover and
  what shows on hover is always there without a pointer.
- Lookups by handle: `core/schema.ts` (`sectionOf`, `siteOf`, ...); paths of
  pages: `core/paths.ts`; lists of a field: `core/list.ts`;
  links: `fields/link.ts`; dates and sizes: `core/format.ts`.
- A field never writes its value when it mounts (opening an entry must
  not make a draft); new fields go into `fields/registry.ts`.

## Decisions

What the owner decided, so it doesn't get asked again:

- The content model is code, also roles, SEO defaults, translations keys,
  offline sites and image widths. Editors change content, never the model;
  they see the model read only under Setup (a role can be without it).
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
- The coding agent in the chat is the plugin `crates/zipp-agent`
  (operator 2026-10-28): development only, admins only, in a sandbox VM.
- The studio's look follows the operator's design (2026-10-29, Claude Design
  "Variante E"): light sidebar with the site and the nav groups, a white bar
  with breadcrumbs, state and actions on every page, Inter (bundled, nothing
  from other servers), one blue accent; collapsible blocks with an outline
  in the sidebar, error counts on the tabs. Pages are paths under `/admin`.
- AI models run through soe-llms on OpenRouter, never a provider's api
  directly; the AI only writes drafts and can do what its user can.
  Spending limits are set on the key, not in zipp.
- GraphiQL for admins (with their session) and in development.
- Big JavaScript in the studio is fine, only editors load it, once.

## Before a commit

- `cargo fmt`, `cargo clippy --workspace --all-targets --features
  zipp/testing,zipp/craft`, `cargo test --workspace --features
  zipp/craft` (the import is a feature, see docs/operations.md), and in
  `studio/` `npx prettier --write src` and `npx svelte-check`.
- A change of the studio: `npm run build` in `studio/` and commit the
  embedded build in `crates/zipp/studio`.
- A change of the example's schema: regenerate `0001_initial`, then
  `cargo run -- check --queries ../crelte/svelte/src/queries` in
  `example/cms`, it fails on what `serve` would refuse.
- New features get a line in `docs/` and, if the example can show them,
  in the example and `example/readme.md`.

## Releasing

- `RELEASING.md`: one version for all crates in `[workspace.package]`,
  a tag publishes them through GitHub Actions.
