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
before.

A token reads only its sections and sites: `entries`, `entry`,
`entryCount`, `globalSet`, `translations`, relations and links leave out
the other entries, `sites` and `crelteSites` the other sites (the ids
stay). Plugins load entries the same way: the SEO plugin's `seoJson`
reads its settings from the section `seoSettings`, a token without it
gets the defaults. Assets aren't scoped, a token gets the assets of the
entries it reads.

## GraphiQL

A GET on `/api/graphql` opens GraphiQL. Its queries send the
session of the studio, so an admin sees drafts and unpublished entries
like in the studio (Try it on the GraphQL page opens it); without a
session it goes to the login. In development (`serve --dev`,
`ZIPP_DEV=1`) GraphQL is open without a token, unless `serve` has one;
a wrong token gets 401 there too and a token reads only its scope.
GraphiQL loads its scripts from unpkg.com.

## Craft's arguments and fields

Queries take `id` (one, several or
`["not", 1, 2]`), `type` (one or several) and `orderBy` (`postDate
DESC`, `title`, an indexed field); without an order a channel is newest
first and a structure in its order, like Craft. Every relation field
of the entry types is an argument too: `entries(section: "offers",
offerCategory: 5)` the entries pointing to 5, `offerCategory:
":notempty:"` to anything, `":empty:"` to nothing. Image transforms may
have Craft's names like `land_w800`. Ids of entries and
assets are strings (`ID`) like Craft's. Entries have
`postDate`, `expiryDate`, `dateCreated`, `dateUpdated`, `language`,
`siteHandle` and `localized`, the entry in the other sites for a
language switch (`localized { siteId siteHandle language title uri url
}`): only where it is live and the request may read it, like any query
of that site (published, no offline site for frontends, the sites of
the token; a preview shows its entry in every site). Images have
`focalPoint`, `title` and `url(width: 800)` for the widths of `.image_widths([400, 800, 1600])`,
`url(transform: "hero", format: "webp")` and `srcset(transforms: [...],
format: "webp")` in another type (`webp`, `jpg`, `png`), and `alt` of
the site asked, else of the primary site.
Every asset has `kind` (`image` or `file`), `mimeType` (`video/mp4`
for a video) and `size`, and the schema's asset fields
(`image { url credit }`).

## crelte

zipp answers crelte like Craft: `crelteSites`, site ids
(the position of a site, from 1), `entry(uri: $uri, siteId: $siteId)`
with Craft's argument types, `__home__`, `url` on entries and
templates by `<sectionHandle>-<typeHandle>`. The base url of each site
is `--site-url de=https://example.ch`. crelte reads its settings from
`../craft/.env`, see `example/crelte/craft/.env.example`.
