use super::*;

pub(super) fn entry_common() -> Vec<(&'static str, TypeRef)> {
	let s = || TypeRef::named_nn(TypeRef::STRING);
	vec![
		// Craft's ids are strings
		("id", TypeRef::named_nn(TypeRef::ID)),
		("sectionHandle", s()),
		("typeHandle", s()),
		("site", s()),
		// Craft's name of it
		("siteHandle", s()),
		// the position of the site, crelte knows sites by number
		("siteId", TypeRef::named_nn(TypeRef::INT)),
		("url", TypeRef::named(TypeRef::STRING)),
		("title", s()),
		("slug", s()),
		("uri", TypeRef::named(TypeRef::STRING)),
		("published", TypeRef::named_nn(TypeRef::BOOLEAN)),
		("parent", TypeRef::named("Entry")),
		("children", TypeRef::named_nn_list_nn("Entry")),
		// milliseconds since 1970, too big for Int
		("createdAt", TypeRef::named_nn(TypeRef::FLOAT)),
		("updatedAt", TypeRef::named_nn(TypeRef::FLOAT)),
		// Craft's, as `2026-10-28T08:00:00+00:00`
		("postDate", s()),
		// none: it doesn't expire
		("expiryDate", TypeRef::named(TypeRef::STRING)),
		("dateCreated", s()),
		("dateUpdated", s()),
		("language", s()),
		// the entry in the other sites, for a language switch
		("localized", TypeRef::named_nn_list_nn("Entry")),
	]
}

/// The dates of entries, with the arguments of [`dates`].
pub(super) const DATES: [&str; 4] =
	["postDate", "expiryDate", "dateCreated", "dateUpdated"];

pub(super) fn entry_common_field(name: &'static str) -> GqlField {
	let ty = entry_common()
		.into_iter()
		.find(|(n, _)| *n == name)
		.unwrap()
		.1;
	let field = GqlField::new(name, ty, move |ctx| {
		FieldFuture::new(async move {
			let e = ctx.parent_value.try_downcast_ref::<Entry>()?;
			let date = |ms| dates::format_ms(&ctx, ms, &e.site);
			let v = match name {
				"id" => GqlValue::from(e.id.to_string()),
				"sectionHandle" => GqlValue::from(e.section.clone()),
				"typeHandle" => GqlValue::from(e.ty.clone()),
				"site" | "siteHandle" => GqlValue::from(e.site.clone()),
				"siteId" => GqlValue::from(site_id(&ctx, &e.site)?),
				"url" => match &e.uri {
					Some(uri) => GqlValue::from(entry_url(&ctx, &e.site, uri)?),
					None => GqlValue::Null,
				},
				"title" => GqlValue::from(e.title.clone()),
				"slug" => GqlValue::from(e.slug.clone()),
				"uri" => e.uri.clone().map(GqlValue::from).unwrap_or_default(),
				"published" => GqlValue::from(e.published),
				"createdAt" => GqlValue::from(e.created_at as f64),
				"updatedAt" => GqlValue::from(e.updated_at as f64),
				// not published yet: when it was made
				"postDate" => date(e.dates.post_date.unwrap_or(e.created_at))?,
				"expiryDate" => match e.dates.expiry_date {
					Some(d) => date(d)?,
					None => GqlValue::Null,
				},
				"dateCreated" => date(e.created_at)?,
				"dateUpdated" => date(e.updated_at)?,
				"language" => {
					let schema = ctx.data::<Arc<Schema>>()?;
					let site =
						schema.get_site(&e.site).map(|s| s.language.clone());
					GqlValue::from(site.unwrap_or_default())
				}
				// live there and readable, like any query of the site
				"localized" => {
					let schema = ctx.data::<Arc<Schema>>()?;
					let mut out = vec![];
					for s in schema.sites.iter().filter(|s| s.handle != e.site)
					{
						let found =
							entries_by_id(&ctx, &s.handle, &[e.id]).await?;
						out.extend(found.into_iter().map(entry_value));
					}
					return Ok(Some(FieldValue::list(out)));
				}
				"parent" => {
					let Some(p) = e.parent_id else {
						return Ok(None);
					};
					let parent = entries_by_id(&ctx, &e.site, &[p]).await?;
					return Ok(parent.into_iter().next().map(entry_value));
				}
				"children" => {
					let args = params_from_args(&ctx)?;
					let children =
						children_of(&ctx, &e.site, e.id, args).await?;
					return Ok(Some(FieldValue::list(
						children.into_iter().map(entry_value),
					)));
				}
				_ => unreachable!(),
			};
			Ok(Some(FieldValue::value(v)))
		})
	});
	match DATES.contains(&name) {
		true => dates::args().into_iter().fold(field, GqlField::argument),
		false => field,
	}
}

