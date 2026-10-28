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

The [documentation](docs/README.md) explains the content model, editing,
GraphQL, SEO, extending and running a site.

## Layout

- `crates/zipp`: the cms, a library the site crate calls
- `studio`: the editing ui (Svelte), its build is embedded in the zipp
  crate and served on `/admin`
- `crates/zipp-button-group`: a field type of another crate, as an example
  of a plugin
- `example/cms`: the cms crate of an example site, with its migrations;
  `example/readme.md` is its first page in the studio and says where to
  try which feature
- `example/crelte`: the same site as a crelte project, with crelte's
  default template and no Craft

## Run the example

```sh
cd example/cms
cargo run -- user admin@example.com          # asks for a password
cargo run -- serve --dev                      # studio on http://127.0.0.1:8090/admin
```

Then open the studio, its first page says where to try which feature.
See [Running and deploying](docs/operations.md) for the rest.
