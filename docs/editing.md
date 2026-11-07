# Editing

What editors get in the studio (`/admin`).

## Drafts, revisions and previews

The studio saves every change into
a draft of the entry (one per entry, all sites), the website keeps the
live content until it is published. Publishing checks every site and
writes them in one transaction and keeps a revision; an old revision
can be restored into the draft. A preview token shows the draft on the
website: crelte passes `?token=` on to GraphQL. The database keeps only
the hash of a token (and of a session), a token ends after a day or
with the user who made it.

*Live preview* in the bar of an entry with a page puts the website next
to the editor, like Craft's: the page with the draft, as wide as a
desktop, a tablet or a phone, the border dragged for more room (both
kept in the browser, the switch too). One token per opening (it lasts a
day, a new one comes before); after every save of the draft the page
loads again where it is, like Craft's (crelte caches the queries of a
page in the browser, only a load shows new content). A new entry has a
preview once it is created, an entry in the flyout none. The studio's
navigation makes room while it is on, the Menu button before the title
slides it in.

What a site's frontend needs for it (and for the AI page):

- The studio may frame it: no `X-Frame-Options` and no CSP
  `frame-ancestors` that leave out the studio's origin, at least for
  urls with `?token=`. Its origin is in `--site-url` (the studio's CSP
  allows frames of those). The frame is sandboxed (scripts, its own
  origin, forms and popups, but it can't navigate the studio), a tab of
  View opens without `opener`.
- crelte's live preview mode (`x-craft-live-preview`, the studio adds
  it): it keeps the scroll position when the page loads again.
- The bridge (`example/crelte/svelte/src/components/PreviewBridge.svelte`,
  in `App.svelte`): it tells the studio which page it shows
  (`zipp:location`), so a reload stays on a page clicked to in the
  frame, and opens pages for the AI chat (`zipp:open`).

The editor has the content on the left and a sidebar like Craft's on the
right: published in the current site with the status in every site (a
badge switches to that site), post and expiry date (a day and its time
in one box, the x empties it; lists show Pending and Expired), type,
parent and slug, copying the content from another site, and the
revisions. The url isn't repeated there, View and the live preview open
it. A global (a single without a page)
has no title and only the copying and revisions there.

A narrow editor (the live preview, a tablet, a phone) has no room for
the sidebar: the status, dates, type and slug go between the title and
the fields, compact, the rest under the fields with the revisions
closed; the buttons of the bar keep only their icons. On a phone the
navigation is behind the Menu button on every page.

## Users

Admins manage the users on the Users page. A new user without a
password is invited: the mail (or a copied link) lets them choose one
within a week. "Forgot your password?" on the login mails a link for an
hour; an admin sends or copies one on the user's page too. Links work
once and need `serve --public-url` (and `--smtp-url` to mail them, see
operations.md). An admin can log in as a user who isn't an admin, like
Craft's impersonation: here, or with a copied link in a private window
(ten minutes, once). The session lasts two hours, a bar shows it with
"Back to your account"; what is saved meanwhile is the user's.

The studio speaks English and German. Each user picks the language on
their account page (an admin can set it for a new user, like a client
who wants German); without a choice it follows the browser: German if
the browser asks for German first, else English. The login page always
follows the browser. Dates and numbers are written in that language
(with the browser's region when it matches, so a Swiss browser writes
Swiss numbers). What the site's code names (sections, types, field
labels, instructions) stays as the code writes it, it is the site's
language already; messages of the server (a failed save's reason) are
still English.

## Trash

Like Craft's, deleting an entry (in a structure with the entries below
it) or an asset moves it to the trash: it is gone for the studio, the
website and GraphQL, relations to it are left out (saving what links to
it drops the link, like one to an entry that is gone), its uris are free.
The Trash page (for who may delete something, a role sees what it may
delete) restores it, with its children and at the end of its
siblings; a restored entry whose uri was taken meanwhile gets `-2` on
its slug. "Delete for good" and "Empty trash" delete it (a child trashed
on its own before stays, on the top), anything older
than 30 days (`trash::KEEP_DAYS`) goes on its own. The api is the same:
`DELETE /entries/{id}` trashes, `/trash` lists, restores and purges.

## The content model in the studio

Everyone sees it read only under
Setup (but a role made `without_setup()`; a role only its sections, with what it may do there, and its own
role; the header shows who is logged in with which role), like Craft's settings with admin changes off: sections (kind,
entry types, uri, and each section's entries with whether they are live
per site and have unpublished changes), entry types and blocks with
their fields (handle, kind and settings, per site or shared, where
they are used, the GraphQL type), sites, transforms and roles. Sections,
entry types and blocks are tables that link to a page each. A section's
page shows its settings (kind, navigation, GraphQL, entry types with
their uri, the urls per site, which role may do what, the SEO defaults)
and its entries as a short list on the side that links to the editor; a
type's page its fields, with where it is used on the side.

Setup starts with an overview of what is where: the counts (sections,
entry types, blocks, fields, entries, sites), each nav group with its
sections, their entries, entry types and the blocks and groups of their
fields (a level deeper where blocks hold blocks), blocks no field uses,
what each plugin added to the content model (sections, types, fields,
field types, transforms) and the links to GraphQL, roles and mail. The
Fields tab lists every field handle with its labels, kinds, the blocks
it holds and where it is, like Craft's field usage; its search finds a
handle, a label or a block (`button` shows every field that holds the
button block).

