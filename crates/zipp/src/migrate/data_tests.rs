use std::sync::atomic::{AtomicUsize, Ordering};

use serde_json::json;

use super::*;
use crate::auth::Authed;
use crate::db::Db;
use crate::drafts::{SiteDoc, get_draft, get_revision, put_draft, revisions};
use crate::entries::query::Query;
use crate::entries::{get_entry, list, save};
use crate::fixtures::site;
use crate::migrate::apply::apply_migrations;
use crate::migrate::{Migration, Migrations, diff};
use crate::schema::Field;
use crate::testing::input;

fn migration(name: &str, old: &Schema, new: &Schema) -> Migration {
	Migration {
		name: name.into(),
		ops: diff(old, new, &[]).unwrap(),
		schema: new.clone(),
		..Default::default()
	}
}

/// A data migration with its step, on the schema of the one before.
fn data(
	name: &str,
	schema: &Schema,
	step: impl Fn(&DataTx) -> DataResult + Send + Sync + 'static,
) -> Migration {
	Migration {
		name: name.into(),
		data: true,
		schema: schema.clone(),
		step: Some(DataStep(Arc::new(step))),
		..Default::default()
	}
}

fn apply(conn: &Connection, migrations: &[Migration]) -> Vec<String> {
	let code = &migrations.last().unwrap().schema;
	apply_migrations(conn, migrations, code).unwrap()
}

fn draft(fields: Value) -> SiteDoc {
	SiteDoc {
		ty: "event".into(),
		dates: Default::default(),
		title: "E".into(),
		slug: "e".into(),
		published: true,
		fields,
	}
}

/// The schema with `town` next to `city` on events, and without `city`.
fn with_town() -> (Schema, Schema) {
	let mut both = site();
	let event = both.get_mut("event").unwrap();
	event.fields.push(Field::text("town").indexed());
	let mut town = both.clone();
	town.get_mut("event").unwrap().fields.remove(0);
	(both, town)
}

/// Fills `town` from `city`, between adding one and removing the other.
fn town_from_city(tx: &DataTx) -> DataResult {
	tx.each("event", |c| {
		if let Some(city) = c.get::<String>("city") {
			c.set("town", format!("{city}town"))?;
		}
		Ok(())
	})
}

#[test]
fn a_step_runs_once_between_the_ops() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let first = migration("0001", &Schema::new(), &s);
	apply(&conn, std::slice::from_ref(&first));

	let fields = json!({ "city": "Bern" });
	let e = save(&conn, &s, None, input("events", "event", "E", fields), None)
		.unwrap();
	put_draft(
		&conn,
		&s,
		e.id,
		"de",
		draft(json!({ "city": "Thun" })),
		None,
		None,
	)
	.unwrap();

	let runs = Arc::new(AtomicUsize::new(0));
	let counted = runs.clone();
	let (both, town) = with_town();
	let migrations = [
		first,
		migration("0002", &s, &both),
		data("0003", &both, move |tx| {
			counted.fetch_add(1, Ordering::SeqCst);
			town_from_city(tx)
		}),
		migration("0004", &both, &town),
	];
	assert_eq!(apply(&conn, &migrations), ["0002", "0003", "0004"]);
	assert!(apply(&conn, &migrations).is_empty());
	assert_eq!(runs.load(Ordering::SeqCst), 1);

	// live in every site, the draft and the revision
	for site in ["de", "en"] {
		let entry = get_entry(&conn, e.id, site).unwrap();
		assert_eq!(entry.fields, json!({ "listed": true, "town": "Berntown" }));
	}
	let d = get_draft(&conn, e.id).unwrap().unwrap();
	assert_eq!(d.doc["de"].fields["town"], "Thuntown");
	// city is shared, the draft has it in both sites
	assert_eq!(d.doc["en"].fields["town"], "Thuntown");
	let rev = revisions(&conn, e.id).unwrap()[0].id;
	let rev = get_revision(&conn, e.id, rev).unwrap();
	assert_eq!(rev["de"].fields["town"], "Berntown");

	// the indexes have the new values
	let params = vec![
		("type".to_string(), "event".to_string()),
		("f.town".to_string(), "Berntown".to_string()),
	];
	let q = Query::parse(&town, &params).unwrap();
	assert_eq!(list(&conn, &q).unwrap().1, 1);
	let admin = Authed {
		user_id: 1,
		role: None,
		token: String::new(),
	};
	let found =
		crate::search::search(&conn, &town, &admin, "de", "berntown").unwrap();
	assert_eq!(found.entries.len(), 1);
}

