# Zipp

A headless cms where the content model is code.

Writing code got cheap, so the schema doesn't need a builder in the
admin anymore: a site has a small Rust crate that describes its entries
and blocks, zipp turns that into an api, a studio for the editors,
migrations and TypeScript types. The frontend (crelte or anything else)
is a separate project that reads the api.

```rust
use zipp::{Field, Schema, Section};

fn schema() -> Schema {
	Schema::new()
		.site("de", "Deutsch", "de-CH")
		.site("en", "English", "en")
		.section(Section::single("home", "page").uri(""))
		.section(Section::structure("pages", ["page"]).uri("{parent.uri}/{slug}"))
		.section(Section::channel("events", ["event"]).uri("events/{slug}"))
		.entry("page", "Page", [
			Field::text("intro").multiline().translatable(),
			Field::blocks("body", ["heading", "button"]).translatable(),
		])
		.entry("event", "Event", [
			Field::text("date").required().indexed(),
			Field::text("city").required().indexed(),
		])
		.block("heading", "Heading", [Field::text("text").required()])
		.block("button", "Button", [
			Field::text("label").required(),
			Field::relation("link", ["page", "event"]).max(1),
		])
}

fn main() {
	zipp::run(schema(), zipp::migrations!());
}
```

A site can extend its cms in Rust, the project owns the code:

```rust
Cms::new(schema(), zipp::migrations!())
	// routes of the site next to zipp's, with State<AppState>
	.routes(Router::new().route("/sitemap.xml", get(sitemap)))
	// after every save, publish, move and delete, e.g. to clear a cache
	.on_change(|c| println!("{:?} {}", c.kind, c.entry_id))
	.run();
```

## Layout

- `crates/zipp`: the cms, a library the site crate calls
- `studio`: the editing ui (Svelte), its build is embedded in the zipp
  crate and served on `/`
- `example/cms`: the cms crate of an example site, with its migrations;
  `example/readme.md` is its first page in the studio and says where to
  try which feature
- `example/frontend`: a small Svelte frontend for it
- `example/crelte`: the same site as a crelte project, with crelte's
  default template and no Craft

## Concepts

- **Sections, entry types and sites, like in Craft.** A single has
  exactly one entry (created on start), a channel a list and a structure
  a tree. A section allows one or more entry types and has a uri format
  (`events/{slug}`, `{parent.uri}/{slug}`). Every entry has a row per
  site with its own title, slug, uri, published flag and translatable
  fields; the other fields are shared by all sites.
- **Entry types and blocks.** Both are components made of fields: text,
  number, boolean, relation (ids of entries) and blocks (a list of
  blocks, nested as deep as needed).
- **Content is JSON.** An entry's fields are stored as one JSON document,
  a block is `{ "id", "type", "fields" }`.
- **Queries go through index tables.** Fields marked `indexed()` are
  copied to `entry_values` on every save, every relation (also inside
  blocks) to `entry_relations`. So entries can be filtered and sorted by
  them and you can ask what links to an entry:
  `/api/entries?type=event&f.city=Bern&f.date.gte=2026-11-01&order=date`,
  `/api/entries?related=5`, `/api/entries?site=en&uri=about/team`,
  `/api/entries?section=pages&order=structure`. See
  `crates/zipp/src/query.rs`.
- **Migrations from the code.** `makemigration` compares the schema in
  code with the last file in `migrations/` and writes the difference as
  ops. Renames are hints (`--rename event.city=town`), otherwise a rename
  is a remove and an add, and the tool says which content is lost. `serve`
  applies new migrations to all content and refuses to start when the
  code changed without one. Labels, names, `required` and the field order
  change freely.
- **Drafts, revisions and previews.** The studio saves every change into
  a draft of the entry (one per entry, all sites), the website keeps the
  live content until it is published. Publishing checks every site and
  writes them in one transaction and keeps a revision; an old revision
  can be restored into the draft. A preview token shows the draft on the
  website: crelte passes `?token=` on to GraphQL.
- **GraphQL for the frontend.** `/api/graphql` is generated from the
  schema: every entry type is `<handle>_Entry`, every block
  `<handle>_Block`, like Craft. Relations resolve to entries with their
  fields, so one query loads a page with everything it shows:
  `entry(uri: "about/team", site: "en") { title ... on page_Entry { body
  { ... on button_Block { label link { title uri } } } } }`. `entries`
  and `entryCount` take the same filters as the rest api (`where: [{
  field: "city", value: "Bern" }]`). Without a session only published
  entries show.
- **Roles.** Like Craft Pro's user groups, roles are code:
  `.role(Role::new("author", "Author").section("events", Access::Edit)
  .section("pages", Access::View).assets(Access::Edit))`, with View <
  Edit (drafts, new entries unpublished) < Publish (also moving in a
  structure) < Delete per section and for assets, and `.ai()` for the
  chat. Which user has which role is in the database (`user <email>
  --role author`, or the Users page of an admin); a user without a role
  is an admin. The server checks every request, the AI can only do what
  its user can, the studio hides the rest. Only admins see drafts in
  GraphQL with their session.
