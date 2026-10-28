# GraphQL and crelte

How frontends read the content.

## GraphQL

`/api/graphql` is generated from the
schema: every entry type is `<handle>_Entry`, every block
`<handle>_Block`, like Craft. Relations resolve to entries with their
fields, so one query loads a page with everything it shows:
`entry(uri: "about/team", site: "en") { title ... on page_Entry { body
{ ... on button_Block { label link { title uri } } } } }`. `entries`
and `entryCount` take the same filters as the rest api (`where: [{
field: "city", value: "Bern" }]`). Without a session only published
entries show.

## Private GraphQL, dev and prod

Like Craft's schema tokens, GraphQL
only answers frontends that send `Authorization: Bearer <token>`, the
token of `serve` (`ZIPP_GRAPHQL_TOKEN`; crelte reads it as
`ENDPOINT_TOKEN` from `../craft/.env`), and the studio with its
session. `serve --dev` (`ZIPP_DEV=1`) turns on GraphiQL with a GET on
`/api/graphql` and opens GraphQL when there is no token. The rest api
is the studio's, it needs a session.

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
`postDate`, `dateCreated`, `dateUpdated`, `language` and `localized`
(the entry in the other sites), images `focalPoint`, `title` and
`url(width: 800)` for the widths of `.image_widths([400, 800, 1600])`.

## crelte

zipp answers crelte like Craft: `crelteSites`, site ids
(the position of a site, from 1), `entry(uri: $uri, siteId: $siteId)`
with Craft's argument types, `__home__`, `url` on entries and
templates by `<sectionHandle>-<typeHandle>`. The base url of each site
is `--site-url de=https://example.ch`. crelte reads its settings from
`../craft/.env`, see `example/crelte/craft/.env.example`.
