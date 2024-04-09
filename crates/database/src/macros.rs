#[macro_export]
macro_rules! migration_files {
	($($file:expr),* $(,)?) => {
		&[
			$(
				(
					stringify!($file),
					include_str!(concat!("../migrations/", $file, ".sql")),
				)
			),*
		]
	};
}
