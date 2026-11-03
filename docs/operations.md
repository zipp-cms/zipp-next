# Running and deploying

How a site's cms is run, migrated and checked.

## Commands

The cms crate of a site is a binary with these commands (`--db` is the
SQLite database, default `zipp.db`):

- `serve`: applies new migrations and serves the api, the studio on
  `/admin` and the assets
- `makemigration [--name n] [--rename old=new]`: writes a migration from
  the changed schema; `--data --name n` an empty data migration, see
  below
- `check [--queries dir]`: what CI runs before a deploy, see below
- `user <email> [--role r]`: adds a user or sets a new password and role,
  reads the password from stdin; without a role an admin
- `types`: prints TypeScript types of the content
- `mail <to> [--smtp-url url --mail-from from]`: sends a test mail, see
  Mail below
- `import-craft <export.json> [--volume handle=dir] [--site craft=zipp]`:
  imports a Craft site into a database without content, only with the
  feature `craft`, see below
- `backup <dir> [--files dir] [--keep n]`: a backup of the database and
  the files, also while the server runs, see Backups
- `restore <backup> [--files dir]`: puts a backup back, the server
  stopped

`--db` can also be `ZIPP_DB`, `--files` `ZIPP_FILES`.

## Sessions

A login lasts 30 days; the studio renews it when it opens and less than
10 days are left, so who uses it stays logged in. Logging out ends the
session, a new password (`user`) ends all of the user's. The database
keeps only a hash of each session's token. After 10 wrong passwords of
an email from one address, that address waits 15 minutes for it (others
can't lock a user out); an address waits after 50 wrong passwords of
any emails, an email after 100 from any addresses. The address is the
visitor's, see Visitors' addresses.

When a session ends while the studio is open, the page stays and a
dialog asks for the password again; the draft that couldn't be saved
meanwhile is saved after it. Another user logging in there loads the
studio new, logged in as someone else it only goes back to the own
account or to the login.

## Users

Admins manage the users in the studio (Users): a page per user with
name, email, role (and what it may do, from the code), a new password
and the sessions, which "Log out everywhere" ends. Every user opens
their own account from the user menu: name, email and password, not the
role; a new password or email of one's own needs the current password,
a new password logs out the other browsers, a new email is mailed to
the old address (with `--smtp-url`). zipp sends no mails, an admin tells a new user the
password.

## serve

| Option | Environment | |
|---|---|---|
| `--listen` | `ZIPP_LISTEN` | address, default `127.0.0.1:8090` |
| `--files` | `ZIPP_FILES` | where uploads go, default `files` |
| `--public-url` | `ZIPP_PUBLIC_URL` | put before asset urls when the frontend runs elsewhere |
| `--site-url de=https://…` | `ZIPP_SITE_URL` | the url of each site (environment: separated by commas), another origin than the cms, see below |
| `--graphql-token` | `ZIPP_GRAPHQL_TOKEN` | a token frontends send that reads everything; admins make more in the studio (GraphQL), see docs/graphql.md |
| `--dev` | `ZIPP_DEV` | development: GraphQL open without a token, noindex |
| `--noindex` | `ZIPP_NOINDEX` | a staging site: noindex everywhere, robots.txt disallows all |
| `--openrouter-key-file` | `ZIPP_OPENROUTER_KEY_FILE` | turns on the AI chat |
| `--frame-ancestor` | `ZIPP_FRAME_ANCESTOR` | an origin that may show the studio in a frame, `https://dash.example.com` |
| `--trusted-proxy` | `ZIPP_TRUSTED_PROXY` | a proxy whose `X-Forwarded-For` names the visitor, an address or a network, see below |
| `--backup-dir` | `ZIPP_BACKUP_DIR` | a backup a day into it, see Backups |
| `--backup-keep` | `ZIPP_BACKUP_KEEP` | how many stay, default 14 |
| `--smtp-url` | `ZIPP_SMTP_URL` | the SMTP server of mails, see below; without it mails are only logged |
| `--mail-from` | `ZIPP_MAIL_FROM` | their sender, `Website <cms@example.ch>`, needed with `--smtp-url` |

### Limits

An upload is at most 200 MB (also a zip), written to `--files` while it
comes in (hidden temporary files, backups leave them out). Images are
at most 40 megapixels and 65535 pixels on a side, also after the edits
of the image editor (which refuses more on save). Files and transforms
are cached for five minutes (`Cache-Control: public, max-age=300`), then
caches ask again with `If-Modified-Since`: a deleted asset leaves a CDN
soon. GraphQL's limits are in docs/graphql.md.

### Mail

The cms sends mails over SMTP (lettre): `--smtp-url
smtp://user:pass@mail.example.ch:587?tls=required` for STARTTLS,
`smtps://…:465` for TLS, `smtp://localhost:1025` for a local catcher.
A url with a user and password needs TLS (`smtps://` or
`?tls=required`), `serve` refuses it otherwise.
`<cms> mail you@example.ch` (with the same options or environment)
sends a test mail from the shell; admins see the server in Setup →
Sites, transforms and roles and send themselves one there. In
development Mailpit catches them: `mailpit --smtp 127.0.0.1:1025`, its
inbox on `http://127.0.0.1:8025`.

### Visitors' addresses

The limits of logins and forms count the address of the connection. A
proxy in front of the cms (traefik, the crelte server that forwards a
form) is that connection for everybody, so from a proxy the cms takes
the right-most address in `X-Forwarded-For` that isn't a proxy. Each
proxy adds the address it got the request from at the end, so what a
visitor writes into the header before doesn't count. `--trusted-proxy`
names the proxies (`127.0.0.1`, a network like `172.16.0.0/12` for
docker); without it the header is ignored and the proxy counts as one
address for all visitors, so behind traefik or a crelte server set it,
or all logins and forms share one limit.

### Security headers

The studio's pages and GraphiQL are served with a Content-Security-
Policy: scripts, styles, fonts and requests only from the cms (inline
styles allowed), images also `data:` and `blob:`, frames of the cms
and the origins of `--site-url` (the live preview, the AI page), no
plugins, no `<base>`, framed only by the cms and `--frame-ancestor`
(an origin, `serve` refuses anything else). GraphiQL is bundled with
the studio, nothing loads from other servers. The dev server of the
studio (vite) sends none.

The live preview shows the website in a frame that may run its scripts
with its own origin (`allow-scripts allow-same-origin`); it is only kept
apart from the studio while the sites have other origins than the cms
(`https://example.ch` and `https://cms.example.ch`, not both on one
host). The log of requests (`RUST_LOG=tower_http=debug`) has their urls
with `?token=` of previews: keep it at the default level in production.

### Backups

`<cms> backup /backups` writes `/backups/2026-11-01T08-00-00/` with
`zipp.db` and `files/`. The database is a snapshot (`VACUUM INTO`), so
it works while the server runs; never copy the `.db` of a running
server, its last changes are in the `-wal` file. Files that didn't
change since the last backup are hard links to it: each backup is a
whole copy to restore from, but an asset library takes its space once.
Image transforms are left out, they are made again, and so is a file
deleted while the backup runs (logged). `--keep 14`
removes all but the last 14.

`serve --backup-dir /backups` does it itself: at the start when the
last backup is older than a day, then daily, keeping `--backup-keep`
(14). A failed backup is logged and tried again an hour later; the
server goes on. Copy the backups to another machine too (rsync with
`-H` keeps the links), a backup on the same disk only helps against
mistakes.

To restore: stop the server, `<cms> restore /backups/2026-11-01T08-00-00`
(with the `--db` and `--files` of `serve`), start it. `restore`
replaces the database and the files folder and refuses while a server
has the database open. By hand it is the same: stop, copy `zipp.db` to
`--db` (delete an old `zipp.db-wal` and `-shm`), copy `files/` to
`--files`, start. A backup of an older version of the cms is migrated
by `serve` like any database.

A release build is a single binary with the api, the studio and the
migrations; debug builds read the migrations from disk.

## Hosting

A site's cms runs as a container image: `example/cms/Dockerfile` builds
the release binary (Rust image) and puts it into a slim Debian image as
`cms`, run by the user `zipp` (uid 1000). Everything it keeps is in the
volume `/data`: `zipp.db`, `files/` and a backup a day in `backups/`
(`ZIPP_DB`, `ZIPP_FILES`, `ZIPP_BACKUP_DIR` are set). It listens on
port 8090.

- The example's context is the repository, zipp is a path dependency:
  `docker build -f example/cms/Dockerfile -t example-cms .` (with the
  `.dockerignore` of the repository).
- A site's own cms crate (zipp as a git dependency) copies the
  Dockerfile into its repository, builds from there and sets
  `--build-arg PACKAGE=<its crate>`, the name of the binary.
- The commands run in the container: `docker compose exec cms cms user
  anna@example.ch`, `docker compose exec cms cms backup`; a restore
  with the cms stopped: `docker compose stop cms`, `docker compose run
  --rm cms restore /data/backups/2026-11-01T08-00-00`, `docker compose
  start cms`.

Next to a crelte frontend, behind traefik (on its network `traefik`,
the hosts are the site's):

```yaml
services:
  cms:
    image: registry.example.ch/site/cms:latest
    restart: unless-stopped
    environment:
      ZIPP_PUBLIC_URL: https://cms.example.ch
      ZIPP_SITE_URL: de=https://example.ch,en=https://example.ch/en
      ZIPP_GRAPHQL_TOKEN: ${GRAPHQL_TOKEN}
      ZIPP_SMTP_URL: ${SMTP_URL}
      ZIPP_MAIL_FROM: Website <cms@example.ch>
      # traefik and the frontend name the visitor (logins, forms)
      ZIPP_TRUSTED_PROXY: 172.16.0.0/12
    volumes:
      - ./data:/data
      # backups on another disk than the data
      - /mnt/backups/site:/data/backups
    labels:
      - traefik.enable=true
      - traefik.http.routers.site-cms.rule=Host(`cms.example.ch`)
      - traefik.http.routers.site-cms.entrypoints=websecure
      - traefik.http.routers.site-cms.tls.certresolver=letsencrypt
      - traefik.http.services.site-cms.loadbalancer.server.port=8090
    networks: [traefik]

  frontend:
    image: registry.example.ch/site/frontend:latest
    restart: unless-stopped
    volumes:
      # crelte reads ENDPOINT_URL (https://cms.example.ch/api/graphql),
      # ENDPOINT_TOKEN (the GRAPHQL_TOKEN), FRONTEND_URL and
      # CRAFT_BASE_CP_URL (https://cms.example.ch) from ../craft/.env
      - ./frontend.env:/craft/.env:ro
    labels:
      - traefik.enable=true
      - traefik.http.routers.site-web.rule=Host(`example.ch`)
      - traefik.http.routers.site-web.entrypoints=websecure
      - traefik.http.routers.site-web.tls.certresolver=letsencrypt
      - traefik.http.services.site-web.loadbalancer.server.port=8080
    networks: [traefik]

networks:
  traefik:
    external: true
```

`GRAPHQL_TOKEN` and `SMTP_URL` come from an `.env` next to the compose
file; instead of `GRAPHQL_TOKEN` an admin can make the token of the
website in the studio (GraphQL), with only the sections and sites it
needs. `./data` belongs to uid 1000 (`chown 1000:1000 data`). An update
is a new image: `serve` applies its migrations at the start.

### The website

- Previews carry their token in the url (`?token=`, a site's lasts a
  day). The website sends no more than the origin to other sites: the
  example sets `<meta name="referrer"
  content="strict-origin-when-cross-origin">` in `index.html`, or the
  proxy sends the header `Referrer-Policy` with that.
- Anyone may search the website, and every search page asks the cms
  twice (`search`, `searchCount`). Limit how often one visitor may load
  the search page (the example's `/suche`, `/en/search`) at the proxy
  (traefik's `ratelimit` middleware, a few a second per address) or
  cache its answers for a minute.

## Migrations from the code

`makemigration` compares the schema in
code with the last file in `migrations/` and writes the difference as
ops. Renames are hints (`--rename event.city=town`), otherwise a rename
is a remove and an add, and the tool says which content is lost. `serve`
applies new migrations to all content and refuses to start when the
code changed without one. Labels, names, `required` and the field order
change freely. Deleted entries leave their children to their parent (or
the top), drafts and revisions of a type that goes take their entry's
type, a field of another kind (also a group of another component)
starts over with its default, and content that doesn't parse stops the
migration instead of being emptied; it runs in one transaction.

## Data migrations

What the ops can't express is a step in Rust: fill a new field from old
ones, split a name, turn a text into a dropdown value, set a value for
old entries. `makemigration --data --name split_names` writes
`0003_split_names.json`, a file without ops that only marks the place
of the step among the migrations, and prints the code to register it:

```rust
use zipp::migrate::{DataResult, DataTx};

fn split_names(tx: &DataTx) -> DataResult {
	tx.each("person", |c| {
		let name: String = c.get("name").unwrap_or_default();
		let (first, last) = name.split_once(' ').unwrap_or((&name, ""));
		c.set("firstName", first);
		c.set("lastName", last);
		Ok(())
	})
}

Cms::new(schema(), zipp::migrations!())
	.data_migration("0003_split_names", split_names)
	.run();
```

The schema has to be migrated first, so a split is three migrations:
one adds `firstName` and `lastName`, the data migration fills them,
the next removes `name`. The step runs once, after the migrations
before it and in their transaction: an error (`?` takes any) stops
the start and rolls back all of them. It sees the schema of its file,
so fields that a later migration removes are still there.

`each(handle, f)` calls `f` with the fields of every entry of an entry
type, or of every block or group of a component, in every site: the
live content, drafts and revisions, also of entries in the trash, like
the ops, so a draft published or a revision restored later has the new
shape. `c.get::<T>(field)`, `c.set(field, value)` and `c.remove(field)`
change the fields as JSON; `c.entry_id`, `c.site` and `c.title` say
where. Values are written as they are, without the checks of a save.
`tx.conn()` is the transaction for anything else in SQL. The indexes,
the search and the uris are rebuilt from the code's schema after the
migrations, as after the ops. Asset fields aren't reached yet.

A data migration file without its step, or a step without its file,
stops `serve` and fails `check`; the step stays in the code as long as
its file. When the database (`--db`) is there, `check` also lists the
migrations it hasn't applied yet, without changing it.

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

The import is only in a binary whose crate turns on zipp's feature
`craft` (`zipp = { path = "…", features = ["craft"] }`), so a site that
never came from Craft doesn't ship it; a port turns it on and off again
once it is live.

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
they are only a link). A group that Craft didn't have takes the flat
fields of the same entry or block with the handles of its fields, or
with the group's handle before them: a group `sale` with `price` and
`format` takes Craft's `price` and `format`, a group `contact` with
`heading` takes `contactHeading` (or `heading`). A Craft field fills one
place only: a field of the entry type itself wins, and a Craft field
that fits two places (or two that fit one) is left out and listed, so
rename one of them. Assets come with their title, alt text (per
site, `export.php` from before gives only the primary site's) and focal
point (a path out of its volume, absolute or with `..`, is left out
and listed); entries keep Craft's post date, structures their order. What
the schema doesn't have is left out and listed; slugs are made zipp's
(`teddy--co.` becomes `teddy-co`), so redirect such urls in the
frontend. SEO fields aren't imported yet.
