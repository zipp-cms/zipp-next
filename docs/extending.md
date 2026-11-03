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

`state.mail` sends mails, e.g. of a contact form of the site:
`state.mail.send(Mail::new(to, "Thanks", text).html(html)
.reply_to(sender)).await?` (`zipp::mail::Mail`); how they are sent is
`serve`'s, see operations.md.

## Field types of other crates

A crate implements `FieldType`:
it checks its options and values, says how GraphQL shows them (a scalar,
an object, or a type of its plugin it resolves itself with the request),
what entries and assets a value relates to, and brings the editor of the
studio as an ES module with `mount(target, props)`, built with its own
framework and compiled into the crate. The props are `FieldProps` of
`zipp-ui/field`: `field, value, onchange, entryId, site, readonly,
error, host`, again with `update` when they change; a Svelte editor is
`export const mount = fieldModule(MyField)`. The site registers it with `Schema::field_type(...)`, the
studio loads the module from `/_fields/<type>.js` when a field needs it.
`crates/zipp-button-group` is such a crate: Craft's button group, options
in colors.

## The studio's components

The editors of field types and the modules of plugins look like the
studio with `zipp-ui` (`studio/ui`), the components the studio itself is
built of: `Table`, `Segmented`, `Popover`, `Flyout`, `ItemCard`,
`ItemActions`, `AddGroup`, `SearchInput`, `DatePicker` (a day, with
`time` its time, like the date field), `FieldGroup` and `FieldRow` (a
card of values with their labels on the left, like a group field; the
SEO field uses them), `Icon` and more. A module
bundles them like its Svelte; they look right because they use the
studio's CSS variables and some of its global classes, which are on the
page (listed in `studio/ui/README.md`, they are part of the contract).
A module imports only what `exports` of its `package.json` lists:

```js
import Table from 'zipp-ui/Table.svelte';
```

zipp-ui is on npm with the version of zipp, a module bundles it, so it
is a devDependency. The crates of this repository link the copy in the
repository instead (`"zipp-ui": "file:../../../studio/ui"`), so a change
of a component reaches them with their next build. The module's
`vite.config.js` is `pluginConfig` of `zipp-ui/vite` (one ES module in
`../assets`, Svelte with `vitePreprocess`); the components are scss, so
it needs `sass` too, like `crates/zipp-seo/studio`. Its own styles get
the breakpoints and mixins with `@use 'zipp-ui/vars.scss' as *;`.

The components write their few own texts (Today, Close, Nothing found)
and dates in the user's language: the studio sets them in
`zipp-ui/texts`, which a plugin's copy of zipp-ui shares. A plugin's
own texts are its own; `<html lang>` says the studio's language (`en`
or `de`) to pick them by. The SEO plugin has a small map of English and
German in `studio/src/texts.js`.

`host` is what the studio lends a module: `host.request(method, path,
body)` calls the api as the logged in user, `host.mountField(target, {
field, value, onchange })` mounts one of the studio's own fields (images
and files, texts, dropdowns, checkboxes, tables, markdown, dates), like
the image picker.

## A studio module from scratch

The editor of a field type in a crate of its own, in Svelte. Next to the
crate's `Cargo.toml`, a folder `studio` with this `package.json`:

```json
{
	"name": "my-field-studio",
	"private": true,
	"type": "module",
	"scripts": { "build": "vite build" },
	"devDependencies": {
		"@sveltejs/vite-plugin-svelte": "^7.0.0",
		"sass": "^1.63.4",
		"svelte": "^5.0.0",
		"vite": "^8.0.0",
		"zipp-ui": "^0.1.0"
	}
}
```

zipp-ui is Svelte and TypeScript source, not built: it needs vite,
vite-plugin-svelte and sass (its peer dependencies; vite 6 to 8). Its
`vite.config.js`:

```js
import { svelte, vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { pluginConfig } from 'zipp-ui/vite';

export default pluginConfig({ svelte, vitePreprocess, file: 'my-field.js' });
```

`src/index.svelte.js` is the module, `src/MyField.svelte` the editor:

```js
import { fieldModule } from 'zipp-ui/field';
import MyField from './MyField.svelte';

export const mount = fieldModule(MyField);
```

`npm run build` writes `../assets/my-field.js`, one ES module with its
Svelte and css. Commit it: the crate embeds it, so the crate builds
without node. Its `FieldType` returns it:

```rust
fn studio(&self) -> StudioModule {
	StudioModule::new(include_str!("../assets/my-field.js"))
}
```

