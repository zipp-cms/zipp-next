use super::*;
use crate::fixtures::site;

fn parse(q: &str) -> Result<Query, String> {
	let params: Vec<_> = q
		.split('&')
		.filter_map(|p| p.split_once('='))
		.map(|(k, v)| (k.to_string(), v.to_string()))
		.collect();
	Query::parse(&site(), &params)
}

#[test]
fn parses_filters() {
	let q = parse("f.price.lt=20&type=event&f.city=Bern&order=-price").unwrap();
	assert_eq!(q.site, "de");
	assert_eq!(q.filters[0].op, "<");
	assert_eq!(q.filters[0].value, Value::Real(20.0));
	assert_eq!(q.filters[1].value, Value::Text("Bern".into()));
	assert_eq!(q.order, Order::Field("price".into(), true));
	assert!(q.desc);
}

#[test]
fn rejects_bad_queries() {
	assert!(parse("f.city=Bern").unwrap_err().contains("need a type"));
	assert!(
		parse("type=event&f.page=1")
			.unwrap_err()
			.contains("not indexed")
	);
	assert!(parse("type=event&f.price=cheap").is_err());
	assert!(
		parse("type=text")
			.unwrap_err()
			.contains("unknown entry type")
	);
	assert!(parse("site=fr").unwrap_err().contains("unknown site"));
	assert!(parse("color=red").is_err());
}

#[test]
fn builds_sql_in_param_order() {
	let q = parse("type=event&f.city=Bern&order=price&site=en").unwrap();
	let (sql, params) = q.from_where();
	assert_eq!(
		params,
		[
			Value::Text("en".into()),
			Value::Text("city".into()),
			Value::Text("price".into()),
			Value::Text("event".into()),
			Value::Text("Bern".into()),
		]
	);
	assert!(sql.contains("f0.text = ?"));
	assert_eq!(q.order_by(), "ORDER BY o.num ASC, e.id ASC");
}

#[test]
fn structure_order_uses_the_tree() {
	let q = parse("section=pages&order=structure&uri=/about/").unwrap();
	let (sql, _) = q.from_where();
	let (with, params) = q.with();
	assert!(with.starts_with("WITH RECURSIVE tree"));
	assert_eq!(params, [Value::Text("pages".into())]);
	assert!(sql.contains("JOIN tree t"));
	assert_eq!(q.uri.as_deref(), Some("about"));
	assert_eq!(q.order_by(), "ORDER BY t.sort ASC, e.id ASC");

	// children need no tree
	let q = parse("section=pages&parent=5").unwrap();
	assert_eq!(q.with().0, "");
	assert!(!q.from_where().0.contains("tree"));
	assert_eq!(
		q.order_by(),
		"ORDER BY e.parent_id, e.position ASC, e.id ASC"
	);
}
