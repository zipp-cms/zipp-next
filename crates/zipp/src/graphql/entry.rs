use super::*;

pub(super) fn entry_common() -> Vec<(&'static str, TypeRef)> {
	let s = || TypeRef::named_nn(TypeRef::STRING);
	vec![
		// Craft's ids are strings
		("id", TypeRef::named_nn(TypeRef::ID)),
		("sectionHandle", s()),
		("typeHandle", s()),
		("site", s()),
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

pub(super) fn entry_common_field(name: &'static str) -> GqlField {
	let ty = entry_common()
		.into_iter()
		.find(|(n, _)| *n == name)
		.unwrap()
		.1;
	GqlField::new(name, ty, move |ctx| {
		FieldFuture::new(async move {
			let e = ctx.parent_value.try_downcast_ref::<Entry>()?;
			let v = match name {
				"id" => GqlValue::from(e.id.to_string()),
				"sectionHandle" => GqlValue::from(e.section.clone()),
				"typeHandle" => GqlValue::from(e.ty.clone()),
				"site" => GqlValue::from(e.site.clone()),
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
				"postDate" => GqlValue::from(crate::db::iso(
					e.dates.post_date.unwrap_or(e.created_at),
				)),
				"expiryDate" => e
					.dates
					.expiry_date
					.map(|d| GqlValue::from(crate::db::iso(d)))
					.unwrap_or_default(),
				"dateCreated" => GqlValue::from(crate::db::iso(e.created_at)),
				"dateUpdated" => GqlValue::from(crate::db::iso(e.updated_at)),
				"language" => {
					let schema = ctx.data::<Arc<Schema>>()?;
					let site =
						schema.get_site(&e.site).map(|s| s.language.clone());
					GqlValue::from(site.unwrap_or_default())
				}
				"localized" => {
					let schema = ctx.data::<Arc<Schema>>()?;
					let mut out = vec![];
					for s in schema.sites.iter().filter(|s| s.handle != e.site)
					{
						let params =
							site_params(&s.handle, [("ids", e.id.to_string())]);
						out.extend(
							load(&ctx, &params)?.into_iter().map(entry_value),
						);
					}
					return Ok(Some(FieldValue::list(out)));
				}
				"parent" => {
					let Some(p) = e.parent_id else {
						return Ok(None);
					};
					let params = site_params(&e.site, [("ids", p.to_string())]);
					let parent = load(&ctx, &params)?.into_iter().next();
					return Ok(parent.map(entry_value));
				}
				"children" => {
					let params = site_params(
						&e.site,
						[
							("parent", e.id.to_string()),
							("order", "structure".into()),
							// all of them, not the first page of a list
							("limit", MAX_LIMIT.to_string()),
						],
					);
					let children = load(&ctx, &params)?;
					return Ok(Some(FieldValue::list(
						children.into_iter().map(entry_value),
					)));
				}
				_ => unreachable!(),
			};
			Ok(Some(FieldValue::value(v)))
		})
	})
}

/// A field of the content model, read from the JSON of its entry or block.
pub(super) fn content_field(
	schema: &Schema,
	comp: &str,
	field: &crate::schema::Field,
) -> GqlField {
	// a custom type turns the stored value into its output
	let custom = match &field.kind {
		FieldKind::Custom { ty, options } => schema
			.field_types
			.get(ty)
			.map(|t| (t.clone(), options.clone())),
		_ => None,
	};
	let ty = match &field.kind {
		// as it is stored, `2026-11-14` or `2026-11-14T20:00`
		FieldKind::Date { .. } => TypeRef::named(TypeRef::STRING),
		FieldKind::Custom { .. } => {
			match custom.as_ref().map(|(t, _)| t.graphql()) {
				Some(GraphqlType::Object { name, .. }) => TypeRef::named(name),
				Some(GraphqlType::Scalar(s)) => TypeRef::named(scalar_name(s)),
				Some(GraphqlType::Named(name)) => TypeRef::named(name),
				None => TypeRef::named(TypeRef::STRING),
			}
		}
		FieldKind::Text { .. } => TypeRef::named(TypeRef::STRING),
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
		FieldKind::Group { component } => {
			TypeRef::named(format!("{component}_Fields"))
		}
		FieldKind::Blocks { .. } => TypeRef::named_nn_list_nn("Block"),
	};
	let handle = field.handle.clone();
	let kind = field.kind.clone();

	GqlField::new(field.handle.clone(), ty, move |ctx| {
		let handle = handle.clone();
		let kind = kind.clone();
		let custom = custom.clone();
		FieldFuture::new(async move {
			let (fields, site) = match ctx.parent_value.downcast_ref::<Entry>()
			{
				Some(e) => (&e.fields, e.site.clone()),
				None => {
					let b = ctx.parent_value.try_downcast_ref::<Block>()?;
					(&b.fields, b.site.clone())
				}
			};
			let value = fields.get(&handle).cloned().unwrap_or(Value::Null);

			match kind {
				FieldKind::Relation { .. } => {
					let ids: Vec<i64> = value
						.as_array()
						.map(|a| a.iter().filter_map(Value::as_i64).collect())
						.unwrap_or_default();
					if ids.is_empty() {
						return Ok(Some(FieldValue::list(
							Vec::<FieldValue>::new(),
						)));
					}
					let list: Vec<String> =
						ids.iter().map(i64::to_string).collect();
					// all of them, not the first page of a list
					let params = site_params(
						&site,
						[
							("ids", list.join(",")),
							("limit", ids.len().to_string()),
						],
					);
					let mut entries = load(&ctx, &params)?;
					// in the order of the field
					entries
						.sort_by_key(|e| ids.iter().position(|i| *i == e.id));
					Ok(Some(FieldValue::list(
						entries.into_iter().map(entry_value),
					)))
				}
				FieldKind::Assets { .. } => {
					let ids: Vec<i64> = value
						.as_array()
						.map(|a| a.iter().filter_map(Value::as_i64).collect())
						.unwrap_or_default();
					let db = ctx.data::<Db>()?;
					let assets = get_many(&db.conn(), &ids)?;
					Ok(Some(FieldValue::list(
						assets.into_iter().map(FieldValue::owned_any),
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
				FieldKind::Group { component } if value.is_object() => {
					// read like a block, it has no id or type of its own
					Ok(Some(FieldValue::owned_any(Block {
						id: String::new(),
						ty: component,
						site,
						fields: value,
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
					let link = resolve_link(&ctx, &value, &site, new_tab)?;
					Ok(link.map(FieldValue::value))
				}
				FieldKind::Custom { .. } => match custom {
					Some((t, options)) => t.resolve(&ctx, &options, &value),
					None => Ok(None),
				},
				FieldKind::Checkboxes { .. } | FieldKind::Table { .. } => {
					let items = value.as_array().cloned().unwrap_or_default();
					Ok(Some(FieldValue::list(
						items
							.into_iter()
							.map(|v| {
								GqlValue::from_json(v).map(FieldValue::value)
							})
							.collect::<std::result::Result<Vec<_>, _>>()?,
					)))
				}
				FieldKind::Navigation { .. } => {
					let items = nav_items(&ctx, &value, &site)?;
					Ok(Some(FieldValue::list(
						items.into_iter().map(FieldValue::value),
					)))
				}
				FieldKind::Texts { keys } => {
					let texts = crate::content::texts_of(&keys, &value);
					let json = serde_json::to_string(&texts)?;
					Ok(Some(FieldValue::value(GqlValue::from(json))))
				}
				// a value of another shape (a draft, an older migration) is
				// none rather than JSON in the place of an object
				FieldKind::Group { .. }
				| FieldKind::Markdown
				| FieldKind::RichText { .. }
				| FieldKind::Link { .. } => Ok(None),
				FieldKind::Text { .. }
				| FieldKind::Number { .. }
				| FieldKind::Boolean
				| FieldKind::Date { .. }
				| FieldKind::Dropdown { .. } => match value {
					Value::Null => Ok(None),
					v => Ok(Some(FieldValue::value(GqlValue::from_json(v)?))),
				},
			}
		})
	})
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
			obj = obj.field(content_field(schema, &comp.handle, field));
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
				let column = c.handle.clone();
				row = row.field(GqlField::new(
					c.handle.clone(),
					TypeRef::named(TypeRef::STRING),
					move |ctx| {
						let column = column.clone();
						FieldFuture::new(async move {
							let v = ctx.parent_value.try_to_value()?;
							let GqlValue::Object(o) = v else {
								return Ok(None);
							};
							Ok(o.get(column.as_str())
								.cloned()
								.map(FieldValue::value))
						})
					},
				));
			}
			objects.push(row);
		}
	}

	for comp in &schema.components {
		let mut obj = if comp.entry {
			let mut obj = Object::new(format!("{}_Entry", comp.handle))
				.implement("Entry");
			for (name, _) in entry_common() {
				obj = obj.field(entry_common_field(name));
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
		for field in &comp.fields {
			obj = obj.field(content_field(schema, &comp.handle, field));
		}
		objects.push(obj);
	}
	objects
}

/// The type of the rows of a table field.
pub(super) fn table_row(comp: &str, field: &str) -> String {
	format!("{comp}_{field}_Row")
}
