# Changelog

## 0.2.0 (2026-11-07)

- Website members, like Craft users without access to the control
  panel: `Role::no_studio()` keeps a role out of the studio, any user
  logs in on the website (`/site-auth/login`, `me`, `logout`, `reset`,
  `password`; site sessions never open the studio's api). Entries have
  an author (`authorId`, `author` in GraphQL, `?author=`, admins change
  it), `Role::own(section, level)` gives access to a user's own entries.
  The new plugin `zipp-members` lets members list, create, edit and
  delete their own entries and upload images from the frontend, through
  the same checks as the studio. See docs/members.md.
- zipp-forms limits submissions per address only (5 in 10 minutes): the
  cap of 50 per form from all addresses let about ten addresses block a
  form for everyone.
- zipp-seo's root `seo` and `seoJson` are null for a site the token's
  GraphQL schema doesn't read; plugins check it with
  `graphql::ext::reads_site`.
- A switched off block in rich text (`"enabled": false`) is left out of
  GraphQL's `chunks` like a switched off block of a blocks field.
- GraphQL's `related` and the relation field arguments (`offerCategory:
  5`, `":notempty:"`, `":empty:"`) only count the entries the viewer
  may see, like `relatedTo`: a token learned through them that an entry
  points to an unpublished one or to one outside its sections or sites.
- A zip upload checks its folder before it is unpacked, and a server
  unpacks one zip at a time: another one meanwhile is a conflict (409),
  the studio sends its zips one after another.
- The file url of an edited image is always the edited image, `?original`
  is gone: anyone with the url got the uploaded file and the crop of an
  editor back. The studio's image editor and Download get the uploaded
  file with the session from `GET /api/assets/{id}/original`, for roles
  that see assets.
- GraphQL schemas are code, like Craft's in its project config:
  `Schema::graphql_schema(GraphqlSchema::new("website", "Website")
  .sections(["pages", "news"]).assets().previews())` says which
  sections and sites a frontend reads, whether it lists the assets
  (`assets`, `assetCount`: only with `.assets()` now), reads users and
  shows drafts in previews. A token of the studio gets one of them
  instead of its own sections and sites; tokens made before have none
  and read nothing, make them again. The GraphQL page shows the schemas
  and the token of `serve` (`ZIPP_GRAPHQL_TOKEN`) as its first row,
  GraphiQL's picker reads as a schema (`x-zipp-schema: <handle>`).
- A field type can hold blocks with more around each one, like a place
  in a grid: `Field::custom(..).item_blocks(["image", "text"])` makes its
  value a list of items with a block in `block`. zipp checks, migrates
  and indexes those blocks like a blocks field's, the type the rest.
  For its GraphQL `GraphqlType::List` of a plugin's type,
  `graphql::ext::site` and `graphql::ext::block`; its studio module gets
  `blocks` with the block types, `create`, `copy` and `mount`, the
  studio's editor of a block.
- Public user profiles, like Craft's user fields and WordPress' author
  pages: `.user_fields([...])` (a component `user`, in migrations), a
  unique slug of every user (admins change it), and
  `.user_pages(UserPages::new("cooks/{slug}").with_entries_in("recipes"))`
  for pages of the users with a live entry. GraphQL has `users(hasEntries:)`,
  `userCount` and `user(slug:)` for schemas with `.users()`,
  `author` is a `User` with the profile; the SEO sitemap lists the user
  pages; logged in website users edit their own profile with `PUT
  /site-auth/me`. The studio's user page edits the profile.
- Changing an entry's type asks nothing anymore: fields of the same
  handle and kind stay, the type's defaults fill what is empty, and
  while the editor is open each type keeps its values, so switching
  back brings them again. Of a saved entry the change is in the draft:
  Discard or an older revision bring back the old type with every
  field. A new entry with only its type's defaults has no content,
  leaving it doesn't ask.
- A narrow editor (a phone, or next to a wide live preview) has a ⋯
  menu in its bar with "Copy content from <site>" and Move to the
  trash, instead of the copy card at the end of the page.
- The site box shows that it is a choice when there are several sites:
  a hover background and a focus ring for the keyboard.
- The site box at the top of the sidebar shows the website's favicon
  instead of the site's first letter, the letter when it has none. The
  cms fetches it (`GET /api/sites/{site}/favicon`, with a session) from
  the site's url, only from that host, and keeps it an hour.
- An icon next to the site at the top of the sidebar opens the website
  of the site the studio edits (its url from `--site-url`), the first
  site's when it has none; the rest of the box still switches sites.
- A field that takes only one type leaves the type out: relation chips
  show only the title, block rows only their summary (the type stays
  when a block has no summary). Moving a chip of a relation, which wrap
  side by side, says Move earlier / Move later.
- GraphiQL has a schema picker like Craft's: an admin reads as a
  frontend with one of the GraphQL schemas (its sections and sites,
  nothing unpublished) instead of everything.
- The live preview's website gets as wide as the window allows: the
  editor keeps 20rem, before it kept 40% of the window. A narrower
  window shrinks the website, it gets its width back when there is
  room (also the AI page's chat).
- The editor in a narrow pane (a phone, next to the live preview):
  the top bar stays one line (only the crumb one up and the title,
  shortened, a page's note under them), the rich text toolbar is one
  line that scrolls sideways, cards in cards have less padding, the
  field layout link is an icon, and a block's hide, duplicate and
  remove go into a ⋯ menu.
- Every label on the left of its field (in a group, a block or a card)
  sits at the top of its row, level with the field's first line, with
  the handle on the name's line (under it only when the column is too
  narrow) and the instructions below, for every kind: lists, groups and
  other fields without one input had it centred with the handle beside.
- A button group field looks like the studio's other choices (the
  segmented control of Alle / Bilder / Videos): one grey track, the
  picked option raised in white. Colored options keep their dot in
  their color, picked or not, and their icon. The arrow keys move the
  pick, Tab stops once, on the first option while none is picked.
- The entries of a relation field are compact chips side by side that
  wrap, like Craft's: the handle, the title (an ellipsis when it is
  long), the type and remove; the Choose button follows the last one.
  They move by dragging or with the arrow keys, left and right too.
- A field's handle sits on the line of its name whenever the label is
  above the field (half width fields in a group, narrow boxes), for
  every kind: text and number fields had it on a line of their own.
- A change undone deletes the draft: when the draft is the live content
  again, the Draft badge, Discard and Publish go away as if there never
  was one. `PUT /entries/{id}/draft` (and `drafts::put_draft`) answers
  `null` then.
- An old revision opens like the editor, read only, without the white
  box around it: title, tabs and fields as when editing, under a
  banner with its date, Restore and Back to editing.
- Content without a revision (imported, or from before revisions were
  kept) is kept as a revision of no one before the first save or
  publish over it, so the original can be restored; before, the first
  revision already held the edit. `import-craft` records the imported
  state as the first revision.
- A new block (also a copy, a block in a rich text) gets its id in the
  studio: it stays open after saving instead of closing when the
  content loads again.
- A rich text with buttons has a button icon in its toolbar, after the
  link and a line, that inserts a button; the Insert menu has only the blocks.
- `Field::rich_text(..).toolbar([..])` sets the studio's bar of a rich
  text: which tools, in which order, `|` for a line between them;
  `block:<type>` gives an embedded block an icon of its own. Buttons
  and blocks without a tool go into the Insert menu. The schema check
  refuses unknown tools and tools the field doesn't allow.
- An old revision shows the entry type's tabs like the editor, opens
  on the editor's tab and has a dot on the tabs it changed.
- Live preview also for a new entry of a section with pages: the button
  saves it unpublished first (like a new entry in Craft is a draft) and
  opens it with the website. Thumbnails of images keep the focal point
  in view.
- A field that changes its kind keeps plain values the new kind takes
  as they are (text to a dropdown or button group whose options have
  them, a dropdown to a button group); the others start over with the
  default as before.
- A group without a value yet (a new entry) checks the fields it
  requires: a new Mappe of textrem could be saved without its price.
- A group is always an object in GraphQL, also for an entry saved
  before it had a value (a new entry, a draft), like Craft's content
  block: a frontend no longer breaks on `sale.format` of a new entry.
- `sort-assets` puts assets into the folders of the asset fields that
  use them, `{slug}` and the others filled in like uploads in the
  studio: after an import from Craft, or when a field got a folder.
  Assets of several entries stay where they are.
- A role can be without Setup, `Role::without_setup()`, like an editor
  in Craft who only writes: no Setup in the sidebar, no links to it, no
  change requests. Roles see it read only as before by default.
- The live preview opens as wide as a phone (400px) and goes down to
  280px. The entry editor lays out by its own width (container
  queries, mixins in `vars.scss`): too narrow for the sidebar, the
  entry's status, dates, type and slug are a closed line on top with
  the state and the post date; labels of fields in blocks and
  `FieldRow`s go above their value when their box is narrow, not only
  on a phone.
- An asset field's folder can be per entry, like Craft's upload
  subpath: `.folder("Topics/{slug}")`, also `{section}` and `{type}`;
  the studio fills them in from the entry being edited (also in its
  blocks), without a slug yet the path ends before it. The example's
  event program goes into `Events/{slug}`.
- The field kind button group, `Field::button_group`: one of some
  options like a dropdown, as buttons like Craft's; GraphQL gives
  `ButtonGroupOption { value label color icon }`. It was the example
  crate `zipp-button-group`, which is gone. An option can have a color,
  an icon, both or neither: `("pdf", "PDF")`, `("concert", "Konzert",
  "#2b6cb0")` or `ButtonOption::new("pdf", "PDF").icon("file-text")`;
  the studio shows a dot in the color and the icon on the button.
- `import-craft` imports meierlabs-seo's SEO field into zipp-seo's,
  TableMaker tables into table fields (by the order of the columns) and
  the fields of assets into the asset fields; each volume is a folder
  with Craft's folders in it (`--volume handle=folder:dir` names it).
- `import-craft` keeps a link's url suffix: `#contact` is the link's
  anchor, `?a=1` goes on the end of a url (`export.php` exports it). It
  leaves out Craft's temporary uploads without a volume instead of
  failing on them.
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
