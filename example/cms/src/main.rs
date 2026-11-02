//! The cms of the example site: the content model in `content.rs`, a
//! route and a hook in `routes.rs`.

mod content;
mod routes;
#[cfg(test)]
mod tests;

use zipp::Cms;

fn main() {
	Cms::new(content::schema(), zipp::migrations!())
		.routes(routes::routes())
		.on_change(routes::on_change)
		.run();
}
