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

## Layout

- `crates/zipp`: the cms, a library the site crate calls
- `studio`: the editing ui (Svelte), its build is embedded in the zipp
  crate and served on `/`
- `example/cms`: the cms crate of an example site, with its migrations
- `example/frontend`: a small Svelte frontend for it

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
- **GraphQL for the frontend.** `/api/graphql` is generated from the
  schema: every entry type is `<handle>_Entry`, every block
  `<handle>_Block`, like Craft. Relations resolve to entries with their
  fields, so one query loads a page with everything it shows:
  `entry(uri: "about/team", site: "en") { title ... on page_Entry { body
  { ... on button_Block { label link { title uri } } } } }`. `entries`
  and `entryCount` take the same filters as the rest api (`where: [{
  field: "city", value: "Bern" }]`). Without a session only published
  entries show. A GET opens GraphiQL to try queries.
- **Assets and transforms.** Uploads live in `--files`, `Field::images`
  and `Field::assets` link them. Transforms are named in code
  (`Transform::new("hero").width(1600).height(700).crop()`), generated
  on their first request and cached; crops keep the focal point editors
  set in the studio. GraphQL asks like Craft: `image { url(transform:
  "hero") width(transform: "hero") srcset(transforms: ["small", "hero"])
  alt }`. Only named transforms exist, so nobody can make the server
  render arbitrary sizes. `--public-url` makes the urls absolute for
  frontends on other domains.
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
