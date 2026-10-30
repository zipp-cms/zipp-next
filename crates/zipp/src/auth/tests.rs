use super::*;
use crate::schema::tests::site;

#[test]
fn roles_allow_levels() {
	let s = site().role(
		Role::new("editor", "Editor")
			.section("events", Access::Publish)
			.section("pages", Access::View)
			.assets(Access::Edit),
	);
	s.check().unwrap();
	let user = |role: Option<&str>| Authed {
		user_id: 1,
		role: role.map(Into::into),
	};

	let editor = user(Some("editor"));
	assert!(editor.can(&s, "events", Access::Edit));
	assert!(editor.can(&s, "events", Access::Publish));
	assert!(!editor.can(&s, "events", Access::Delete));
	assert!(editor.can(&s, "pages", Access::View));
	assert!(!editor.can(&s, "pages", Access::Edit));
	assert!(!editor.can(&s, "home", Access::View));
	assert!(editor.can_assets(&s, Access::Edit));
	assert!(!editor.can_assets(&s, Access::Delete));
	assert!(!editor.can_ai(&s));

	// an admin can everything, a role that is gone nothing
	assert!(user(None).can(&s, "home", Access::Delete));
	assert!(!user(Some("gone")).can(&s, "events", Access::View));

	let bad = site().role(Role::new("x", "X").section("nope", Access::View));
	assert!(bad.check().unwrap_err().contains("unknown section nope"));
}
