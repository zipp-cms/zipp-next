use super::*;

/// `asked` is the editor's last message, its language is the answer's.
pub(super) fn system_prompt(
	schema: &Schema,
	view: &str,
	asked: &str,
) -> String {
	// the content model, so the model knows sections, types and fields
	let model = json!({
		"sites": schema.sites,
		"sections": schema.sections,
		"components": schema.components,
	});
	format!(
		"You help the editors of a website that runs on the headless CMS zipp. You \
		see and change everything an editor can: entries of every section (pages, \
		globals like header and footer), images and the texts of the website's code \
		(the single `translations`, a texts field per namespace: key to text). \
		Next to the chat the editor sees the website as a preview with every draft.\n\n\
		- Changes of entries are drafts, the website stays as it is until the editor \
		publishes them in the editor. Say what you changed in a sentence or two.\n\
		- Before changing an entry, read it with get_entry and keep the JSON shape of \
		its fields exactly (blocks keep their id and type, rich text is a TipTap \
		document, links are {{\"type\", \"value\", \"label\"}}).\n\
		- Fields marked translatable have a value per site, the others are the same \
		in every site.\n\
		- After a change, look at the page the editor sees (its fields are below). If \
		the change shows there, stay: the preview reloads and keeps its scroll \
		position. It shows there if it is the page itself, a global like the header \
		or footer, or an entry a block of the page lists: a block that lists entries \
		of a section (like upcoming events) also shows new ones. Only if it doesn't \
		show there, open a page where it does with show_page. Also use show_page \
		when the editor wants to see something.\n\n\
		The content model:\n{}\n\n\
		{view}\n\n\
		Today is {}.\n\n\
		Write your answers in the language the editor's last message is written \
		in, not in the language of the content: an English message gets an English \
		answer, even if it names a German page. Content you write into entries is \
		in the language of its site. The editor's last message, answer in its \
		language: «{}»",
		serde_json::to_string(&model).unwrap(),
		today(),
		asked.chars().take(500).collect::<String>()
	)
}

pub(super) fn today() -> String {
	date(crate::db::now())
}

/// The date in UTC of milliseconds since 1970, `2026-10-28`.
pub(super) fn date(ms: i64) -> String {
	let days = ms.div_euclid(86_400_000);
	// civil from days, Howard Hinnant's algorithm
	let z = days + 719_468;
	let era = z.div_euclid(146_097);
	let doe = z - era * 146_097;
	let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
	let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
	let mp = (5 * doy + 2) / 153;
	let d = doy - (153 * mp + 2) / 5 + 1;
	let m = if mp < 10 { mp + 3 } else { mp - 9 };
	let y = yoe + era * 400 + i64::from(m <= 2);
	format!("{y:04}-{m:02}-{d:02}")
}

/// What the editor sees, from the url of the preview: the site and entry
/// of the page, for the system prompt.
pub(super) fn describe_view(state: &AppState, url: Option<&str>) -> String {
	let Some(url) = url else {
		return "The preview shows no page yet.".into();
	};
	// the longest base first, `/en` is longer than the site without it
	let mut sites: Vec<_> = state.site_urls.iter().collect();
	sites.sort_by_key(|(_, base)| std::cmp::Reverse(base.len()));
	let path = url.split(['?', '#']).next().unwrap_or(url);
	let found = sites.into_iter().find_map(|(site, base)| {
		let base = base.trim_end_matches('/');
		let rest = path.strip_prefix(base)?;
		if !(rest.is_empty() || rest.starts_with('/')) {
			return None;
		}
		Some((site.clone(), rest.trim_matches('/').to_string()))
	});
	// anything else comes from whatever the preview navigated to
	let Some((site, uri)) = found else {
		return "The preview shows a page outside the site.".into();
	};
	let conn = state.db.conn();
	let uri = percent_decode(&uri);
	let entry: Option<(i64, String, String)> = conn
		.query_row(
			"SELECT e.id, e.section, s.title FROM entries e
			JOIN entry_sites s ON s.entry_id = e.id
			WHERE s.site = ?1 AND s.uri = ?2",
			rusqlite::params![site, uri],
			|r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
		)
		.optional()
		.ok()
		.flatten();
	let Some((id, section, title)) = entry else {
		return format!("The editor sees the page {url} in site {site}.");
	};
	// what is on the page, as the preview shows it
	let content = current(&conn, id, &site)
		.ok()
		.and_then(|(c, _)| serde_json::to_string(&c.fields).ok())
		.unwrap_or_default();
	let content: String = content.chars().take(20_000).collect();
	format!(
		"The editor sees the page {url} in site {site}: entry {id} \"{title}\" \
		of section {section}, plus the globals (header, footer). Its fields:\n{content}"
	)
}

/// `%C3%BC` in a path to `ü`.
pub(super) fn percent_decode(s: &str) -> String {
	let bytes = s.as_bytes();
	let mut out = Vec::with_capacity(bytes.len());
	let mut i = 0;
	while i < bytes.len() {
		if bytes[i] == b'%'
			&& let Some(b) = s
				.get(i + 1..i + 3)
				.and_then(|h| u8::from_str_radix(h, 16).ok())
		{
			out.push(b);
			i += 3;
			continue;
		}
		out.push(bytes[i]);
		i += 1;
	}
	String::from_utf8_lossy(&out).into_owned()
}
