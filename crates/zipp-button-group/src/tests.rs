use super::*;
use zipp::Schema;

fn schema() -> Schema {
	Schema::new()
		.site("de", "Deutsch", "de-CH")
		.field_type(ButtonGroup)
		.section(zipp::Section::channel("events", ["event"]).uri("{slug}"))
		.entry(
			"event",
			"Event",
			[button_group(
				"genre",
				[
					("concert", "Konzert", "#2b6cb0"),
					("reading", "Lesung", "#2f855a"),
				],
			)],
		)
}

#[test]
fn checks_options_and_values() {
	let s = schema();
	s.check().unwrap();
	let comp = s.get("event").unwrap();
	let ok = zipp::content::validate(&s, comp, &json!({ "genre": "reading" }));
	assert_eq!(ok.unwrap(), json!({ "genre": "reading" }));
	let err = zipp::content::validate(&s, comp, &json!({ "genre": "film" }));
	assert_eq!(err.unwrap_err()[0].message, "not one of the options");

	let bad = Schema::new().site("de", "Deutsch", "de").entry(
		"event",
		"Event",
		[button_group("genre", [("a", "A", "red")])],
	);
	assert!(bad.check().unwrap_err().contains("unknown field type"));
	let bad = bad.field_type(ButtonGroup);
	assert!(bad.check().unwrap_err().contains("#rrggbb"));
}

#[test]
fn conditions_take_its_options() {
	use zipp::Condition;
	let with = |when: Condition| {
		let mut s = schema();
		let doors = Field::text("doors").when(when);
		s.get_mut("event").unwrap().fields.push(doors);
		s.check()
	};
	with(Condition::is("genre", "concert")).unwrap();
	assert_eq!(
		with(Condition::is("genre", "film")).unwrap_err(),
		"event.doors: condition: genre can't be \"film\""
	);
}

#[test]
fn graphql_gives_the_option() {
	let options =
		json!([{ "value": "concert", "label": "Konzert", "color": "#2b6cb0" }]);
	let out = ButtonGroup.output(&options, &json!("concert"));
	assert_eq!(out["label"], "Konzert");
	assert_eq!(ButtonGroup.output(&options, &json!("gone")), Value::Null);
}
