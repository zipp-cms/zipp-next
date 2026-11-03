<img src="studio/src/logo/full.svg" alt="zipp" width="200">

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
		.section(
			Section::structure("pages", ["page"]).uri("{parent.uri}/{slug}"),
		)
		.section(Section::channel("events", ["event"]).uri("events/{slug}"))
		// tabs like Craft's field layout: the intro, then the blocks
		.entry_tabs("page", "Page", [
			("Intro", vec![Field::rich_text("intro").links().translatable()]),
			("Content", vec![Field::blocks("content", ["text", "images"])]),
		])
		.entry("event", "Event", [
			Field::text("date").required().indexed(),
			Field::text("city").required().indexed(),
		])
		.block("text", "Text", [
			Field::rich_text("text").headings([2, 3]).lists().links(),
		])
		.block("images", "Images", [Field::images("images").required()])
}

fn main() {
	zipp::run(schema(), zipp::migrations!());
}
```

A site depends on the package `zipp-cms`, the crate is called `zipp`:

```toml
[dependencies]
zipp = { package = "zipp-cms", version = "0.1" }
```

The [documentation](docs/README.md) explains the content model, editing,
GraphQL, SEO, extending and running a site.

## Layout

- `crates/zipp`: the cms, a library the site crate calls (crates.io:
  `zipp-cms`)
- `studio`: the editing ui (Svelte), its build is embedded in the zipp
  crate and served on `/admin`; `studio/ui` is `zipp-ui` on npm, the
  components for the studio modules of plugins
- `crates/zipp-seo`, `crates/zipp-forms`: plugins (SEO, forms of the
  website), on crates.io under their names
- `crates/zipp-agent`: the coding agent in the AI chat, a plugin for
  development, not published
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

## License

MIT or Apache-2.0, at your option: [LICENSE-MIT](LICENSE-MIT),
[LICENSE-APACHE](LICENSE-APACHE). What changed in each version is in
[CHANGELOG.md](CHANGELOG.md), how to release in
[RELEASING.md](RELEASING.md).
