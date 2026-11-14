use super::*;

/// A plugin with the tables of `TABLES[..n]`, like a newer version of it
/// adds more.
struct Tabled(usize);

const TABLES: &[&str] = &[
	"CREATE TABLE tabled_a (id INTEGER PRIMARY KEY);",
	"CREATE TABLE tabled_b (id INTEGER PRIMARY KEY);",
];

impl crate::Plugin for Tabled {
	fn name(&self) -> &'static str {
		"tabled"
	}

	fn tables(&self) -> &'static [&'static str] {
		&TABLES[..self.0]
	}
}

#[test]
fn plugin_tables_are_made_once_and_grow() {
	let db = crate::db::Db::memory().unwrap();
	let conn = db.conn();
	let schema = |n| crate::Schema::new().plugin(Tabled(n));

	create_tables(&conn, &schema(1)).unwrap();
	conn.execute("INSERT INTO tabled_a (id) VALUES (1)", [])
		.unwrap();
	// again: nothing runs twice, the row stays
	create_tables(&conn, &schema(1)).unwrap();
	create_tables(&conn, &schema(2)).unwrap();
	let rows: i64 = conn
		.query_row("SELECT count(*) FROM tabled_a", [], |r| r.get(0))
		.unwrap();
	assert_eq!(rows, 1);
	conn.execute("INSERT INTO tabled_b (id) VALUES (1)", [])
		.unwrap();

	// an older version of the plugin doesn't run on its newer tables
	let err = create_tables(&conn, &schema(1)).unwrap_err().to_string();
	assert!(err.contains("newer version"), "{err}");
}
