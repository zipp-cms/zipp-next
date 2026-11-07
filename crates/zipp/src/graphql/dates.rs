//! Dates formatted like Craft's `@formatDateTime(format: "d.m.Y",
//! timezone: "Europe/Zurich", locale: "de-CH")`: PHP's date format, in
//! a time zone, with month and day names of a language.
//!
//! The dynamic schema of async-graphql has no own directives, so the
//! directive is a rewrite: [`FormatDateTime`] moves its arguments to the
//! field before the query is validated, `postDate(format: "d.m.Y")`.

use async_graphql::extensions::NextParseQuery;
use async_graphql::parser::types::{
	DocumentOperations, ExecutableDocument, Field, Selection, SelectionSet,
};
use async_graphql::{Positioned, ServerResult, Variables};
use jiff::tz::TimeZone;
use jiff::{Timestamp, Zoned, civil};

use super::*;

/// The arguments of a date field.
pub(super) fn args() -> [InputValue; 3] {
	let s = || TypeRef::named(TypeRef::STRING);
	[
		InputValue::new("format", s()),
		InputValue::new("timezone", s()),
		InputValue::new("locale", s()),
	]
}

/// Craft's format without one: `2026-10-28T08:00:00+00:00`.
const ATOM: &str = "Y-m-d\\TH:i:sP";

/// A date of zipp (milliseconds since 1970), as `iso` without arguments.
pub(super) fn format_ms(
	ctx: &ResolverContext,
	ms: i64,
	site: &str,
) -> async_graphql::Result<GqlValue> {
	let Some(f) = DateArgs::of(ctx, site)? else {
		return Ok(GqlValue::from(crate::db::iso(ms)));
	};
	let ts = Timestamp::from_millisecond(ms)?;
	Ok(GqlValue::from(f.format(&ts.to_zoned(f.tz.clone()))))
}

/// A value of a date field, `2026-11-14` or `2026-11-14T20:00`, as it is
/// without arguments. It is a time of the schema's time zone, the one
/// editors are in, like Craft's.
pub(super) fn format_field(
	ctx: &ResolverContext,
	value: &str,
	site: &str,
) -> async_graphql::Result<GqlValue> {
	let Some(f) = DateArgs::of(ctx, site)? else {
		return Ok(GqlValue::from(value));
	};
	let dt: civil::DateTime = value
		.parse()
		.map_err(|_| GqlError::new(format!("{value} is not a date")))?;
	let zoned = schema_tz(ctx.data::<Arc<Schema>>()?).to_zoned(dt)?;
	Ok(GqlValue::from(
		f.format(&zoned.with_time_zone(f.tz.clone())),
	))
}

fn schema_tz(schema: &Schema) -> TimeZone {
	// checked when the schema is
	match &schema.timezone {
		Some(tz) => TimeZone::get(tz).unwrap_or(TimeZone::UTC),
		None => TimeZone::UTC,
	}
}

struct DateArgs {
	format: String,
	tz: TimeZone,
	/// `de` of `de-CH`
	lang: String,
}

impl DateArgs {
	/// None without arguments.
	fn of(
		ctx: &ResolverContext,
		site: &str,
	) -> async_graphql::Result<Option<Self>> {
		let text = |k: &str| -> async_graphql::Result<Option<String>> {
			match arg(ctx, k) {
				Some(v) => Ok(Some(v.string()?.to_string())),
				None => Ok(None),
			}
		};
		let (format, tz, locale) =
			(text("format")?, text("timezone")?, text("locale")?);
		if format.is_none() && tz.is_none() && locale.is_none() {
			return Ok(None);
		}

		let schema = ctx.data::<Arc<Schema>>()?;
		let tz = match tz {
			Some(name) => TimeZone::get(&name).map_err(|_| {
				GqlError::new(format!("unknown timezone {name}"))
			})?,
			None => schema_tz(schema),
		};
		// the language of the site the date is of
		let locale = locale.unwrap_or_else(|| {
			schema
				.get_site(site)
				.map(|s| s.language.clone())
				.unwrap_or_default()
		});
		let lang = locale.split(['-', '_']).next().unwrap_or_default();
		Ok(Some(Self {
			format: format.unwrap_or_else(|| ATOM.into()),
			tz,
			lang: lang.to_ascii_lowercase(),
		}))
	}

