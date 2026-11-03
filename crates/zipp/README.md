# zipp-cms

A headless cms where the content model is code. A site has a small Rust
crate that describes its entries and blocks; zipp turns that into an api
(REST and GraphQL), a studio for the editors on `/admin`, migrations and
TypeScript types. The frontend (crelte or anything else) is a separate
project that reads the api.

The package is `zipp-cms`, the crate is `zipp`:

```toml
[dependencies]
zipp = { package = "zipp-cms", version = "0.1" }
```

```rust,ignore
use zipp::{Field, Schema, Section};

fn schema() -> Schema {
	Schema::new()
		.site("en", "English", "en")
		.section(Section::channel("news", ["article"]).uri("news/{slug}"))
		.entry("article", "Article", [
			Field::rich_text("intro").links(),
			Field::blocks("content", ["text"]),
		])
		.block("text", "Text", [Field::rich_text("text").headings([2, 3])])
}

fn main() {
	zipp::run(schema(), zipp::migrations!());
}
```

Then `cargo run -- makemigration --name initial`, `cargo run -- user
admin@example.com` and `cargo run -- serve --dev`.

Features: `testing` (`zipp::testing`, a test server for the tests of a
site and its plugins), `craft` (`import-craft`, the import of a Craft
site).

Plugins: [zipp-seo](https://crates.io/crates/zipp-seo),
[zipp-forms](https://crates.io/crates/zipp-forms).

The [documentation](https://github.com/zipp-cms/zipp-next/blob/main/docs/README.md) explains the content model, editing,
GraphQL, SEO, extending and running a site.

## License

MIT or Apache-2.0, at your option.
