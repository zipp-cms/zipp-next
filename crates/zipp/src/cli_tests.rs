use super::*;

#[test]
fn refuses_an_empty_graphql_token() {
	let serve = |token: &str| {
		Opts::try_parse_from(["zipp", "serve", "--graphql-token", token])
	};
	assert!(serve("").is_err());
	assert!(serve("secret").is_ok());
}

#[test]
fn a_data_step_gets_a_rust_name() {
	assert_eq!(step_name("0003_split_names"), "split_names");
	assert_eq!(step_name("0003_split-names"), "split_names");
	assert_eq!(step_name("0003_2fa"), "step_2fa");
	assert_eq!(step_name("0003"), "data_step");
	let stub = data_stub("0003_split-names");
	assert!(
		stub.contains(".data_migration(\"0003_split-names\", split_names)")
	);
	assert!(stub.contains("fn split_names(tx: &DataTx)"));
}

#[test]
fn a_user_gets_a_name() {
	let db = crate::db::Db::memory().unwrap();
	let conn = db.conn();
	let opts = Opts::try_parse_from([
		"zipp",
		"user",
		"s@x.ch",
		"--name",
		"Sören Meier",
	])
	.unwrap();
	assert!(matches!(
		opts.cmd,
		Cmd::User { name: Some(ref n), .. } if n == "Sören Meier"
	));

	// a new user, the slug of the name
	store_user(&conn, "s@x.ch", "password1", None, Some("Sören Meier"))
		.unwrap();
	let u = &crate::users::list(&conn).unwrap()[0].user;
	assert_eq!(
		(u.name.as_str(), u.slug.as_str()),
		("Sören Meier", "soeren-meier")
	);

	// one without a name gets it later, a slug of a name stays
	store_user(&conn, "a@x.ch", "password1", None, None).unwrap();
	store_user(&conn, "a@x.ch", "password2", None, Some("Anna")).unwrap();
	store_user(&conn, "s@x.ch", "password2", None, Some("Sören M.")).unwrap();
	let users = crate::users::list(&conn).unwrap();
	let users: Vec<_> = users
		.iter()
		.map(|u| (u.user.name.as_str(), u.user.slug.as_str()))
		.collect();
	assert!(users.contains(&("Anna", "anna")));
	assert!(users.contains(&("Sören M.", "soeren-meier")));
}
