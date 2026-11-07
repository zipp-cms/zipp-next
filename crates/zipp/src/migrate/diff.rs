use super::*;

/// What has to happen to go from `old` to `new`.
pub(crate) fn diff(
	old: &Schema,
	new: &Schema,
	renames: &[Rename],
) -> Result<Vec<Op>, String> {
	// notes hold no content, adding, changing or removing one is free
	let (old, new) = (&without_notes(old), &without_notes(new));
	let mut ops = vec![];
	let mut cur = old.clone();
	let mut push = |op: Op, cur: &mut Schema| {
		op.apply_schema(cur);
		ops.push(op);
	};

	for rename in renames {
		let op = match rename {
			Rename::Component { from, to } => {
				if cur.get(from).is_none() || new.get(to).is_none() {
					return Err(format!("can't rename {from} to {to}"));
				}
				Op::RenameComponent {
					from: from.clone(),
					to: to.clone(),
				}
			}
			Rename::Field {
				component,
				from,
				to,
			} => {
				let has = |s: &Schema, f: &str| {
					s.get(component)
						.is_some_and(|c| c.fields.iter().any(|x| x.handle == f))
				};
				if !has(&cur, from) || !has(new, to) {
					return Err(format!(
						"can't rename {component}.{from} to {to}"
					));
				}
				Op::RenameField {
					component: component.clone(),
					from: from.clone(),
					to: to.clone(),
				}
			}
		};
		push(op, &mut cur);
	}

	// a component that switches between entry and block starts over
	let gone: Vec<String> = cur
		.components
		.iter()
		.filter(|c| new.get(&c.handle).is_none_or(|n| n.entry != c.entry))
		.map(|c| c.handle.clone())
		.collect();
	for component in gone {
		push(Op::RemoveComponent { component }, &mut cur);
	}

	// new components first, a field added to another one may be a group
	// of them and start with their defaults
	for comp in &new.components {
		if cur.get(&comp.handle).is_none() {
			let op = Op::AddComponent {
				component: comp.clone(),
			};
			push(op, &mut cur);
		}
	}

	for comp in &new.components {
		let old_comp = cur.get(&comp.handle).cloned().unwrap();
		let component = comp.handle.clone();

		for f in &old_comp.fields {
			if !comp.fields.iter().any(|n| n.handle == f.handle) {
				let op = Op::RemoveField {
					component: component.clone(),
					field: f.handle.clone(),
				};
				push(op, &mut cur);
			}
		}

		for f in &comp.fields {
			let op = match old_comp.field(&f.handle) {
				None => Op::AddField {
					component: component.clone(),
					field: f.clone(),
				},
				Some(o) if migrated_field(o) != migrated_field(f) => {
					Op::ChangeField {
						component: component.clone(),
						field: f.clone(),
					}
				}
				Some(_) => continue,
			};
			push(op, &mut cur);
		}
	}

	// add sites first, they copy the primary site of before
	for site in &new.sites {
		if cur.get_site(&site.handle).is_none() {
			push(Op::AddSite { site: site.clone() }, &mut cur);
		}
	}
	let gone: Vec<String> = cur
		.sites
		.iter()
		.filter(|s| new.get_site(&s.handle).is_none())
		.map(|s| s.handle.clone())
		.collect();
	for site in gone {
		push(Op::RemoveSite { site }, &mut cur);
	}

	let gone: Vec<String> = cur
		.sections
		.iter()
		.filter(|s| new.get_section(&s.handle).is_none())
		.map(|s| s.handle.clone())
		.collect();
	for section in gone {
		push(Op::RemoveSection { section }, &mut cur);
	}
	for section in &new.sections {
		let op = match cur.get_section(&section.handle) {
			None => Op::AddSection {
				section: section.clone(),
			},
			Some(o) if migrated_section(o) != migrated_section(section) => {
				Op::ChangeSection {
					section: section.clone(),
				}
			}
			Some(_) => continue,
		};
		push(op, &mut cur);
	}

	Ok(ops)
}

fn without_notes(schema: &Schema) -> Schema {
	let mut schema = schema.clone();
	for c in &mut schema.components {
		c.fields.retain(|f| f.kind.has_value());
	}
	schema
}

/// A field as migrations compare it: without what only the studio shows
/// or checks, so a new property makes a migration unless it is left out
/// here.
fn migrated_field(f: &Field) -> Field {
	Field {
		label: String::new(),
		required: false,
		instructions: None,
		default: None,
		width: 0,
		when: None,
		inline: None,
		..f.clone()
	}
}

/// A section as migrations compare it, see [`migrated_field`].
fn migrated_section(s: &Section) -> Section {
	Section {
		name: String::new(),
		..s.clone()
	}
}
