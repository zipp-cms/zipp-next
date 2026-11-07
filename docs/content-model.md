# The content model

The content model is code: a site's crate builds a `Schema` and zipp
makes the api, the studio and the migrations from it. See
`example/cms/src/content.rs` for one that uses everything.

## Sections, entry types and sites

A single has
exactly one entry (created on start), a channel a list and a structure
a tree. A section allows one or more entry types and has a uri format
(`events/{slug}`, `{parent.uri}/{slug}`). Every entry has a row per
site with its own title, slug, uri, published flag and translatable
fields (the slug comes from the title, a title without letters or
digits needs one typed); the other fields are shared by all sites. Like Craft an entry
has a post date (set when it is first published) and an expiry date,
the same in all sites: it is live in a site when it is published there
and the date is between them, the website and GraphQL only show it
then. Nothing is saved when a date passes, `serve` tells the
`on_change` hooks (`ChangeKind::Dated`) within a minute, so a cache can
clear. A block can be switched off (`"enabled": false`): it stays in the
entry, the website leaves it out.

The type of an entry can change to another of its section, like
Craft's: a field with the same handle and kind (and options) keeps its
content, the others are dropped in every site; blocks keep theirs. In
the studio it is a draft change (the editor asks before content goes),
publishing writes the type, its uris and index.

`uri_of("home", "")` gives one entry type another uri (a section
without `uri` and with `uri_of("blog", "blog/{slug}")` gives only blogs a
page), `uri_in("fr",
"offres/{slug}")` one site, `uri_if("hasPage")` gives only entries with
that boolean field on a page, like Craft's `{% if hasPage %}` in a uri
format.

`Section::structure("pages", [..]).max_level(2)` limits how deep a
structure goes, like Craft's max levels (1 is only the top). Creating
and moving refuse an entry, with its children, below it; the studio
offers no such place. An entry from the trash whose parent is too deep
now comes back on the top. Changing it is a migration that leaves
deeper entries where they are, only new places are checked.

## Entry types and blocks

