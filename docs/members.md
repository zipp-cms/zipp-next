# Website users

Visitors of the website can have an account and log in there, like
Craft users without "Access the control panel": members who edit their
own content in the frontend. Admins create them on the Users page of the
studio (no sign-up yet); the website routes that let them edit come from
the plugin `zipp-members` (below).

## Members: a role without the studio

```rust
.role(
	Role::new("member", "Member")
		.section("recipes", Access::Edit)
		.no_studio(),
)
```

`.no_studio()` keeps the role's users out of the studio (and Setup): a
login there answers "This account has no access to the studio, log in on
the website" after a right password, a session of the studio they still
have from another role is refused like an ended one. The Users page
lists, creates and edits them like everybody (the role shows "Studio: No,
website only", Setup's roles too); their invite and reset links lead to
the website (also when they ask the studio's login page), and there is no "Log in as" for them.

## Logging in on the website

Any user logs in on the website, members and editors (so admins can
edit in the frontend too). The frontend (the crelte server, server to
server) calls the cms on its own routes, not under `/api`:

| Request | Answer |
| --- | --- |
| `POST /site-auth/login {email, password}` | `{ session: { token, expiresAt }, user: { id, name, email, role } }` |
| `GET /site-auth/me` | the same, the session renewed when it ends soon |
| `POST /site-auth/logout` | ends the session |
| `POST /site-auth/reset {email, site?}` | mails a link to the website (a user of the studio gets the studio's), the same answer for unknown emails |
| `POST /site-auth/password {token, password}` | sets the password of an invite or reset link and logs in, like login |

The frontend keeps the token in an httpOnly cookie of the site and sends
it as `Authorization: Bearer <token>`. A site session is not a session of
the studio: its token never opens the studio's api, and a studio token
isn't one of the website. It lasts 30 days and is renewed by
`/site-auth/me` when less than 10 are left, like the studio's (see
Sessions in operations.md); a new password ends all sessions of the user,
of both kinds. Wrong passwords wait like in the studio and count
together with it: the frontend forwards the visitor's address in
`X-Forwarded-For` and is a `serve --trusted-proxy`, else all visitors
share one limit (Visitors' addresses in operations.md).

Reset and invite links go to `<site url>/reset-password?token=…` (the
site's `--site-url`, `site` picks one, else the first site with a url);
the frontend shows a form there and posts the token with the new
password to `/site-auth/password`. A link works once, an invite a week,
a reset an hour, and a reset mail goes at most every 2 minutes per user.
Mails need `serve --smtp-url`.

## Their profile

Every user has a public profile: their name (the display name), a slug
and the user fields of the schema, see [User fields and user
pages](content-model.md#user-fields-and-user-pages). The users with
live entries are public in GraphQL (`users`, `author`, see
[GraphQL](graphql.md#users)), so on chochbuech a recipe's cook is its
author: the cooks are the members themselves, with a page at
`cooks/{slug}`. The user page of the studio shows and edits the profile,
the slug only for admins. On the website the logged in user edits their
own:

| Request | Answer |
| --- | --- |
| `PUT /site-auth/me {name?, fields?}` | the user, like `user` of login: `{ id, name, email, role, slug, fields }` |

A new name and the fields it sends replace theirs, the others stay;
they are checked like in the studio (`INVALID` with paths like
`fields.bio`, an empty name is refused). Anything else, the slug, the
email or the role, is refused with 400: those are an admin's. `GET
/site-auth/me` and the login have the slug and fields too.

## Website routes of plugins

`zipp::auth::SiteUser` is the extractor of a logged in website user
(`id`, `name`, `email`, `role`, `None` an admin): a route of `routes()`
(the website's router) with it answers 401 without a bearer site token
and 403 with an ended one.

```rust
async fn own_recipes(user: SiteUser, State(state): State<AppState>) -> … {
	// only the entries of `user.id`
}
```

In tests, `ts.visitor(token)` sends requests like the frontend (no
`/api`, the token as bearer) and `ts.site_user(role)` is a user logged in
on the website.

### Acting as the user

A route of a plugin changes content as the user, never around zipp's
checks: `Authed::of_site_user(&user)` is the user as the studio knows
them, and these functions do what the studio's api does with it (the
role's sections and own access, validation, drafts and revisions, the
index, the search and `on_change`):

| Function | Does |
| --- | --- |
| `entries::list_as(state, &auth, query)` | the entries of a `Query` the user sees, unpublished ones too, each with its draft (`WithDraft`), and the total |
| `entries::get_as(state, &auth, id, site)` | one of them |
| `entries::create_as(state, &auth, input)` | a new entry of the user, published only if they may publish |
| `entries::update_as(state, &auth, id, input)` | saves into the draft, which goes live at once with Publish (`Saved::Live`), else stays for an editor (`Saved::Draft`) |
| `entries::delete_as(state, &auth, id)` | to the trash, with Delete |
| `assets::upload_as(state, &auth, &mut multipart, &rules)` | the first file of a multipart request, with asset Edit; `UploadRules::new(max_bytes).images_only().folder("Members/4")` |

The author is the user who creates an entry and can't change; a
relation to an entry the role can't view is left out like in the studio.

## Editing on the website: zipp-members

The plugin `zipp-members` lets members edit their own entries of some
sections in the frontend:

```rust
Schema::new()
	// ...
	.role(
		Role::new("member", "Member")
			.own("recipes", Access::Delete)
			.section("recipes", Access::View)
			.assets(Access::Edit)
			.no_studio(),
	)
	.plugin(
		Members::new()
			.section("recipes")
			.images(10 * 1024 * 1024, "Members/{user}"),
	)
```

Only the sections of `.section` are edited there, also when a role has
access to others. Of them a user edits what their role allows, as in
the studio: a member with own access their own entries, an admin or an
editor with the section's access the members' entries too. What they may
do with them is their role's: with Publish their saves are live at once,
with only Edit a new entry stays unpublished and a change is a draft
that an editor publishes in the studio. The frontend forwards these
routes with the visitor's site token (401 without one, 403 for an entry
the role may not touch or a section that isn't the plugin's):

| Request | Answer |
| --- | --- |
| `GET /members/entries?section=recipes&site=&offset=&all=` | `{ entries, total }`, theirs, the newest first, 100 at a time; with `all=true` all of the section (needs View of the section, 403 with only own access) |
| `GET /members/entries/{id}?site=` | the entry |
| `POST /members/entries {section, type, title, fields, published?, site?}` | the new entry |
| `PUT /members/entries/{id} {title, fields, published?, site?}` | `{ saved: "live" \| "draft", entry }` |
| `DELETE /members/entries/{id}` | to the trash |
| `POST /members/assets`, multipart | `{ id, url, width, height }`, an image for an image field |

An entry is `{ id, section, type, site, title, slug, uri, url, published,
postDate, fields, hasDraft }`, with the content of its draft if it has
one (`published` is what is live). A change replaces the fields it
sends, the others stay, and keeps the type, slug and dates; invalid
fields answer `INVALID` with their paths like the studio's api. Images
go into the folder of `.images` (`{user}` is the user's id), up to its
size, only images. A user may change or upload 60 times in 10 minutes.
