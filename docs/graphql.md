# GraphQL and crelte

How frontends read the content.

## GraphQL

`/api/graphql` is generated from the
schema: every entry type is `<handle>_Entry`, every block
`<handle>_Block`, like Craft. Relations resolve to entries with their
fields, so one query loads a page with everything it shows:
`entry(uri: "about/team", site: "en") { title ... on page_Entry {
content { ... on images_Block { images { url alt } } } } }`. `entries`
and `entryCount` take the same filters as the rest api (`where: [{
field: "city", value: "Bern" }]`). Without a session only live entries
show: published, after their post date and before their expiry date;
switched off blocks never.
Values are typed like their fields: numbers `Float`, booleans
`Boolean`, links `Link` (`url label defaultLabel target`), also in the
cells of a table row; rich text has `html` and `chunks`, its styles as
`class="…"`.

A query is limited, so one request can't stall the cms: at most 15
deep, a complexity of 2000, and at most 10000 loaded entries and assets
(each list, relation and `children` counts as one more); past it the
answer is an error. A list takes at most 1000 entries (`limit`), an
entry's `children` are the first 500, `id` and `ids` at most 1000 ids.
Pages of real sites load a few hundred.

The website's search is full text like Craft's `search`, over the same
index as the studio's (the live texts of entries, not drafts): every
word the start of a word in the title, slug or texts, accents and
umlauts folded (`konz` finds *Konzert*, `zurich` *Zürich*), signs like
`"`, `*` or `OR` are no syntax. A single letter only finds that word
and only the first five words count, so a search stays cheap for
anyone on the website (see "The website" in docs/operations.md). `entries(search: "konzert")` and
`entryCount(search:)` take it with the other filters, the best match
first unless `orderBy` asks for another order; the title counts most,
then the slug. A search page asks
`search(query: $q, siteId: $siteId, section: "news", limit: 10, offset: 10)
{ excerpt excerptHtml score entry { title url } }`: only entries with a
url, `excerpt` the words around the match (the start of the texts when
only the title matched), `excerptHtml` the same escaped with the
matched words in `<mark>`; `searchCount` with the same arguments counts
the hits for the pages. What a viewer may see counts like everywhere
(published, the token's sections and sites, a preview's drafts).

Relations, links, parents, `children`, `localized` and assets load
together per request: the images of 30 events are one query, not 30,
and an entry that many link to loads once. `entries` doesn't count,
only `entryCount` does. A test of the example
(`pages_load_in_a_few_statements`) keeps its page queries at a few
statements, `TestServer::statements` counts them.

## Private GraphQL, dev and prod

Like Craft's schema tokens, GraphQL
only answers frontends that send `Authorization: Bearer <token>` (crelte
reads it as `ENDPOINT_TOKEN` from `../craft/.env`), and the studio with
its session. An unknown, revoked or expired token gets 401 like none.
The rest api is the studio's, it needs a session.

## Tokens

Admins make the tokens of frontends in the studio
under GraphQL, like Craft's GraphQL schemas: a name, the sections and
sites it reads (all or some), whether preview links show drafts with it
(`?token=` of a preview, for the website that shows the studio's
previews; without it they show what is live), an optional expiry. The
token shows once when it is made, zipp keeps only its hash; the list
shows when each was used last, a token is revoked there. The token of
`serve --graphql-token` (`ZIPP_GRAPHQL_TOKEN`) reads everything, like
before; an empty one is refused at the start.

A token reads only its sections and sites: `entries`, `entry`,
`entryCount`, `globalSet`, `translations`, relations and links leave out
the other entries, `sites` and `crelteSites` the other sites (the ids
stay). Plugins load entries the same way: the SEO plugin's `seoJson`
reads its settings from the section `seoSettings`, a token without it
gets the defaults. Assets aren't scoped, their files are public by
url: a token gets the assets of the entries it reads, and `assets`
lists the library for a site it reads.

## GraphiQL

A GET on `/api/graphql` opens GraphiQL. Its queries send the
session of the studio, so an admin sees drafts and unpublished entries
like in the studio (Try it on the GraphQL page opens it); without a
session it goes to the login. In development (`serve --dev`,
`ZIPP_DEV=1`) GraphQL is open without a token, unless `serve` has one;
a wrong token gets 401 there too and a token reads only its scope.
GraphiQL is bundled with the studio (`studio/graphiql.html`, its own
page of the build) and has the studio's CSP, it loads nothing from other
servers.

## Craft's arguments and fields

