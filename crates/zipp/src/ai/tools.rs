use super::*;

pub(super) fn tools() -> Vec<Tool> {
	let tools = json!([
		{
			"name": "list_entries",
			"description": "Lists entries with id, section, type, title, uri, whether they are \
				published and whether they have a draft, as the editor would see them. \
				Filter by section, type or a part of the title.",
			"input_schema": {
				"type": "object",
				"properties": {
					"site": { "type": "string", "description": "site handle, default the primary site" },
					"section": { "type": "string" },
					"type": { "type": "string" },
					"search": { "type": "string", "description": "part of the title" }
				},
				"additionalProperties": false
			}
		},
		{
			"name": "get_entry",
			"description": "The content of an entry in a site: title, slug, published and \
				its fields as JSON. If there is a draft, the draft.",
			"input_schema": {
				"type": "object",
				"properties": {
					"id": { "type": "integer" },
					"site": { "type": "string" }
				},
				"required": ["id", "site"],
				"additionalProperties": false
			}
		},
		{
			"name": "update_entry",
			"description": "Changes an entry in one site as a draft. Give only what changes: \
				title, slug, published and/or fields (top level field handles, each with its \
				whole new value in the shape get_entry returned). Fields that are the same in \
				all sites change everywhere.",
			"input_schema": {
				"type": "object",
				"properties": {
					"id": { "type": "integer" },
					"site": { "type": "string" },
					"title": { "type": "string" },
					"slug": { "type": "string" },
					"published": { "type": "boolean" },
					"fields": { "type": "object" }
				},
				"required": ["id", "site"],
				"additionalProperties": false
			}
		},
		{
			"name": "create_entry",
			"description": "Creates an entry in a section (not a single) in every site. It \
				stays off the website until an editor publishes it; the preview already \
				shows it. Give title and fields for one site, then translate the other \
				sites with update_entry. In a structure, parent_id puts it under a page.",
			"input_schema": {
				"type": "object",
				"properties": {
					"section": { "type": "string" },
					"type": { "type": "string" },
					"site": { "type": "string" },
					"title": { "type": "string" },
					"slug": { "type": "string" },
					"parent_id": { "type": "integer" },
					"fields": { "type": "object" }
				},
				"required": ["section", "type", "title"],
				"additionalProperties": false
			}
		},
		{
			"name": "list_assets",
			"description": "Images and files with id, filename, kind and alt text, for \
				image and asset fields (they hold a list of asset ids).",
			"input_schema": {
				"type": "object",
				"properties": {
					"search": { "type": "string", "description": "part of the filename" }
				},
				"additionalProperties": false
			}
		},
		{
			"name": "get_translations",
			"description": "The texts of the website's code in a site (menu labels, \
				messages), by namespace and key.",
			"input_schema": {
				"type": "object",
				"properties": { "site": { "type": "string" } },
				"required": ["site"],
				"additionalProperties": false
			}
		},
		{
			"name": "set_translation",
			"description": "Sets one text of the website's code in a site. Unlike entries \
				this is live at once, there are no drafts of texts. An empty value goes \
				back to the default.",
			"input_schema": {
				"type": "object",
				"properties": {
					"site": { "type": "string" },
					"namespace": { "type": "string" },
					"key": { "type": "string" },
					"value": { "type": "string" }
				},
				"required": ["site", "namespace", "key", "value"],
				"additionalProperties": false
			}
		},
		{
			"name": "show_page",
			"description": "Opens a page of the website in the preview next to the chat: \
				the page of an entry, or a path of the site like `events`.",
			"input_schema": {
				"type": "object",
				"properties": {
					"entry_id": { "type": "integer" },
					"path": { "type": "string" },
					"site": { "type": "string" }
				},
				"additionalProperties": false
			}
		}
	]);
	tools
		.as_array()
		.unwrap()
		.iter()
		.map(|t| Tool {
			name: t["name"].as_str().unwrap().into(),
			description: t["description"].as_str().unwrap().into(),
			parameters: Some(t["input_schema"].clone()),
		})
		.collect()
}

