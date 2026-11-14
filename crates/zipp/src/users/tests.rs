use super::profile::{fill_slugs, set_slug};
use super::*;
use crate::db::Db;

#[test]
fn slugs_come_from_the_name() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let mia = create(&conn, "m@x.ch", "Mia Müller", "", None, "").unwrap();
	assert_eq!(user(&conn, mia).unwrap().slug, "mia-mueller");

	// without a name the id, a name set later replaces it
	let anon = create(&conn, "a@x.ch", " ", "", None, "").unwrap();
	assert_eq!(user(&conn, anon).unwrap().slug, format!("user-{anon}"));
	set_name(&conn, anon, "Mia Müller").unwrap();
	assert_eq!(user(&conn, anon).unwrap().slug, "mia-mueller-2");
	// then it stays
	set_name(&conn, anon, "Sam").unwrap();
	assert_eq!(user(&conn, anon).unwrap().slug, "mia-mueller-2");
}

#[test]
fn slugs_of_names_are_ascii_given_ones_stay() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let id = create(&conn, "s@x.ch", "Sören Meier", "", None, "").unwrap();
	assert_eq!(user(&conn, id).unwrap().slug, "soeren-meier");

	let given = super::profile::check_slug(&conn, id, "Sören").unwrap();
	assert_eq!(given.unwrap(), "sören");
}

#[test]
fn users_from_before_slugs_get_one() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let a = create(&conn, "a@x.ch", "Sam", "", None, "").unwrap();
	let b = create(&conn, "b@x.ch", "Sam", "", None, "").unwrap();
	set_slug(&conn, a, "").unwrap();
	set_slug(&conn, b, "").unwrap();

	fill_slugs(&conn).unwrap();
	assert_eq!(user(&conn, a).unwrap().slug, "sam");
	assert_eq!(user(&conn, b).unwrap().slug, "sam-2");
}
