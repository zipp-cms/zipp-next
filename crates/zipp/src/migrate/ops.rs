use super::*;

impl Op {
	pub(crate) fn apply_schema(&self, schema: &mut Schema) {
		match self {
			Self::AddComponent { component } => {
				schema.components.push(component.clone())
			}
			Self::RemoveComponent { component } => {
				schema.components.retain(|c| &c.handle != component)
			}
			Self::RenameComponent { from, to } => {
				if let Some(c) = schema.get_mut(from) {
					c.handle = to.clone();
				}
			}
			Self::AddField { component, field } => {
				if let Some(c) = schema.get_mut(component) {
					c.fields.push(field.clone());
				}
			}
			Self::RemoveField { component, field } => {
				if let Some(c) = schema.get_mut(component) {
					c.fields.retain(|f| &f.handle != field);
				}
			}
			Self::RenameField {
				component,
				from,
				to,
			} => {
				let c = schema.get_mut(component);
				let f = c.and_then(|c| {
					c.fields.iter_mut().find(|f| &f.handle == from)
				});
				if let Some(f) = f {
					f.handle = to.clone();
				}
			}
			Self::ChangeField { component, field } => {
				let c = schema.get_mut(component);
				let f = c.and_then(|c| {
					c.fields.iter_mut().find(|f| f.handle == field.handle)
				});
				if let Some(f) = f {
					*f = field.clone();
				}
			}
			Self::AddSite { site } => schema.sites.push(site.clone()),
			Self::RemoveSite { site } => {
				schema.sites.retain(|s| &s.handle != site)
			}
			Self::AddSection { section } => {
				schema.sections.push(section.clone())
			}
			Self::RemoveSection { section } => {
				schema.sections.retain(|s| &s.handle != section)
			}
			Self::ChangeSection { section } => {
				let s = schema
					.sections
					.iter_mut()
					.find(|s| s.handle == section.handle);
				if let Some(s) = s {
					*s = section.clone();
				}
			}
		}
	}

	/// Changes one entry. `schema` is the schema before this op. Returns
	/// false when the entry has to be deleted.
	pub(crate) fn apply_entry(
		&self,
		schema: &Schema,
		ty: &mut String,
		fields: &mut Value,
	) -> bool {
		let Some(comp) = schema.get(ty) else {
			return true;
		};

		match self {
			// sites and sections change rows, see apply::apply_migrations
			Self::AddComponent { .. }
			| Self::AddSite { .. }
			| Self::RemoveSite { .. }
			| Self::AddSection { .. }
			| Self::RemoveSection { .. }
			| Self::ChangeSection { .. } => {}
			Self::RemoveComponent { component } => {
				if ty == component {
					return false;
				}
				walk_mut(schema, comp, fields, &mut |c, obj| {
					block_lists(c, obj, &mut |list| {
						list.retain(|b| b["type"] != component.as_str())
					});
				});
			}
			Self::RenameComponent { from, to } => {
				// walk with the new names, a block is renamed before the
				// walk goes into it
				let mut after = schema.clone();
				self.apply_schema(&mut after);
				if ty == from {
					*ty = to.clone();
				}
				let comp = after.get(ty).unwrap();
				walk_mut(&after, comp, fields, &mut |c, obj| {
					block_lists(c, obj, &mut |list| {
						for b in list
							.iter_mut()
							.filter(|b| b["type"] == from.as_str())
						{
							b["type"] = to.clone().into();
						}
					});
				});
			}
			// the content there starts with the default too, so a new
			// boolean that is on is on for the old entries
			Self::AddField { component, field } => {
				let Some(default) = default_of(schema, field) else {
					return true;
				};
				walk_mut(schema, comp, fields, &mut |c, obj| {
					if &c.handle == component {
						obj.entry(&field.handle).or_insert(default.clone());
					}
				});
			}
			Self::RemoveField { component, field } => {
				walk_mut(schema, comp, fields, &mut |c, obj| {
					if &c.handle == component {
						obj.remove(field);
					}
				});
			}
			Self::RenameField {
				component,
				from,
				to,
			} => {
				walk_mut(schema, comp, fields, &mut |c, obj| {
					if &c.handle == component
						&& let Some(v) = obj.remove(from)
					{
						obj.insert(to.clone(), v);
					}
				});
			}
			Self::ChangeField { component, field } => {
				let old = schema.get(component).and_then(|c| {
					c.fields.iter().find(|f| f.handle == field.handle)
				});
				// a group of another component is another kind too, its
				// keys are those of the other component
				let same_kind =
					old.is_some_and(|o| match (&o.kind, &field.kind) {
						(
							FieldKind::Custom { ty: a, .. },
							FieldKind::Custom { ty: b, .. },
						) => a == b,
						(
							FieldKind::Group { component: a },
							FieldKind::Group { component: b },
						) => a == b,
						(a, b) => {
							std::mem::discriminant(a)
								== std::mem::discriminant(b)
						}
					});
				// another kind starts over like a new field, with its default
				if !same_kind {
					let default = default_of(schema, field);
					walk_mut(schema, comp, fields, &mut |c, obj| {
						if &c.handle != component {
							return;
						}
						match &default {
							Some(d) => {
								obj.insert(field.handle.clone(), d.clone())
							}
							None => obj.remove(&field.handle),
						};
					});
				}
				// a column of a table likewise, its cells go
				let changed = match (old.map(|o| &o.kind), &field.kind) {
					(
						Some(FieldKind::Table { columns: before }),
						FieldKind::Table { columns },
					) => changed_columns(before, columns),
					_ => vec![],
				};
				if !changed.is_empty() {
					walk_mut(schema, comp, fields, &mut |c, obj| {
						if &c.handle != component {
							return;
						}
						let rows = obj
							.get_mut(&field.handle)
							.and_then(Value::as_array_mut);
						for row in rows.into_iter().flatten() {
							if let Some(row) = row.as_object_mut() {
								for h in &changed {
									row.remove(h);
								}
							}
						}
					});
				}
			}
		}

		true
	}

