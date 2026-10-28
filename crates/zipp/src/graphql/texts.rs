use super::*;

/// The texts of the frontend, for crelte's translations plugin:
/// `translations: translations(siteId: $siteId) { common }` in global.graphql
pub(super) fn translations_field(
	schema: &Schema,
) -> Option<(Object, GqlField)> {
	if schema.translations.is_empty() {
		return None;
	}
	let mut obj = Object::new("Translations");
	for g in &schema.translations {
		obj = obj.field(GqlField::new(
			g.namespace.clone(),
			TypeRef::named_nn(TypeRef::STRING),
			{
				let ns = g.namespace.clone();
				move |ctx| {
					let ns = ns.clone();
					FieldFuture::new(async move {
						let GqlValue::Object(o) =
							ctx.parent_value.try_to_value()?
						else {
							return Ok(None);
						};
						Ok(o.get(ns.as_str()).cloned().map(FieldValue::value))
					})
				}
			},
		));
	}
	let field = GqlField::new(
		"translations",
		TypeRef::named_nn("Translations"),
		|ctx| {
			FieldFuture::new(async move {
				let params = params_from_args(&ctx)?;
				let schema = ctx.data::<Arc<Schema>>()?;
				let site = params
					.iter()
					.find(|(k, _)| k == "site")
					.map(|(_, v)| v.clone())
					.unwrap_or_else(|| schema.primary_site().handle.clone());
				let db = ctx.data::<Db>()?;
				let texts = translations::texts(&db.conn(), schema, &site)?;
				// each namespace as a JSON string, crelte parses it
				let mut o = async_graphql::indexmap::IndexMap::new();
				for (ns, keys) in texts {
					let json = serde_json::to_string(&keys)?;
					o.insert(
						async_graphql::Name::new(ns),
						GqlValue::from(json),
					);
				}
				Ok(Some(FieldValue::value(GqlValue::Object(o))))
			})
		},
	)
	.argument(InputValue::new(
		"site",
		TypeRef::named_list(TypeRef::STRING),
	))
	.argument(InputValue::new(
		"siteId",
		TypeRef::named_list("QueryArgument"),
	));
	Some((obj, field))
}
