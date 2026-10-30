# zipp-ui

The components of the zipp studio: the studio is built of them, and the
editors of field types and the studio modules of plugins use them to
look the same (see `docs/extending.md`). Svelte 5 components as source,
in scss: a module bundles them with `svelte({ preprocess:
vitePreprocess() })` and `sass`, like `crates/zipp-seo/studio`.

What a module may import is in `exports` of `package.json`, nothing
else: the components (`zipp-ui/Table.svelte`, …), `zipp-ui/sortable`
(dragging rows), `zipp-ui/layers` (Escape closes the top one),
`zipp-ui/field` (the contract of a field editor: `FieldProps`,
`FieldModule`, `fieldModule(Component)` for its `mount`) and
`zipp-ui/vars.scss` (the breakpoints and mixins: `@include hover`,
`max-tablet`, `focus-ring`).

## What the page gives them

A module renders in the studio's page, the components lean on what the
studio puts there:

- CSS variables: colors (`--text`, `--muted`, `--accent`, `--line`,
  `--border`, …), `--radius`, the type scale `--fs-*`, `--z-*`,
  `--shadow-*`.
- Base styles of `button`, `input`, `select`, `textarea` and `h1`–`h3`.
- Classes: buttons `.primary`, `.light`, `.icon`, `.link`, `.plain`,
  `.danger`; `.popup` and `.popup-item` (menus and popovers); `.card`,
  `.box`, `.truncate`, `.note`, `.muted`, `.empty`, `.error`, `.badge`,
  `.changed-dot`, `.toolbar-sep`, `.reveal` (shows on hover).

They are part of the contract: renaming one of them breaks modules.
A component's own classes are scoped, a module's too.

A component imports nothing of the studio.
