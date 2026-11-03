# Plan

## Slice 1: code-first end to end (done)

- [x] Schema as Rust code: entries, blocks, text/number/boolean/relation/blocks
- [x] Content as JSON, index tables for indexed fields and relations
- [x] Query api: filters, order, reverse relations, published only for the public
- [x] Migrations generated from schema diffs, renames as hints, applied on start
- [x] Studio: entry lists, editing with nested blocks, relation picker, field errors
- [x] TypeScript types and an example frontend

## Slice 2: the Craft model, GraphQL, images

- [x] Sections (single, channel, structure) with several entry types and uri formats
- [x] Sites: a row per site, translatable and shared fields, migrations for sites and sections
- [x] Studio: sections, site switcher, tree of a structure
- [x] GraphQL generated from the schema: `entry(uri, site)`, `entries`, `entryCount`,
      `sites`; relations resolve with their fields in the same query
- [x] Assets: uploads, images and assets fields, focal point, named transforms in code,
      generated on first request and cached, url/width/height/srcset in GraphQL
- [x] A crelte project on zipp: Craft compatible GraphQL (crelteSites, siteId, __home__,
      url), example/crelte with crelte's default template, blocks with loadData

## Next

- [x] `localized` on entries (the same entry in the other sites) for language switches
- [x] Globals as singles without a url, `globalSet(handle:)` in GraphQL, navigation groups
      in the schema (`.nav("Settings", [...])`), preview button, status per site in lists

- [x] Drafts (one per entry, all sites, saved while typing), publish in one transaction,
      revisions with restore, preview tokens that crelte passes to GraphQL
- [ ] Releases: several drafts published together (Craft 6), on top of the drafts;
      postponed to 0.2 (operator 2026-11-01)