Both are components made of fields: text,
number, boolean, date (`Field::date("date")`, a day `2026-11-14`;
`.time()` a day and time `2026-11-14T19:30`, as the editor sees it,
without a time zone, it sorts and filters as text), relation (ids of
entries) and blocks (a list of blocks, nested as deep as needed).
A number (`Field::number("price")`) takes `.range(0.0, 1000.0)`,
`.decimals(2)`, at most that many places after the point (`0` for whole
numbers; the server refuses more, the studio shows them, `12.50`), and
`.unit("CHF")`, shown after the input. GraphQL gives numbers as `Float`,
also with `decimals(0)`.
A relation (`Field::relation("speakers", ["person"])`) takes entries
of its types, `.sections(["team"])` only those of some sections; links
to entries take it too (a link field, a navigation, the links and
buttons of a rich text), `.sections(["pages"])` for a menu only of
pages. The studio's picker shows only these sections, saving refuses
an entry of another one. The schema check refuses an unknown section,
one without pages for links, and a relation whose types and sections
don't fit (a type in none of them, a section with none of the types).
Like the types, a change of the sections is a migration; entries
already chosen that it no longer allows stay, and saving refuses them
until an editor removes them (`makemigration` says so).
`Field::email("formEmail")` takes one address, `name@host.tld` without
spaces (the server refuses anything else, the same check as email links
and table columns), a `String` in GraphQL.
`.default(true)`, `.default("Mehr")` or a dropdown's value is what a new
entry or block starts with: the studio fills it in, an entry created
through the api without the field gets it, and a single starts with it.
The schema check refuses a default the field wouldn't take; only fields
of plain values (text, email, number, boolean, date, markdown, dropdown,
button group, checkboxes, a plugin's) have one. Existing content keeps what it has,
but a migration that adds a field with a default writes it into the
entries, drafts and revisions of before, so a new boolean that is on is
on for old content too. Changing a default later needs no migration.
`.label("…")` names a field, `.instructions("…")` tells editors what
to put there, under the label in the studio (markdown: bold, links);
the AI chat reads them too. Both are only looks, no migration.
`.width(50)` gives a field half of a row in the editor (25, 50, 75 or
100 percent, 100 without it), like Craft's field widths: date, doors and
city of an event sit in one row. A narrow editor (a phone, the flyout,
a small block) shows every field full width; inside a block a field
that shares its row has its label above. Also only looks.
`Field::note("imageHint", "**Bild** im Querformat …")` is a note for
editors in the field layout, like Craft's tip: its markdown text from
the code shows in a grey box, with a width too. It holds nothing: the
server refuses a value for it, GraphQL and the TypeScript types don't
have it, and adding, changing or removing a note needs no migration.
Its handle places it in tabs like a field's.
`.when(Condition::on("hasPage"))` shows a field only while another
field of the same entry type or block has a value, like Craft's field
layout conditions: `Condition::on`/`off` for a boolean (one never set
is off), `Condition::is("genre", "concert")` or `one_of("genre",
["concert", "reading"])` for a dropdown, a button group or a plugin's
field, `Condition::filled("note")`/`empty` for any field but a
group. The studio hides and shows it live while editing, also in blocks
and groups, and a field hanging on a hidden one hides too. A hidden
field keeps its value (GraphQL returns it as stored, like Craft) and
isn't required: neither it nor the fields and blocks inside it need to
be filled, other mistakes in its value still count. The schema check
refuses a condition on an unknown field, on the field itself, on a value
the field doesn't take, and conditions that go in a circle. A condition
is only looks, changing it needs no migration; Setup shows it.

`entry_tabs("page", "Page", [("Content", vec![...]), ("SEO",
vec![seo_field("seo")])])` puts the fields of an entry type in tabs of
the editor, like Craft's field layout; the title is in the first tab. Tabs
are only looks: the fields are one list, moving a field to another tab
needs no migration. Setup shows each type's editor as editors see it.
`.tab_when("page", "Page", Condition::on("hasPage"))` after the type
shows a tab only while the condition holds, for all its fields; it
can't look at a field of the tab itself.
`.tab_card("article", "Intro")` shows a tab's fields in one card with
the labels on the left, like a group, `.entry_card("event")` all tabs of
a type or its one page without tabs; the title stays above. Only looks
too, no migration.
`block_tabs("teaser", "Teaser", [("Content", vec![...]), ("Settings",
vec![...])])` does the same for a big block: an open block shows a small
tab bar on top and the fields of one tab, the studio keeps the chosen tab
per block in the browser. `.tab_when` works for its tabs too, `.tab_card`
not (a block is a card already). A group of a block with tabs and a
GraphQL query see all its fields as usual.

`.icon("event", "calendar")` gives an entry type or a block an icon of
[Lucide](https://lucide.dev/icons) by its name, `.icon("logo",
Icon::svg(include_str!("logo.svg")))` one of its own (drawn with
`currentColor`); `.nav_icon("Content", "folder-open")` one of the
studio's nav groups. The studio shows them in lists, menus and the
blocks' heads; they are only looks like the tabs.

`.block_title("feedback", "author")` names a block by one of its fields,
like Craft shows a nested entry: its head and the outline show
`Feedback · Anna Muster`, live while typing. Text, email, number, date,
dropdown and button group (its label), rich text, markdown, link (its label or target) and
relation (the entry's title) fields can give it, the check refuses others.
Without one, or while it is empty, a block shows its first text or link.
GraphQL gives a block a `slug` of the text or number of that field, for
anchors.

## Content is JSON

An entry's fields are stored as one JSON document,
a block is `{ "id", "type", "fields" }`.

## Queries go through index tables

Fields marked `indexed()` are
copied to `entry_values` on every save, every relation (also inside
blocks) to `entry_relations`. So entries can be filtered and sorted by
them and you can ask what links to an entry:
`/api/entries?type=event&f.city=Bern&f.date.gte=2026-11-01&order=date`,
`/api/entries?related=5`, `/api/entries?site=en&uri=about/team`,
`/api/entries?section=pages&order=structure` (the tree; with
`parent=5` the children in their order, `level=1` the top entries),
`/api/entries?section=pages,news&relatedTo=5` (both sections, linking
to 5 or linked from it). See
`crates/zipp/src/query.rs`.

## Rich text

`Field::rich_text("text").headings([3]).lists().links()
.buttons().embed(["image"])` says what an editor may contain: bold and
italic always, the rest only if the field allows it, blocks of the
schema as boxes in the text. It is edited with TipTap and stored as its
JSON; publishing removes empty paragraphs, `&nbsp;` and trailing line
breaks, so frontends need no margin hacks. GraphQL has `html` and
`chunks` (`Html`, `Buttons` with a row of links, the blocks): each
button is a node of its own in the text, the ones next to each other
arrive as one row. Links in the text point to entries by id.
`.styles([Style::block("lead", "Lead"), Style::inline("highlight",
"Highlight")])` gives editors a Styles menu like CKEditor's: a block
style is a class on a paragraph or heading (`attrs.class`), an inline
style a mark `style` with the class on a part of the text. `html` and
the chunks render them as `<p class="lead">` and `<span
class="highlight">`, the frontend's CSS gives their look; the studio
shows every style alike (a line on the left, a tint). Unknown classes
are refused; a removed style stays in the content until it is saved
again, which refuses it, like a removed heading level.

The studio's bar over the text shows what the field allows: Format
(the headings), Styles, bold, italic, the lists, link, a line and a button
icon, then an Insert menu with the embedded blocks. `.toolbar(["format",
"|", "bold", "italic", "|", "link", "button", "block:image",
"insert"])` sets it instead: the tools in order, `|` a line between
them. The tools are `format`, `styles`, `bold`, `italic`,
`bulletList`, `orderedList`, `link`, `button`, `block:<type>` (an icon
that inserts a block of an embedded type, the block's icon) and
`insert` (the Insert menu). Buttons and blocks without a tool of their
own go into the Insert menu, at the end of the bar unless `insert`
places it; with nothing left there is none. The check refuses unknown
tools, tools the field doesn't allow (`button` without `.buttons()`,
a block it doesn't embed), a tool twice, a line at an end or next to
another and an `insert` with nothing left. Shortcuts like Ctrl+B work
without the tool in the bar.

## Groups and markdown

`Field::group("seo", "seo")` keeps the fields
of another component together under one name (an object, not a list
like blocks), in a box of the editor. Fields that are used only once
don't need a component: `Field::group_of("tickets", [Field::number(
"price"), Field::boolean("soldOut")])` lists them right there, the
schema makes them a block named after the entry type and the field,
`eventTickets` (GraphQL `eventTickets_Fields`; an inline group `sale`
in it becomes `eventTicketsSale`), with the field's label as name. Setup shows
it under its entry type, not with the blocks; a block of the same name
is refused. Indexed fields stay at the top of an entry type, only those
are in `entry_values`. `Field::markdown("text")` is shown rendered in the studio
and comes as `markdown` and `html` from GraphQL.

## Dropdowns

`Field::dropdown("width", [("small", "Small"), ...])`
stores the value of one option, GraphQL gives that value like Craft.
The studio shows up to three options as buttons, more as a select.
Dropdowns can be indexed.

`Field::button_group("genre", [("concert", "Konzert", "#2b6cb0"),
...])` is a dropdown as buttons, like Craft's button group. An option
can have a color (`#rrggbb`), an icon, both or neither: `("pdf",
"PDF")` has none, `ButtonOption::new("pdf", "PDF").icon("file-text")`
an icon (a Lucide name or `Icon::svg`), `.color("#2b6cb0")` a color.
The studio shows the options as a segmented control (the picked one
raised in white), each with a dot in its color and its icon, the
content stores the value, GraphQL gives the option as
`ButtonGroupOption { value label color icon }` (none for an option
removed since; `color` and `icon` null if it has none, `icon` is the
Lucide name, an own svg is only the studio's), so the website shows the
label in its color or with its icon. It takes a default, conditions and
`.indexed()` like a dropdown.

## Checkboxes, tables and navigations

`Field::checkboxes` stores some
of its options (GraphQL: a list of values). `Field::table("links",
[Column::text("label"), Column::url("url")])` stores rows, GraphQL has a
row type per table (`<component>_<field>_Row`). A column is like the
field of its kind, small enough for a row: `Column::text`, `url`,
`email`, `number("price").decimals(2).unit("CHF")` (a `Float`),
`boolean("vegan")` (a switch, `Boolean!`, no is stored as nothing),
`date("from")` (a day), `dropdown("size", [("s", "Small")])` (the value)
and `link("more", [LinkKind::Entry, LinkKind::Url])` (a link without a
label, GraphQL's `Link`, a relation). Empty rows are left out, the
search finds the text columns. A column that gets another kind loses
its cells in the migration, like a field. `Field::navigation("main", [LinkKind::Entry,
LinkKind::Url]).depth(2)` is a menu: links with their own label and the
links under them, GraphQL gives `{ url label defaultLabel target
download children }`; links to entries count as relations.

## Assets and transforms

Uploads live in `--files`, `Field::images`
and `Field::assets` link them. Transforms are named in code
(`Transform::new("hero").width(1600).height(700).crop()`), generated
on their first request and cached; crops keep the focal point editors
set in the studio. GraphQL asks like Craft: `image { url(transform:
"hero") width(transform: "hero") srcset(transforms: ["small", "hero"])
alt }`. Only named transforms and the sizes the schema lists exist,
so nobody can make the server render arbitrary sizes: Craft's
`url(width: 800)`, `url(height: 800)` and `url(width: 800, height:
450)` work for `.image_widths([800, 1600])`, `.image_heights([800])`
and `.image_sizes([(800, 450)])`; width and height crop around the
focal point like Craft's default mode. Their names are `w800`, `h800`
and `w800h450` (for `srcset(transforms: [...])`); a size that isn't
listed is an error naming the allowed ones. `--public-url` makes the urls absolute for
frontends on other domains. Like in Craft an asset has a title (GraphQL
`title`, from the filename while it is empty) and an alt text per site
(the studio edits the one of the site it shows; GraphQL gives the one
of the site asked, else the primary site's).

A transform can make another type, like Craft's transform format:
`.format(Format::Webp)` (also `Jpeg` and `Png`). The type is in the
url's extension, `/assets/5/hero/piano.webp`; the image's own name
gives its own type, so `url(transform: "hero", format: "webp")` and
`srcset(transforms: [...], format: "webp")` ask for any transform in
another type. WebP is lossy (libwebp at quality 80), 15 to 35% smaller
than the jpegs at 82 on photos; transparency stays, a jpeg of a
transparent image gets white. No AVIF yet: rav1e takes seconds per
image. There is no `Accept` negotiation either, the url says the type,
so caches and CDNs need no `Vary`.

Asset urls carry a version (`?v=…`) that changes with the edits, the
focal point and a replaced file, also with the same name, so browsers
and CDNs never show the image from before; transforms are kept under
their version too.

In the studio assets are in folders (only there, urls don't change). An
asset field can say where its uploads go and its picker opens,
`.folder("Events/Programs")` (made on the first upload), and which
files it takes, `.extensions(["pdf"])`; saving checks it. A folder per
entry like Craft's upload subpath: `.folder("Topics/{slug}")`, with
`{slug}`, `{section}` and `{type}` of the entry being edited (also for a
field in its blocks and groups; others are an error). The studio fills
them in at each upload, so a renamed slug leaves the files where they
are and only new ones go into the new folder. While the entry has no
slug yet, the path ends before it: `Topics`, or the top of the library
for `{slug}` alone. A folder's ⋯
downloads it with its subfolders as a zip (`GET /api/assets/zip`, the
files as uploaded, nothing from the trash), and a zip uploaded in the
library can be unpacked into folders (`POST /api/assets/unzip`: folders
of the same name merge, hidden files and `__MACOSX` stay out, at most
2000 files, 500 folders 20 deep and 1 GB unpacked; folder names have no
`/` or `\`). The library filters by type: images,
videos or documents (`GET /api/assets?kind=video`). Frontends list the
assets of a folder through GraphQL, `assets(folder: "Partners")`.

Assets can have fields of their own beside title and alt, like Craft's
asset field layout: `.asset_fields([Field::text("credit"),
Field::dropdown("licence", [...])])`. They are the component `asset`,
so they are checked like a block's fields, take part in migrations
(`makemigration --rename asset.credit=photographer` renames the key in
every asset) and are edited in the asset panel; GraphQL has them on
`Asset` (`image { url credit }`), `zipp types` writes `AssetFields`.
Their values are the same in all sites (unlike the alt text), they have
no defaults and can't be indexed; their relations don't count as the
usage of an asset or entry.

Videos are a kind of their own like in Craft: `Field::videos("clip")`
takes mp4, webm and mov (`.extensions` of those), GraphQL's `kind` is
`video` and `mimeType` `video/mp4`, so a frontend knows to show a
`<video>`. The studio plays
them in the asset panel and shows their first frame and length in the
library. The files are served with ranges, so a video seeks without
loading all of it. The server makes no posters; a frontend takes the
first frame or an image field next to the video (the example's video
block has one).

## Translations

The texts of the frontend are keys in the code with
the text of the primary site: `.translations("common", [("menu.open",
"Menü öffnen")])`. They are a single `translations` with a texts field
per namespace, so editors translate them per site with drafts,
revisions and the preview like any entry (put `translations` into a
nav group); an empty text is the code's.
crelte's translations plugin gets them with `translations:
translations(siteId: $siteId) { common }` in `global.graphql`.

## Docs

`.doc("readme", "Readme", include_str!("../readme.md"))`
shows a markdown file of the repository as a read-only page of the
studio; put its handle into a nav group like a section.

## Offline sites

`.offline("fr")` keeps a site off the website
like a disabled site in Craft: frontends get none of its entries and it
is not in `crelteSites` or the sitemaps; editors fill it in, previews and
admins see it. The preview link of one entry shows only that entry of an
offline site, not the rest of it.

## Designing a content model

What an editor sees is the field list, so it should read like the page:

- Fields that belong together go into a group: an inline group
  (`Field::group_of`) when they are used once, a block component and
  `Field::group` when several types share them (an intro, SEO). A
  bordered box with a name is easier to read than a long flat list.
- Big entry types get tabs (`entry_tabs`), e.g. Content and SEO; a big
  block too (`block_tabs`), e.g. Content and Layout.
- A tab (or a type without tabs) of several short fields goes into a
  card (`.tab_card`, `.entry_card`): a column of full-width fields with
  their labels above reads badly, rows with the labels on the left like
  a group read well. A tab of blocks stays without one.
- Short fields share a row with `.width(25)` or `.width(50)`: a date,
  a city, a price, a switch.
- Labels in the editors' language, `.instructions("…")` where a field
  needs an explanation, `Field::note` for a how-to that concerns
  several fields (image sizes, what shows where).
- Fields the website filters or sorts by stay at the top with
  `.indexed()`; they can't be in a group.

The example's event and pages show it.

## Roles

Like Craft Pro's user groups, roles are code:
`.role(Role::new("author", "Author").section("events", Access::Edit)
.section("pages", Access::View).assets(Access::Edit))`, with View <
Edit (drafts, new entries unpublished) < Publish (also moving in a
structure) < Delete per section and for assets, and `.ai()` for the
chat. Which user has which role is in the database (`user <email>
--role author`, or the Users page of an admin); a user without a role
is an admin. The server checks every request, the AI can only do what
its user can, the studio hides the rest.

Entries have an author, like Craft's: the user who created them (in the
studio, the api or the AI chat), none for the command line, imports and
a deleted user. Drafts and revisions keep it; admins change it in the
entry's sidebar, others see it there. `.own("recipes",
Access::Delete)` gives a role access to the entries its user made, on
top of `.section`: with only `.own` the user sees and edits their own
recipes and no others (new ones are theirs, so `Edit` creates), with
`.section("recipes", Access::View)` too they read the others, like
Craft's "view other authors' entries". Every request about an entry
checks it (drafts, publishing, moving, the trash, revisions, the AI's
tools), lists, pickers and the search show only what the user may see,
and the author can't be changed by such a role. `/api/entries?author=4`
lists what user 4 made. A relation or link to an
entry of a section the role can't view (or to an asset without asset
access) is left out on save like one to a deleted entry, unless the
entry linked to it before. Only admins see drafts in GraphQL with their
session. `.no_studio()` makes a role of members who only log in on the
website, see [Website users](members.md).
