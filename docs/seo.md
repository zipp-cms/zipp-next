# SEO

zipp does what the Craft plugin meierlabs-seo does, so its npm package
works unchanged.

## SEO

SEO is a plugin, the crate `zipp-seo`. `.plugin(Seo::new()
.section("events", SeoDefaults::new().title_pattern("{title} – Events |
{siteName}")).entry_type("page", SeoDefaults::new()
.description_from("intro").image_from("hero")))` adds the single
`seoSettings` (site name, title pattern, description, image, robots,
robots.txt, structured data of the organization), the field type for
`seo_field("seo")` of single entries and the 1200×630 transform
`seoSocial`; a section's setup page shows its defaults. Under "Per section and entry type" in the settings
editors override title pattern, description, image, robots and the
sitemap of a section or an entry type, per site (other sites inherit all
but the description from the primary site). Every value comes from the
entry's SEO field, its entry type (settings, then code), its section
(settings, then code) or the global settings. Which fields the description
and image come from stays code, it depends on the content model; such a
field's text comes before the descriptions of the settings. GraphQL has `seoJson` on every entry
and the root `seoJson(siteId:)` in the plugin's shape (title, meta, links,
JSON-LD with Organization, WebSite, WebPage and breadcrumbs, hreflang
alternates, a 1200×630 social image), so the npm package `meierlabs-seo`
(`<Seo>`, `seoRoutes`) works unchanged: zipp answers its sitemap and
robots.txt actions. `serve --noindex` (and `--dev`) makes every page
noindex and robots.txt disallow everything.

## Differences from the plugin

- Defaults of sections and entry types are code, the settings override
  them (half code, half database, operator 2026-10-28); the fields the
  description and image come from are only code. `Cms` calls
  `Schema::finish`, which fills the choice of sections and types.
- Title patterns know `{title}`, `{siteName}` and `{sectionName}`, not
  Craft's object templates.
- `postDate` is when the entry was created; zipp has no scheduled posts.
- Without a site name the title uses the site's name (like "Deutsch"):
  set the site name in the settings.