/// A field of the content model, read from the JSON of its entry or block;
/// none for a note, it has no value.
pub(super) fn content_field(
	schema: &Schema,
	comp: &str,
	field: &crate::schema::Field,
) -> Option<GqlField> {
	// a custom type turns the stored value into its output
	let custom = match &field.kind {
		FieldKind::Custom { ty, options } => schema
			.field_types
			.get(ty)
			.map(|t| (t.clone(), options.clone())),
		_ => None,
	};
	let ty = type_of(comp, field, custom.as_ref().map(|(t, _)| t.as_ref()))?;
	let handle = field.handle.clone();
	let kind = field.kind.clone();
	// Craft's `images(kind: "image")`, see [`kinds_of`]
	let kind_arg = matches!(kind, FieldKind::Assets { .. })
		.then(|| InputValue::new("kind", TypeRef::named_list(TypeRef::STRING)));

	let is_date = matches!(field.kind, FieldKind::Date { .. });
	let f = GqlField::new(field.handle.clone(), ty, move |ctx| {
		let handle = handle.clone();
		let kind = kind.clone();
		let custom = custom.clone();
		FieldFuture::new(async move {
			let parent = &ctx.parent_value;
			let (fields, site) = if let Some(e) = parent.downcast_ref::<Entry>()
			{
				(&e.fields, e.site.clone())
			} else if let Some(a) = parent.downcast_ref::<SiteAsset>() {
				(&a.asset.fields, a.site.clone())
			} else {
				let b = parent.try_downcast_ref::<Block>()?;
				(&b.fields, b.site.clone())
			};
			let value = fields.get(&handle).cloned().unwrap_or(Value::Null);
			resolve_value(&ctx, kind, custom, value, site).await
		})
	});
	// Craft's `@formatDateTime`
	let date_args = is_date.then(dates::args).into_iter().flatten();
	Some(
		kind_arg
			.into_iter()
			.chain(date_args)
			.fold(f, GqlField::argument),
	)
}

/// The GraphQL type of a field; none for a note.
fn type_of(
	comp: &str,
	field: &crate::schema::Field,
	custom: Option<&dyn FieldType>,
) -> Option<TypeRef> {
	let ty = match &field.kind {
		// as it is stored, `2026-11-14` or `2026-11-14T20:00`
		FieldKind::Date { .. } => TypeRef::named(TypeRef::STRING),
		FieldKind::Custom { .. } => match custom.map(|t| t.graphql()) {
			Some(GraphqlType::Object { name, .. }) => TypeRef::named(name),
			Some(GraphqlType::Scalar(s)) => TypeRef::named(scalar_name(s)),
			Some(GraphqlType::Named(name)) => TypeRef::named(name),
			None => TypeRef::named(TypeRef::STRING),
		},
		FieldKind::Text { .. } | FieldKind::Email => {
			TypeRef::named(TypeRef::STRING)
		}
		FieldKind::Number { .. } => TypeRef::named(TypeRef::FLOAT),
		FieldKind::Boolean => TypeRef::named(TypeRef::BOOLEAN),
		FieldKind::Relation { .. } => TypeRef::named_nn_list_nn("Entry"),
		FieldKind::Assets { .. } => TypeRef::named_nn_list_nn("Asset"),
		FieldKind::Link { .. } => TypeRef::named("Link"),
		FieldKind::RichText { .. } => TypeRef::named("RichText"),
		FieldKind::Markdown => TypeRef::named("Markdown"),
		// as JSON, like crelte's translations want them
		FieldKind::Texts { .. } => TypeRef::named_nn(TypeRef::STRING),
		FieldKind::Checkboxes { .. } => {
			TypeRef::named_nn_list_nn(TypeRef::STRING)
		}
		FieldKind::Table { .. } => {
			TypeRef::named_nn_list_nn(table_row(comp, &field.handle))
		}
		FieldKind::Navigation { .. } => TypeRef::named_nn_list_nn("NavItem"),
		// the value, like Craft's dropdown
		FieldKind::Dropdown { .. } => TypeRef::named(TypeRef::STRING),
		FieldKind::ButtonGroup { .. } => TypeRef::named("ButtonGroupOption"),
		FieldKind::Group { component } => {
			TypeRef::named(format!("{component}_Fields"))
		}
		FieldKind::Blocks { .. } => TypeRef::named_nn_list_nn("Block"),
		FieldKind::Note { .. } => return None,
	};
	Some(ty)
}

