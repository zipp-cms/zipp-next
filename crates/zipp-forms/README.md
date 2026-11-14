# zipp-forms

Forms of the website for [zipp](https://crates.io/crates/zipp-cms), like
Craft's Formie: editors build them in the studio, the website shows
them and sends what visitors fill in, the studio lists the submissions.

```toml
[dependencies]
zipp = { package = "zipp-cms", version = "0.3" }
zipp-forms = "0.3"
```

```rust,ignore
use zipp_forms::{Forms, form_field};

Schema::new()
	// ...
	.plugin(Forms::new())
	.block("contact", "Form", [form_field("form").required()])
```

The crelte frontend is the npm package `zipp-forms` (`crelte/`). See
[Forms](https://github.com/zipp-cms/zipp-next/blob/main/docs/forms.md)
in the documentation.

## License

MIT or Apache-2.0, at your option.