Because the model is code, an editor asks for a change on the page
where it is: "Request a change" in the bar of every Setup page takes a
text ("I need a subtitle field") with the page it was written on. Admins
see the requests under Setup → Requests, the open ones first, and tick
them done; with mails set up (`serve --smtp-url`) each admin gets a mail
of a new one. The api: `POST /change-requests` for everyone (`page` is a
path of the studio, `/admin/...`, anything else is refused), `GET` and
`PUT /change-requests/{id}` (`done`) for admins.

Admins see each field's handle next to its label, like Craft's "show
field handles"; the handle links to the field in Setup, the entry type
in the sidebar to the type's fields.

Everything with an order moves the same way: the pages of a structure,
the links of a navigation, blocks, relations, files, the rows of a
table, buttons and blocks in a rich text. The handle (⠿, on the left
of each row when there is more than one) drags it, left and right for the level
in a tree, as deep as allowed; a click on it opens a menu of the moves
and the arrow keys move it too, so it needs no dragging. Pages move at
once, the rest goes into the draft.

Lists of structures (Pages) have a search too; while searching, entries
can't be moved.

The search at the top of the sidebar (or Cmd+K, Ctrl+K, on every page)
finds entries and files like Craft's: entries of the site by their
title, slug and the text of their fields (text, markdown, rich text,
tables, also in blocks and groups), files by name, title and alt text.
Words match from their start, accents don't matter (`zurich` finds
Zürich), every word has to be there. The results come as you type, by
section, with the text around the match; the arrow keys choose, Enter
opens an entry in its editor and a file in the flyout, Escape closes.
Each role finds only the sections it may view, files only with access to
them. Only the live content is searched, not drafts; entries in the
trash are gone from it. The texts are kept in `entry_texts` with every
save (and rebuilt with the other index rows), SQLite's FTS5 searches
them. Globals (singles without a url) have no title field, the
bar at the top names them (Settings / Header).

## Entries and images in other fields

Relations and entry links choose in a flyout, like the asset library:
on the left the sections of their types as the nav groups them with
their counts (a row above the list on a phone, none with only one
section), the last one opens
again; on the right the entries of one with their status, a structure
as its tree (the second level and below closed, open with the arrow or
→ and ←), a channel newest first with its dates. The search above them
goes over all sections, by title and slug without accents, and shows
each match with its path (Über uns › Team) and section. Chosen entries
have a check, entries of other types (parents in a tree) show but can't
be chosen; ↑ ↓ and Enter pick without the mouse. A link takes one and
closes, a relation toggles several up to its max and counts them, Done
closes. The picker loads the first 1000 entries of each section
(`/api/entries/choices?type=page,event`, the sections of a field's
limit with `&section=pages,news`), the rest of a section as it scrolls
(`&offset=1000`, a tree keeps its children below their parent). While a
section isn't whole the search asks the server (`&search=team`, the
words of the studio's search, the first 1000, with their parents for
the path), else it searches what is there. Enter in an empty search
picks nothing. A chosen entry or image opens in a flyout from
the right, like Craft's slideout: the entry with its whole editor, an image
with its details (below); Done or Escape closes it and the
page behind stays as it was. In rich text, a button is a link like the
link field, right in the text (buttons next to each other are one row on
the website), a block is a card that opens its form; both have + Line
above and below to write before or after them.

## Images and files

An image opens with an editor, like Craft's: crop (free, the original
ratio, square, 16:9, 4:3, 3:2, also upright; past the image for room
around it), rotate, flip, a new size (also bigger), a canvas around it,
the background around the image in a color or transparent (a JPEG with
transparent parts is delivered as PNG) and the focal point the
crops of the transforms keep in view. The focal point, the crop and its
corners also move with the arrow keys (Shift for bigger steps). The uploaded file stays as it
was: the edits are stored next to it, every transform starts from the
edited image, Reset brings the original back. Next to it: title, alt
text (of the site the studio shows, the primary site's as placeholder)
and the filename, size, dimensions (edited and uploaded), who
uploaded it and when, download the original, copy the url, replace the
file (resets the edits) and the entries the file is used in. Save and
Cancel are in the bar of the flyout, Cancel and Escape ask before
throwing changes away.

## AI chat

`--openrouter-key-file <file>` adds *AI* to the studio:
a chat on the left, the website on the right. The model (Claude
Sonnet, GPT Sol or Gemini over OpenRouter, through soe-llms) sees and
changes what an editor can: entries of every section, new entries,
images, the texts of the code. Entries change as drafts, the website on
the right shows every draft with a token of the whole site
(`POST /api/preview`; the drafts of the sections the user sees, the
others live), an editor publishes. The model knows the page on
the right and its content; it stays there when the change shows on it
and opens another page when not. A site tells the studio its page with
a small bridge (`example/crelte/svelte/src/components/PreviewBridge.svelte`,
see the live preview above), crelte's live preview mode keeps the scroll
position on reloads.
