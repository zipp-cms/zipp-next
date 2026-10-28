//! What the SEO of a site is made of, like the Craft plugin meierlabs-seo.
//! The global settings (site name, title pattern, description, image,
//! robots, structured data) are the single `seoSettings`, so editors
//! change them with drafts; defaults per section and entry type are code.
//! Every value comes from the first place that has one: the entry's SEO
//! field, its entry type, its section, the global settings.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{Field, Schema, Section};

/// The single of the global SEO settings.
pub const SEO_SETTINGS: &str = "seoSettings";
/// The block of the organization in the global settings.
pub const SEO_ORGANIZATION: &str = "seoOrganization";
/// The block of a day's opening hours.
pub const SEO_OPENING_HOURS: &str = "seoOpeningHours";

pub const ROBOTS: &[(&str, &str)] = &[
	("index, follow", "Index, follow links"),
	("noindex, follow", "No index, follow links"),
	("index, nofollow", "Index, don't follow links"),
	("noindex, nofollow", "No index, don't follow links"),
];

/// schema.org types of the organization, local businesses get opening
/// hours, coordinates and a price range.
pub const ORGANIZATION_TYPES: &[(&str, bool)] = &[
	("Organization", false),
	("Corporation", false),
	("LocalBusiness", true),
	("ProfessionalService", true),
	("MedicalBusiness", true),
	("MedicalClinic", true),
	("Physician", true),
	("Dentist", true),
	("LegalService", true),
	("FinancialService", true),
	("RealEstateAgent", true),
	("Store", true),
	("Restaurant", true),
	("LodgingBusiness", true),
	("EducationalOrganization", false),
	("NGO", false),
	("GovernmentOrganization", false),
	("Person", false),
];

const DAYS: &[&str] = &[
	"Monday",
	"Tuesday",
	"Wednesday",
	"Thursday",
	"Friday",
	"Saturday",
	"Sunday",
];

/// The SEO of a site, see [`Schema::seo`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Seo {
	/// by section handle
	pub sections: BTreeMap<String, SeoDefaults>,
	/// by entry type handle, they win over their section
	pub types: BTreeMap<String, SeoDefaults>,
	/// entries per sitemap file
	pub sitemap_page_size: usize,
}

/// Defaults of a section or an entry type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeoDefaults {
	/// like `{title} | {siteName}`, also `{sectionName}`
	pub title_pattern: Option<String>,
	pub description: Option<String>,
	pub robots: Option<String>,
	/// a field the description comes from, `pageIntro.text` inside a group
	pub description_from: Option<String>,
	/// an images field the image comes from
	pub image_from: Option<String>,
	/// whether the section is in the sitemap
	pub sitemap: bool,
}

impl Default for SeoDefaults {
	fn default() -> Self {
		Self {
			title_pattern: None,
			description: None,
			robots: None,
			description_from: None,
			image_from: None,
			sitemap: true,
		}
	}
}

impl Seo {
	pub fn new() -> Self {
		Self {
			sitemap_page_size: 2000,
			..Default::default()
		}
	}

	pub fn section(mut self, handle: &str, defaults: SeoDefaults) -> Self {
		self.sections.insert(handle.into(), defaults);
		self
	}

	pub fn entry_type(mut self, handle: &str, defaults: SeoDefaults) -> Self {
		self.types.insert(handle.into(), defaults);
		self
	}
}

impl SeoDefaults {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn title_pattern(mut self, pattern: &str) -> Self {
		self.title_pattern = Some(pattern.into());
		self
	}

	pub fn description(mut self, text: &str) -> Self {
		self.description = Some(text.into());
		self
	}

	pub fn robots(mut self, robots: &str) -> Self {
		self.robots = Some(robots.into());
		self
	}

	pub fn description_from(mut self, path: &str) -> Self {
		self.description_from = Some(path.into());
		self
	}

	pub fn image_from(mut self, path: &str) -> Self {
		self.image_from = Some(path.into());
		self
	}

	/// Leaves the section out of the sitemap.
	pub fn no_sitemap(mut self) -> Self {
		self.sitemap = false;
		self
	}
}

impl Schema {
	/// SEO like the Craft plugin meierlabs-seo: adds the single
	/// `seoSettings` for the global settings (put it into a nav group) and
	/// gives every entry `seoJson` in GraphQL; give entry types a
	/// [`Field::seo`] to override single entries.
	pub fn seo(mut self, seo: Seo) -> Self {
		let robots = || Field::dropdown("robots", ROBOTS.iter().copied());
		let types = ORGANIZATION_TYPES.iter().map(|(t, _)| (*t, *t));
		self = self
			.section(Section::single(SEO_SETTINGS, SEO_SETTINGS).name("SEO"))
			.entry(
				SEO_SETTINGS,
				"SEO",
				[
					Field::text("siteName")
						.translatable()
						.label("Site name, empty: the name of the site"),
					Field::text("titlePattern")
						.translatable()
						.label("Title pattern, empty: {title} | {siteName}"),
					Field::text("description")
						.multiline()
						.max(300)
						.translatable(),
					Field::images("image").max(1).label("Image for sharing"),
					robots().label("Robots, empty: index, follow"),
					Field::text("twitterHandle").label("X handle"),
					Field::text("robotsTxt")
						.multiline()
						.translatable()
						.label("robots.txt, empty: allow everything"),
					Field::group("organization", SEO_ORGANIZATION)
						.label("Structured data"),
				],
			)
			.block(
				SEO_ORGANIZATION,
				"Organization",
				[
					Field::dropdown("type", types).label("Type"),
					Field::text("name"),
					Field::text("alternateName"),
					Field::text("legalName"),
					Field::text("description").multiline(),
					Field::text("url").label("URL, empty: the site"),
					Field::images("logo").max(1),
					Field::images("image").max(1),
					Field::text("email"),
					Field::text("telephone"),
					Field::text("street"),
					Field::text("postalCode"),
					Field::text("locality"),
					Field::text("region"),
					Field::text("country"),
					Field::number("latitude"),
					Field::number("longitude"),
					Field::text("priceRange"),
					Field::blocks("openingHours", [SEO_OPENING_HOURS]),
					Field::text("sameAs")
						.multiline()
						.label("Profiles, one url per line"),
					Field::text("creatorName").label("Made by"),
					Field::dropdown(
						"creatorType",
						[
							("Organization", "Organization"),
							("Person", "Person"),
						],
					),
					Field::text("creatorUrl"),
				],
			)
			.block(
				SEO_OPENING_HOURS,
				"Opening hours",
				[
					Field::dropdown("day", DAYS.iter().map(|d| (*d, *d)))
						.required(),
					Field::text("opens").required().label("Opens (09:00)"),
					Field::text("closes").required().label("Closes (18:00)"),
				],
			);
		self.seo = Some(seo);
		self
	}
}