	/// One line for the review of a new migration, `!` marks lost content.
	pub(crate) fn describe(&self) -> String {
		match self {
			Self::AddComponent { component } => {
				format!("  add component {}", component.handle)
			}
			Self::RemoveComponent { component } => format!(
				"! remove component {component}, its entries and blocks are deleted (children of its entries move up)"
			),
			Self::RenameComponent { from, to } => {
				format!("  rename component {from} to {to}")
			}
			Self::AddField { component, field } => {
				format!("  add field {component}.{}", field.handle)
			}
			Self::RemoveField { component, field } => format!(
				"! remove field {component}.{field}, its content is deleted"
			),
			Self::RenameField {
				component,
				from,
				to,
			} => format!("  rename field {component}.{from} to {to}"),
			// chosen entries and assets stay, a save refuses those the
			// field no longer allows (fewer types, sections or files)
			Self::ChangeField { component, field }
				if matches!(
					field.kind,
					FieldKind::Relation { .. } | FieldKind::Assets { .. }
				) =>
			{
				format!(
					"  change field {component}.{} (content starts over if the kind changed, a save refuses what it no longer allows until it is removed)",
					field.handle
				)
			}
			Self::ChangeField { component, field } => format!(
				"  change field {component}.{} (content starts over if the kind changed)",
				field.handle
			),
			Self::AddSite { site } => format!(
				"  add site {}, it starts as a copy of the primary site",
				site.handle
			),
			Self::RemoveSite { site } => {
				format!("! remove site {site}, its content is deleted")
			}
			Self::AddSection { section } => {
				format!("  add section {}", section.handle)
			}
			Self::RemoveSection { section } => {
				format!("! remove section {section}, its entries are deleted")
			}
			Self::ChangeSection { section } => format!(
				"  change section {} (entries of types it no longer has are deleted, their children move up)",
				section.handle
			),
		}
	}
}

/// The columns of a table that are still there with another kind.
fn changed_columns(before: &[Column], after: &[Column]) -> Vec<String> {
	after
		.iter()
		.filter(|c| {
			before.iter().any(|b| {
				b.handle == c.handle
					&& std::mem::discriminant(&b.kind)
						!= std::mem::discriminant(&c.kind)
			})
		})
		.map(|c| c.handle.clone())
		.collect()
}

/// Calls `f` with each list of blocks in the fields of `comp`: a blocks
/// field, the blocks embedded in a rich text (taken out and written back,
/// removed ones leave the document). Values that only look like blocks,
/// navigation items or table rows with a `type`, stay as they are; a
/// group's fields come with `walk_mut`.
fn block_lists(
	comp: &Component,
	obj: &mut Map<String, Value>,
	f: &mut impl FnMut(&mut Vec<Value>),
) {
	for field in &comp.fields {
		let Some(v) = obj.get_mut(&field.handle) else {
			continue;
		};
		match field.kind.holds() {
			Holds::Blocks => {
				if let Value::Array(list) = v {
					f(list)
				}
			}
			Holds::RichText => embedded_blocks(v, f),
			Holds::Group(_) | Holds::Nothing => {}
		}
	}
}

/// The blocks of a rich text document as one list for `f`.
fn embedded_blocks(doc: &mut Value, f: &mut impl FnMut(&mut Vec<Value>)) {
	let Some(Value::Array(content)) = doc.get_mut("content") else {
		return;
	};
	let mut attrs: Vec<Value> = content
		.iter()
		.filter(|n| n["type"] == "embed")
		.map(|n| n["attrs"].clone())
		.collect();
	f(&mut attrs);

	// put the changed blocks back, the removed ones go away
	let mut out = vec![];
	for node in content.drain(..) {
		if node["type"] != "embed" {
			out.push(node);
		} else if let Some(a) =
			attrs.iter().find(|a| a["id"] == node["attrs"]["id"])
		{
			out.push(serde_json::json!({ "type": "embed", "attrs": a }));
		}
	}
	*content = out;
}
