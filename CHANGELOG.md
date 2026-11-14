# Changelog

## Unreleased

### Upgrading

- `zipp-forms`: forms in code (`Form`, `FormField`) and the single
  `formSettings` are gone, build the forms in the studio and pick them
  with `form_field`; the website posts to `/forms/{id}` instead of
  `/forms/<handle>` (the npm package `zipp-forms` does it).

### Forms

- `zipp-forms` is a form builder like Formie instead of Craft's Contact
  Form: editors build forms as entries of the section `forms` (single
  line text, multi-line text, email and phone fields as blocks, per
  site), the website loads one by its id (`GET /forms/{id}`) and sends
  to it, the submissions are stored and listed in the studio's new
  page Submissions. The builder shows a form's pages (steps, each
  checked before the next) as tabs and its fields as a grid of their
  widths (¼ to full, like Formie's columns), a flyout edits a field, a
  ⋯ menu has the rest. The fields are the same in all sites, their
  texts per site. Nothing is mailed for now. The npm package
  `zipp-forms` is its crelte frontend (`loadForm`, `formRoutes`,
  `Form`, `Fields`, `Back`, `Submit`). See docs/forms.md.

## 0.3.0 (2026-11-14)

New entries that save themselves and required fields only to publish,
like Craft; ASCII and unique auto slugs; members link only assets they
may show; plugins with tables and studio pages of their own.

### Upgrading

- Required fields only have to be filled to publish: an unpublished
  entry or a draft can miss them now. Code that reads entries that
  aren't live gets the draft types (see Content model and fields).
- `types` writes draft variants (`PageDraftFields`, `TextDraftBlock`)
  and `Entry` takes them as a third parameter: regenerate the types of
  a site and add it where `Entry` is written by hand.
- Slugs made of a title or a name are ASCII; saved slugs stay as they
  are, but an entry saved before with the unicode slug of its title
  (`über-uns`) no longer has its slug follow the title in the studio.
- Users with only own access to a section link only assets they may
  show (see Security): their drafts lose other assets. Drafts saved
  before keep theirs until they are saved again, so check the pending
  drafts of members before an editor publishes them.

### Security

- Users with only own access to a section (`Role::own`, website
  members) link only assets they uploaded or that a live entry shows
  (with a page, or of a section they see all of, on a site that is
  online), else the field is invalid: a guessed id no longer publishes
  another member's private photo. Their studio drafts lose such assets,
  an editor's publish would make them public. Links an entry has stay,
  roles with edit access to the section link any asset. The same for
  the user fields of one's own profile, only admins link any asset
  there.

### Content model and fields

- Required fields only have to be filled to publish, like Craft's: an
  unpublished entry or a draft saves without them, publishing (the
  studio, the api, the members' routes, the AI's new entries) refuses
  as before, per site for translatable fields. Other checks stay on
  every save.
- `types` writes a draft variant of every entry type's and block's
  fields with all of them optional (`PageDraftFields`,
  `TextDraftBlock`): an entry with `published: false` has those, so
  have the members' routes' entries (their drafts too). `Entry` takes
  them as a third parameter. A required field with a condition is
  optional in both.
- Slugs made of a title or a name are ASCII like Craft's
  `limitAutoSlugsToAscii`: `Über uns` is `ueber-uns`, `Crème brûlée`
  `creme-brulee`, other scripts keep their letters. Saved and typed
  slugs stay as they are (`über-uns`), only new entries and users get
  the new ones; the slug of a block (anchors) changes the same way.
  Text in NFD (macOS: `U` and a combining `¨`) is composed first, so it
  is `ueber` too and a typed slug keeps its `ü`.
- A slug made of the title gets `-2`, `-3`, … when another entry of the
  site (also one that isn't live) has its uri, like Craft's, instead of
  the error "the uri … is already used": a second "Zopf" is `zopf-2`,
  also from the members' routes and the AI. A slug that is typed or
  sent is still refused, saved slugs stay. The studio sends no slug for
  a new entry whose slug follows the title.
- An entry that isn't live may have no title (no uri, *Untitled* in the
  studio), and a new one that isn't live takes the next free slug
  (`-2`) when its uri is taken. An entry that was never live keeps the
  slug of its title free too when it is published, also when the draft
  sends it.

### GraphQL

- `assets(id: [3, 4])` and `asset(id: 3)` read assets by their ids,
  for a token with the library.

### Studio

- The studio creates a new entry with its first change, like Craft's:
  not live, then its changes are a draft, so a reload or a failed
  publish no longer loses a new page. The address switches to the
  entry, Create is Save (Publish when switched on), a structure's
  parent can still change while the editor is open.
- Replace in the ⋯ menu of an image or a video in an asset field picks
  another one from the library in its place.
- An empty image or video field is a low row like the one of other
  files (`Drag an image here`, Choose, Upload); next to its cards the
  tile to add more has their shape: a click anywhere chooses from the
  library, files still drop on it and Upload is in its bar. A field of
  one file says so in the singular (`Drag a PDF file here`).
- The image editor's crop has handles in the middle of its sides that
  move only that side; Alt while dragging resizes around the middle;
  edges dragged near the image's border snap to it, so does the whole
  crop when it moves.

### Plugins and zipp-ui

- Plugins have tables of their own (`Plugin::tables`: SQL that runs
  once each before the migrations, counted per plugin) and pages in the
  studio's sidebar (`PluginStudio::page` with a `PluginPage`, shown to
  who may view its section, the module's `page` renders it).
- zipp-ui: `ContextMenu` opens a menu at the pointer without a ⋯
  button, for the right clicks of a module (`open(e)` in its
  `oncontextmenu`, `MenuItem`s like `Menu`); `Popover` needs no
  `trigger` when it opens at a point.
- zipp-ui: svelte-check of a TypeScript module finds the types of its
  imports (`zipp-ui/Icon.svelte`, `zipp-ui/field`) also without a
  tsconfig (`typesVersions`); docs/extending.md has the tsconfig of a
  module with `"moduleResolution": "bundler"`.

### Cli

- `user --name` sets a user's display name too.

## 0.2.0 (2026-11-07)

Website members and public user profiles, GraphQL schemas as code, more
of Craft's GraphQL, a studio that works next to the live preview, and
the fixes of a review of the whole codebase. New crate: `zipp-members`.

### Upgrading

- GraphQL tokens of the studio read what a GraphQL schema of the code
  says (see below); tokens made before have none and read nothing:
  add a `GraphqlSchema` and make them again. `ZIPP_GRAPHQL_TOKEN`
  reads everything as before.
- `assets` and `assetCount` in GraphQL need a schema with `.assets()`.
- The file url of an edited image is always the edited image,
  `?original` is gone; the uploaded file is
  `GET /api/assets/{id}/original` with a session.
- Videos are the asset kind `video` like in Craft, they were `file`;
  `GET /api/assets?kind=file` lists files without the videos now.
- Asset fields can't be named `folderId`, `folder` or `path` anymore
  (GraphQL fields of assets).
- `zipp-button-group` is gone, the button group is a field kind of
  zipp (`Field::button_group`).
- zipp-forms limits submissions per address only (5 in 10 minutes),
  the cap of 50 per form is gone.
- The search leaves out switched off blocks from an entry's next save
  or the next migration on (it rebuilds the index); until then their
  text is still found.
- Emails of users made before are kept as they are: look for ones with
  several addresses (`,`, `;` or spaces) and fix them, mails still go
  to every address in them.

### Security

- Emptying the trash purges only what the user may delete: a role with
  delete rights on its own entries purged the children of others that
  went to the trash with its page.
- GraphQL's `relatedTo`, `related` and the relation field arguments
  (`offerCategory: 5`, `":notempty:"`, `":empty:"`) only count the
  entries the viewer may see: a token learned through them what an
  unpublished entry, or one outside its sections or sites, points to.
- Switched off blocks (`"enabled": false`, also in rich text) are left
  out of the search and of GraphQL's `chunks`: their text was found and
  shown in excerpts.
- A one-entry preview link checks its user's access each time it is
  used: it showed the draft for 24 hours after the user lost access,
  and of entries in the trash.
- The uploaded file of an edited image is only for the studio (see
  Upgrading): anyone with the url got the editor's crop undone.
- A user's email is checked like an email field: `a@x.ch, b@y.ch` was
  taken, so reset and invite mails went to both.
- Links of the studio's markdown starting with `/\` count as another
  site (browsers open them like `//`).
- A zip upload checks its folder before it is unpacked, and a server
  unpacks one zip at a time: another one meanwhile is a conflict (409),
  the studio sends its zips one after another.
- A transform at the image's size (the edited image) skips the
  full-size resize.
- zipp-seo's root `seo` and `seoJson` are null for a site the token's
  schema doesn't read; plugins check it with
  `graphql::ext::reads_site`.
- zipp-forms: about ten addresses could block a form for everyone with
  the per-form cap (see Upgrading).
- zipp-agent's docs say it is for development and admins only, runs
  with every permission and can be steered by content other users
  write.

### Members and users

- Website members, like Craft users without access to the control
  panel: `Role::no_studio()` keeps a role out of the studio, any user
  logs in on the website (`/site-auth/login`, `me`, `logout`, `reset`,
  `password`; site sessions never open the studio's api). Entries have
  an author (`authorId`, `author` in GraphQL, `?author=`, admins change
  it), `Role::own(section, level)` gives access to a user's own entries.
  The new plugin `zipp-members` lets members list, create, edit and
  delete their own entries and upload images from the frontend, through
  the same checks as the studio. See docs/members.md.
- Public user profiles, like Craft's user fields and WordPress' author
  pages: `.user_fields([...])` (a component `user`, in migrations), a
  unique slug of every user (admins change it), and
  `.user_pages(UserPages::new("cooks/{slug}").with_entries_in("recipes"))`
  for pages of the users with a live entry. GraphQL has
  `users(hasEntries:)`, `userCount` and `user(slug:)` for schemas with
  `.users()`, `author` is a `User` with the profile; the SEO sitemap
  lists the user pages; logged in website users edit their own profile
  with `PUT /site-auth/me`. The studio's user page edits the profile.
- A role can be without Setup, `Role::without_setup()`, like an editor
  in Craft who only writes: no Setup in the sidebar, no links to it, no
  change requests. Roles see it read only as before by default.

### Content model and fields

- The field kind button group, `Field::button_group`: one of some
  options like a dropdown, as buttons like Craft's; GraphQL gives
  `ButtonGroupOption { value label color icon }`. An option can have a
  color, an icon, both or neither: `("pdf", "PDF")`, `("concert",
  "Konzert", "#2b6cb0")` or `ButtonOption::new("pdf", "PDF")
  .icon("file-text")`. The studio shows it as a segmented control: the
  picked option raised in white, a dot in the option's color and its
  icon, the arrow keys move the pick.
- `Field::rich_text(..).toolbar([..])` sets the studio's bar of a rich
  text: which tools, in which order, `|` for a line between them;
  `block:<type>` gives an embedded block an icon of its own, `button`
  a button tool. Without it a rich text with buttons has the button
  tool after the link. Buttons and blocks without a tool go into the
  Insert menu. The schema check refuses unknown tools and tools the
  field doesn't allow.
- A field type can hold blocks with more around each one, like a place
  in a grid: `Field::custom(..).item_blocks(["image", "text"])` makes
  its value a list of items with a block in `block`. zipp checks,
  migrates and indexes those blocks like a blocks field's, the type the
  rest. For its GraphQL `GraphqlType::List` of a plugin's type,
  `graphql::ext::site` and `graphql::ext::block`; its studio module gets
  `blocks` with the block types, `create`, `copy` and `mount`, the
  studio's editor of a block.
- An asset field's folder can be per entry, like Craft's upload
  subpath: `.folder("Topics/{slug}")`, also `{section}` and `{type}`;
  the studio fills them in from the entry being edited (also in its
  blocks), without a slug yet the path ends before it. The picker opens
  that folder before it exists, the first upload makes it.
- `sort-assets` puts assets into the folders of the asset fields that
  use them, `{slug}` and the others filled in like uploads in the
  studio: after an import from Craft, or when a field got a folder.
  Assets of several entries stay where they are.
- Changing an entry's type asks nothing anymore: fields of the same
  handle and kind stay, the type's defaults fill what is empty, and
  while the editor is open each type keeps its values, so switching
  back brings them again. Of a saved entry the change is in the draft:
  Discard or an older revision bring back the old type with every
  field.
- A field that changes its kind keeps plain values the new kind takes
  as they are (text to a dropdown or button group whose options have
  them, a dropdown to a button group); the others start over with the
  default as before.
- A group without a value yet (a new entry) checks the fields it
  requires: a new Mappe of textrem could be saved without its price.
- A change undone deletes the draft: when the draft is the live content
  again, the Draft badge, Discard and Publish go away as if there never
  was one. `PUT /entries/{id}/draft` (and `drafts::put_draft`) answers
  `null` then.
- Content without a revision (imported, or from before revisions were
  kept) is kept as a revision of no one before the first save or
  publish over it, so the original can be restored.

### GraphQL

- GraphQL schemas are code, like Craft's in its project config:
  `Schema::graphql_schema(GraphqlSchema::new("website", "Website")
  .sections(["pages", "news"]).assets().previews())` says which
  sections and sites a frontend reads, whether it lists the assets,
  reads users and shows drafts in previews. A token of the studio gets
  one of them. The GraphQL page shows the schemas and the token of
  `serve` (`ZIPP_GRAPHQL_TOKEN`) as its first row; GraphiQL has a
  schema picker like Craft's that reads as a frontend with one of them
  (`x-zipp-schema: <handle>`, nothing unpublished).
- Several sections (`section: ["news", "pages"]`, an unknown one is an
  error instead of only the first counting), `["not", ...]` for
  sections and types, `level`, `orderBy: "score"`, Craft's `relatedTo`
  and `relatedToEntries`, and `children(...)` with the arguments about
  the children (`limit`, `type`, `orderBy`, ...). Blocks have a `slug`
  of their title field, for anchors.
- The assets of the library like Craft's: `assets` and `assetCount`
  with `folder` (Craft's `volume`), `folderId`, `kind`, `search`,
  `orderBy`, `limit` and `offset`; assets have `folderId`, `folder` and
  `path`. Asset fields take Craft's `kind`: `images(kind: "image")`,
  also a list and `video`, `file`, `pdf` and `word`.
- Images in Craft's sizes: `url(height: 800)` and `url(width: 800,
  height: 450)` (cropped around the focal point), also on `width` and
  `height`, for the sizes of `Schema::image_heights` and
  `Schema::image_sizes` besides `Schema::image_widths`; a size that
  isn't listed is an error naming the allowed ones.
- Craft's `@formatDateTime(format:, timezone:, locale:)` on the dates
  of entries and date fields, PHP's date format, also as field
  arguments; `Schema::timezone` for the time zone of formatted dates.
- A group is always an object, also for an entry saved before it had a
  value, like Craft's content block: a frontend no longer breaks on
  `sale.format` of a new entry.

### Studio

- The editor lays out by its own width (container queries, mixins in
  `vars.scss`), so it fits a phone or a narrow pane next to the live
  preview: the top bar stays one line, the entry's status, dates, type
  and slug are a closed line on top, the rich text toolbar scrolls
  sideways, a block's hide, duplicate and remove and "Copy content from
  …" and Move to the trash go into ⋯ menus.
- The live preview opens as wide as a phone (400px), goes down to 280px
  and up to as wide as the window allows (the editor keeps 20rem). Live
  preview also for a new entry of a section with pages: the button
  saves it unpublished first and opens it with the website.
- Labels on the left of their field (in a group, a block or a card) sit
  at the top of their row with the handle on the name's line; fields
  with a width keep their own row there, the width is their share of
  the value column. Above its field (half width fields, narrow boxes) a
  label has the handle on its line too, for every kind.
- The entries of a relation field are compact chips side by side that
  wrap, like Craft's; they move by dragging or with the arrow keys
  (Move earlier / Move later). A field that takes only one type leaves
  the type out of its chips and block rows.
- The site box at the top of the sidebar shows the website's favicon
  (the site's letter without one; the api answers 204 then), an icon
  next to it opens the website, and it shows that it is a choice when
  there are several sites.
- An old revision opens like the editor, read only, under a banner with
  its date, Restore and Back to editing, on the editor's tab and with a
  dot on the tabs it changed.
- A new block (also a copy, a block in a rich text) stays open after
  saving. Thumbnails of images keep the focal point in view.
- Fixes: plugin fields were editable while publishing, deleting while
  searching counted only the matching children, dragging could go past
  a section's max level, New chat came back after a reload, `/admin/ai`
  opened for roles without the AI chat, a file without an extension
  showed its whole name as its type.

### Craft import

- `import-craft` imports meierlabs-seo's SEO field into zipp-seo's,
  TableMaker tables into table fields (by the order of the columns) and
  the fields of assets into the asset fields; each volume is a folder
  with Craft's folders in it (`--volume handle=folder:dir` names it).
- It keeps a link's url suffix: `#contact` is the link's anchor, `?a=1`
  goes on the end of a url (`export.php` exports it), leaves out
  Craft's temporary uploads without a volume instead of failing on
  them, and records the imported state as the first revision.

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
