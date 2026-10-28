use super::*;

use crate::schema::TRANSLATIONS;

/// The texts of the frontend, for crelte's translations plugin:
/// `translations: translations(siteId: $siteId) { common }` in
/// global.graphql. They are the fields of the single `translations`, a
/// preview shows its draft.
pub(super) fn translations_field(
	schema: &Schema,
) -> Option<(Object, GqlField)> {
	let comp = schema.get(TRANSLATIONS)?;
	let mut obj = Object::new("Translations");
	for field in &comp.fields {
		let FieldKind::Texts { keys } = field.kind.clone() else {
			continue;
		};
		let handle = field.handle.clone();
		obj = obj.field(GqlField::new(
			field.handle.clone(),
			TypeRef::named_nn(TypeRef::STRING),
			move |ctx| {
				let (handle, keys) = (handle.clone(), keys.clone());
				FieldFuture::new(async move {
					let entry =
						ctx.parent_value.downcast_ref::<Option<Entry>>();
					let value = entry
						.and_then(|e| e.as_ref())
						.map(|e| e.fields[&handle].clone())
						.unwrap_or_default();
					let texts = crate::content::texts_of(&keys, &value);
					Ok(Some(GqlValue::from(serde_json::to_string(&texts)?)))
				})
			},
		));
	}

	let field = GqlField::new(
		"translations",
		TypeRef::named_nn("Translations"),
		|ctx| {
			FieldFuture::new(async move {
				let mut params = params_from_args(&ctx)?;
				params.push(("section".into(), TRANSLATIONS.into()));
				params.push(("limit".into(), "1".into()));
				let entry: Option<Entry> =
					load(&ctx, &params)?.into_iter().next();
				Ok(Some(FieldValue::owned_any(entry)))
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
