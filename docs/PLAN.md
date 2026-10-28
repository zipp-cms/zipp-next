# Plan

## Slice 1: code-first end to end (done)

- [x] Schema as Rust code: entries, blocks, text/number/boolean/relation/blocks
- [x] Content as JSON, index tables for indexed fields and relations
- [x] Query api: filters, order, reverse relations, published only for the public
- [x] Migrations generated from schema diffs, renames as hints, applied on start
- [x] Studio: entry lists, editing with nested blocks, relation picker, field errors
- [x] TypeScript types and an example frontend

## Next

- [ ] Drafts and revisions: a row per saved version, publish a version
- [ ] Live preview: preview tokens, frontend renders unsaved content
- [ ] Assets: uploads, image field with focal point, transforms
- [ ] Rich text field
- [ ] Field kinds as plugins: a crate with the Rust side and a studio
      package with its Svelte field
- [ ] Sites and languages
- [ ] Data migrations in code, for changes the ops can't express
- [ ] crelte adapter, so a crelte project can use zipp instead of Craft
- [ ] Full text search
