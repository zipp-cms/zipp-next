use super::*;

/// A relation in the content: `path` is where it is, `body.2.link`.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RelationRef {
	pub path: String,
	pub target: Target,
	pub ids: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Target {
	/// entries of these types, only of these sections if any
	Entries {
		types: Vec<String>,
		sections: Vec<String>,
	},
	/// assets, only images if `images`, only these extensions if any
	Assets {
		images: bool,
		extensions: Vec<String>,
	},
}

/// All relations to entries and assets, also those inside blocks.
pub(crate) fn relations(
	schema: &Schema,
	comp: &Component,
	fields: &Value,
) -> Vec<RelationRef> {
	let mut out = vec![];
	collect_relations(schema, comp, fields, "", &mut out);
	out
}

/// The relation of one link, to an entry (of `sections` if any) or an
/// asset.
pub(super) fn link_relation(
	schema: &Schema,
	link: &Value,
	sections: &[String],
	path: &str,
) -> Option<RelationRef> {
	let target = match link_kind(link)? {
		LinkKind::Entry => Target::Entries {
			types: schema
				.components
				.iter()
				.filter(|c| c.entry)
				.map(|c| c.handle.clone())
				.collect(),
			sections: sections.to_vec(),
		},
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
			FieldKind::Relation {
				types, sections, ..
			} if value.is_array() => out.push(RelationRef {
				path: path.clone(),
				target: Target::Entries {
					types: types.clone(),
					sections: sections.clone(),
				},
				ids: ids(),
			}),
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
			FieldKind::Link { sections, .. } => {
				out.extend(link_relation(schema, value, sections, &path))
			}
			FieldKind::RichText { sections, .. } => {
				for link in richtext::links(value) {
					out.extend(link_relation(schema, link, sections, &path));
				}
			}
			FieldKind::Navigation { sections, .. } => {
				nav_relations(schema, value, sections, &path, out)
			}
			// and so is one in a cell of a table
			FieldKind::Table { columns } => {
				let rows = value.as_array().into_iter().flatten();
				for (i, row) in rows.enumerate() {
					for c in columns {
						if matches!(c.kind, ColumnKind::Link { .. }) {
							let p = format!("{path}.{i}.{}", c.handle);
							out.extend(link_relation(
								schema,
								&row[&c.handle],
								&[],
								&p,
							));
						}
					}
				}
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
			| FieldKind::Email
			| FieldKind::Markdown
			| FieldKind::Date { .. }
			| FieldKind::Checkboxes { .. }
			| FieldKind::Texts { .. }
			| FieldKind::Dropdown { .. }
			| FieldKind::ButtonGroup { .. }
			| FieldKind::Blocks { .. }
			| FieldKind::Note { .. }
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
	sections: &[String],
	path: &str,
	out: &mut Vec<RelationRef>,
) {
	for (i, item) in items.as_array().into_iter().flatten().enumerate() {
		let p = format!("{path}.{i}");
		out.extend(link_relation(schema, item, sections, &p));
		let children = format!("{p}.children");
		nav_relations(schema, &item["children"], sections, &children, out);
	}
}

/// Takes the relations and links whose target `gone(kind, id)` says is
/// gone out of the content: ids from relation and asset fields, the
/// target from a link (a link field, a cell or a button without a target
/// is left out when validated again), the link mark from a text; an item
/// of a navigation gives its place to the items under it. Field types of
/// other crates keep theirs.
pub(crate) fn drop_links(
	schema: &Schema,
	comp: &Component,
	fields: &mut Value,
	gone: &dyn Fn(LinkKind, i64) -> bool,
) {
	walk_mut(schema, comp, fields, &mut |c, obj| {
		for field in &c.fields {
			let Some(value) = obj.get_mut(&field.handle) else {
				continue;
			};
			match &field.kind {
				FieldKind::Relation { .. } => {
					drop_ids(value, LinkKind::Entry, gone)
				}
				FieldKind::Assets { .. } => {
					drop_ids(value, LinkKind::Asset, gone)
				}
				FieldKind::Link { .. } => clear_link(value, gone),
				FieldKind::RichText { .. } => drop_text_links(value, gone),
				FieldKind::Navigation { .. } => drop_nav_links(value, gone),
				FieldKind::Table { columns } => {
					let rows = value.as_array_mut().into_iter().flatten();
					for row in rows.filter_map(Value::as_object_mut) {
						for c in columns {
							if matches!(c.kind, ColumnKind::Link { .. })
								&& let Some(cell) = row.get_mut(&c.handle)
							{
								clear_link(cell, gone);
							}
						}
					}
				}
				_ => {}
			}
		}
	});
}

fn drop_ids(
	value: &mut Value,
	kind: LinkKind,
	gone: &dyn Fn(LinkKind, i64) -> bool,
) {
	if let Value::Array(ids) = value {
		ids.retain(|id| id.as_i64().is_none_or(|id| !gone(kind, id)));
	}
}

/// Whether a link points to something gone.
fn links_gone(link: &Value, gone: &dyn Fn(LinkKind, i64) -> bool) -> bool {
	match (link_kind(link), link["value"].as_i64()) {
		(Some(kind), Some(id)) => gone(kind, id),
		_ => false,
	}
}

fn clear_link(link: &mut Value, gone: &dyn Fn(LinkKind, i64) -> bool) {
	if links_gone(link, gone) {
		link["value"] = Value::Null;
	}
}

fn drop_text_links(node: &mut Value, gone: &dyn Fn(LinkKind, i64) -> bool) {
	if node["type"] == "button"
		&& let Some(link) = node.pointer_mut("/attrs/link")
	{
		clear_link(link, gone);
	}
	if let Some(Value::Array(marks)) = node.get_mut("marks") {
		marks.retain(|m| {
			m["type"] != "link" || !links_gone(&m["attrs"]["link"], gone)
		});
		if marks.is_empty() {
			node.as_object_mut().unwrap().remove("marks");
		}
	}
	if let Some(Value::Array(content)) = node.get_mut("content") {
		for child in content {
			drop_text_links(child, gone);
		}
	}
}

fn drop_nav_links(items: &mut Value, gone: &dyn Fn(LinkKind, i64) -> bool) {
	let Value::Array(list) = items else { return };
	let mut out = vec![];
	for mut item in list.drain(..) {
		if let Some(children) = item.get_mut("children") {
			drop_nav_links(children, gone);
		}
		if !links_gone(&item, gone) {
			out.push(item);
			continue;
		}
		if let Some(Value::Array(children)) = item.get_mut("children") {
			out.append(children);
		}
	}
	*list = out;
}
