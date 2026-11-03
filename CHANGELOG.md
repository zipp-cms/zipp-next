# Changelog

## Unreleased

- The field kind button group, `Field::button_group`: one of some
  options like a dropdown, as buttons in a color per option like
  Craft's; GraphQL gives `ButtonGroupOption { value label color }`. It
  was the example crate `zipp-button-group`, which is gone.
- `import-craft` imports meierlabs-seo's SEO field into zipp-seo's,
  TableMaker tables into table fields (by the order of the columns) and
  the fields of assets into the asset fields; each volume is a folder
  with Craft's folders in it (`--volume handle=folder:dir` names it).

## 0.1.0 (2026-11-03)

The first release: zipp as a library on crates.io (`zipp-cms`, the crate
`zipp`), the plugins `zipp-seo` and `zipp-forms`, and the studio's
components `zipp-ui` on npm.

- The content model as code: sites and languages, singles, channels and
  structures, entry types, blocks, field kinds with options, roles,
  field types and plugins of other crates.
- Migrations made from the changed schema (`makemigration`), data
  migrations in Rust, `check` for CI with the frontend's queries.
- The studio on `/admin`: entries, drafts, revisions, previews and live
  preview, the trash, assets with focal point and image editing, users
  and sessions, GraphQL tokens, the AI chat.
- The api: GraphQL like Craft's for crelte (private, with tokens), the
  typed rest api the studio uses, TypeScript types (`types`).
- Running a site: `serve`, backups and restore, mail over SMTP, a
  Dockerfile, the import of a Craft site (feature `craft`).
- `zipp::testing::TestServer` (feature `testing`) for the tests of sites
  and plugins.
