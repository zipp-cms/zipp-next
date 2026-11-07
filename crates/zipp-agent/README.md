# zipp-agent

A [zipp](https://crates.io/crates/zipp-cms) plugin for development: the
AI chat of the studio hands everything to a coding agent (Claude Code):
content as drafts through the api, the content model in the cms crate,
the frontend. It only works with `serve --dev` and for admins.

The agent runs with all permissions (the shell, the project, an admin's
session of the cms), and content that other users wrote can steer it
(prompt injection). Don't run it on an instance shared with editors you
don't trust.

It is not on crates.io yet, only in the repository.

```rust,ignore
Schema::new()
	// ...
	.plugin(zipp_agent::Agent::new())
```

Where it runs is set by the environment: `ZIPP_AGENT_COMMAND` (default
`claude`), `ZIPP_AGENT_DIR` (the project, default `..`),
`ZIPP_AGENT_RESTART` and `ZIPP_AGENT_API`. See the crate docs and
[Extending](https://github.com/zipp-cms/zipp-next/blob/main/docs/extending.md).

## License

MIT or Apache-2.0, at your option.
