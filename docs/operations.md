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