The studio loads it from `/_fields/<type>.js` and calls `mount` with
`FieldProps` (`zipp-ui/field`). What the editor needs to know:

- `onchange(value)` sends a value, `null` for none. The studio sends a
  value back with `update` only when it changed elsewhere (a restore,
  another site); `fieldModule` does that.
- `error` is what the server says is wrong with the value. The studio
  already shows it under the field; the editor needn't, but may mark
  its input (`aria-invalid`).
- Read-only (a revision, a role that may not edit): the field is in a
  disabled `<fieldset>`, so the browser disables its inputs and buttons.
  `readonly` says it for the rest, like a contenteditable or a drag
  handle.
- `host.request(method, path, body)` calls `/api<path>` as the user. A
  failed one throws an `ApiErrorLike` (`zipp-ui/field`): `message` in
  the user's language, `type` (`INVALID`, `FORBIDDEN`, `NOT_FOUND`, …),
  `fields` of `INVALID` and `status`; a plain `Error` when the network
  is down. The editor shows the message where it asked.
- Texts: zipp-ui's components write their own (Today, Close) in the
  user's language. The editor's own texts are its own, picked by
  `document.documentElement.lang` (`en` or `de`).
- The editor has its own Svelte and zipp-ui, not the studio's: Svelte
  state isn't shared between them, only what goes through the props
  and `host`.

A plugin's module (`PluginStudio`) is made the same way and exports
`sectionSetup` too, like `crates/zipp-seo/studio`.

What breaks a module, so it changes only with a new minor version of
zipp and zipp-ui before 1.0 (0.2), and a major one after: `FieldProps`,
`Host` and `ApiErrorLike`, the exports of zipp-ui and their props, and
the CSS variables and classes of the studio that `studio/ui/README.md`
lists. A module built with zipp-ui 0.1 runs in the studio of zipp 0.1.

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
  the frontend proxies; both get the finished schema.
- `studio`: an ES module served as `/_plugins/<name>.js` with data for it;
  `sectionSetup(target, { schema, section, data, host })` adds a card to
  the setup page of a section. `PluginStudio::default().data(..)` gives
  the studio only data, like the agent's `{ "chat": true }` that turns
  on the coding agent in the chat.

What a plugin hands zipp is made with constructors (`GraphqlExt::new()`
with `types`, `entry_fields` and `root_fields`, `EntryField::new`,
`PluginStudio::new(js).data(..)`, `StudioModule::new(js)`,
`FieldRelation::new(..).at(..)`). The enums it matches (`FieldKind`,
`ChangeKind`, `Target`, `Error`, …) and the structs of the content
model and of entries (`Field`, `Section`, `Entry`, `SiteDoc`, …) are
`#[non_exhaustive]`: zipp can add to them without breaking a plugin.
Their fields are read, made with constructors (`Choice::new`,
`Site::new`, `SiteDoc::new`, `EntryInput::new`, the builders of the
schema) and matched with `..`. Names of plugins and field types are
unique, `check` says so.

zipp re-exports the crates of its api, `zipp::axum`,
`zipp::async_graphql` and `zipp::rusqlite` (the `Connection` of
`zipp::db`): a plugin uses these instead of its own dependency, so the
versions always match.

`Schema::plugin_of::<T>()` finds a plugin's settings. What editors set
for a plugin lives in a single: `entries::single_fields(conn, schema,
section, site)` reads its published fields, `Query::published(schema,
&[("type", "event")])` lists what is live. `crates/zipp-seo`
is the first plugin: SEO like the Craft plugin meierlabs-seo, see
[SEO](seo.md).

`crates/zipp-agent` is the second: in development (`serve --dev`) the
studio's AI chat hands everything to a coding agent (Claude Code) for
admins, content as drafts through the api with a session of the user
for the run (it ends with the run, after two hours at most), the
content model and the frontend in the code; it shows the admin what it
changed and commits when they say so. Its prompt says that the site's
content is data, never instructions: editors write it, the agent runs
without asking. The
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
`ts.site_urls(...)` the urls of the sites, `ts.indexing(false)` like
`serve --noindex`,
`ts.login(email, password)` logs in like the studio, `ts.graphql(query,
variables)` asks like a frontend with its token; `client.get(path)`
fetches a page or a file, `client.send(...)` any request and
`client.upload(...)` files; `testing::input(section, type, title,
fields)` is an entry to create; `ts.mails()` are the mails sent, a test
server keeps them. The
example (`example/cms/src/tests.rs`) and the plugins test themselves this
way.

## Types for the rest api

`types` prints TypeScript types of all
entries and blocks.
