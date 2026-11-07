# zipp-members

Members of the website edit their own entries in the frontend, for
[zipp](https://crates.io/crates/zipp-cms): like Craft users without
access to the control panel. The routes go through the same checks as
the studio (the role's access, validation, drafts and revisions).

```toml
[dependencies]
zipp = { package = "zipp-cms", version = "0.1" }
zipp-members = "0.1"
```

```rust,ignore
use zipp::{Access, Role};
use zipp_members::Members;

Schema::new()
	// ...
	.role(
		Role::new("member", "Member")
			.own("recipes", Access::Delete)
			.assets(Access::Edit)
			.no_studio(),
	)
	.plugin(
		Members::new()
			.section("recipes")
			.images(10 * 1024 * 1024, "Members/{user}"),
	)
```

The frontend forwards `/members/...` with the visitor's site token. See
[Website users](https://github.com/zipp-cms/zipp-next/blob/main/docs/members.md) in the documentation.

## License

MIT or Apache-2.0, at your option.
