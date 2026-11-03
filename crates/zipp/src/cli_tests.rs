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
