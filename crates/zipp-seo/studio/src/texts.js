// The plugin's texts in English and German. The studio sets `<html lang>`
// to its user's language; a view reads it when it opens.
const all = {
	en: {
		preview: 'Preview',
		searchResult: 'Search result',
		title: 'Title',
		description: 'Description',
		image: 'Image',
		robots: 'Robots',
		inherit: 'Inherit',
		canonicalUrl: 'Canonical URL',
		showSharing: 'Show sharing (Open Graph, X)',
		hideSharing: 'Hide sharing (Open Graph, X)',
		socialTitle: 'Social title',
		socialDescription: 'Social description',
		socialImage: 'Social image',
		section: 'Section',
		imageFrom: 'Image from',
		sitemap: 'Sitemap',
		from: 'from',
		yes: 'Yes',
		no: 'No',
		setupNote:
			'The defaults of the code, from the SEO plugin. Editors override them per site in the',
		settings: 'SEO settings',
		setupNoteEnd:
			"(Per section and entry type), an entry's SEO field overrides both; empty values come from the SEO settings.",
	},
	de: {
		preview: 'Vorschau',
		searchResult: 'Suchergebnis',
		title: 'Titel',
		description: 'Beschreibung',
		image: 'Bild',
		robots: 'Robots',
		inherit: 'Geerbt',
		canonicalUrl: 'Kanonische URL',
		showSharing: 'Teilen zeigen (Open Graph, X)',
		hideSharing: 'Teilen ausblenden (Open Graph, X)',
		socialTitle: 'Titel beim Teilen',
		socialDescription: 'Beschreibung beim Teilen',
		socialImage: 'Bild beim Teilen',
		section: 'Abschnitt',
		imageFrom: 'Bild aus',
		sitemap: 'Sitemap',
		from: 'aus',
		yes: 'Ja',
		no: 'Nein',
		setupNote:
			'Die Vorgaben des Codes, vom SEO-Plugin. Redaktoren überschreiben sie pro Website in den',
		settings: 'SEO-Einstellungen',
		setupNoteEnd:
			'(«Per section and entry type»), das SEO-Feld eines Eintrags überschreibt beide; leere Werte kommen aus den SEO-Einstellungen.',
	},
};

/** The texts in the studio's language, English for any other. */
export function texts() {
	return all[document.documentElement.lang] ?? all.en;
}
