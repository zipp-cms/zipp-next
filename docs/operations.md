# Running and deploying

How a site's cms is run, migrated and checked.

## Commands

The cms crate of a site is a binary with these commands (`--db` is the
SQLite database, default `zipp.db`):

- `serve`: applies new migrations and serves the api, the studio on
  `/admin` and the assets
- `makemigration [--name n] [--rename old=new]`: writes a migration from
  the changed schema
- `check [--queries dir]`: what CI runs before a deploy, see below
- `user <email> [--role r]`: adds a user or sets a new password and role,
  reads the password from stdin; without a role an admin
- `types`: prints TypeScript types of the content
- `import-craft <export.json> [--volume handle=dir] [--site craft=zipp]`:
  imports a Craft site into a database without content, see below

## Sessions

A login lasts 30 days; the studio renews it when it opens and less than
10 days are left, so who uses it stays logged in. Logging out ends the
session, a new password (`user`) ends all of the user's. After 10 wrong
passwords an email waits 15 minutes.

## serve

| Option | Environment | |
|---|---|---|
| `--listen` | | address, default `127.0.0.1:8090` |
| `--files` | | where uploads go, default `files` |
| `--public-url` | | put before asset urls when the frontend runs elsewhere |
| `--site-url de=https://…` | | the url of each site |
| `--graphql-token` | `ZIPP_GRAPHQL_TOKEN` | the token frontends send, GraphQL is closed without it |
| `--dev` | `ZIPP_DEV` | development: GraphiQL, GraphQL open without a token, noindex |
| `--noindex` | `ZIPP_NOINDEX` | a staging site: noindex everywhere, robots.txt disallows all |
| `--openrouter-key-file` | | turns on the AI chat |
| `--frame-ancestor` | | a site that may show the studio in a frame |

A release build is a single binary with the api, the studio and the
migrations; debug builds read the migrations from disk.

## Migrations from the code

`makemigration` compares the schema in
code with the last file in `migrations/` and writes the difference as
ops. Renames are hints (`--rename event.city=town`), otherwise a rename
is a remove and an add, and the tool says which content is lost. `serve`
applies new migrations to all content and refuses to start when the
code changed without one. Labels, names, `required` and the field order
change freely.

## Check before a deploy

Every command checks the schema first (the
handles it names: types of sections, relations, groups, blocks, nav,
roles). `serve` applies new migrations and refuses to start when the
code changed since the last one. `check --queries <dir>` does both
without a server and also checks the `.graphql` files of frontends
against the schema (unknown fields, wrong arguments), for CI:
`cargo run -- check --queries ../crelte/svelte/src/queries`.

## Working on zipp

- The studio: `npm run dev` in `studio/` (it proxies `/api` to the cms),
  `npm run build` updates the build embedded in `crates/zipp/studio`;
  commit it with the change.
- The website of the example: `npm run dev` in `example/crelte/svelte`,
  it reads `example/crelte/craft/.env` (see `.env.example`).
- While zipp is not live, the example has one migration,
  `0001_initial`: after changing the schema delete it and run
  `makemigration --name initial` again. Real migrations start with the
  first live site.

## Import from Craft

`crates/zipp/craft/export.php` writes the content of a Craft 5 site as
JSON (`ddev craft exec "if (true) { $(sed 1d export.php) }"` in the
Craft project, it writes `storage/zipp-export.json`), `import-craft`
puts it into a new database: `cargo run -- import-craft export.json
--volume default=../craft/web/assets/default`. Everything matches by
handle, so the zipp schema uses Craft's handles: sections, global sets
(singles in zipp) and category groups go to the section of the same
handle, entry types and nested entry types to the entry type or block,
fields to the field. Values follow the zipp field: a matrix becomes
blocks or a navigation (each nested entry's link and the nested entries
under it), a content block a group, categories a relation, CKEditor's
html rich text (nested entries become embedded blocks, or buttons if
they are only a link). Assets come with their title, alt text and focal
point; entries keep Craft's post date, structures their order. What
the schema doesn't have is left out and listed; slugs are made zipp's
(`teddy--co.` becomes `teddy-co`), so redirect such urls in the
frontend. SEO fields aren't imported yet.
