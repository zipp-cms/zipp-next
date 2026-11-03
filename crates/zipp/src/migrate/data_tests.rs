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
			c.set("town", format!("{city}town"));
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
				c.set("town", "half");
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
				c.set("content", text.to_uppercase());
				Ok(())
			})?;
			tx.each("seo", |c| {
				c.set("description", format!("{} of {}", c.title, c.site));
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