Queries take `id` (one, several or
`["not", 1, 2]`), `section` and `type` (one, several or `["not",
"wikiList"]`, an unknown one is an error), `level` (of a structure, 1
the top entries) and `orderBy` (`postDate DESC`, `title`, an indexed
field, `score` the best match of `search` and without one ignored like
in Craft); without an order a channel is newest first and a structure
in its order, like Craft. Every relation field
of the entry types is an argument too: `entries(section: "offers",
offerCategory: 5)` the entries pointing to 5, `offerCategory:
":notempty:"` to anything, `":empty:"` to nothing. `relatedTo: [1, 2]`
are the entries that point to 1 or 2 or that they point to, in any
field, `["and", 1, 2]` to both; `relatedToEntries: [{ section: "wiki"
}]` the same with the entries of criteria (the arguments of `entries`
without site, order and page, in the site of the query and only what
the viewer may see, without another `relatedToEntries`), each criteria
of the list must hold, like Craft's.
`children` takes the arguments of `entries` that are about the
children (`type`, `id`, `slug`, `relatedTo`, `relatedToEntries`,
relation fields, `where`, `orderBy`, `limit` and `offset` per parent):
`children(relatedToEntries: [{ section: "wiki" }]) { title }`. A
block has a `slug` for anchors, like a nested entry in Craft: of the
text of its title field (`.block_title`), none without one or when the
block has a field named so. Image transforms may
have Craft's names like `land_w800`. Ids of entries and
assets are strings (`ID`) like Craft's. Entries have
`postDate`, `expiryDate`, `dateCreated`, `dateUpdated`, `language`,
`siteHandle` and `localized`, the entry in the other sites for a
language switch (`localized { siteId siteHandle language title uri url
}`): only where it is live and the request may read it, like any query
of that site (published, no offline site for frontends, the sites of
the token; a preview shows its entry in every site). Images have
`focalPoint`, `title` and Craft's `url(width: 800)`, `url(height:
800)` and `url(width: 800, height: 450)` (cropped around the focal
point; also on `width` and `height`) for the sizes the schema lists,
`.image_widths([400, 800])`, `.image_heights([800])` and
`.image_sizes([(800, 450)])` (`w800`, `h800`, `w800h450` in
`srcset(transforms: ["w400", "w800"])`); Craft's `mode` and
`position` aren't there,
`url(transform: "hero", format: "webp")` and `srcset(transforms: [...],
format: "webp")` in another type (`webp`, `jpg`, `png`), and `alt` of
the site asked, else of the primary site.
The dates (`postDate`, `expiryDate`, `dateCreated`, `dateUpdated` and
date fields) take Craft's `@formatDateTime(format: "j. F Y", timezone:
"Europe/Zurich", locale: "de-CH")`, or the same as arguments,
`postDate(format: "d.m.Y")`: async-graphql's dynamic schema has no
directives of its own, so zipp moves the directive's arguments to the
field before it checks the query. `format` is PHP's `date()` format
like Craft's (`d D j l N S w z W F m M n t L o Y y a A g G h H i s v
u e T O P p Z c r U`, `\` keeps a letter; Craft's `short`, `long` aren't
there), without one Craft's `2026-10-28T09:00:00+01:00`; a date
without any argument stays as it is, in UTC. Months and days are
German, French, Italian or English, the language of `locale` or else of
the site asked. The time zone is `timezone`, else the schema's
`.timezone("Europe/Zurich")`, else UTC; a date field is a time in the
schema's time zone, like editors entered it.

Every asset has `kind` (`image`, `video` or `file`), `mimeType`
(`video/mp4`) and `size`, the schema's asset fields (`image { url
credit }`), `folderId`, `folder` (the path of its folder,
`Timetables/2025`, null on the top) and Craft's `path`, the one in its
volume: below its top folder, `2025/1a.pdf`. An asset field takes
Craft's `kind`, `images(kind: "image")` or `media(kind: ["image",
"video"])`, also `file` (neither), `pdf` and `word`.

The library is `assets` like Craft's, `assetCount` with the same
arguments: `assets(folder: "Clubs", kind: "image", orderBy: "title")
{ title url alt }`. Craft's `volume` is `folder` (the import makes a
folder of each volume): a path, any case, with the folders in it,
several in a list; `folderId` only the folder itself. `search` matches
every word in the filename, title or alt text; `orderBy` is `title`,
`filename`, `dateCreated`, `size` or `id` with `ASC` or `DESC`, the
newest first without; `limit` and `offset` page it. `site` or `siteId`
is the site of the alt texts, the primary site without. Each asset
counts against the 10000 of a request.

## crelte

zipp answers crelte like Craft: `crelteSites`, site ids
(the position of a site, from 1), `entry(uri: $uri, siteId: $siteId)`
with Craft's argument types, `__home__`, `url` on entries and
templates by `<sectionHandle>-<typeHandle>`. The base url of each site
is `--site-url de=https://example.ch`. crelte reads its settings from
`../craft/.env`, see `example/crelte/craft/.env.example`.
