# Editing

What editors get in the studio (`/admin`).

## Drafts, revisions and previews

The studio saves every change into
a draft of the entry (one per entry, all sites), the website keeps the
live content until it is published. Publishing checks every site and
writes them in one transaction and keeps a revision; an old revision
can be restored into the draft. A preview token shows the draft on the
website: crelte passes `?token=` on to GraphQL.

The editor has the content on the left and a sidebar like Craft's on the
right: published in the current site with the status in every site (a
badge switches to that site), slug, post and expiry date (lists show
Pending and Expired), type and parent, copying the content
from another site, and the revisions. A global (a single without a page)
has no title and only the copying and revisions there.

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

## Trash

Like Craft's, deleting an entry (in a structure with the entries below
it) or an asset moves it to the trash: it is gone for the studio, the
website and GraphQL, relations to it are left out, its uris are free.
The Trash page (for who may delete something, a role sees what it may
delete) restores it, with its children and at the end of its
siblings; a restored entry whose uri was taken meanwhile gets `-2` on
its slug. "Delete for good" and "Empty trash" delete it, anything older
than 30 days (`trash::KEEP_DAYS`) goes on its own. The api is the same:
`DELETE /entries/{id}` trashes, `/trash` lists, restores and purges.

## The content model in the studio

Everyone sees it read only under
Setup (a role only its sections, with what it may do there, and its own
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
can't be moved. Globals (singles without a url) have no title field, their
name is the section's.

## Entries and images in other fields

Relations and entry links choose with a button that lists the entries of
their types, with a search. A chosen entry or image opens in a flyout from
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
crops of the transforms keep in view. The uploaded file stays as it
was: the edits are stored next to it, every transform starts from the
edited image, Reset brings the original back. Next to it: title, alt
text and the filename, size, dimensions (edited and uploaded), who
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
(`POST /api/preview`), an editor publishes. The model knows the page on
the right and its content; it stays there when the change shows on it
and opens another page when not. A site tells the studio its page with
a small bridge (`example/crelte/svelte/src/components/PreviewBridge.svelte`),
crelte's live preview mode keeps the scroll position on reloads.
