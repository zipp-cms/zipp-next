use serde_json::Map;

use super::*;

/// When a field or a tab shows in the editor: by the value of another
/// field of the same entry type or block, like Craft's field layout
/// conditions. A hidden field keeps its value, GraphQL returns it as
/// stored, and it isn't required while it is hidden.
///
/// `Field::text("intro").when(Condition::on("hasPage"))`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "test", rename_all = "camelCase")]
#[non_exhaustive]
pub enum Condition {
	/// The field has one of the values: a boolean on or off, an option of
	/// a dropdown or a button group.
	OneOf { field: String, values: Vec<Value> },
	/// The field has a value.
	Filled { field: String },
	/// The field has no value.
	Empty { field: String },
}

impl Condition {
	/// A boolean field is on.
	pub fn on(field: &str) -> Self {
		Self::is(field, true)
	}

	/// A boolean field is off, also when it was never set.
	pub fn off(field: &str) -> Self {
		Self::is(field, false)
	}

	/// The field has this value: `Condition::is("genre", "concert")`.
	pub fn is(field: &str, value: impl Into<Value>) -> Self {
		Self::one_of(field, [value])
	}

	/// The field has one of the values, like options of a dropdown.
	pub fn one_of<V: Into<Value>>(
		field: &str,
		values: impl IntoIterator<Item = V>,
	) -> Self {
		Self::OneOf {
			field: field.into(),
			values: values.into_iter().map(Into::into).collect(),
		}
	}

	pub fn filled(field: &str) -> Self {
		Self::Filled {
			field: field.into(),
		}
	}

	pub fn empty(field: &str) -> Self {
		Self::Empty {
			field: field.into(),
		}
	}

	/// The field it looks at.
	pub fn field(&self) -> &str {
		match self {
			Self::OneOf { field, .. }
			| Self::Filled { field }
			| Self::Empty { field } => field,
		}
	}

	/// Whether it holds for the values of the component's fields.
	fn holds(&self, comp: &Component, values: &Map<String, Value>) -> bool {
		let Some(field) = comp.fields.iter().find(|f| f.handle == self.field())
		else {
			return true;
		};
		let value = values.get(&field.handle).unwrap_or(&Value::Null);
		match self {
			// a boolean that was never set is off
			Self::OneOf { values, .. }
				if value.is_null() && field.kind == FieldKind::Boolean =>
			{
				values.contains(&Value::Bool(false))
			}
			Self::OneOf { values, .. } => values.contains(value),
			Self::Filled { .. } => !crate::content::is_empty(field, value),
			Self::Empty { .. } => crate::content::is_empty(field, value),
		}
	}
}

impl Component {
	/// The conditions of a field: its own and the one of its tab.
	pub(crate) fn conditions_of(&self, handle: &str) -> Vec<&Condition> {
		let own = self.fields.iter().find(|f| f.handle == handle);
		let tab = self
			.tabs
			.iter()
			.find(|t| t.fields.iter().any(|f| f == handle));
		[
			own.and_then(|f| f.when.as_ref()),
			tab.and_then(|t| t.when.as_ref()),
		]
		.into_iter()
		.flatten()
		.collect()
	}

	/// Whether a field shows with these values: its conditions hold and
	/// the fields they look at show too, a field under a hidden switch
	/// doesn't count.
	pub fn is_shown(&self, handle: &str, values: &Map<String, Value>) -> bool {
		self.conditions_of(handle)
			.into_iter()
			.all(|c| c.holds(self, values) && self.is_shown(c.field(), values))
	}
}

impl Schema {
	/// Shows a tab of an entry type or block only when the condition holds, after
	/// the type: `.tab_when("page", "Page", Condition::on("hasPage"))`.
	pub fn tab_when(
		mut self,
		component: &str,
		tab: &str,
		condition: Condition,
	) -> Self {
		let t = self
			.get_mut(component)
			.and_then(|c| c.tabs.iter_mut().find(|t| t.name == tab));
		let Some(t) = t else {
			panic!("tab_when: {component} has no tab {tab}");
		};
		t.when = Some(condition);
		self
	}

	/// A condition looks at a field of the component that takes such a
	/// test, `of` is what has it: a field or a tab's fields.
	pub(super) fn check_condition(
		&self,
		comp: &Component,
		cond: &Condition,
		of: &[String],
	) -> Result<(), String> {
		let name = cond.field();
		let field = comp.fields.iter().find(|f| f.handle == name);
		let Some(field) = field.filter(|f| f.kind.has_value()) else {
			return Err(format!("condition: unknown field {name}"));
		};
		if of.iter().any(|h| h == name) {
			return Err(format!("condition: it can't look at {name} itself"));
		}

		match cond {
			Condition::OneOf { values, .. } => {
				if !matches!(
					field.kind,
					FieldKind::Boolean
						| FieldKind::Dropdown { .. }
						| FieldKind::Custom { .. }
				) {
					return Err(format!(
						"condition: {name} is no boolean, dropdown or field \
						type, test whether it is filled"
					));
				}
				if values.is_empty() {
					return Err("condition: it needs values".into());
				}
				if let Some(v) = values
					.iter()
					.find(|v| !crate::content::takes(self, field, v))
				{
					return Err(format!("condition: {name} can't be {v}"));
				}
			}
			Condition::Filled { .. } | Condition::Empty { .. } => {
				if matches!(field.kind, FieldKind::Group { .. }) {
					return Err(format!(
						"condition: a group like {name} is never empty"
					));
				}
			}
		}
		Ok(())
	}

	/// No field hides behind itself through other fields.
	pub(super) fn check_cycles(comp: &Component) -> Result<(), String> {
		fn walk<'a>(
			comp: &'a Component,
			handle: &'a str,
			path: &mut Vec<&'a str>,
		) -> Result<(), String> {
			if path.contains(&handle) {
				path.push(handle);
				return Err(format!(
					"conditions go in a circle: {}",
					path.join(" → ")
				));
			}
			path.push(handle);
			for c in comp.conditions_of(handle) {
				walk(comp, c.field(), path)?;
			}
			path.pop();
			Ok(())
		}
		for f in &comp.fields {
			walk(comp, &f.handle, &mut vec![])?;
		}
		Ok(())
	}
}
