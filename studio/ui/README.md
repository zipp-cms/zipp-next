# zipp-ui

The components of the zipp studio: the studio is built of them, and the
editors of field types and the studio modules of plugins use them to
look the same (see `docs/extending.md`). Svelte 5 components as source,
in scss: a module bundles them with `pluginConfig` of `zipp-ui/vite`
(its `vite.config.js`) and `sass`, like `crates/zipp-seo/studio`.

What a module may import is in `exports` of `package.json`, nothing
else: the components (`zipp-ui/Table.svelte`, …), `zipp-ui/sortable`
(dragging rows), `zipp-ui/layers` (Escape closes the top one),
`zipp-ui/field` (the contract of a field editor: `FieldProps`,
`FieldModule`, `fieldModule(Component)` for its `mount`) and
`zipp-ui/vars.scss` (the breakpoints and mixins: `@include hover`,
`max-tablet`, `focus-ring`, `visually-hidden`) and `zipp-ui/vite` (the
vite config of a module).

For the values of a field, beside the inputs of the page: `Segmented`
(one of a few), `CheckGroup` (some of a few, chips that wrap),
`Switch` (yes or no, its text on the right; `onchange` only when
clicked), `DatePicker`, `SearchInput`.

## Texts

What the components write themselves (Today, Clear, Close, Nothing
found, the moves of `DragHandle`) and the locale of their dates are in
`zipp-ui/texts`: `ui.texts` (English unless set) and `ui.locale`. The
studio sets both in its user's language, `<html lang>` too. The object
lives on `globalThis`, so the zipp-ui a plugin bundles reads the same;
it reads it when it renders, a changed language shows in a plugin's view
when it opens again. A component's text props (`placeholder`, `label`,
`empty`) win over them. A plugin translates its own texts itself, by
`document.documentElement.lang`.

## What the page gives them

A module renders in the studio's page, the components lean on what the
studio puts there:

- CSS variables: colors (`--text`, `--muted`, `--accent`, `--line`,
  `--border`, …), `--radius`, the type scale `--fs-*`, `--z-*`,
  `--shadow-*`.
- Base styles of `button`, `input`, `select`, `textarea` and `h1`–`h3`.
- Classes: buttons `.primary`, `.light`, `.icon`, `.link`, `.plain`,
  `.danger`; `.popup` (the look, its place is the component's) and
  `.popup-item` (menus and popovers); `.backdrop` (under a flyout or
  dialog); `.card`, `.box`, `.truncate`, `.note`, `.muted`, `.empty`,
  `.error`, `.badge`, `.changed-dot`, `.toolbar-sep`, `.reveal` (shows on
  hover), `.sort-dragging` (the row `sortable` drags).

They are part of the contract: renaming one of them breaks modules.
A component's own classes are scoped, a module's too.

A component imports nothing of the studio.
