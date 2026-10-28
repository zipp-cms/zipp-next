# SEO

zipp does what the Craft plugin meierlabs-seo does, so its npm package
works unchanged.

## SEO

`.seo(Seo::new()
.section("events", SeoDefaults::new().title_pattern("{title} – Events |
{siteName}")).entry_type("page", SeoDefaults::new()
.description_from("intro").image_from("hero")))` adds the single
`seoSettings` (site name, title pattern, description, image, robots,
robots.txt, structured data of the organization) and `Field::seo` for
single entries. Every value comes from the entry's SEO field, its entry
type, its section or the settings. GraphQL has `seoJson` on every entry
and the root `seoJson(siteId:)` in the plugin's shape (title, meta, links,
JSON-LD with Organization, WebSite, WebPage and breadcrumbs, hreflang
alternates, a 1200×630 social image), so the npm package `meierlabs-seo`
(`<Seo>`, `seoRoutes`) works unchanged: zipp answers its sitemap and
robots.txt actions. `serve --noindex` (and `--dev`) makes every page
noindex and robots.txt disallow everything.
