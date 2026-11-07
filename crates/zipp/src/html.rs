//! The attributes of an HTML tag, for the icons of a page and the rich
//! text of a Craft import.

/// The attributes after a tag's name: names in lower case, values as they
/// are written, without their quotes. A `/` between them is skipped like
/// browsers do.
pub(crate) fn attributes(tag: &str) -> Vec<(String, &str)> {
	let mut attrs = vec![];
	let mut s = tag;
	loop {
		s = s.trim_start_matches(|c: char| c.is_whitespace() || c == '/');
		let name_end = s
			.find(|c: char| c.is_whitespace() || c == '=' || c == '/')
			.unwrap_or(s.len());
		if name_end == 0 {
			return attrs;
		}
		let name = s[..name_end].to_ascii_lowercase();
		s = s[name_end..].trim_start();

		let Some(after) = s.strip_prefix('=') else {
			attrs.push((name, ""));
			continue;
		};
		let after = after.trim_start();
		let (value, rest) = match after.chars().next() {
			Some(q @ ('"' | '\'')) => {
				let inner = &after[1..];
				inner.split_once(q).unwrap_or((inner, ""))
			}
			_ => after.split_once(char::is_whitespace).unwrap_or((after, "")),
		};
		attrs.push((name, value));
		s = rest;
	}
}

/// The value of the attribute `name`.
pub(crate) fn attr<'a>(
	attrs: &'a [(String, String)],
	name: &str,
) -> Option<&'a str> {
	attrs
		.iter()
		.find(|(n, _)| n == name)
		.map(|(_, v)| v.as_str())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn attributes_of_a_tag() {
		let attrs = attributes(
			r#" REL="Icon" href=/i/32.png sizes='16x16' hidden data-x = "a b" /"#,
		);
		assert_eq!(
			attrs,
			[
				("rel".into(), "Icon"),
				("href".into(), "/i/32.png"),
				("sizes".into(), "16x16"),
				("hidden".into(), ""),
				("data-x".into(), "a b"),
			]
		);
		// spaces in quotes stay, an open quote takes the rest
		assert_eq!(attributes(r#"a=" x ""#), [("a".into(), " x ")]);
		assert_eq!(attributes(r#"a=" x b"#), [("a".into(), " x b")]);
		assert_eq!(
			attributes(r#"a=b c="d"#),
			[("a".into(), "b"), ("c".into(), "d")]
		);
		assert!(attributes("").is_empty());
	}
}
