# Website users

Visitors of the website can have an account and log in there, like
Craft users without "Access the control panel": members who edit their
own content in the frontend. Admins create them on the Users page of the
studio (no sign-up yet); the website routes that let them edit come from
plugins (`zipp-members`, next).

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
the website, and there is no "Log in as" for them.

## Logging in on the website

Any user logs in on the website, members and editors (so admins can
edit in the frontend too). The frontend (the crelte server, server to
server) calls the cms on its own routes, not under `/api`:

| Request | Answer |
| --- | --- |
| `POST /site-auth/login {email, password}` | `{ session: { token, expiresAt }, user: { id, name, email, role } }` |
| `GET /site-auth/me` | the same, the session renewed when it ends soon |
| `POST /site-auth/logout` | ends the session |
| `POST /site-auth/reset {email, site?}` | mails a link to the website, the same answer for unknown emails |
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
