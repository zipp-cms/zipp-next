use super::throttle::{FAIL_WINDOW_MS, MAX_FAILS};
use super::*;
use crate::fixtures::site;

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
		token: String::new(),
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

#[test]
fn wrong_passwords_wait_per_address_and_email() {
	let t = Throttle::default();
	let now = 1_000_000;
	for _ in 0..MAX_FAILS {
		assert!(t.attempt("1.1.1.1", "a@x.ch", now));
	}
	// that address waits for that email, others don't
	assert!(!t.attempt("1.1.1.1", "a@x.ch", now));
	assert!(t.attempt("2.2.2.2", "a@x.ch", now));
	assert!(t.attempt("1.1.1.1", "b@x.ch", now));
	// until the window ends
	assert!(t.attempt("1.1.1.1", "a@x.ch", now + FAIL_WINDOW_MS));

	// a right password starts the email anew from the address
	let t = Throttle::default();
	for _ in 0..MAX_FAILS - 1 {
		t.attempt("1.1.1.1", "a@x.ch", now);
	}
	t.attempt("1.1.1.1", "a@x.ch", now);
	t.succeeded("1.1.1.1", "a@x.ch");
	assert!(t.attempt("1.1.1.1", "a@x.ch", now));
}

#[test]
fn many_emails_or_addresses_wait_too() {
	let now = 1_000_000;
	// one address trying many emails
	let t = Throttle::default();
	let tries = (0..100)
		.take_while(|i| t.attempt("1.1.1.1", &format!("{i}@x.ch"), now))
		.count();
	assert_eq!(tries, 50);

	// many addresses trying one email
	let t = Throttle::default();
	let tries = (0..200)
		.take_while(|i| t.attempt(&format!("10.0.0.{i}"), "a@x.ch", now))
		.count();
	assert_eq!(tries, 100);
}

#[test]
fn the_counts_stay_few() {
	let t = Throttle::default();
	let now = 1_000_000;
	// three counts an attempt: a new one waits once 10'000 are full
	let tries = (0..10_000)
		.take_while(|i| t.attempt(&format!("a{i}"), &format!("{i}@x.ch"), now))
		.count();
	assert_eq!(tries, 3333);
	assert!(t.attempt("new", "new@x.ch", now + FAIL_WINDOW_MS));
}
