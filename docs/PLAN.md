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
- [ ] GraphQL generated from the schema, `entry(uri, site)` in the shape crelte expects,
      relations resolved with their fields in the same query
- [ ] Assets: uploads, image field with focal point, named transforms in code,
      generated on first request and cached
- [ ] A crelte project on zipp as the real test

## Next

- [ ] Drafts and revisions: a row per saved version, publish a version
- [ ] Live preview: preview tokens, frontend renders unsaved content
- [ ] Assets: uploads, image field with focal point, transforms
- [ ] Rich text field
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
