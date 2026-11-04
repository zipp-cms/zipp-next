use jiff::tz::TimeZone;
use serde_json::json;

use super::dates::php_date;
use super::tests::run;
use super::*;
use crate::entries::{Dates, save};
use crate::schema::{Field, Section};
use crate::testing::input;

/// 2026-03-05 23:30 UTC, in Zurich already the 6th.
const MS: i64 = 1_772_753_400_000;

fn zurich() -> jiff::Zoned {
	jiff::Timestamp::from_millisecond(MS)
		.unwrap()
		.to_zoned(TimeZone::get("Europe/Zurich").unwrap())
}

#[test]
fn formats_like_php() {
	let z = zurich();
	assert_eq!(php_date(&z, "d.m.Y", "de"), "06.03.2026");
	assert_eq!(php_date(&z, "j. F Y", "de"), "6. März 2026");
	assert_eq!(php_date(&z, "d F Y", "en"), "06 March 2026");
	assert_eq!(php_date(&z, "D, jS M y", "en"), "Fri, 6th Mar 26");
	assert_eq!(php_date(&z, "l j F", "fr"), "vendredi 6 mars");
	assert_eq!(php_date(&z, "G:i A", "en"), "0:30 AM");
	assert_eq!(
		php_date(&z, "H:i:s P e", "de"),
		"00:30:00 +01:00 Europe/Zurich"
	);
	assert_eq!(php_date(&z, "c", "de"), "2026-03-06T00:30:00+01:00");
	// a backslash keeps a letter, unknown ones stay
	assert_eq!(php_date(&z, "\\d\\a\\y j", "de"), "day 6");
	assert_eq!(php_date(&z, "N w z W t L", "de"), "5 5 64 10 31 0");
	assert_eq!(php_date(&z, "U", "de"), (MS / 1000).to_string());
}

fn dated_site() -> Schema {
	Schema::new()
		.site("de", "Deutsch", "de-CH")
		.site("en", "English", "en")
		.section(Section::channel("news", ["news"]).uri("news/{slug}"))
		.entry(
			"news",
			"News",
			[Field::date("day"), Field::date("start").time()],
		)
		.timezone("Europe/Zurich")
}

#[tokio::test]
async fn format_date_time_like_craft() {
	let db = Db::memory().unwrap();
	let s = Arc::new(dated_site());
	let gql =
		build(s.clone(), db.clone(), String::new(), SiteUrls::new()).unwrap();
	{
		let conn = db.conn();
		let fields =
			json!({ "day": "2026-03-06", "start": "2026-03-06T20:15" });
		let mut news = input("news", "news", "Neu", fields);
		news.dates = Dates {
			post_date: Some(MS),
			expiry_date: None,
		};
		save(&conn, &s, None, news, None).unwrap();
	}

	let q = r#"
		fragment dates on news_Entry {
			fragmentDate: postDate @formatDateTime(format: "d.m.Y")
			day @formatDateTime(format: "l, j. F Y")
			start @formatDateTime(format: "H:i", timezone: "UTC")
		}
		{
			entry(uri: "news/neu") {
				iso: postDate
				utc: postDate @formatDateTime(format: "d.m.Y H:i", timezone: "UTC")
				zurich: postDate @formatDateTime(format: "j. F Y", locale: "de-CH")
				en: postDate @formatDateTime(format: "j F Y", locale: "en")
				atom: postDate @formatDateTime(timezone: "Europe/Zurich")
				arg: postDate(format: "Y-m-d")
				... on news_Entry { rawDay: day ...dates }
			}
		}
	"#;
	let data = run(&gql, q, true).await;
	assert_eq!(
		data["entry"],
		json!({
			"iso": "2026-03-05T23:30:00+00:00",
			"utc": "05.03.2026 23:30",
			"zurich": "6. März 2026",
			"en": "6 March 2026",
			"atom": "2026-03-06T00:30:00+01:00",
			"arg": "2026-03-06",
			"rawDay": "2026-03-06",
			"fragmentDate": "06.03.2026",
			"day": "Freitag, 6. März 2026",
			// a date field is a time of the schema's time zone
			"start": "19:15",
		})
	);

	// the language of the site asked
	let q = r#"{ entry(uri: "news/neu", site: "en") {
		postDate @formatDateTime(format: "j F Y") } }"#;
	let data = run(&gql, q, true).await;
	assert_eq!(data["entry"]["postDate"], "6 March 2026");

	// a zone that doesn't exist, a directive on a field without dates
	let errors = check_query(
		&gql,
		r#"{ entry(uri: "news/neu") {
			postDate @formatDateTime(format: "Y", timezone: "Mars/Base") } }"#,
	)
	.await;
	assert!(
		errors[0].contains("unknown timezone Mars/Base"),
		"{errors:?}"
	);
	let errors = check_query(
		&gql,
		r#"{ entry(uri: "news/neu") { title @formatDateTime(format: "Y") } }"#,
	)
	.await;
	assert!(
		errors[0].contains("Unknown argument \"format\""),
		"{errors:?}"
	);
}