- [x] Live preview: the website next to the editor with the draft, updated after
      every save (preview tokens, crelte's live preview mode keeps the scroll)
- [x] WebP output for transforms (`.format`, `url(format:)`), alt text per site;
      AVIF left out (rav1e takes seconds per image)
- [x] Rich text field (TipTap): headings, lists, links, button rows, embedded blocks, html + chunks
- [x] Field kinds as plugins: a crate with the Rust side and a studio
      package with its Svelte field
- [x] Data migrations in code, for changes the ops can't express
- [ ] crelte adapter, so a crelte project can use zipp instead of Craft:
      a new adapter in crelte (no craft/.env, no Craft variable names); no
      hacks in the textrem port meanwhile (operator 2026-11-01)
- [x] Full text search in the studio (FTS5 over the live texts of entries,
      assets by name) and for the website (`search`, `entries(search:)`)
- [ ] Storage backends: SQLite (today), in memory (SQLite `:memory:`,
      already used by the tests; for previews and throwaway instances),
      Postgres for bigger sites. Needs a small store interface around the
      SQL in entries.rs, auth.rs and query.rs; the index tables stay the
      same in both

## Release 0.1.0 (operator 2026-11-03)

- Published on crates.io as `zipp-cms` (`[lib] name = "zipp"`, so `use zipp::`
  stays; whether the name `zipp` can be had is checked separately), the
  plugins as `zipp-seo`, `zipp-forms`; `zipp-agent` stays unpublished,
  `zipp-button-group` became the field kind `Field::button_group` of zipp
  (operator 2026-11-03)
- Released by GitHub Actions with Trusted Publishing (`RELEASING.md`)
- License MIT OR Apache-2.0
- zipp-ui is published on npm too, for plugin authors
- Order: the fixes of the reviews first (zipp-tools reviews release-0.1,
  sec-0.1, svelte-review-0.1), then API narrowing, freezing zipp's tables
  and the migration format, packaging, then check readiness; publishing
  only after the operator's go

## Structure (2026-10-30)

- [x] The rest api as typed requests (`request!`, `Req`, `.api::<R, _>`),
      permissions as extractors, the SQL out of the handlers
- [x] `testing::TestServer`: every request tested over HTTP with each role,
      on the real SQL; the example and the plugins use it too

## Slice 3: what real sites need

- [x] Navigation groups, globals as singles, preview button, status per site
- [x] Drafts, revisions with what changed, preview tokens, conflicts between editors
- [x] Link field, rich text (TipTap) with button rows and embedded blocks
- [x] Translations defined in code
- [x] Cms builder: routes and on_change hooks of the site
- [x] Group field (zandi's ContentBlock); `localized` is above
- [ ] Port textrem, then zandi (operator 2026-10-28): first 1:1, GraphQL with Craft's
      names (`CkeditorMarkup`, `LinkData`, `<type>_Entry` for blocks, `<handle>_GlobalSet`)
      and query arguments, so the frontends' queries stay; later remove what isn't needed
      (e.g. `defaultLabel` of links)
- [x] Importer from Craft (content, structure, assets with focal points), `import-craft`
- [x] SEO native in zipp like the Craft plugin meierlabs-seo: a seo field, defaults in code
      per entry type and section, global settings as a single, `seoJson` on every entry and
      `seo(siteId)` in the plugin's shape so the npm `<Seo>` works unchanged, `/sitemap.xml`
      and `/robots.txt` for its `seoRoutes`

## Ideas of the operator (2026-10-28)

- Globals as singles; the studio navigation configurable in the schema, so settings
  (header, footer, translations) have their own place in the top bar
- Preview button, later live preview like Craft (drafts + preview tokens)
- Status per site in the lists (DE, EN)
- [x] AI mode: chat with the whole website in the studio ("fix the translation on this
  page"), split view like noah: chat on the left, the live site on the right, changes as
  drafts the editor publishes
- Rich text in Svelte instead of CKEditor, a mix of Notion style typing and blocks
  inside the text (like Craft's nested entries in CKEditor, but nicer)
- Port textrem (crelte frontend of the Craft port) to zipp as the first real site
- Dashboard with visitor statistics like the old Zipp (later)

## Open questions and later (operator 2026-10-28)

- Users read only too: roles are code already; a read-only view of who has
  which role for non-admins (question to the operator)
- [x] "Request a change" on read-only Setup pages: the editor writes what
  they want, admins see it under Setup → Requests (and get a mail)
- AI that changes the content model and Rust code, only in dev mode and for
  admins: it edits the schema code, the dev server rebuilds, the migration
  check shows the ops. In dev mode it gets merge rights too, otherwise its
  changes wouldn't land (operator 2026-10-28); other rights still open (later)
- [x] Options tab replaced by a sidebar like Craft (status per site, slug, type,
  parent, copy from site, revisions), globals only copy and revisions; post date
  comes with the textrem port
- [x] SEO half code, half database: field mappings in code, title pattern,
  description, image, robots and sitemap per section and entry type
  editable in `seoSettings` over the code's defaults
- [x] SEO as the plugin `zipp-seo`, the test of the plugin system (operator
  2026-10-28); field handles for admins; tabs of entry types, shown in Setup
- Two AI features (operator 2026-10-30, later): the AI chat in the core
  (`crates/zipp/src/ai`, a model on OpenRouter that writes drafts, for
  roles with `.ai()`) and the coding agent of the plugin `zipp-agent`
  (development, admins); the studio's AI page shows the agent when the
  plugin is on. Should the chat be a plugin too, or both one feature?
- Where translation keys live: in code (today) or in the frontend's JSON
  (question, open since the textrem plans)

## Backlog (operator 2026-11-01)

The operator's list; `?` were open questions on it. Done ones keep their
commit.

- [x] Conditions: fields (and tabs) shown only when another field has a
      value, like Craft's field conditions (zandi's `hasPage`)
      (`.when(Condition::on("hasPage"))`, `Schema::tab_when`)
- [x] A read-only markdown field: it only shows its text in the field
      layout (a note, a how-to), like Craft's tip; editors don't edit it
      (`Field::note`)
- [x] Field width in the editor: 25 %, 50 %, 75 %, 100 % (`.width(50)`)
- [x] Settings (globals) without the big title above the fields, 4b457a1
- [x] Tabs inside blocks (`Schema::block_tabs`)
- [x] Max level of a structure, 6317a46
- [x] Custom fields on assets (besides title, alt, focal point)
      (`Schema::asset_fields`)
- [x] Better support for video assets (player, first frame, type filter,
      `Field::videos`, ranges)
- [x] A field type per column of a table: number, boolean, date,
      dropdown and link besides text, url and email (`Column::number`)
- [x] Number field: `Field::number` with min and max, `.unit("CHF")` and
      `.decimals(2)`
- [x] Custom styles in the rich text (CKEditor's styles: classes on text
      and blocks, `.styles([Style::block("lead", "Lead")])`)
- [x] Headings in the rich text: `.headings([2, 3])`
- [x] Switching off a block (`"enabled": false`), e6ac8e3
- [x] Search: full text in the studio (Cmd/Ctrl+K) and on the website
- [x] GraphQL tokens like Craft's schemas: several, each with what it may
      read, made by admins in the studio
- [x] Logging in as a user (impersonation), 4d556eb
- [x] GraphiQL for admins in production too (`/api/graphql` with the
      session)
- [x] Live preview next to the editor (see Next)
- [x] A better Setup page: an overview of what is where, field usage,
      what plugins add, change requests
- [x] Post date, e6ac8e3
- [x] Expiry date, e6ac8e3
- [x] Changing the type of an entry later, 1ede0fb
- [x] Download assets or a folder as a zip
- [x] Upload a zip and unpack it into folders
- [x] Backups of the database and files: `backup`, `restore`, daily by
      `serve --backup-dir`; a container image (`example/cms/Dockerfile`)
- [ ] The coding AI (`zipp-agent`, development only today): how far?
- [ ] A section builder in the studio (maybe, operator 2026-11-01): a
      rough click-together of sections, types and fields that doesn't have
      to be exact and outputs pseudo code, which an AI then turns into the
      real schema code
- [x] The studio in the user's language: English and German, per user
- [x] The Craft import out of the main binary: zipp's feature `craft`
- [x] A better page picker (the tree, search) for relations and links
- [x] The entry picker as a flyout with the sections on the left and the
      tree on the right, like the asset library (operator 2026-11-03)
- [x] A login dialog when the session expires, so unsaved work stays, 4b457a1
- [x] The link box of a link field clickable over its whole height, not
      only on its text (operator 2026-11-01)
- [ ] The entry picker as a flyout: sections on the left like the Content
      nav, the nested structure on the right (operator 2026-11-03)
- [x] Limit which sections a relation field and a link field's entry links
      (and rich text / navigation links) may pick from, besides the types
      (operator 2026-11-03)
- textrem's forms (contact, Mappenabo, orders with prices from the cms)
  stay in the site, not in zipp-forms: in the crelte server as today or
  later as routes of its cms crate with `state.mail`, either is fine
  (operator 2026-11-01)
- [x] Import mapping for Craft: fill a group from flat Craft fields, so a
      port can group fields that were flat in Craft (textrem, 2026-11-01);
      by handle, also `contactHeading` into `contact.heading`
