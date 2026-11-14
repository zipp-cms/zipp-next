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
`eMail`), required, a placeholder and instructions. So the builder is
the entry editor: drag the fields into their order, duplicate one,
switch one off. The block list is translatable: each site has its own
labels (and could have its own fields), *Copy content* takes them from
another site. A form has the label of its button (*Send*) and the
message after sending, per site. Like every entry it is on the website
only where it is published, with drafts and revisions.

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
  where it isn't: `{ id, title, fields: [{ type, handle, label,
  required, placeholder, instructions }], submitLabel, successMessage,
  honeypot }`, `type` is `text`, `textarea`, `email` or `phone`.
- `POST /forms/{id}?site=de`, JSON of the values by handle: stored,
  `{ "success": true, "message": "…" }`. Invalid values answer `422 {
  "success": false, "errors": [{ "path", "code", "message" }] }`: the
  `code` (`required`, `oneLine`, `tooLong`, `email`, `phone`, `text`)
  for the website's own text, the message in English. Text and email
  take 500 characters, a phone number 50 (digits with `+ ( ) - . /`),
  multi-line text 10 000.

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
	import { Form, Fields, Submit } from 'zipp-forms';

	let { zippForm } = $props();
</script>

{#if zippForm}
	<Form form={zippForm}>
		<Fields />
		<Submit />
	</Form>
{/if}
```

`Form` is the `<form>` with the honeypot; after sending it shows the
message (`sent` snippet for another look), why it couldn't send above
the button. `Fields` shows each field with `Field` (label, input,
instructions, errors), or a `field` snippet of the site's own built of
`Label`, `Input`, `Instructions` and `Errors`; `getForm()` gives the
form to the site's own components. The `ZippForm` has the values
(`values`, by handle), `errors`, `sending`, `sent` and `failed`;
`zippForm.messages` replaces the cms's English errors by code, and
`failed` when it can't send (see `example/crelte`, which takes them
from its translations). It has no styles but hiding the honeypot: the
classes are `zipp-form-field`, `zipp-form-<type>`, `required`,
`instructions`, `error`, `zipp-form-sent` and `zipp-form-failed`.
