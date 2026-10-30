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
	crate::page_url(&state.site_urls, site, uri?)
}

/// The content of an entry in a site as the editor sees it, draft first.
pub(super) fn current(
	conn: &Connection,
	id: i64,
	site: &str,
) -> Result<(SiteDoc, bool)> {
	let (doc, has_draft) = crate::drafts::draft_or_live(conn, id)?;
	let content = doc
		.get(site)
		.cloned()
		.ok_or_else(|| Error::Request(format!("unknown site {site}")))?;
	Ok((content, has_draft))
}

/// What a tool gives the model and what the studio shows of it.
type ToolResult = Result<(Value, Option<Action>)>;

/// What every tool works with.
struct Ctx<'a> {
	state: &'a AppState,
	conn: &'a Connection,
	schema: &'a Schema,
	/// the site of the input, the primary one if it has none
	site: String,
	user_id: i64,
	/// the AI can do what the user's role can
	auth: &'a Authed,
}

impl Ctx<'_> {
	fn need(&self, id: i64, access: Access) -> Result<()> {
		let section = crate::entries::section_of(self.conn, id)?;
		self.auth.check(self.schema, &section, access)
	}
}

/// Runs one tool, returns its result for the model and what the studio
/// shows.
pub(super) fn run_tool(
	state: &AppState,
	auth: &Authed,
	name: &str,
	input: &Value,
) -> ToolResult {
	let conn = state.db.conn();
	let schema = &state.schema;
	let primary = &schema.primary_site().handle;
	let t = Ctx {
		state,
		conn: &conn,
		schema,
		site: input["site"].as_str().unwrap_or(primary).to_string(),
		user_id: auth.user_id,
		auth,
	};
	match name {
		"list_entries" => tool_list_entries(&t, input),
		"get_entry" => tool_get_entry(&t, input),
		"update_entry" => tool_update_entry(&t, input),
		"create_entry" => tool_create_entry(&t, input),
		"list_assets" => tool_list_assets(&t, input),
		"show_page" => tool_show_page(&t, input),
		_ => Err(Error::Request(format!("unknown tool {name}"))),
	}
}

/// Entries with the state the editor sees.
fn tool_list_entries(t: &Ctx, input: &Value) -> ToolResult {
	let conn = t.conn;
	let schema = t.schema;
	let site = t.site.clone();

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
	let (entries, _) = list(conn, &q)?;
	let mut out = vec![];
	for e in entries
		.iter()
		.filter(|e| t.auth.can(schema, &e.section, Access::View))
	{
		let (content, draft) = current(conn, e.id, &site)?;
		out.push(json!({ "id": e.id, "section": e.section, "type": e.ty,
				"title": content.title, "uri": e.uri,
				"published": content.published, "draft": draft,
				"parentId": e.parent_id }));
	}
	Ok((json!(out), None))
}

/// An entry in a site, its draft if there is one.
fn tool_get_entry(t: &Ctx, input: &Value) -> ToolResult {
	let conn = t.conn;
	let site = t.site.clone();

	let id = input["id"].as_i64().unwrap_or_default();
	t.need(id, Access::View)?;
	let e = get_entry(conn, id, &site)?;
	let (content, draft) = current(conn, id, &site)?;
	Ok((
		json!({ "id": id, "section": e.section, "type": e.ty, "site": site,
				"uri": e.uri, "draft": draft, "content": content }),
		None,
	))
}

/// Changes an entry in a site as a draft.
fn tool_update_entry(t: &Ctx, input: &Value) -> ToolResult {
	let state = t.state;
	let conn = t.conn;
	let schema = t.schema;
	let site = t.site.clone();
	let user_id = t.user_id;

	let id = input["id"].as_i64().unwrap_or_default();
	t.need(id, Access::Edit)?;
	let entry = get_entry(conn, id, &site)?;
	let base = get_draft(conn, id)?.map(|d| d.updated_at);
	let (mut content, _) = current(conn, id, &site)?;
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
			return Err(Error::Request("the entry has no fields".into()));
		};
		for (k, v) in changes {
			fields.insert(k.clone(), v.clone());
		}
	}
	let title = content.title.clone();
	put_draft(conn, schema, id, &site, content, base, Some(user_id))?;

	let action = Action {
		text: format!("Changed {title} ({site})"),
		entry_id: Some(id),
		section: Some(entry.section.clone()),
		url: page_url(state, &site, entry.uri.as_deref()),
		navigate: false,
	};
	Ok((json!({ "ok": true, "draft": true }), Some(action)))
}

/// A new entry, live off, its draft publishes it in every site.
fn tool_create_entry(t: &Ctx, input: &Value) -> ToolResult {
	let state = t.state;
	let conn = t.conn;
	let schema = t.schema;
	let site = t.site.clone();
	let user_id = t.user_id;

	let req = |k: &str| input[k].as_str().unwrap_or_default().to_string();
	t.auth.check(schema, &req("section"), Access::Edit)?;
	// live it is off, the draft publishes it in every site
	let entry = save(
		conn,
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
		Some(t.user_id),
	)?;
	// like a new entry of the studio
	state.changed(entry.id, &entry.section, crate::ChangeKind::Saved);
	for s in &schema.sites {
		let base = get_draft(conn, entry.id)?.map(|d| d.updated_at);
		let (mut content, _) = current(conn, entry.id, &s.handle)?;
		content.published = true;
		put_draft(
			conn,
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

/// Images and files for the fields that hold them.
fn tool_list_assets(t: &Ctx, input: &Value) -> ToolResult {
	t.auth.check_assets(t.schema, Access::View)?;
	let conn = t.conn;

	let search = format!("%{}%", input["search"].as_str().unwrap_or_default());
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

/// A page of the website for the preview.
fn tool_show_page(t: &Ctx, input: &Value) -> ToolResult {
	let state = t.state;
	let conn = t.conn;
	let site = t.site.clone();

	let url = match (input["entry_id"].as_i64(), input["path"].as_str()) {
		(Some(id), _) => {
			t.need(id, Access::View)?;
			let e = get_entry(conn, id, &site)?;
			let Some(url) = page_url(state, &site, e.uri.as_deref()) else {
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
