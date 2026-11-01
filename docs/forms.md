# Forms

Forms of the website like Craft's Contact Form: a plugin, the crate
`zipp-forms`. The fields are code, a submission is checked and mailed,
nothing is stored.

```rust
.plugin(Forms::new().form(
	Form::new("contact", "Contact")
		.field(FormField::text("name").required())
		.field(FormField::email("email").required().reply_to())
		.field(FormField::text("phone"))
		.field(FormField::multiline("message").required()),
))
```

`FormField::text`, `email` and `multiline` (at the end of the mail),
with `.label()` (its name in the mail), `.required()`, `.max(chars)` and
`.reply_to()` (answers go to that address). It adds the single
`formSettings` ("Forms", put it in the nav): per form `contactTo`, the
recipients in all sites (commas between them), `contactSubject` and
`contactSuccess`, the message after sending, per site. A form without a
recipient answers that it can't send.

The website posts JSON to `POST /forms/<handle>?site=<site>` on the cms;
the crelte server proxies it and passes `X-Forwarded-For` (see
`example/crelte/svelte/src/server.js`). The answer is `{ "success":
true, "message": "…" }`, invalid fields `422 { "success": false,
"errors": [{ "path": "email", "message": "An email address" }] }`, too
many `429`, a mail that can't go `503`. Against spam: a filled honeypot
field (`website`, `.honeypot("company")` for another name, hidden from
people) is answered like a success and dropped, and an address sends at
most 5 in 10 minutes. Mails need `serve --smtp-url`, see operations.md.
