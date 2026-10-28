# Editing

What editors get in the studio (`/admin`).

## Drafts, revisions and previews

The studio saves every change into
a draft of the entry (one per entry, all sites), the website keeps the
live content until it is published. Publishing checks every site and
writes them in one transaction and keeps a revision; an old revision
can be restored into the draft. A preview token shows the draft on the
website: crelte passes `?token=` on to GraphQL.

## The content model in the studio

Everyone sees it read only under
Setup (a role only its sections, with what it may do there, and its own
role; the header shows who is logged in with which role), like Craft's settings with admin changes off: sections (kind,
entry types, uri, and each section's entries with whether they are live
per site and have unpublished changes), entry types and blocks with
their fields (handle, kind and settings, per site or shared, where
they are used, the GraphQL type), sites, transforms and roles.

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
