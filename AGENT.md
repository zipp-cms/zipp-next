# Agent Instructions

Instructions for anyone (human or coding agent) changing this repository.

- Keep it simple. Prefer straightforward code that follows nearby patterns
  over abstractions, extra validation or options for theoretical cases.
  Build the smallest version that solves the actual need; every code path
  must be reachable today.
- The content model is code. Don't add ways to change it at runtime.
- Content stays plain JSON. Whatever has to be queried gets its own rows
  (`entry_values`, `entry_relations`), written on every save and rebuilt
  after migrations, never a second copy that can drift.
- `db.rs` holds the tables of zipp itself (append only). Changes of a
  site's content model are schema migrations (`migrate.rs`).
- Rust is formatted with `cargo fmt` (hard tabs, max width 80), checked
  with `cargo clippy --all-targets`. Svelte and TypeScript use prettier
  (tabs, single quotes) and `npm run check`.
- Use blank lines to separate logical steps inside functions.
- Tests are small, offline unit tests next to the code (`#[cfg(test)] mod
  tests`). Database tests use `Db::memory()`.
- Commits: a short imperative subject in plain words, then a body that
  says what changed and why.
