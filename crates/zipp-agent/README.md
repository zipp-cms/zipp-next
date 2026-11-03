# zipp-agent

A [zipp](https://crates.io/crates/zipp-cms) plugin for development: the
AI chat of the studio hands everything to a coding agent (Claude Code):
content as drafts through the api, the content model in the cms crate,
the frontend. It only works with `serve --dev` and for admins.

It is not on crates.io yet, only in the repository.

```rust,ignore
Schema::new()
	// ...
	.plugin(zipp_agent::Agent::new())
```

Where it runs is set by the environment: `ZIPP_AGENT_COMMAND` (default
`claude`), `ZIPP_AGENT_DIR` (the project, default `..`),
`ZIPP_AGENT_RESTART` and `ZIPP_AGENT_API`. See the crate docs and
[Extending](https://github.com/zipp-cms/zipp-next/blob/code-first/docs/extending.md).

## License

MIT or Apache-2.0, at your option.
