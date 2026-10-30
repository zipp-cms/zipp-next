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
	let target = match link["type"].as_str() {
		Some("entry") => Target::Entries(
			schema
				.components
				.iter()
				.filter(|c| c.entry)
				.map(|c| c.handle.clone())
				.collect(),
		),
		Some("asset") => Target::Assets {
			images: false,
			extensions: vec![],
		},
		_ => return None,
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

		// links in a rich text and the relations of its blocks
		if let FieldKind::RichText { .. } = field.kind {
			let doc = &fields[&field.handle];
			for link in richtext::links(doc) {
				out.extend(link_relation(schema, link, &path));
			}
			let content = doc["content"].as_array().into_iter().flatten();
			for (i, node) in content.enumerate() {
				let bt = node["attrs"]["type"].as_str().unwrap_or_default();
				if let (true, Some(c)) =
					(node["type"] == "embed", schema.get(bt))
				{
					let prefix = format!("{path}.{i}.");
					collect_relations(
						schema,
						c,
						&node["attrs"]["fields"],
						&prefix,
						out,
					);
				}
			}
			continue;
		}

		if let FieldKind::Group { component } = &field.kind {
			if let (Some(c), Some(group)) =
				(schema.get(component), fields.get(&field.handle))
			{
				collect_relations(schema, c, group, &format!("{path}."), out);
			}
			continue;
		}

		// a link to an entry or an asset is a relation too
		if let FieldKind::Link { .. } = field.kind {
			out.extend(link_relation(schema, &fields[&field.handle], &path));
			continue;
		}
		// a field type of another crate says what it relates to
		if let FieldKind::Custom { ty, options } = &field.kind {
			let value = &fields[&field.handle];
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
			continue;
		}
		if let FieldKind::Navigation { .. } = field.kind {
			nav_relations(schema, &fields[&field.handle], &path, out);
			continue;
		}

		let Some(Value::Array(items)) = fields.get(&field.handle) else {
			continue;
		};

		match &field.kind {
			FieldKind::Relation { types, .. } => out.push(RelationRef {
				path,
				target: Target::Entries(types.clone()),
				ids: items.iter().filter_map(Value::as_i64).collect(),
			}),
			FieldKind::Assets {
				images, extensions, ..
			} => out.push(RelationRef {
				path,
				target: Target::Assets {
					images: *images,
					extensions: extensions.clone(),
				},
				ids: items.iter().filter_map(Value::as_i64).collect(),
			}),
			FieldKind::Blocks { .. } => {
				for (i, block) in items.iter().enumerate() {
					let ty = block["type"].as_str().unwrap_or_default();
					if let Some(c) = schema.get(ty) {
						let prefix = format!("{path}.{i}.");
						collect_relations(
							schema,
							c,
							&block["fields"],
							&prefix,
							out,
						);
					}
				}
			}
			_ => {}
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
