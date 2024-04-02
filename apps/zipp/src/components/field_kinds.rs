use tracing::error;

use super::{Field, FieldDto};

#[derive(Clone)]
pub struct FieldKind {
	pub name: String,
	pub settings: Vec<String>,
	pub rules: Vec<String>,
}

impl FieldKind {
	pub fn new(name: String) -> Self {
		Self {
			name,
			settings: Vec::new(),
			rules: Vec::new(),
		}
	}

	pub fn with_settings(mut self, settings: Vec<&str>) -> Self {
		self.settings = settings.iter().map(|s| s.to_string()).collect();
		self
	}
}

pub struct FieldKinds(Vec<FieldKind>);

#[derive(Debug, thiserror::Error)]
pub enum ParseFieldError {
	#[error("Field has unknown kind: {kind}")]
	KindNotFound { kind: String },
	#[error("Invalid settings: {settings:?}")]
	InvalidSettings { settings: Vec<String> },
}

impl FieldKinds {
	pub fn push(&mut self, kind: FieldKind) {
		self.0.push(kind);
	}

	pub fn field_from_dto(
		&self,
		dto: FieldDto,
	) -> Result<Field, ParseFieldError> {
		// todo:
		// [x] - check if kind exists
		// [x] - check if settings are valid

		// check if kind exists
		let kind = self
			.0
			.iter()
			.find(|k| k.name == dto.kind)
			.ok_or(ParseFieldError::KindNotFound { kind: dto.kind })?
			.clone();

		// check if settings are valid
		if dto.settings.iter().any(|s| !kind.settings.contains(&s.0)) {
			error!("Invalid settings: {:?}", dto.settings);
			return Err(ParseFieldError::InvalidSettings {
				settings: dto.settings.into_iter().map(|s| s.0).collect(),
			});
		}

		Ok(Field {
			kind: kind.name,
			settings: dto.settings,
		})
	}
}

impl Default for FieldKinds {
	fn default() -> Self {
		FieldKinds(vec![
			FieldKind::new("boolean".to_string()).with_settings(vec![]),
			FieldKind::new("number".to_string())
				.with_settings(vec!["min", "max", "step"]),
			FieldKind::new("text".to_string()).with_settings(vec!["rules"]),
			FieldKind::new("richtext".to_string()).with_settings(vec![]),
			FieldKind::new("relation".to_string())
				.with_settings(vec!["collection"]),
			FieldKind::new("media".to_string()).with_settings(vec!["mimetype"]),
			FieldKind::new("component".to_string()).with_settings(vec![
				"component",
				"min",
				"max",
			]),
		])
	}
}
