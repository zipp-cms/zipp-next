# Welcome to the zipp demo

zipp is a headless CMS where the **content model is code**. This studio, the GraphQL API and the migrations are all made from one Rust file: [`example/cms/src/main.rs`](https://github.com/zipp-cms/zipp-next/blob/code-first/example/cms/src/main.rs). The website is a separate crelte project that reads the content over GraphQL.

- **Website (crelte):** <https://zipp-crelte.noah.meierlabs.dev> — and `/en` for English
- **GraphQL** is private like Craft's: frontends send a token, the playground (GraphiQL) is only on in development (`serve --dev`)

This page is itself a feature: a markdown file of the repository ([`example/readme.md`](https://github.com/zipp-cms/zipp-next/blob/code-first/example/readme.md)), shown in the navigation with `.doc("readme", "Readme", include_str!("../../readme.md"))`. Editors read it, the code changes it.

## Where to try what

| Feature | Where in the studio | What to look for |
|---|---|---|
| Sections: single, channel, structure | Settings → Header, Footer · News, Events · Pages, Categories | a single opens its entry, a channel is a list, a structure a tree |
| Like the crelte tutorial | Pages, News, Categories | pages and articles start with the **Intro** tab (a rich text intro, an image), the blocks are in **Content**; the news page on the website lists the articles and filters them by category (`?category=`) |
| The home page in the tree | Pages → Willkommen | the entry type `home` gets the uri `/` (`uri_of`) |
| Drag and drop | Pages | drag the handle before a title: before, after or into a page |
| Sites and languages | the language switch top right | titles, slugs and text per site; fields marked *all sites* are shared |
| Status per site | every list | `DE EN`: a badge per site, grey where it is not published |
| Drafts | change any field | the orange *Unpublished changes*, the website stays as it is until **Publish** |
| Preview | an entry with a draft → **Preview** | the website with your draft, for a day |
| What changed | the orange dots | fields that differ from the live version, also inside blocks |
| Revisions | Options → Revisions | what each one changed; click to look, **Restore** to go back |
| Two editors at once | log in twice | the second one is told instead of overwriting the first |
| Blocks | Pages → Über uns → Content | text, images, button, upcoming events |
| Rich text | the Intro tab · the Text block | the intro only with links; the block also with headings, lists, **buttons** (next to each other one row) and **images** inside the text |
| Links | a Button block | entry, URL, email, phone or file; external ones open in a new tab |
| Group | a page → SEO | fields of another component under one name |
| Markdown | Events → Description | written as markdown, the website gets it as html |
| Navigation, table, checkboxes | Settings → Header, Footer · Events → Für wen | a menu with a second level, footer links as rows, some of the options |
| Dropdown | Images block → Width (3 options: buttons), Events → Sprache (5: a select) | the website sizes the image, filters events by language |
| A field type of another crate | Events → Genre | `crates/zipp-button-group`: Craft's button group with a color per option, its own editor and GraphQL type |
| Boolean, number, indexed fields | Events | *sold out*, price; the website filters and sorts by them |
| Date, date and time | Events → Date, Doors open | the date picker; a day or a day and time, the website sorts events by the date |
| Relations | News → Categories, Events → More info, Header → navigation | the website gets the linked entries with their fields, and filters by them |
| Images and transforms | Assets | click an image to set the focal point; crops follow it |
| Folders, file types | Assets · Events → Programm (PDF) | folders for the library; the event's program takes only PDFs and puts them into Events/Programs, its image picker opens in Events |
| Icons | the navigation, Pages, a page's blocks | entry types, blocks and nav groups with Lucide icons from the code |
| Globals | Settings → Header, Footer | singles without a url, crelte reads them like Craft's globals |
| Translations | Settings → Translations | the keys come from the code, editors only translate |
| SEO | Settings → SEO, a page → SEO | the search result preview; on the website title, meta, JSON-LD, `/sitemap.xml` and `/robots.txt` from the npm package `meierlabs-seo` (the demo is noindex) |
| Roles | Users (admins) | log in as `second@meierlabs.dev` (Author): only events and news, as drafts someone else publishes; pages and categories read only |
| AI chat | AI in the top bar | ask for anything, also a new event; it writes drafts, stays on your page when the change shows there and opens another page when not |

## On the website

- One GraphQL query per page, in Craft's shape, so crelte runs unchanged: `entry(uri: $uri, siteId: $siteId)`, `globalSet`, `crelteSites`.
- Rich text comes as `html` or as `chunks`: html, a row of buttons, an embedded block. Empty paragraphs are removed when publishing, so no margin hacks.
- Each page takes about 1–2 ms in zipp.

## For developers

- `cargo run -- makemigration` turns a change of the schema into a migration; renames are hints (`--rename event.city=town`).
- The site adds its own routes and hooks: this demo has a [`/sitemap.xml`](https://zipp-studio.noah.meierlabs.dev/sitemap.xml) and logs every change.
- `cargo run -- types` prints TypeScript types of the content.
