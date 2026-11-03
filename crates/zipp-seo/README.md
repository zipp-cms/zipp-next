# zipp-seo

SEO for [zipp](https://crates.io/crates/zipp-cms) like the Craft plugin
meierlabs-seo: global settings, defaults per section and entry type,
`seoJson` for the npm package `meierlabs-seo` and the sitemaps.

```toml
[dependencies]
zipp = { package = "zipp-cms", version = "0.1" }
zipp-seo = "0.1"
```

```rust,ignore
use zipp_seo::{Seo, SeoDefaults, seo_field};

Schema::new()
	// ...
	.entry("page", "Page", [Field::text("intro"), seo_field("seo")])
	.plugin(
		Seo::new().entry_type("page", SeoDefaults::new().description_from("intro")),
	)
```

See [SEO](https://github.com/zipp-cms/zipp-next/blob/main/docs/seo.md) in the documentation.

## License

MIT or Apache-2.0, at your option.
