use super::*;

impl Op {
	pub fn apply_schema(&self, schema: &mut Schema) {
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
	pub fn apply_entry(
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
			| Self::AddField { .. }
			| Self::AddSite { .. }
			| Self::RemoveSite { .. }
			| Self::AddSection { .. }
			| Self::RemoveSection { .. }
			| Self::ChangeSection { .. } => {}
			Self::RemoveComponent { component } => {
				if ty == component {
					return false;
				}
				walk_mut(schema, comp, fields, &mut |_, obj| {
					for v in obj.values_mut() {
						for_blocks(v, &mut |list| {
							list.retain(|b| b["type"] != component.as_str())
						});
					}
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
				walk_mut(&after, comp, fields, &mut |_, obj| {
					for v in obj.values_mut() {
						for_blocks(v, &mut |list| {
							for b in list
								.iter_mut()
								.filter(|b| b["type"] == from.as_str())
							{
								b["type"] = to.clone().into();
							}
						});
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
				let same_kind =
					old.is_some_and(|o| match (&o.kind, &field.kind) {
						(
							FieldKind::Custom { ty: a, .. },
							FieldKind::Custom { ty: b, .. },
						) => a == b,
						(a, b) => {
							std::mem::discriminant(a)
								== std::mem::discriminant(b)
						}
					});
				if !same_kind {
					walk_mut(schema, comp, fields, &mut |c, obj| {
						if &c.handle == component {
							obj.remove(&field.handle);
						}
					});
				}
			}
		}

		true
	}

	/// One line for the review of a new migration, `!` marks lost content.
	pub fn describe(&self) -> String {
		match self {
			Self::AddComponent { component } => {
				format!("  add component {}", component.handle)
			}
			Self::RemoveComponent { component } => format!(
				"! remove component {component}, its entries and blocks are deleted"
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
			Self::ChangeField { component, field } => format!(
				"  change field {component}.{} (content is dropped if the kind changed)",
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
				"  change section {} (entries of types it no longer has are deleted)",
				section.handle
			),
		}
	}
}

/// Calls `f` with the blocks of a value: a blocks list as it is, the
/// blocks of a rich text as their attrs (they look the same: id, type,
/// fields). Afterwards the rich text gets them back.
pub(super) fn for_blocks(v: &mut Value, f: &mut impl FnMut(&mut Vec<Value>)) {
	match v {
		Value::Array(list) => f(list),
		Value::Object(o) if o.get("type").is_some_and(|t| t == "doc") => {
			let Some(Value::Array(content)) = o.get_mut("content") else {
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
					out.push(
						serde_json::json!({ "type": "embed", "attrs": a }),
					);
				}
			}
			*content = out;
		}
		_ => {}
	}
}
