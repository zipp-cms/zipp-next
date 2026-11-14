# Forms

Forms editors build in the studio, like Craft's Formie: a plugin, the
crate `zipp-forms`, and its crelte frontend, the npm package
`zipp-forms` (`crates/zipp-forms/crelte`).

```rust
use zipp_forms::{Forms, form_field};

Schema::new()
	// ...
	.plugin(Forms::new())
	// a block (or a page) that shows one of them
	.block("contact", "Form", [form_field("form").required()])
```

## In the studio

A form is an entry of the section `forms` (put it in the nav). Its
fields are blocks: *Single line text*, *Multi-line text*, *Email* and
*Phone*, each with a label, a handle (the name of the value for the
website and in the submissions; empty: made of the label, `E-Mail` is
`eMail`), required, a placeholder and instructions, and a width (¼, ⅓,
½, ⅔, ¾ or full) like Formie's columns: fields side by side that fit
share a row. They are one field of the type `formFields`, whose studio
editor is the builder:

- the pages are tabs above the fields, a form always has one: drag a
  tab (or its ⋯: Move left, Move right) to order them, ⋯ Rename (or a
  double click) names one, the step's title on the website, ⋯ Delete
  page removes it with its fields, *Add page* makes a new one;
- the fields of the page on screen are a grid of their widths with a
  preview of their inputs: drag one to move it, a click opens its
  settings and width in a flyout, its ⋯ menu (or a right click) has
  Edit, Duplicate, To another page, Switch off and Delete;
- *Add field* at the end of the grid picks the type and opens the new
  field.

The field is translatable: each site has its own labels (and could
have its own fields and pages), *Copy content* takes them from another
site. A form has the labels of its buttons (*Send*, between pages
*Next* and *Back*) and the message after sending, per site. Like every
entry it is on the website only where it is published, with drafts and
revisions.

The value is a list of items (the blocks of `Field::item_blocks`): `{
"width": 50, "block": { … } }` a field, `{ "block": { "type":
"formPage", "fields": { "label": "Über Sie" } } }` where a page starts;
the fields before the first page item are on a page without a name, a
page without fields isn't a step on the website. GraphQL only counts
them (`{ pages fields }`), the website gets a form through its routes.

The page **Submissions** (in the sidebar, for who may view `forms`)
lists what came in, the newest first, filtered by form; a click on a
row shows all values. Deleting one needs Delete on `forms`. A
submission keeps the labels its fields had, so a changed form doesn't
change it; a form deleted for good (out of the trash) takes its
submissions.

## The website

`form_field` is a relation to one form, the page's GraphQL gives its
id (`form { id }`). The website gets the form by it and sends to it,
both through the frontend's server (crelte proxies them, the cms
doesn't need to be public):

- `GET /forms/{id}?site=de`: the form as it is live in the site, 404
  where it isn't: `{ id, title, pages: [{ label, fields: [{ type,
  handle, label, required, placeholder, instructions, width }] }],
  submitLabel,
  nextLabel, backLabel, successMessage, honeypot }`, `type` is `text`,
  `textarea`, `email` or `phone`; one page at least.
- `POST /forms/{id}?site=de`, JSON of the values by handle: stored,
  `{ "success": true, "message": "…" }`. Invalid values answer `422 {
  "success": false, "errors": [{ "path", "code", "message" }] }`: the
  `code` (`required`, `oneLine`, `tooLong`, `email`, `phone`, `text`)
  for the website's own text, the message in English. Text and email
  take 500 characters, a phone number 50 (digits with `+ ( ) - . /`),
  multi-line text 10 000.
- `POST /forms/{id}?site=de&page=0`: before the visitor goes on to the
  next page, checks only that page's values: `{ "success": true }` or
  the errors as above, nothing is stored and it doesn't count for the
  limit. Sending without `page` checks all pages again.

Against spam: a filled honeypot field (`honeypot` of the form,
`website` unless a field has that handle; `Forms::new().honeypot(..)`
for another name) is answered like a success and dropped, an address
(IPv6: its /64) sends at most 5 in 10 minutes. The limit is per
address only: a few addresses can't block a form for everyone. The
frontend passes the visitor's address in `X-Forwarded-For`, the cms
reads it only from a `serve --trusted-proxy` (operations.md), else the
connection counts.

## crelte

The npm package `zipp-forms` is Svelte and TypeScript source like
zipp-ui, the site's vite builds it. Its server routes answer the query
of `loadForm` (so the form comes with the page's server rendering, the
browser doesn't load it again) and pass the submissions on:

```js
// server.js
import { formRoutes } from 'zipp-forms/server';

export async function routes(router) {
	formRoutes(router);
}
```

A block loads its form, `null` where it isn't live:

```svelte
<script module>
	import { loadForm } from 'zipp-forms';

	export async function loadData(cr, block) {
		return { zippForm: await loadForm(cr, block.form[0]?.id) };
	}
</script>

<script>
	import { Back, Form, Fields, Submit } from 'zipp-forms';

	let { zippForm } = $props();
</script>

{#if zippForm}
	<Form form={zippForm}>
		<Fields />
		<Back />
		<Submit />
	</Form>
{/if}
```

`Form` is the `<form>` with the honeypot; submitting goes to the next
page (once the cms finds this one right) or sends on the last, after
sending it shows the message (`sent` snippet for another look), why it
couldn't send above the button. `Submit` says *Next* before the last
page, `Back` goes to the page before and keeps what was typed (nothing
on the first page); a mistake found when sending opens its page.
`Fields` shows each field of the page on screen in rows by their
widths (each its own row below 40rem, the gap is `--zipp-form-gap`) with
`Field` (label, input,
instructions, errors), or a `field` snippet of the site's own built of
`Label`, `Input`, `Instructions` and `Errors`; `getForm()` gives the
form to the site's own components. The `ZippForm` has the values
(`values`, by handle), `errors`, `sending`, `sent` and `failed`, the
steps (`pages`, `page` the index on screen, `isLast`; a progress bar
of the site's own reads them) and `next()`, `back()`, `submit()`;
`zippForm.messages` replaces the cms's English errors by code, and
`failed` when it can't send (see `example/crelte`, which takes them
from its translations). It has no styles but hiding the honeypot: the
classes are `zipp-form-field`, `zipp-form-<type>`, `required`,
`instructions`, `error`, `zipp-form-sent` and `zipp-form-failed`.
