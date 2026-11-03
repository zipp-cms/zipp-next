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

#[test]
fn orders_by_our_names_and_crafts() {
	let order = |q: &str| parse(q).map(|q| (q.order, q.desc));
	let created = Ok((Order::Column("created_at"), false));
	assert_eq!(order("order=created"), created);
	assert_eq!(order("orderBy=dateCreated"), created);
	assert_eq!(
		order("orderBy=postDate DESC"),
		Ok((Order::Column("post_date"), true))
	);
	assert!(order("order=-updated").unwrap().1);
	assert!(order("orderBy=price").unwrap_err().contains("need a type"));

	// without one the section's: a structure as its tree, a channel
	// newest first
	assert_eq!(order("section=pages"), Ok((Order::Structure, false)));
	assert_eq!(
		order("section=events"),
		Ok((Order::Column("post_date"), true))
	);
	assert_eq!(order(""), Ok((Order::Column("updated_at"), true)));
	// a full text search the best match first, also in a section
	let best = Ok((Order::Relevance, false));
	assert_eq!(order("text=konzert&section=events"), best);
	// signs alone search nothing, there is no relevance
	assert_eq!(order("text=-&section=pages"), Ok((Order::Structure, false)));
	assert_eq!(
		order("text=konzert&order=title"),
		Ok((Order::Column("title"), false))
	);
}

#[test]
fn published_queries_take_str_params() {
	let q = Query::published(&site(), &[("site", "en"), ("type", "event")]);
	let q = q.unwrap();
	assert!(q.published_only);
	assert_eq!((&*q.site, q.ty.as_deref()), ("en", Some("event")));
	assert!(Query::published(&site(), &[("site", "fr")]).is_err());
}
