# zipp-ui

The components of the zipp studio: the studio is built of them, and the
editors of field types and the studio modules of plugins use them to
look the same (see `docs/extending.md`). Svelte 5 components as source,
in scss; they only use the studio's CSS variables (`--accent`,
`--line`, `--radius`, …), which the studio puts on the page.

- `src/`: the components and `icons/` (Lucide)
- `vars.scss`: the breakpoints and mixins (`@include hover`,
  `max-tablet`, `focus-ring`)

A component imports nothing of the studio.