/// The value of a field for GraphQL, from its JSON in the entry or block.
async fn resolve_value<'a>(
	ctx: &ResolverContext<'a>,
	kind: FieldKind,
	custom: Option<(Arc<dyn FieldType>, Value)>,
	value: Value,
	site: String,
) -> async_graphql::Result<Option<FieldValue<'a>>> {
	match kind {
		FieldKind::Date { .. } => match value.as_str() {
			Some(d) => {
				Ok(Some(FieldValue::value(dates::format_field(ctx, d, &site)?)))
			}
			None => Ok(None),
		},
		FieldKind::Relation { .. } => {
			let ids: Vec<i64> = value
				.as_array()
				.map(|a| a.iter().filter_map(Value::as_i64).collect())
				.unwrap_or_default();
			// in the order of the field
			let entries = entries_by_id(ctx, &site, &ids).await?;
			Ok(Some(FieldValue::list(entries.into_iter().map(entry_value))))
		}
		FieldKind::Assets { .. } => {
			let ids: Vec<i64> = value
				.as_array()
				.map(|a| a.iter().filter_map(Value::as_i64).collect())
				.unwrap_or_default();
			let mut assets = assets_by_id(ctx, &ids).await?;
			let kinds = kinds_of(ctx)?;
			if !kinds.is_empty() {
				assets.retain(|a| kinds.iter().any(|k| k.of(a)));
			}
			Ok(Some(FieldValue::list(
				assets.into_iter().map(|a| asset_value(a, &site)),
			)))
		}
		FieldKind::Blocks { .. } => {
			let blocks = value.as_array().into_iter().flatten();
			// a switched off block isn't on the website
			let blocks = blocks
				.filter(|b| b["enabled"] != false)
				.map(|b| block_of(b, &site));
			Ok(Some(FieldValue::list(blocks)))
		}
		FieldKind::Group { component } => {
			// read like a block, it has no id or type of its own; always
			// there like Craft's content block, also when an entry was
			// saved before it had a value (a new entry, a draft)
			let fields = match value {
				Value::Object(_) => value,
				_ => Value::Object(Default::default()),
			};
			Ok(Some(FieldValue::owned_any(Block {
				id: String::new(),
				ty: component,
				site,
				fields,
			})))
		}
		FieldKind::Markdown if value.is_string() => {
			let md = value.as_str().unwrap_or_default().to_string();
			let html = markdown_html(&md);
			Ok(Some(FieldValue::value(GqlValue::from_json(
				serde_json::json!({ "markdown": md, "html": html }),
			)?)))
		}
		FieldKind::RichText { .. } if !value.is_null() => {
			let rt = RichText { doc: value, site };
			Ok(Some(FieldValue::owned_any(rt)))
		}
		FieldKind::Link { new_tab, .. } if !value.is_null() => {
			let to = targets(ctx, &site, &value).await?;
			let link = resolve_link(ctx, &value, &site, new_tab, &to)?;
			link.as_ref().map(link_value).transpose()
		}
		FieldKind::Custom { .. } => match custom {
			Some((t, options)) => t.resolve(ctx, &options, &value),
			None => Ok(None),
		},
		FieldKind::Table { .. } => {
			let rows = value.as_array().cloned().unwrap_or_default();
			Ok(Some(FieldValue::list(rows.into_iter().map(|cells| {
				FieldValue::owned_any(TableRow {
					cells,
					site: site.clone(),
				})
			}))))
		}
		FieldKind::Checkboxes { .. } => {
			let items = value.as_array().cloned().unwrap_or_default();
			Ok(Some(FieldValue::list(
				items
					.into_iter()
					.map(|v| GqlValue::from_json(v).map(FieldValue::value))
					.collect::<std::result::Result<Vec<_>, _>>()?,
			)))
		}
		FieldKind::Navigation { .. } => {
			let to = targets(ctx, &site, &value).await?;
			let items = nav_items(ctx, &value, &site, &to)?;
			let items = items.iter().map(link_value);
			Ok(Some(FieldValue::list(
				items.collect::<async_graphql::Result<Vec<_>>>()?,
			)))
		}
		// the option, one that was removed since is none
		FieldKind::ButtonGroup { options } => {
			let option = options.into_iter().find(|o| value == o.value);
			match option {
				Some(o) => Ok(Some(FieldValue::value(GqlValue::from_json(
					serde_json::to_value(o)?,
				)?))),
				None => Ok(None),
			}
		}
		FieldKind::Texts { keys } => {
			let texts = crate::content::texts_of(&keys, &value);
			let json = serde_json::to_string(&texts)?;
			Ok(Some(FieldValue::value(GqlValue::from(json))))
		}
		// a value of another shape (a draft, an older migration) is
		// none rather than JSON in the place of an object
		FieldKind::Markdown
		| FieldKind::RichText { .. }
		| FieldKind::Link { .. }
		| FieldKind::Note { .. } => Ok(None),
		FieldKind::Text { .. }
		| FieldKind::Email
		| FieldKind::Number { .. }
		| FieldKind::Boolean
		| FieldKind::Dropdown { .. } => match value {
			Value::Null => Ok(None),
			v => Ok(Some(FieldValue::value(GqlValue::from_json(v)?))),
		},
	}
}