	fn format(&self, z: &Zoned) -> String {
		php_date(z, &self.format, &self.lang)
	}
}

/// A time in PHP's `date()` format, the one of Craft's `format`: `d.m.Y`,
/// `j. F Y`. Names are German, French, Italian or else English; letters
/// it doesn't know stay, `\` keeps the next one.
pub(crate) fn php_date(z: &Zoned, format: &str, lang: &str) -> String {
	let names = Names::of(lang);
	let wd = z.weekday();
	let hour12 = match z.hour() % 12 {
		0 => 12,
		h => h,
	};
	let offset = z.offset().seconds();
	let hhmm = |colon: &str| {
		let sign = if offset < 0 { '-' } else { '+' };
		let o = offset.abs();
		format!("{sign}{:02}{colon}{:02}", o / 3600, o % 3600 / 60)
	};

	let mut out = String::new();
	let mut chars = format.chars();
	while let Some(c) = chars.next() {
		match c {
			'\\' => out.extend(chars.next()),
			// day
			'd' => out += &format!("{:02}", z.day()),
			'D' => out += names.days_short[day_index(wd)],
			'j' => out += &z.day().to_string(),
			'l' => out += names.days[day_index(wd)],
			'N' => out += &wd.to_monday_one_offset().to_string(),
			// English ordinals only, like ICU
			'S' if names.english => out += suffix(z.day()),
			'S' => {}
			'w' => out += &wd.to_sunday_zero_offset().to_string(),
			'z' => out += &(z.day_of_year() - 1).to_string(),
			// week
			'W' => out += &format!("{:02}", z.date().iso_week_date().week()),
			// month
			'F' => out += names.months[z.month() as usize - 1],
			'm' => out += &format!("{:02}", z.month()),
			'M' => out += names.months_short[z.month() as usize - 1],
			'n' => out += &z.month().to_string(),
			't' => out += &z.days_in_month().to_string(),
			// year
			'L' => out += if z.in_leap_year() { "1" } else { "0" },
			'o' => out += &z.date().iso_week_date().year().to_string(),
			'Y' => out += &z.year().to_string(),
			'y' => out += &format!("{:02}", z.year() % 100),
			// time
			'a' => out += if z.hour() < 12 { "am" } else { "pm" },
			'A' => out += if z.hour() < 12 { "AM" } else { "PM" },
			'g' => out += &hour12.to_string(),
			'G' => out += &z.hour().to_string(),
			'h' => out += &format!("{hour12:02}"),
			'H' => out += &format!("{:02}", z.hour()),
			'i' => out += &format!("{:02}", z.minute()),
			's' => out += &format!("{:02}", z.second()),
			'v' => out += &format!("{:03}", z.millisecond()),
			'u' => out += &format!("{:06}", z.subsec_nanosecond() / 1000),
			// time zone
			'e' => out += z.time_zone().iana_name().unwrap_or("UTC"),
			'T' => {
				let info = z.time_zone().to_offset_info(z.timestamp());
				out += info.abbreviation();
			}
			'O' => out += &hhmm(""),
			'P' => out += &hhmm(":"),
			'p' if offset == 0 => out.push('Z'),
			'p' => out += &hhmm(":"),
			'Z' => out += &offset.to_string(),
			// full
			'c' => out += &php_date(z, ATOM, lang),
			'r' => out += &php_date(z, "D, d M Y H:i:s O", "en"),
			'U' => out += &z.timestamp().as_second().to_string(),
			c => out.push(c),
		}
	}
	out
}

/// Monday is 0.
fn day_index(wd: jiff::civil::Weekday) -> usize {
	wd.to_monday_zero_offset() as usize
}

fn suffix(day: i8) -> &'static str {
	match (day % 10, day) {
		(_, 11..=13) => "th",
		(1, _) => "st",
		(2, _) => "nd",
		(3, _) => "rd",
		_ => "th",
	}
}

