# Welcome to the zipp demo

zipp is a headless CMS where the **content model is code**. This studio, the GraphQL API and the migrations are all made from one Rust file: [`example/cms/src/content.rs`](https://github.com/zipp-cms/zipp-next/blob/main/example/cms/src/content.rs). The website is a separate crelte project that reads the content over GraphQL.

- **Website (crelte):** <https://zipp-crelte.noah.meierlabs.dev> — and `/en` for English
- **GraphQL** is private like Craft's: frontends send a token, admins make tokens with the sections and sites they read under GraphQL in the studio; the playground (GraphiQL) is for admins (Try it) and in development (`serve --dev`)

This page is itself a feature: a markdown file of the repository ([`example/readme.md`](https://github.com/zipp-cms/zipp-next/blob/main/example/readme.md)), shown in the navigation with `.doc("readme", "Readme", include_str!("../../readme.md"))`. Editors read it, the code changes it.

## Where to try what

| Feature | Where in the studio | What to look for |
|---|---|---|
| Sections: single, channel, structure | Settings → Header, Footer · News, Events · Pages, Categories | a single opens its entry, a channel is a list, a structure a tree |
| Like the crelte tutorial | Pages, News, Categories | pages and articles start with the **Intro** tab (a rich text intro, an image), the blocks are in **Content**; the news page on the website lists the articles and filters them by category (`?category=`), their dates formatted by GraphQL (`@formatDateTime`, `.timezone("Europe/Zurich")`) |
| The home page in the tree | Pages → Willkommen | the entry type `home` gets the uri `/` (`uri_of`) |
| Change the type | a page → Type in the sidebar → News | the intro stays, the blocks go after asking; the website keeps the page until **Publish** |
| Drag and drop | Pages | drag the handle before a title: before, after or into a page |
| Max level | Categories | two levels (`max_level(2)`): a category goes into a top one, not deeper; the Parent of a new one lists only top categories |
| Sites and languages | the language switch top right | titles, slugs and text per site; fields marked *all sites* are shared |
| Language switch | the website's DE EN top right, on any page | `localized` in GraphQL: it goes to the same page in the other language (`/über-uns` → `/en/about-us`), or the other site's home page where the page isn't published |
| Status per site | every list | `DE EN`: a badge per site, grey where it is not published |
| Drafts | change any field | the orange *Unpublished changes*, the website stays as it is until **Publish** |
| Preview | an entry with a draft → **Preview** | the website with your draft, for a day |
| Live preview | an event → **Live preview**, type in the title | the page next to the editor, it shows the new title a moment later where you are; desktop, tablet or phone width, drag the border; the navigation makes room, the editor gets compact |
| What changed | the orange dots | fields that differ from the live version, also inside blocks |
| Trash | delete a page or an asset → Trash in the navigation | it is gone from the website, **Restore** brings it back with the pages below it; after 30 days it is deleted for good |
| Revisions | Options → Revisions | what each one changed; click to look, **Restore** to go back |
| Two editors at once | log in twice | the second one is told instead of overwriting the first |
| Blocks | Pages → Über uns → Content | text, images, video, button, upcoming events |
| Rich text | the Intro tab · the Text block | the intro only with links; the block also with headings, lists, **buttons** (next to each other one row), **images** inside the text and **Styles** (*Lead* for a paragraph, *Highlight* for words; the website styles their classes); its toolbar is set in the code (`.toolbar([...])`): images have an icon of their own instead of the Insert menu |
| Links | a Button block | entry, URL, email, phone or file; external ones open in a new tab |
| Group, inline group | a page → SEO, Events → Tickets | fields of another component under one name in a box; the event's tickets are an inline group, its fields written right in the field (`Field::group_of`) |
| Tabs in a block | Pages → Über uns → Content → an Images block | **Content** (the images, the caption) and **Layout** (the width) as tabs inside the block (`block_tabs`); the block keeps its tab |
| Tab in a card | News → an article → Intro · Events | the fields of a tab or a whole type in one card with the labels on the left, like a group (`.tab_card`, `.entry_card`) |
| Markdown | Events → Description | written as markdown, the website gets it as html |
| Navigation, table, checkboxes | Settings → Header, Footer · Events → Für wen | a menu with a second level, footer links as rows, some of the options |
| Typed table columns | Events → Tickets → Kategorien | a text, a price in CHF and a *Ausverkauft* switch per row; the event page lists them |
| Dropdown | Images block → Width (3 options: buttons), Events → Sprache (5: a select) | the website sizes the image, filters events by language |
| Field widths, notes | Events | date, doors and city with widths: in the event's card each in its own row with a narrower value, so the labels line up (on a phone all full width); a note on how the event's image should be |
| Conditions | Events → Genre, Tickets → Sold out | *Doors open* only for a concert, *Warteliste* only while the event is sold out; the fields come and go as you click |
| Default values | Images block → Width, Events → Sprache | a new block starts *Wide*, a new event *Deutsch* |
| Email | Settings → Footer → Contact email | the server takes only an address, the website links it with `mailto:` |
| Button group | Events → Genre | `Field::button_group`: Craft's button group, options with a color and a Lucide icon (`ButtonOption`); the website shows the genre's label in its color |
| Boolean, number, indexed fields | Events | *sold out*, price (CHF, two decimals) in the tickets; the website sorts by date and filters by city |
| Date, date and time | Events → Date, Doors open | the date picker; a day or a day and time, the website sorts events by the date |
| Relations | News → Categories, Events → More info, Header → navigation | the website gets the linked entries with their fields, and filters by them; the picker is a flyout with the sections on the left and the pages as their tree, search "team" for a match with its path; an article takes up to 3 categories (`.max(3)`); the header's navigation links only to pages (`.sections(["pages"])`), its picker has no News or Events |
| Images and transforms | Assets | click an image to set the focal point; crops follow it; the hero transform is WebP (`.format(Format::Webp)`), the website's hero srcset too |
| Alt text per site | Assets → an image → Alt text, switch the site | each site its own alt text, a site without has the primary site's |
| Asset fields | Assets → an image → Credit, Licence | fields of every asset beside title and alt (`.asset_fields`); the website shows the credit under the images |
| Videos | Assets → Videos · a page → Content → Video block | the library filters by type, a video plays in its panel and shows its first frame; the block takes only videos (`Field::videos`), a poster image and *loop* for a muted moving image |
| Folders, file types, zips | Assets · Events → Tickets → Programm (PDF) · a folder's ⋯ → Download as zip · upload a zip | folders for the library, downloaded as a zip or unpacked from one; the event's program takes only PDFs and puts them into a folder of the event, Events/{slug}, its image picker opens in Events |
| Icons | the navigation, Pages, a page's blocks | entry types, blocks and nav groups with Lucide icons from the code |
| Block titles | a page's blocks: Button, Images, Upcoming events · Setup → Blocks → Button | a block's head and the outline show the field the code names (`.block_title`): the button's link, the caption, the city |
| Globals | Settings → Header, Footer | singles without a url, crelte reads them like Craft's globals |
| Session ends | type in an entry, in a second browser: your user → **Log out everywhere else** · type on | a dialog asks for the password, the page stays; after it the draft has what you typed |
| Users, invites, login as | Users → New user without a password · a user's page → Access · the login → Forgot your password? | the invite and reset mails (on dev in Mailpit), **Log in as** with the bar to come back |
| Contact form | the website → /kontakt · Settings → Forms | `crates/zipp-forms`: the form is code, the mail goes to who Forms names (on dev: Mailpit), the message after sending is from there |
| Mail | Setup → Sites, transforms and roles → Mail (admins) | where mails go; **Send a test mail to me**. Without `--smtp-url` they are only logged |
| Setup overview | Setup → Overview · Setup → Fields, search `button` | what is where: nav groups, sections with their entries, types and the blocks of their fields; the plugins forms and seo with what they added; every field handle and where it is |
| Request a change | as an editor: Setup → Entry types → Page → **Request a change** · as an admin: Setup → Requests | the request with its page; admins tick it done and get a mail of it (on dev in Mailpit) |
| Translations | Settings → Translations | the keys come from the code, editors only translate |
| SEO | Settings → SEO, a page → SEO | the search result preview; on the website title, meta, JSON-LD, `/sitemap.xml` and `/robots.txt` from the npm package `meierlabs-seo` (the demo is noindex) |
| Search | **Search** at the top of the sidebar, or Cmd/Ctrl+K | type a word of a page's text: entries by section with where it matched and files, Enter opens one; the author finds nothing of Settings (header, footer), which they may not see |
| Roles | Users (admins) | log in as `second@meierlabs.dev` (Author): only events and news; events as drafts someone else publishes, their own news they publish and delete, the others' news, pages and categories read only |
| AI chat | AI in the top bar | ask for anything, also a new event; it writes drafts, stays on your page when the change shows there and opens another page when not |

## On the website

- One GraphQL query per page, in Craft's shape, so crelte runs unchanged: `entry(uri: $uri, siteId: $siteId)`, `globalSet`, `crelteSites`.
- The search in the header goes to `/suche?q=konzert` (`/en/search` in English): the pages with the words, the best first, with the words around the match marked; a route of the frontend, not a section in zipp (`src/search.js`, `templates/search.svelte`).
- Rich text comes as `html` or as `chunks`: html, a row of buttons, an embedded block. Empty paragraphs are removed when publishing, so no margin hacks.
- Each page takes about 1–2 ms in zipp.

## For developers

- `cargo run -- makemigration` turns a change of the schema into a migration; renames are hints (`--rename event.city=town`).
- What the ops can't do is a data migration in Rust: `cargo run -- makemigration --data --name split_names` writes its file and prints the step to register with `Cms::data_migration`.
- The site adds its own routes and hooks: this demo has a [`/sitemap.xml`](https://zipp-studio.noah.meierlabs.dev/sitemap.xml) and logs every change.
- `cargo run -- types` prints TypeScript types of the content.
