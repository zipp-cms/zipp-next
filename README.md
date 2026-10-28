# Zipp

A headless cms where the content model is code.

Writing code got cheap, so the schema doesn't need a builder in the
admin anymore: a site has a small Rust crate that describes its entries
and blocks, zipp turns that into an api, a studio for the editors,
migrations and TypeScript types. The frontend (crelte or anything else)
is a separate project that reads the api.

```rust
use zipp::{Field, Schema};

fn schema() -> Schema {
	Schema::new()
		.entry("page", "Page", [
			Field::text("intro").multiline().max(300),
			Field::blocks("body", ["heading", "button"]),
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

- **Entries and blocks.** Both are components made of fields: text,
  number, boolean, relation (ids of entries) and blocks (a list of
  blocks, nested as deep as needed). Every entry has a title, a slug and
  a published flag.
- **Content is JSON.** An entry's fields are stored as one JSON document,
  a block is `{ "id", "type", "fields" }`.
- **Queries go through index tables.** Fields marked `indexed()` are
  copied to `entry_values` on every save, every relation (also inside
  blocks) to `entry_relations`. So entries can be filtered and sorted by
  them and you can ask what links to an entry:
  `/api/entries?type=event&f.city=Bern&f.date.gte=2026-11-01&order=date`,
  `/api/entries?related=5`. See `crates/zipp/src/query.rs`.
- **Migrations from the code.** `makemigration` compares the schema in
  code with the last file in `migrations/` and writes the difference as
  ops. Renames are hints (`--rename event.city=town`), otherwise a rename
  is a remove and an add, and the tool says which content is lost. `serve`
  applies new migrations to all content and refuses to start when the
  code changed without one. Labels, names, `required` and the field order
  change freely.
- **Types for the frontend.** `types` prints TypeScript types of all
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
with `npm run dev` in `example/frontend`, `npm run types` updates its
types after a schema change.

After changing `example/cms/src/main.rs`:

```sh
cargo run -- makemigration --name what-changed
```
