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
it checks its options and values, says how GraphQL shows them (a scalar,
an object, or a type of its plugin it resolves itself with the request),
what entries and assets a value relates to, and brings the editor of the
studio as an ES module with `mount(target, { field, value, onchange,
entryId, site, host })`, built with its own framework and compiled into
the crate. The site registers it with `Schema::field_type(...)`, the
studio loads the module from `/_fields/<type>.js` when a field needs it.
`crates/zipp-button-group` is such a crate: Craft's button group, options
in colors.

`host` is what the studio lends a module: `host.request(method, path,
body)` calls the api as the logged in user, `host.mountField(target, {
field, value, onchange })` mounts one of the studio's own fields (images
and files, texts, dropdowns, checkboxes, tables, markdown), like the image
picker.

## Plugins

A plugin is a crate that adds a whole feature, turned on with one line:
`.plugin(zipp_seo::Seo::new())`. It implements `Plugin`:

- `schema`: adds sections, entry types, blocks, field types and
  transforms when the site adds it; they are content like the site's own,
  in migrations and in Setup. `finish` runs once the schema is complete
  (`Cms` calls `Schema::finish`), `check` finds mistakes in what the site
  set up for it.
- `graphql`: its types, fields of every entry type (an entry type with a
  field of the same handle keeps its own) and of the root. Resolvers use
  `zipp::graphql::ext` for the request's data and entries loaded with the
  rights of who asks, so previews show drafts.
- `api_routes` under `/api` for the studio, made like zipp's own (see
  [The rest api](#the-rest-api)), `routes` for the website, like sitemaps
  the frontend proxies.
- `studio`: an ES module served as `/_plugins/<name>.js` with data for it;
  `sectionSetup(target, { schema, section, data, host })` adds a card to
  the setup page of a section.

`Schema::plugin_of::<T>()` finds a plugin's settings. `crates/zipp-seo`
is the first plugin: SEO like the Craft plugin meierlabs-seo, see
[SEO](seo.md).

`crates/zipp-agent` is the second: in development (`serve --dev`) the
studio's AI chat hands everything to a coding agent (Claude Code) for
admins, content as drafts through the api with the user's session, the
content model and the frontend in the code; it commits its changes. The
environment says where: `ZIPP_AGENT_COMMAND` (`claude`), `ZIPP_AGENT_DIR`
(the project, `..`), `ZIPP_AGENT_RESTART` (rebuilds and restarts the
cms; its unit needs `KillMode=process`, the agent outlives the restart),
`ZIPP_AGENT_API`. Meant for a sandbox VM: the agent runs without asking.
The agent is a `Harness`: it builds the command for a message and turns
the lines of its output into the chat's events (text, step, error), so
`Agent::with(...)` can run another coding agent than Claude Code
(`Agent::new()`).

## The rest api

Every request of the studio's api is a type that knows its method, path
and response, so a path is written once:

```rust
#[derive(Serialize, Deserialize)]
pub struct DefaultsReq {
	pub id: i64,
	pub site: Option<String>,
}

request!(DefaultsReq, GET "/entries/{id}/seo" => Value);

async fn defaults(
	State(state): State<AppState>,
	_access: EntryAccess<View>,
	Req(req): Req<DefaultsReq>,
) -> ApiResult<DefaultsReq> { ... }

Router::new().api::<DefaultsReq, _>(defaults)
```

Path params like `{id}` are fields of the request, the other fields are
the query of a `GET` or `DELETE` and the JSON body of the others. Who may
call a handler is its first extractor: `Authed` any user, `Admin`,
`EntryAccess<View | Edit | Publish | Delete>` a level in the section of
the entry `{id}`, `AssetAccess<...>` a level for assets, `AiAccess` the
AI chat. Errors are `zipp::error::Error`, sent as `{ message, error: {
type, detail } }` with its status code. Uploads, files, GraphQL and
streams stay plain axum routes.

## Tests

With the feature `testing`, `zipp::testing::TestServer` runs the whole
cms on an in-memory database; requests go through the router without a
port, as an admin, a role or anonymous:

```rust
let ts = TestServer::new(schema());
let author = ts.user(Some("author"));
let jazz = author.call(&CreateReq(event)).await.unwrap();
let err = author.call(&PublishReq { id: jazz.id }).await.unwrap_err();
assert_eq!(err, Error::Forbidden);
```

`ts.routes(...)` adds the site's routes, `ts.dev()` development mode,
`client.send(...)` any request and `client.upload(...)` files. The
example (`example/cms/src/tests.rs`) and the plugins test themselves this
way.

## Types for the rest api

`types` prints TypeScript types of all
entries and blocks.