pub(super) fn block_value(
	ctx: ResolverContext,
	f: impl Fn(&Block) -> GqlValue + Send + Sync + 'static,
) -> FieldFuture {
	FieldFuture::new(async move {
		let b = ctx.parent_value.try_downcast_ref::<Block>()?;
		Ok(Some(FieldValue::value(f(b))))
	})
}

pub(super) fn entry_value(e: Entry) -> FieldValue<'static> {
	let ty = format!("{}_Entry", e.ty);
	FieldValue::owned_any(e).with_type(ty)
}

/// The option of a button group, so a website can show its label in its
/// color.
pub(super) fn button_group_option() -> Object {
	let mut obj = Object::new("ButtonGroupOption");
	for name in ["value", "label", "color"] {
		obj = obj.field(GqlField::new(
			name,
			TypeRef::named_nn(TypeRef::STRING),
			move |ctx| value_field(ctx, name),
		));
	}
	obj
}

/// The object types of the content model: of field types of other crates,
/// of components used by group fields and of every entry type and block.
pub(super) fn content_objects(
	schema: &Schema,
	entry_fields: &[crate::plugin::EntryField],
) -> Vec<Object> {
	let mut objects = vec![];
	// the object types of field types of other crates
	for t in &schema.field_types.0 {
		if let GraphqlType::Object { name, fields } = t.graphql() {
			let mut obj = Object::new(name);
			for (field, scalar) in fields {
				obj = obj.field(GqlField::new(
					field,
					TypeRef::named(scalar_name(scalar)),
					move |ctx| value_field(ctx, field),
				));
			}
			objects.push(obj);
		}
	}

	// the components used by group fields, their fields without id or type
	let grouped: std::collections::BTreeSet<&String> = schema
		.components
		.iter()
		.flat_map(|c| &c.fields)
		.filter_map(|f| match &f.kind {
			FieldKind::Group { component } => Some(component),
			_ => None,
		})
		.collect();
	for handle in grouped {
		let comp = schema.get(handle).unwrap();
		let mut obj = Object::new(format!("{handle}_Fields"));
		for field in &comp.fields {
			if let Some(f) = content_field(schema, &comp.handle, field) {
				obj = obj.field(f);
			}
		}
		objects.push(obj);
	}

	// a type of the rows of each table field, its columns as fields
	for comp in &schema.components {
		for field in &comp.fields {
			let FieldKind::Table { columns } = &field.kind else {
				continue;
			};
			let mut row = Object::new(table_row(&comp.handle, &field.handle));
			for c in columns {
				row = row.field(table_cell(c));
			}
			objects.push(row);
		}
	}

	// the asset fields are on `Asset`
	for comp in schema.components.iter().filter(|c| c.handle != ASSET) {
		let mut obj = if comp.entry {
			let mut obj = Object::new(format!("{}_Entry", comp.handle))
				.implement("Entry");
			for (name, _) in entry_common() {
				let mut field = entry_common_field(name);
				if name == "children" {
					for (arg, ty) in children_args(schema) {
						field = field.argument(InputValue::new(arg, ty));
					}
				}
				obj = obj.field(field);
			}
			// the plugins', unless a field has the name
			for f in entry_fields {
				if !comp.fields.iter().any(|x| x.handle == f.name) {
					obj = obj.field((f.field)());
				}
			}
			obj
		} else {
			Object::new(format!("{}_Block", comp.handle))
				.implement("Block")
				.field(GqlField::new(
					"id",
					TypeRef::named_nn(TypeRef::STRING),
					|ctx| block_value(ctx, |b| GqlValue::from(b.id.clone())),
				))
				.field(GqlField::new(
					"typeHandle",
					TypeRef::named_nn(TypeRef::STRING),
					|ctx| block_value(ctx, |b| GqlValue::from(b.ty.clone())),
				))
		};
		// unless the block has a field named so
		if !comp.entry && !comp.fields.iter().any(|f| f.handle == "slug") {
			obj = obj.field(block_slug(comp.title_field.clone()));
		}
		for field in &comp.fields {
			if let Some(f) = content_field(schema, &comp.handle, field) {
				obj = obj.field(f);
			}
		}
		objects.push(obj);
	}
	objects
}

