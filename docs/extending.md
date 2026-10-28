# Extending

A site owns its cms crate and can add to it in Rust.

## Routes and hooks

```rust
Cms::new(schema(), zipp::migrations!())
	// routes of the site next to zipp's, with State<AppState>
	.routes(Router::new().route("/events.json", get(events)))
	// after every save, publish, move and delete, e.g. to clear a cache
	.on_change(|c| println!("{:?} {}", c.kind, c.entry_id))
	.run();
```

A hook runs in the request, spawn anything slow.

## Field types of other crates

A crate implements `FieldType`:
it checks its options and values, says how GraphQL shows them and
brings the editor of the studio as an ES module with
`mount(target, { field, value, onchange })`, built with its own
framework and compiled into the crate. The site registers it with
`Schema::field_type(...)`, the studio loads the module from
`/_fields/<type>.js` when a field needs it. `crates/zipp-button-group` is
such a crate: Craft's button group, options in colors.

## Types for the rest api

`types` prints TypeScript types of all
entries and blocks.
