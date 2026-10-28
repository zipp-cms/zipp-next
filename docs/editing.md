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
badge switches to that site), slug, type and parent, copying the content
from another site, and the revisions. A global (a single without a page)
has no title and only the copying and revisions there.

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

Lists of structures (Pages) have a search too; while searching, entries
can't be moved. Globals (singles without a url) have no title field, their
name is the section's.

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
