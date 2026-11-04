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
- GraphQL takes several sections (`section: ["news", "pages"]`, an
  unknown one is an error instead of only the first counting), `["not",
  ...]` for sections and types, `level`, `orderBy: "score"`, Craft's
  `relatedTo` and `relatedToEntries`, and `children(...)` the arguments
  about the children (`limit`, `type`, `orderBy`, ...). Blocks have a
  `slug` of their title field, for anchors.
- GraphQL lists the assets of the library like Craft's: `assets` and
  `assetCount` with `folder` (Craft's `volume`), `folderId`, `kind`,
  `search`, `orderBy`, `limit` and `offset`; assets have `folderId`,
  `folder` and `path`, so asset fields can't take these names anymore.
- Asset fields in GraphQL take Craft's `kind`: `images(kind: "image")`,
  also a list and `video`, `file`, `pdf` and `word`.
- Videos are the asset kind `video` like in Craft, they were `file`;
  `GET /api/assets?kind=file` lists files without the videos now.

- Images in Craft's sizes: `url(height: 800)` and `url(width: 800,
  height: 450)` (cropped around the focal point), also on `width` and
  `height`, for the sizes of `Schema::image_heights` and
  `Schema::image_sizes` besides `Schema::image_widths`; a size that
  isn't listed is an error naming the allowed ones.
- Craft's `@formatDateTime(format:, timezone:, locale:)` on the dates
  of entries and date fields, PHP's date format, also as field
  arguments; `Schema::timezone` for the time zone of formatted dates.

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
