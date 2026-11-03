# zipp-forms

Forms of the website for [zipp](https://crates.io/crates/zipp-cms), like
Craft's Contact Form: the fields are code, a submission is checked and
mailed, nothing is stored.

```toml
[dependencies]
zipp = { package = "zipp-cms", version = "0.1" }
zipp-forms = "0.1"
```

```rust,ignore
use zipp_forms::{Form, FormField, Forms};

Schema::new()
	// ...
	.plugin(Forms::new().form(
		Form::new("contact", "Contact")
			.field(FormField::text("name").required())
			.field(FormField::email("email").required().reply_to())
			.field(FormField::multiline("message").required()),
	))
```

The website posts JSON to `POST /forms/<handle>?site=<site>`. See
[Forms](https://github.com/zipp-cms/zipp-next/blob/main/docs/forms.md) in the documentation.

## License

MIT or Apache-2.0, at your option.
