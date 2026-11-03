# Releasing

zipp is released by GitHub Actions (`.github/workflows/publish.yaml`)
with Trusted Publishing: crates.io and npm trust that workflow of
`zipp-cms/zipp-next`, there are no tokens in the repository.

What is published:

- crates.io: `zipp-cms` (the crate `zipp`), `zipp-seo`, `zipp-forms`;
  `zipp-agent` and `example-cms` have `publish = false`
- npm: `zipp-ui` (`studio/ui`), with its own version

## The crates

1. The version is in `[workspace.package]` of `Cargo.toml`, the same for
   all crates; the plugins' dependency `zipp = { package = "zipp-cms",
   path, version }` gets it too. `CHANGELOG.md` gets its section.
2. `npm run build` in `studio/` and commit the embedded studio
   (`crates/zipp/studio`).
3. Before tagging: `cargo package -p zipp-cms -p zipp-seo -p zipp-forms`
   packages and builds them all, also before zipp-cms is on crates.io.
4. Tag and push: `git tag v0.1.0 && git push origin v0.1.0`.

The workflow checks that the tag is the crates' version, builds the
studio and fails if it differs from the committed build, runs the tests
and publishes `zipp-cms`, `zipp-seo`, `zipp-forms` in that order, each
after the one before it is in the index. A version that is on crates.io
already is skipped, so a failed run can be rerun.

## zipp-ui

1. The version is in `studio/ui/package.json`.
2. Tag and push: `git tag zipp-ui@v0.1.0 && git push origin
   zipp-ui@v0.1.0`.

The workflow checks the version, runs `npm run check` of the studio and
publishes with provenance; a prerelease (`0.2.0-alpha.0`) gets the
dist-tag `alpha`.

## The first version

A trusted publisher can only be set for a package that exists, so the
first version of each crate and of zipp-ui is published once by hand
with a token:

- crates.io: a token with the scope publish-new, then `cargo publish -p
  zipp-cms`, `-p zipp-seo`, `-p zipp-forms` (in that order), or
  `cargo publish --workspace`. Revoke the token after.
- npm: `npm login`, then `npm publish --access public` in `studio/ui`.

Then add the trusted publisher on each: crates.io, the crate's
Settings → Trusted Publishing: owner `zipp-cms`, repository
`zipp-next`, workflow `publish.yaml`; npmjs.com, the package's Settings
→ Trusted Publisher: GitHub Actions, the same repository and workflow.
Every later version goes through a tag.