#[test]
fn an_error_rolls_back_the_ops_before_it() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let first = migration("0001", &Schema::new(), &s);
	apply(&conn, std::slice::from_ref(&first));
	let fields = json!({ "city": "Bern" });
	let e = save(&conn, &s, None, input("events", "event", "E", fields), None)
		.unwrap();

	let (both, _) = with_town();
	let migrations = [
		first,
		migration("0002", &s, &both.clone()),
		data("0003", &both, |tx| {
			tx.each("event", |c| {
				c.set("town", "half")?;
				Err("no town".into())
			})
		}),
	];
	let err = apply_migrations(&conn, &migrations, &both).unwrap_err();
	assert!(err.to_string().contains("data migration 0003: no town"));

	let applied: i64 = conn
		.query_row("SELECT COUNT(*) FROM schema_migrations", [], |r| r.get(0))
		.unwrap();
	assert_eq!(applied, 1);
	let entry = get_entry(&conn, e.id, "de").unwrap();
	assert_eq!(entry.fields, json!({ "city": "Bern", "listed": true }));
}

#[test]
fn each_reaches_blocks_and_groups() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let first = migration("0001", &Schema::new(), &s);
	apply(&conn, std::slice::from_ref(&first));
	let fields = json!({
		"body": [
			{ "id": "a", "type": "text", "fields": { "content": "hello" } },
		],
		"seo": { "title": "Old" },
	});
	let p = save(&conn, &s, None, input("pages", "page", "P", fields), None)
		.unwrap();

	let migrations = [
		first,
		data("0002", &s, |tx| {
			tx.each("text", |c| {
				let text: String = c.get("content").unwrap_or_default();
				c.set("content", text.to_uppercase())?;
				Ok(())
			})?;
			tx.each("seo", |c| {
				c.set("description", format!("{} of {}", c.title, c.site))?;
				Ok(())
			})
		}),
	];
	apply(&conn, &migrations);

	let entry = get_entry(&conn, p.id, "en").unwrap();
	assert_eq!(entry.fields["body"][0]["fields"]["content"], "HELLO");
	assert_eq!(entry.fields["seo"]["description"], "P of en");
}

#[test]
fn each_reaches_drafts_and_revisions_of_another_type() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let mut s = site();
	s.sections[1].types.push("event".into());
	let first = migration("0001", &Schema::new(), &s);
	apply(&conn, std::slice::from_ref(&first));

	// a page with an event as draft and as revision
	let p = save(
		&conn,
		&s,
		None,
		input("pages", "page", "P", json!({})),
		None,
	)
	.unwrap();
	let city = |c: &str| draft(json!({ "city": c }));
	put_draft(&conn, &s, p.id, "de", city("thun"), None, None).unwrap();
	let mut doc =
		get_revision(&conn, p.id, revisions(&conn, p.id).unwrap()[0].id)
			.unwrap();
	doc.insert("de".into(), city("bern"));
	conn.execute(
		"UPDATE revisions SET doc = ?1 WHERE entry_id = ?2",
		params![serde_json::to_string(&doc).unwrap(), p.id],
	)
	.unwrap();
	// an event's revision as written by hand: nothing to change, it stays
	let e = save(
		&conn,
		&s,
		None,
		input("events", "event", "E", json!({})),
		None,
	)
	.unwrap();
	let spaced = r#"{ "de": { "type": "event", "title": "E", "slug": "e",
		"published": true, "fields": {} } }"#;
	conn.execute(
		"UPDATE revisions SET doc = ?1 WHERE entry_id = ?2",
		params![spaced, e.id],
	)
	.unwrap();

	let migrations = [
		first,
		data("0002", &s, |tx| {
			tx.each("event", |c| {
				if let Some(city) = c.get::<String>("city") {
					c.set("city", city.to_uppercase())?;
				}
				Ok(())
			})
		}),
	];
	apply(&conn, &migrations);

	let d = get_draft(&conn, p.id).unwrap().unwrap();
	assert_eq!(d.doc["de"].fields["city"], "THUN");
	let rev = revisions(&conn, p.id).unwrap()[0].id;
	assert_eq!(
		get_revision(&conn, p.id, rev).unwrap()["de"].fields["city"],
		"BERN"
	);
	let stored: String = conn
		.query_row(
			"SELECT doc FROM revisions WHERE entry_id = ?1",
			[e.id],
			|r| r.get(0),
		)
		.unwrap();
	assert_eq!(stored, spaced);
}

