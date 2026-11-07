# Getting started

A site's cms is a small Rust crate (a binary) next to its frontend.

```sh
cargo new cms
```

Give it zipp's formatting, so `cargo fmt` keeps the code like zipp's and
its examples (hard tabs, 80 columns), in `cms/rustfmt.toml`:

```toml
hard_tabs = true
max_width = 80
```

zipp is on crates.io as `zipp-cms`, the crate is called `zipp`. Keep the
dependency's name `zipp`: `zipp::migrations!()` refers to it.

```toml
[dependencies]
zipp = { package = "zipp-cms", version = "0.2" }
# plugins, if the site wants them
zipp-seo = "0.2"
zipp-forms = "0.2"
zipp-members = "0.2"

[dev-dependencies]
# zipp::testing for the site's own tests
zipp = { package = "zipp-cms", version = "0.2", features = ["testing"] }
```

The features: `testing` (`zipp::testing::TestServer`, the test server
for the tests of a site and its plugins) and `craft` (the command
`import-craft`, for the crate of a Craft port). The plugin
`zipp-agent` (the coding agent in the AI chat, development only) is
only in the repository, see extending.md.

`src/main.rs` builds the schema and runs zipp:

```rust
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

Then the first migration, an admin and the server:

```sh
cargo run -- makemigration --name initial    # writes migrations/0001_initial.json
cargo run -- user admin@example.com          # asks for a password
cargo run -- serve --dev                     # studio on http://127.0.0.1:8090/admin
```

The content model is in content-model.md, the commands and deploying in
operations.md; `example/cms` is a site that uses everything.
