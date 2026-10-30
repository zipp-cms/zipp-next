//! The plugin: what SEO adds to the content model, GraphQL, the routes
//! and the studio.

use std::sync::Arc;

use serde_json::{Map, Value, json};
use zipp::async_graphql::dynamic::{
	FieldValue, ResolverContext, Scalar as GqlScalar, TypeRef,
};
use zipp::axum::Router;
use zipp::{
	AppState, EntryField, Field, FieldKind, FieldRelation, FieldType,
	GraphqlExt, GraphqlType, Plugin, PluginStudio, Schema, StudioModule,
	Target,
};

use crate::graphql::{
	SEO_JSON, entry_seo, entry_seo_field, root_fields, seo_types,
};
use crate::resolve::social_transform;
use crate::{ROBOTS, Seo, add_settings, add_targets};

/// The handle of the plugin and of its field type.
pub const NAME: &str = "seo";

/// The SEO of single entries, with the defaults as placeholders: title,
/// description, image, social title, description and image, robots and
/// canonical url, each optional.
pub fn seo_field(handle: &str) -> Field {
	Field::custom(handle, NAME, Value::Null)
		.label("SEO")
		.translatable()
}

/// Whether a field is an SEO field.
pub fn is_seo(field: &Field) -> bool {
	matches!(&field.kind, FieldKind::Custom { ty, .. } if ty == NAME)
}

impl Plugin for Seo {
	fn name(&self) -> &'static str {
		NAME
	}

	fn schema(&self, schema: Schema) -> Schema {
		let schema = add_settings(schema)
			.field_type(SeoField)
			// the image of a page when it is shared
			.transform(social_transform());
		add_targets(schema)
	}

	fn finish(&self, schema: Schema) -> Schema {
		add_targets(schema)
	}

	fn check(&self, schema: &Schema) -> Result<(), String> {
		for s in self.sections.keys() {
			if schema.get_section(s).is_none() {
				return Err(format!("unknown section {s}"));
			}
		}
		// the fields defaults come from, in every entry type they apply to
		let mut uses = vec![];
		for (ty, d) in &self.types {
			if schema.entry_type(ty).is_none() {
				return Err(format!("unknown entry type {ty}"));
			}
			uses.push((ty.as_str(), d));
		}
		for (s, d) in &self.sections {
			let types = schema.get_section(s).map(|s| &s.types);
			uses.extend(types.into_iter().flatten().map(|t| (t.as_str(), d)));
		}
		for (ty, d) in uses {
			let comp = schema.entry_type(ty).unwrap();
			let paths = d
				.title_from
				.iter()
				.chain(&d.description_from)
				.chain(&d.image_from);
			for path in paths {
				if schema.field_at(comp, path).is_none() {
					return Err(format!("{ty} has no field {path}"));
				}
			}
			if let Some(path) = &d.image_from
				&& !matches!(
					schema.field_at(comp, path).map(|f| &f.kind),
					Some(FieldKind::Assets { .. })
				) {
				return Err(format!("{ty}.{path} is no images field"));
			}
		}
		for comp in &schema.components {
			for f in &comp.fields {
				if f.handle == "seoJson" {
					return Err(format!(
						"{}.seoJson: the name is the SEO's in GraphQL",
						comp.handle
					));
				}
				if is_seo(f) && !comp.entry {
					return Err(format!(
						"{}.{}: an SEO field belongs to an entry type",
						comp.handle, f.handle
					));
				}
			}
		}
		Ok(())
	}

	fn graphql(&self, _schema: &Schema) -> GraphqlExt {
		let mut types = vec![
			GqlScalar::new(SEO_JSON)
				.description(
					"The resolved SEO as one JSON object, what the npm \
					package meierlabs-seo reads",
				)
				.into(),
		];
		types.extend(seo_types().into_iter().map(Into::into));
		let entry = |name: &'static str, ty: &'static str| EntryField {
			name: name.into(),
			ty: TypeRef::named(ty),
			field: Arc::new(move || entry_seo_field(name, ty)),
		};
		GraphqlExt {
			types,
			// `seo` like the plugin's, unless a type has a field so named
			entry_fields: vec![entry("seoJson", SEO_JSON), entry("seo", "Seo")],
			root_fields: root_fields().into(),
		}
	}

	fn api_routes(&self) -> Router<AppState> {
		crate::routes::routes()
	}

	fn routes(&self, _schema: &Schema) -> Router<AppState> {
		crate::sitemap::routes()
	}

	fn studio(&self) -> Option<PluginStudio> {
		Some(PluginStudio {
			js: include_str!("../assets/seo.js"),
			// the defaults of the code, the setup shows them
			data: json!({ "sections": self.sections, "types": self.types }),
		})
	}
}

/// The SEO field: its values are checked here, GraphQL resolves the
/// SEO of its entry like the plugin's `seo`.
pub struct SeoField;

impl FieldType for SeoField {
	fn name(&self) -> &'static str {
		NAME
	}

	fn validate(
		&self,
		_options: &Value,
		value: &Value,
	) -> Result<Value, String> {
		let Value::Object(seo) = value else {
			return Err("not an object".into());
		};
		let mut out = Map::new();
		for (key, v) in seo {
			let empty = v.is_null() || v == "" || v == &json!([]);
			let valid = match key.as_str() {
				_ if empty => true,
				"title" | "description" | "socialTitle"
				| "socialDescription" => v.is_string(),
				"image" | "socialImage" => v.as_array().is_some_and(|a| {
					a.len() <= 1 && a.iter().all(Value::is_i64)
				}),
				"robots" => ROBOTS.iter().any(|(r, _)| v == *r),
				// absolute, it is what search engines index
				"canonicalUrl" => v.as_str().is_some_and(|u| {
					u.starts_with("https://") || u.starts_with("http://")
				}),
				_ => false,
			};
			if !valid {
				return Err(format!("{key}: not a valid value"));
			}
			if !empty {
				out.insert(key.clone(), v.clone());
			}
		}
		Ok(match out.is_empty() {
			true => Value::Null,
			false => Value::Object(out),
		})
	}

	fn graphql(&self) -> GraphqlType {
		GraphqlType::Named("Seo")
	}

	/// The resolved SEO of the entry, also without values of its own.
	fn resolve<'a>(
		&self,
		ctx: &ResolverContext<'a>,
		_options: &Value,
		_value: &Value,
	) -> zipp::async_graphql::Result<Option<FieldValue<'a>>> {
		entry_seo(ctx)
	}

	/// The images are images like an images field's.
	fn relations(&self, _options: &Value, value: &Value) -> Vec<FieldRelation> {
		["image", "socialImage"]
			.into_iter()
			.filter_map(|key| {
				let ids: Vec<i64> = value[key]
					.as_array()?
					.iter()
					.filter_map(Value::as_i64)
					.collect();
				(!ids.is_empty()).then(|| FieldRelation {
					path: key.into(),
					target: Target::Assets {
						images: true,
						extensions: vec![],
					},
					ids,
				})
			})
			.collect()
	}

	fn typescript(&self) -> String {
		"{ title?: string; description?: string; image?: number[]; \
		socialTitle?: string; socialDescription?: string; \
		socialImage?: number[]; robots?: string; canonicalUrl?: string }"
			.into()
	}

	fn studio(&self) -> StudioModule {
		StudioModule {
			js: include_str!("../assets/seo.js"),
		}
	}
}
