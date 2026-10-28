use super::*;

/// What has to happen to go from `old` to `new`.
pub fn diff(
	old: &Schema,
	new: &Schema,
	renames: &[Rename],
) -> Result<Vec<Op>, String> {
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

	for comp in &new.components {
		let Some(old_comp) = cur.get(&comp.handle).cloned() else {
			let op = Op::AddComponent {
				component: comp.clone(),
			};
			push(op, &mut cur);
			continue;
		};
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
			let op = match old_comp.fields.iter().find(|o| o.handle == f.handle)
			{
				None => Op::AddField {
					component: component.clone(),
					field: f.clone(),
				},
				Some(o)
					if o.kind != f.kind
						|| o.indexed != f.indexed
						|| o.translatable != f.translatable =>
				{
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
			Some(o)
				if o.kind != section.kind
					|| o.types != section.types
					|| o.uri != section.uri
					|| o.type_uris != section.type_uris
					|| o.site_uris != section.site_uris
					|| o.uri_if != section.uri_if =>
			{
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
