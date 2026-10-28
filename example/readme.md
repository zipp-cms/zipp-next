# Welcome to the zipp demo

zipp is a headless CMS where the **content model is code**. This studio, the GraphQL API and the migrations are all made from one Rust file: [`example/cms/src/main.rs`](https://github.com/zipp-cms/zipp-next/blob/code-first/example/cms/src/main.rs). The website is a separate crelte project that reads the content over GraphQL.

- **Website (crelte):** <https://zipp-crelte.noah.meierlabs.dev> — and `/en` for English
- **A plain Svelte frontend:** <https://zipp.noah.meierlabs.dev>
- **GraphQL playground:** <https://zipp-studio.noah.meierlabs.dev/api/graphql>

This page is itself a feature: a markdown file of the repository ([`example/readme.md`](https://github.com/zipp-cms/zipp-next/blob/code-first/example/readme.md)), shown in the navigation with `.doc("readme", "Readme", include_str!("../../readme.md"))`. Editors read it, the code changes it.

## Where to try what

| Feature | Where in the studio | What to look for |
|---|---|---|
| Sections: single, channel, structure | Settings → Header, Footer · Events · Pages | a single opens its entry, a channel is a list, a structure a tree |
| The home page in the tree | Pages → Willkommen | the entry type `home` gets the uri `/` (`uri_of`) |
| Drag and drop | Pages | drag the handle before a title: before, after or into a page |
| Sites and languages | the language switch top right | titles, slugs and text per site; fields marked *all sites* are shared |
| Status per site | every list | `DE 2/2`: published here, and in how many sites |
| Drafts | change any field | the orange *Unpublished changes*, the website stays as it is until **Publish** |
| Preview | an entry with a draft → **Preview** | the website with your draft, for a day |
| What changed | the orange dots | fields that differ from the live version, also inside blocks |
| Revisions | Options → Revisions | what each one changed; click to look, **Restore** to go back |
| Two editors at once | log in twice | the second one is told instead of overwriting the first |
| Blocks | Pages → Über uns → Body | heading, text, image, button, upcoming events |
| Rich text | the Text block | headings, lists, links, a **row of buttons** and an **image** inside the text |
| Links | a Button block | entry, URL, email, phone or file; external ones open in a new tab |
| Group | a page → SEO | fields of another component under one name |
| Markdown | Events → Description | written as markdown, the website gets it as html |
| Boolean, number, indexed fields | Events | *sold out*, price; the website filters and sorts by them |
| Relations | Events → More info, Header → navigation | the website gets the linked entries with their fields |
| Images and transforms | Assets | click an image to set the focal point; crops follow it |
| Globals | Settings → Header, Footer | singles without a url, crelte reads them like Craft's globals |
| Translations | Settings → Translations | the keys come from the code, editors only translate |
| AI chat | AI in the top bar | ask for a change; the AI writes a draft, the website on the right shows it |

## On the website

- One GraphQL query per page, in Craft's shape, so crelte runs unchanged: `entry(uri: $uri, siteId: $siteId)`, `globalSet`, `crelteSites`.
- Rich text comes as `html` or as `chunks`: html, a row of buttons, an embedded block. Empty paragraphs are removed when publishing, so no margin hacks.
- Each page takes about 1–2 ms in zipp.

## For developers

- `cargo run -- makemigration` turns a change of the schema into a migration; renames are hints (`--rename event.city=town`).
- The site adds its own routes and hooks: this demo has a [`/sitemap.xml`](https://zipp-studio.noah.meierlabs.dev/sitemap.xml) and logs every change.
- `cargo run -- types` prints TypeScript types of the content.
