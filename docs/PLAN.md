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

- [ ] `localized` on entries (the same entry in the other sites) for language switches
- [x] Globals as singles without a url, `globalSet(handle:)` in GraphQL, navigation groups
      in the schema (`.nav("Settings", [...])`), preview button, status per site in lists

- [x] Drafts (one per entry, all sites, saved while typing), publish in one transaction,
      revisions with restore, preview tokens that crelte passes to GraphQL
- [ ] Releases: several drafts published together (Craft 6), on top of the drafts
- [ ] Live preview: preview tokens, frontend renders unsaved content
- [ ] WebP/AVIF output for transforms, alt text per site
- [x] Rich text field (TipTap): headings, lists, links, button rows, embedded blocks, html + chunks
- [ ] Field kinds as plugins: a crate with the Rust side and a studio
      package with its Svelte field
- [ ] Data migrations in code, for changes the ops can't express
- [ ] crelte adapter, so a crelte project can use zipp instead of Craft
- [ ] Full text search
- [ ] Storage backends: SQLite (today), in memory (SQLite `:memory:`,
      already used by the tests; for previews and throwaway instances),
      Postgres for bigger sites. Needs a small store interface around the
      SQL in entries.rs, auth.rs and query.rs; the index tables stay the
      same in both

## Slice 3: what real sites need

- [x] Navigation groups, globals as singles, preview button, status per site
- [x] Drafts, revisions with what changed, preview tokens, conflicts between editors
- [x] Link field, rich text (TipTap) with button rows and embedded blocks
- [x] Translations defined in code
- [x] Cms builder: routes and on_change hooks of the site
- [ ] Group field (zandi's ContentBlock), `localized` for language switches
- [ ] Port textrem or zandi to zipp

## Ideas of the operator (2026-10-28)

- Globals as singles; the studio navigation configurable in the schema, so settings
  (header, footer, translations) have their own place in the top bar
- Preview button, later live preview like Craft (drafts + preview tokens)
- Status per site in the lists (DE, EN)
- AI mode: chat with the whole website in the studio ("fix the translation on this
  page"), split view like noah: chat on the left, the live site on the right, changes as
  drafts the editor publishes
- Rich text in Svelte instead of CKEditor, a mix of Notion style typing and blocks
  inside the text (like Craft's nested entries in CKEditor, but nicer)
- Port textrem (crelte frontend of the Craft port) to zipp as the first real site
- Dashboard with visitor statistics like the old Zipp (later)