- **Private GraphQL, dev and prod.** Like Craft's schema tokens, GraphQL
  only answers frontends that send `Authorization: Bearer <token>`, the
  token of `serve` (`ZIPP_GRAPHQL_TOKEN`; crelte reads it as
  `ENDPOINT_TOKEN` from `../craft/.env`), and the studio with its
  session. `serve --dev` (`ZIPP_DEV=1`) turns on GraphiQL with a GET on
  `/api/graphql` and opens GraphQL when there is no token. The rest api
  is the studio's, it needs a session.
- **Groups and markdown.** `Field::group("seo", "seo")` keeps the fields
  of another component together under one name (an object, not a list
  like blocks). `Field::markdown("text")` is shown rendered in the studio
  and comes as `markdown` and `html` from GraphQL.
- **Dropdowns.** `Field::dropdown("width", [("small", "Small"), ...])`
  stores the value of one option, GraphQL gives that value like Craft.
  The studio shows up to three options as buttons, more as a select.
  Dropdowns can be indexed.
- **Field types of other crates.** A crate implements `FieldType`:
  it checks its options and values, says how GraphQL shows them and
  brings the editor of the studio as an ES module with
  `mount(target, { field, value, onchange })`, built with its own
  framework and compiled into the crate. The site registers it with
  `Schema::field_type(...)`, the studio loads the module from
  `/_fields/<type>.js` when a field needs it. `example/button-group` is
  such a crate: Craft's button group, options in colors.
- **Docs.** `.doc("readme", "Readme", include_str!("../readme.md"))`
  shows a markdown file of the repository as a read-only page of the
  studio; put its handle into a nav group like a section.
- **Rich text.** `Field::rich_text("text").headings([3]).lists().links()
  .buttons().embed(["image"])` says what an editor may contain: bold and
  italic always, the rest only if the field allows it, blocks of the
  schema as boxes in the text. It is edited with TipTap and stored as its
  JSON; publishing removes empty paragraphs, `&nbsp;` and trailing line
  breaks, so frontends need no margin hacks. GraphQL has `html` and
  `chunks` (`Html`, `Buttons` with a row of links, the blocks), so a row
  of buttons arrives grouped. Links in the text point to entries by id.
- **Translations.** The texts of the frontend are keys in the code with
  the text of the primary site: `.translations("common", [("menu.open",
  "Menü öffnen")])`. They are a single `translations` with a texts field
  per namespace, so editors translate them per site with drafts,
  revisions and the preview like any entry (put `translations` into a
  nav group); an empty text is the code's.
  crelte's translations plugin gets them with `translations:
  translations(siteId: $siteId) { common }` in `global.graphql`.
- **Assets and transforms.** Uploads live in `--files`, `Field::images`
  and `Field::assets` link them. Transforms are named in code
  (`Transform::new("hero").width(1600).height(700).crop()`), generated
  on their first request and cached; crops keep the focal point editors
  set in the studio. GraphQL asks like Craft: `image { url(transform:
  "hero") width(transform: "hero") srcset(transforms: ["small", "hero"])
  alt }`. Only named transforms exist, so nobody can make the server
  render arbitrary sizes. `--public-url` makes the urls absolute for
  frontends on other domains.
- **crelte.** zipp answers crelte like Craft: `crelteSites`, site ids
  (the position of a site, from 1), `entry(uri: $uri, siteId: $siteId)`
  with Craft's argument types, `__home__`, `url` on entries and
  templates by `<sectionHandle>-<typeHandle>`. The base url of each site
  is `--site-url de=https://example.ch`. crelte reads its settings from
  `../craft/.env`, see `example/crelte/craft/.env.example`.
- **AI chat.** `--openrouter-key-file <file>` adds *AI* to the studio:
  a chat on the left, the website on the right. The model (Claude
  Sonnet, GPT Sol or Gemini over OpenRouter, through soe-llms) sees and
  changes what an editor can: entries of every section, new entries,
  images, the texts of the code. Entries change as drafts, the website on
  the right shows every draft with a token of the whole site
  (`POST /api/preview`), an editor publishes. The model knows the page on
  the right and its content; it stays there when the change shows on it
  and opens another page when not. A site tells the studio its page with
  a small bridge (`example/crelte/svelte/src/components/PreviewBridge.svelte`),
  crelte's live preview mode keeps the scroll position on reloads.
- **Types for the rest api.** `types` prints TypeScript types of all
  entries and blocks.

## Run the example

```sh
cd example/cms
cargo run -- user admin@example.com          # asks for a password
cargo run -- serve                            # studio on http://127.0.0.1:8090
```

A release build is a single binary with the api, the studio and the
migrations; it only needs a place for its database (`--db`). Debug builds
read the migrations from disk, so a new one works without a rebuild.

To work on the studio run `npm run dev` in `studio/` (it proxies `/api`
to the cms), `npm run build` updates the embedded build in
`crates/zipp/studio`, commit it with the change. The frontend runs
with `npm run dev` in `example/frontend`, it loads everything over
GraphQL.

After changing `example/cms/src/main.rs`:

```sh
cargo run -- makemigration --name what-changed
```