#[test]
fn each_reaches_the_fields_of_assets() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site().asset_fields([Field::text("credit")]);
	let first = migration("0001", &Schema::new(), &s);
	apply(&conn, std::slice::from_ref(&first));
	conn.execute(
		r#"INSERT INTO assets (filename, kind, mime, size, created_at, fields)
		VALUES ('a.png', 'image', '', 1, 0, '{"credit":"anna"}')"#,
		[],
	)
	.unwrap();

	let migrations = [
		first,
		data("0002", &s, |tx| {
			tx.each("asset", |c| {
				let credit: String = c.get("credit").unwrap_or_default();
				let at =
					format!("{credit} {} {}{}", c.title, c.entry_id, c.site);
				c.set("credit", at)
			})
		}),
	];
	apply(&conn, &migrations);

	let fields: String = conn
		.query_row("SELECT fields FROM assets", [], |r| r.get(0))
		.unwrap();
	assert_eq!(fields, r#"{"credit":"anna a.png 1"}"#);
}

#[test]
fn each_reaches_the_fields_of_users() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site().user_fields([Field::text("city")]);
	let first = migration("0001", &Schema::new(), &s);
	apply(&conn, std::slice::from_ref(&first));
	conn.execute(
		r#"INSERT INTO users (email, password_hash, created_at, name, fields)
		VALUES ('anna@x.ch', '', 0, 'Anna', '{"city":"Thun"}')"#,
		[],
	)
	.unwrap();

	let migrations = [
		first,
		data("0002", &s, |tx| {
			tx.each("user", |c| {
				let city: String = c.get("city").unwrap_or_default();
				c.set("city", format!("{city} {} {}", c.title, c.entry_id))
			})
		}),
	];
	apply(&conn, &migrations);

	let fields: String = conn
		.query_row("SELECT fields FROM users", [], |r| r.get(0))
		.unwrap();
	assert_eq!(fields, r#"{"city":"Thun Anna 1"}"#);
}

#[test]
fn a_value_that_isn_t_json_fails() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let first = migration("0001", &Schema::new(), &s);
	apply(&conn, std::slice::from_ref(&first));
	save(
		&conn,
		&s,
		None,
		input("events", "event", "E", json!({})),
		None,
	)
	.unwrap();

	let migrations = [
		first,
		data("0002", &s, |tx| {
			tx.each("event", |c| {
				let map = std::collections::HashMap::from([((1, 2), 3)]);
				c.set("city", map)
			})
		}),
	];
	let err = apply_migrations(&conn, &migrations, &s).unwrap_err();
	assert!(err.to_string().contains("key must be a string"), "{err}");
}

#[test]
fn what_a_step_writes_in_sql_is_indexed() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let first = migration("0001", &Schema::new(), &s);
	apply(&conn, std::slice::from_ref(&first));
	let fields = json!({ "city": "Bern" });
	save(&conn, &s, None, input("events", "event", "E", fields), None).unwrap();

	let migrations = [
		first,
		data("0002", &s, |tx| {
			tx.conn().execute(
				"UPDATE entry_sites SET fields =
				json_set(fields, '$.city', 'Thun')",
				[],
			)?;
			Ok(())
		}),
	];
	apply(&conn, &migrations);

	let found = |city: &str| {
		let params = vec![
			("type".to_string(), "event".to_string()),
			("f.city".to_string(), city.to_string()),
		];
		let q = Query::parse(&s, &params).unwrap();
		list(&conn, &q).unwrap().1
	};
	assert_eq!(found("Thun"), 1);
	assert_eq!(found("Bern"), 0);
}

#[test]
fn an_unknown_component_fails() {
	let db = Db::memory().unwrap();
	let s = site();
	let migrations = [
		migration("0001", &Schema::new(), &s),
		data("0002", &s, |tx| tx.each("person", |_| Ok(()))),
	];
	let err = apply_migrations(&db.conn(), &migrations, &s).unwrap_err();
	assert!(err.to_string().contains("no component person"));
}

#[test]
fn steps_and_files_must_match() {
	let file = |name: &str, m: &Migration| {
		let json = serde_json::to_vec(m).unwrap();
		(format!("{name}.json"), json)
	};
	let s = site();
	let first = migration("0001", &Schema::new(), &s);
	let second = Migration {
		data: true,
		schema: s.clone(),
		..Default::default()
	};
	let files = vec![file("0001", &first), file("0002_fill", &second)];
	let migrations = |steps: &[&str]| {
		let mut m = Migrations {
			dir: Default::default(),
			files: files.clone(),
			steps: Default::default(),
		};
		for name in steps {
			m = m.data(name, |_| Ok(()));
		}
		m
	};

	let err = migrations(&[]).load().unwrap_err();
	assert!(err.contains("0002_fill has no step"), "{err}");
	let err = migrations(&["0002_fill", "0001"]).load().unwrap_err();
	assert!(err.contains("\"0001\""), "{err}");
	assert!(!err.contains("0002_fill"), "{err}");

	let loaded = migrations(&["0002_fill"]).load().unwrap();
	assert!(loaded[1].data && loaded[1].step.is_some());
	assert!(loaded[0].step.is_none());
}
