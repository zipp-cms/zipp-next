# The content model

The content model is code: a site's crate builds a `Schema` and zipp
makes the api, the studio and the migrations from it. See
`example/cms/src/main.rs` for one that uses everything.

## Sections, entry types and sites

A single has
exactly one entry (created on start), a channel a list and a structure
a tree. A section allows one or more entry types and has a uri format
(`events/{slug}`, `{parent.uri}/{slug}`). Every entry has a row per
site with its own title, slug, uri, published flag and translatable
fields; the other fields are shared by all sites.

`uri_of("home", "")` gives one entry type another uri, `uri_in("fr",
"offres/{slug}")` one site, `uri_if("hasPage")` gives only entries with
that boolean field on a page, like Craft's `{% if hasPage %}` in a uri
format.

## Entry types and blocks

Both are components made of fields: text,
number, boolean, relation (ids of entries) and blocks (a list of
blocks, nested as deep as needed).

`entry_tabs("page", "Page", [("Content", vec![...]), ("SEO",
vec![seo_field("seo")])])` puts the fields of an entry type in tabs of
the editor, like Craft's field layout; the title is in the first tab. Tabs
are only looks: the fields are one list, moving a field to another tab
needs no migration. Setup shows each type's editor as editors see it.

`.icon("event", "calendar")` gives an entry type or a block an icon of
[Lucide](https://lucide.dev/icons) by its name, `.icon("logo",
Icon::svg(include_str!("logo.svg")))` one of its own (drawn with
`currentColor`); `.nav_icon("Content", "folder-open")` one of the
studio's nav groups. The studio shows them in lists, menus and the
blocks' heads; they are only looks like the tabs.

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
`/api/entries?section=pages&order=structure`. See
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

## Groups and markdown

`Field::group("seo", "seo")` keeps the fields
of another component together under one name (an object, not a list
like blocks). `Field::markdown("text")` is shown rendered in the studio
and comes as `markdown` and `html` from GraphQL.

## Dropdowns

`Field::dropdown("width", [("small", "Small"), ...])`
stores the value of one option, GraphQL gives that value like Craft.
The studio shows up to three options as buttons, more as a select.
Dropdowns can be indexed.

## Checkboxes, tables and navigations

`Field::checkboxes` stores some
of its options (GraphQL: a list of values). `Field::table("links",
[Column::text("label"), Column::url("url")])` stores rows, GraphQL has a
row type per table. `Field::navigation("main", [LinkKind::Entry,
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
alt }`. Only named transforms exist, so nobody can make the server
render arbitrary sizes. `--public-url` makes the urls absolute for
frontends on other domains. Like in Craft an asset has a title (GraphQL
`title`, from the filename while it is empty) and an alt text.

In the studio assets are in folders (only there, urls don't change). An
asset field can say where its uploads go and its picker opens,
`.folder("Events/Programs")` (made on the first upload), and which
files it takes, `.extensions(["pdf"])`; saving checks it.

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
admins see it.

## Roles

Like Craft Pro's user groups, roles are code:
`.role(Role::new("author", "Author").section("events", Access::Edit)
.section("pages", Access::View).assets(Access::Edit))`, with View <
Edit (drafts, new entries unpublished) < Publish (also moving in a
structure) < Delete per section and for assets, and `.ai()` for the
chat. Which user has which role is in the database (`user <email>
--role author`, or the Users page of an admin); a user without a role
is an admin. The server checks every request, the AI can only do what
its user can, the studio hides the rest. Only admins see drafts in
GraphQL with their session.