/// Month and day names of a language, the ones of ICU like Craft's.
struct Names {
	months: [&'static str; 12],
	months_short: [&'static str; 12],
	days: [&'static str; 7],
	days_short: [&'static str; 7],
	english: bool,
}

impl Names {
	fn of(lang: &str) -> Self {
		match lang {
			"de" => Names {
				months: [
					"Januar",
					"Februar",
					"März",
					"April",
					"Mai",
					"Juni",
					"Juli",
					"August",
					"September",
					"Oktober",
					"November",
					"Dezember",
				],
				months_short: [
					"Jan.", "Feb.", "März", "Apr.", "Mai", "Juni", "Juli",
					"Aug.", "Sept.", "Okt.", "Nov.", "Dez.",
				],
				days: [
					"Montag",
					"Dienstag",
					"Mittwoch",
					"Donnerstag",
					"Freitag",
					"Samstag",
					"Sonntag",
				],
				days_short: ["Mo.", "Di.", "Mi.", "Do.", "Fr.", "Sa.", "So."],
				english: false,
			},
			"fr" => Names {
				months: [
					"janvier",
					"février",
					"mars",
					"avril",
					"mai",
					"juin",
					"juillet",
					"août",
					"septembre",
					"octobre",
					"novembre",
					"décembre",
				],
				months_short: [
					"janv.", "févr.", "mars", "avr.", "mai", "juin", "juil.",
					"août", "sept.", "oct.", "nov.", "déc.",
				],
				days: [
					"lundi", "mardi", "mercredi", "jeudi", "vendredi",
					"samedi", "dimanche",
				],
				days_short: [
					"lun.", "mar.", "mer.", "jeu.", "ven.", "sam.", "dim.",
				],
				english: false,
			},
			"it" => Names {
				months: [
					"gennaio",
					"febbraio",
					"marzo",
					"aprile",
					"maggio",
					"giugno",
					"luglio",
					"agosto",
					"settembre",
					"ottobre",
					"novembre",
					"dicembre",
				],
				months_short: [
					"gen", "feb", "mar", "apr", "mag", "giu", "lug", "ago",
					"set", "ott", "nov", "dic",
				],
				days: [
					"lunedì",
					"martedì",
					"mercoledì",
					"giovedì",
					"venerdì",
					"sabato",
					"domenica",
				],
				days_short: ["lun", "mar", "mer", "gio", "ven", "sab", "dom"],
				english: false,
			},
			_ => Names {
				months: [
					"January",
					"February",
					"March",
					"April",
					"May",
					"June",
					"July",
					"August",
					"September",
					"October",
					"November",
					"December",
				],
				months_short: [
					"Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug",
					"Sep", "Oct", "Nov", "Dec",
				],
				days: [
					"Monday",
					"Tuesday",
					"Wednesday",
					"Thursday",
					"Friday",
					"Saturday",
					"Sunday",
				],
				days_short: ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"],
				english: true,
			},
		}
	}
}

/// Turns Craft's `@formatDateTime(...)` on a field into its arguments,
/// see the module docs.
pub(super) struct FormatDateTime;

impl ExtensionFactory for FormatDateTime {
	fn create(&self) -> Arc<dyn Extension> {
		Arc::new(FormatDateTime)
	}
}

#[async_graphql::async_trait::async_trait]
impl Extension for FormatDateTime {
	async fn parse_query(
		&self,
		ctx: &ExtensionContext<'_>,
		query: &str,
		variables: &Variables,
		next: NextParseQuery<'_>,
	) -> ServerResult<ExecutableDocument> {
		let mut doc = next.run(ctx, query, variables).await?;
		match &mut doc.operations {
			DocumentOperations::Single(op) => {
				rewrite(&mut op.node.selection_set);
			}
			DocumentOperations::Multiple(ops) => {
				for op in ops.values_mut() {
					rewrite(&mut op.node.selection_set);
				}
			}
		}
		for f in doc.fragments.values_mut() {
			rewrite(&mut f.node.selection_set);
		}
		Ok(doc)
	}
}

fn rewrite(set: &mut Positioned<SelectionSet>) {
	for item in &mut set.node.items {
		match &mut item.node {
			Selection::Field(f) => rewrite_field(&mut f.node),
			Selection::InlineFragment(f) => rewrite(&mut f.node.selection_set),
			Selection::FragmentSpread(_) => {}
		}
	}
}

fn rewrite_field(field: &mut Field) {
	let (moved, kept) = std::mem::take(&mut field.directives)
		.into_iter()
		.partition(|d| d.node.name.node == "formatDateTime");
	field.directives = kept;
	for d in moved {
		field.arguments.extend(d.node.arguments);
	}
	rewrite(&mut field.selection_set);
}