/// The slug of a block for anchors, like a nested entry's in Craft: of
/// the text or number of its title field ([`Schema::block_title`]), none
/// without one.
fn block_slug(title_field: Option<String>) -> GqlField {
	GqlField::new("slug", TypeRef::named(TypeRef::STRING), move |ctx| {
		let title_field = title_field.clone();
		FieldFuture::new(async move {
			let b = ctx.parent_value.try_downcast_ref::<Block>()?;
			let title = match title_field.map(|f| &b.fields[&f]) {
				Some(Value::String(s)) => s.clone(),
				Some(Value::Number(n)) => n.to_string(),
				_ => return Ok(None),
			};
			let slug = crate::entries::slugify(&title);
			Ok((!slug.is_empty()).then(|| FieldValue::value(slug)))
		})
	})
}

/// A cell of a table row, typed by its column: numbers as floats, a
/// boolean is false when it's not set, links like a link field.
fn table_cell(column: &crate::schema::Column) -> GqlField {
	let ty = match &column.kind {
		ColumnKind::Number { .. } => TypeRef::named(TypeRef::FLOAT),
		ColumnKind::Boolean => TypeRef::named_nn(TypeRef::BOOLEAN),
		ColumnKind::Link { .. } => TypeRef::named("Link"),
		ColumnKind::Text
		| ColumnKind::Url
		| ColumnKind::Email
		| ColumnKind::Date
		| ColumnKind::Dropdown { .. } => TypeRef::named(TypeRef::STRING),
	};
	let column = column.clone();
	GqlField::new(column.handle.clone(), ty, move |ctx| {
		let column = column.clone();
		FieldFuture::new(async move {
			let row = ctx.parent_value.try_downcast_ref::<TableRow>()?;
			let v = &row.cells[&column.handle];
			// a value of another shape (a draft) is none
			let v = match &column.kind {
				ColumnKind::Boolean => GqlValue::from(*v == true),
				ColumnKind::Number { .. } => match v.as_f64() {
					Some(n) => GqlValue::from(n),
					None => return Ok(None),
				},
				ColumnKind::Link { .. } if v.is_object() => {
					let to = targets(&ctx, &row.site, v).await?;
					let link = resolve_link(&ctx, v, &row.site, true, &to)?;
					return link.as_ref().map(link_value).transpose();
				}
				ColumnKind::Link { .. } => return Ok(None),
				ColumnKind::Text
				| ColumnKind::Url
				| ColumnKind::Email
				| ColumnKind::Date
				| ColumnKind::Dropdown { .. } => match v.as_str() {
					Some(s) => GqlValue::from(s),
					None => return Ok(None),
				},
			};
			Ok(Some(FieldValue::value(v)))
		})
	})
}

/// The type of the rows of a table field.
pub(super) fn table_row(comp: &str, field: &str) -> String {
	format!("{comp}_{field}_Row")
}
