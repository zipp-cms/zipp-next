# zipp-button-group

A field type for [zipp](https://crates.io/crates/zipp-cms): a row of
buttons in colors where the editor picks one, like Craft's button group.
GraphQL gives value, label and color. It is also the example of a field
type in another crate.

It is not on crates.io: it becomes a field kind of zipp itself.

```rust,ignore
use zipp_button_group::{ButtonGroup, button_group};

Schema::new()
	.field_type(ButtonGroup)
	.entry("event", "Event", [
		button_group("genre", [
			("concert", "Concert", "#2b6cb0"),
			("reading", "Reading", "#2f855a"),
		]),
	])
```

See [Extending](https://github.com/zipp-cms/zipp-next/blob/code-first/docs/extending.md) in the documentation.

## License

MIT or Apache-2.0, at your option.
