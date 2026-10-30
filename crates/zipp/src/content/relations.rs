use super::*;

/// A relation in the content: `path` is where it is, `body.2.link`.
#[derive(Debug, Clone, PartialEq)]
pub struct RelationRef {
	pub path: String,
	pub target: Target,
	pub ids: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
	/// entries of these types
	Entries(Vec<String>),
	/// assets, only images if `images`, only these extensions if any
	Assets {
		images: bool,
		extensions: Vec<String>,
	},
}

/// All relations to entries and assets, also those inside blocks.
pub fn relations(
	schema: &Schema,
	comp: &Component,
	fields: &Value,
) -> Vec<RelationRef> {
	let mut out = vec![];
	collect_relations(schema, comp, fields, "", &mut out);
	out
}

/// The relation of one link, to an entry or an asset.
pub(super) fn link_relation(
	schema: &Schema,
	link: &Value,
	path: &str,
) -> Option<RelationRef> {
	let target = match link_kind(link)? {
		LinkKind::Entry => Target::Entries(
			schema
				.components
				.iter()
				.filter(|c| c.entry)
				.map(|c| c.handle.clone())
				.collect(),
		),
		LinkKind::Asset => Target::Assets {
			images: false,
			extensions: vec![],
		},
		LinkKind::Url | LinkKind::Email | LinkKind::Phone => return None,
	};
	Some(RelationRef {
		path: path.into(),
		target,
		ids: vec![link["value"].as_i64()?],
	})
}

pub(super) fn collect_relations(
	schema: &Schema,
	comp: &Component,
	fields: &Value,
	prefix: &str,
	out: &mut Vec<RelationRef>,
) {
	for field in &comp.fields {
		let path = format!("{prefix}{}", field.handle);
		let value = &fields[&field.handle];
		let ids = || value.as_array().into_iter().flatten();
		let ids = || ids().filter_map(Value::as_i64).collect::<Vec<_>>();

		// the field's own, every kind says
		match &field.kind {
			FieldKind::Relation { types, .. } if value.is_array() => {
				out.push(RelationRef {
					path: path.clone(),
					target: Target::Entries(types.clone()),
					ids: ids(),
				})
			}
			FieldKind::Assets {
				images, extensions, ..
			} if value.is_array() => out.push(RelationRef {
				path: path.clone(),
				target: Target::Assets {
					images: *images,
					extensions: extensions.clone(),
				},
				ids: ids(),
			}),
			// a link to an entry or an asset is a relation too
			FieldKind::Link { .. } => {
				out.extend(link_relation(schema, value, &path))
			}
			FieldKind::RichText { .. } => {
				for link in richtext::links(value) {
					out.extend(link_relation(schema, link, &path));
				}
			}
			FieldKind::Navigation { .. } => {
				nav_relations(schema, value, &path, out)
			}
			// a field type of another crate says what it relates to
			FieldKind::Custom { ty, options } => {
				let ty = schema.field_types.get(ty);
				for r in ty.map_or(vec![], |t| t.relations(options, value)) {
					out.push(RelationRef {
						path: match r.path.is_empty() {
							true => path.clone(),
							false => format!("{path}.{}", r.path),
						},
						target: r.target,
						ids: r.ids,
					});
				}
			}
			FieldKind::Relation { .. }
			| FieldKind::Assets { .. }
			| FieldKind::Text { .. }
			| FieldKind::Number { .. }
			| FieldKind::Boolean
			| FieldKind::Markdown
			| FieldKind::Checkboxes { .. }
			| FieldKind::Table { .. }
			| FieldKind::Texts { .. }
			| FieldKind::Dropdown { .. }
			| FieldKind::Blocks { .. }
			| FieldKind::Group { .. } => {}
		}

		// and those of its blocks and groups
		for inner in inner(schema, field, value) {
			let prefix = inner.prefix(&path);
			collect_relations(schema, inner.comp, inner.fields, &prefix, out);
		}
	}
}

/// The links of a navigation and the ones under them.
fn nav_relations(
	schema: &Schema,
	items: &Value,
	path: &str,
	out: &mut Vec<RelationRef>,
) {
	for (i, item) in items.as_array().into_iter().flatten().enumerate() {
		let p = format!("{path}.{i}");
		out.extend(link_relation(schema, item, &p));
		nav_relations(schema, &item["children"], &format!("{p}.children"), out);
	}
}