/// What the studio shows of a tool call, and what the preview does.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
	pub text: String,
	pub entry_id: Option<i64>,
	pub section: Option<String>,
	/// a page of the website, without a token
	pub url: Option<String>,
	/// whether the preview opens `url`, else it reloads the page it shows
	pub navigate: bool,
}

/// The url of an entry on the website.
pub(super) fn page_url(
	state: &AppState,
	site: &str,
	uri: Option<&str>,
) -> Option<String> {
	let base = state.site_urls.get(site)?;
	Some(format!("{}/{}", base.trim_end_matches('/'), uri?))
}

/// The content of an entry in a site as the editor sees it, draft first.
pub(super) fn current(
	conn: &Connection,
	id: i64,
	site: &str,
) -> Result<(SiteDoc, bool)> {
	let draft = get_draft(conn, id)?;
	let has_draft = draft.is_some();
	let doc = match draft {
		Some(d) => d.doc,
		None => live_doc(conn, id)?,
	};
	let content = doc
		.get(site)
		.cloned()
		.ok_or_else(|| Error::Request(format!("unknown site {site}")))?;
	Ok((content, has_draft))
}

/// Runs one tool, returns its result for the model and what the studio
/// shows.
pub(super) fn run_tool(
	state: &AppState,
	user_id: i64,
	name: &str,
	input: &Value,
) -> Result<(Value, Option<Action>)> {
	let conn = state.db.conn();
	let schema = &state.schema;
	let primary = schema.primary_site().handle.clone();
	let site = input["site"].as_str().unwrap_or(&primary).to_string();

	match name {
		"list_entries" => {
			let mut params = vec![
				("site".to_string(), site.clone()),
				("limit".to_string(), "200".to_string()),
			];
			for key in ["section", "type", "search"] {
				if let Some(v) = input[key].as_str() {
					params.push((key.into(), v.into()));
				}
			}
			let q = Query::parse(schema, &params).map_err(Error::Request)?;
			let (entries, _) = list(&conn, &q)?;
			let mut out = vec![];
			for e in &entries {
				let (content, draft) = current(&conn, e.id, &site)?;
				out.push(
					json!({ "id": e.id, "section": e.section, "type": e.ty,
					"title": content.title, "uri": e.uri,
					"published": content.published, "draft": draft,
					"parentId": e.parent_id }),
				);
			}
			Ok((json!(out), None))
		}
		"get_entry" => {
			let id = input["id"].as_i64().unwrap_or_default();
			let e = get_entry(&conn, id, &site)?;
			let (content, draft) = current(&conn, id, &site)?;
			Ok((
				json!({ "id": id, "section": e.section, "type": e.ty, "site": site,
					"uri": e.uri, "draft": draft, "content": content }),
				None,
			))
		}
		"update_entry" => {
			let id = input["id"].as_i64().unwrap_or_default();
			let entry = get_entry(&conn, id, &site)?;
			let base = get_draft(&conn, id)?.map(|d| d.updated_at);
			let (mut content, _) = current(&conn, id, &site)?;
			if let Some(t) = input["title"].as_str() {
				content.title = t.into();
			}
			if let Some(t) = input["slug"].as_str() {
				content.slug = t.into();
			}
			if let Some(p) = input["published"].as_bool() {
				content.published = p;
			}
			if let Some(Value::Object(changes)) = input.get("fields") {
				let Some(fields) = content.fields.as_object_mut() else {
					return Err(Error::Request(
						"the entry has no fields".into(),
					));
				};
				for (k, v) in changes {
					fields.insert(k.clone(), v.clone());
				}
			}
			let title = content.title.clone();
			put_draft(&conn, schema, id, &site, content, base, Some(user_id))?;

			let action = Action {
				text: format!("Changed {title} ({site})"),
				entry_id: Some(id),
				section: Some(entry.section.clone()),
				url: page_url(state, &site, entry.uri.as_deref()),
				navigate: false,
			};
			Ok((json!({ "ok": true, "draft": true }), Some(action)))
		}
		"create_entry" => {
			let req =
				|k: &str| input[k].as_str().unwrap_or_default().to_string();
			// live it is off, the draft publishes it in every site
			let entry = save(
				&conn,
				schema,
				None,
				EntryInput {
					section: req("section"),
					ty: req("type"),
					site: Some(site.clone()),
					title: req("title"),
					slug: req("slug"),
					published: false,
					parent_id: input["parent_id"].as_i64(),
					fields: input.get("fields").cloned().unwrap_or(json!({})),
				},
			)?;
			for s in &schema.sites {
				let base = get_draft(&conn, entry.id)?.map(|d| d.updated_at);
				let (mut content, _) = current(&conn, entry.id, &s.handle)?;
				content.published = true;
				put_draft(
					&conn,
					schema,
					entry.id,
					&s.handle,
					content,
					base,
					Some(user_id),
				)?;
			}
			let action = Action {
				text: format!("Created {} ({})", entry.title, entry.section),
				entry_id: Some(entry.id),
				section: Some(entry.section.clone()),
				url: page_url(state, &site, entry.uri.as_deref()),
				navigate: false,
			};
			Ok((
				json!({ "ok": true, "id": entry.id, "uri": entry.uri }),
				Some(action),
			))
		}
		"list_assets" => {
			let search =
				format!("%{}%", input["search"].as_str().unwrap_or_default());
			let mut stmt = conn.prepare(
				"SELECT id, filename, kind, alt FROM assets
				WHERE filename LIKE ?1 ORDER BY id DESC LIMIT 200",
			)?;
			let rows = stmt.query_map([search], |r| {
				Ok(
					json!({ "id": r.get::<_, i64>(0)?, "filename": r.get::<_, String>(1)?,
					"kind": r.get::<_, String>(2)?, "alt": r.get::<_, String>(3)? }),
				)
			})?;
			let out: Vec<Value> = rows.collect::<rusqlite::Result<_>>()?;
			Ok((json!(out), None))
		}
		"get_translations" => {
			let texts = crate::translations::texts(&conn, schema, &site)?;
			Ok((json!(texts), None))
		}
		"set_translation" => {
			let ns = input["namespace"].as_str().unwrap_or_default();
			let key = input["key"].as_str().unwrap_or_default();
			let value = input["value"].as_str().unwrap_or_default();
			crate::translations::set(&conn, schema, &site, ns, key, value)?;
			let action = Action {
				text: format!("Set the text {ns}.{key} ({site}), live at once"),
				entry_id: None,
				section: None,
				url: None,
				navigate: false,
			};
			Ok((json!({ "ok": true }), Some(action)))
		}
		"show_page" => {
			let url = match (input["entry_id"].as_i64(), input["path"].as_str())
			{
				(Some(id), _) => {
					let e = get_entry(&conn, id, &site)?;
					let Some(url) = page_url(state, &site, e.uri.as_deref())
					else {
						return Err(Error::Request(format!(
							"{} has no page, it shows where it is used",
							e.title
						)));
					};
					url
				}
				(None, Some(path)) => {
					page_url(state, &site, Some(path.trim_start_matches('/')))
						.ok_or_else(|| {
						Error::Request(format!("site {site} has no url"))
					})?
				}
				(None, None) => {
					return Err(Error::Request("give entry_id or path".into()));
				}
			};
			let action = Action {
				text: format!("Opened {url}"),
				entry_id: None,
				section: None,
				url: Some(url),
				navigate: true,
			};
			Ok((json!({ "ok": true }), Some(action)))
		}
		_ => Err(Error::Request(format!("unknown tool {name}"))),
	}
}
