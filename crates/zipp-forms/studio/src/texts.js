// The plugin's texts in English and German. The studio sets `<html lang>`
// to its user's language; a view reads it when it opens.
const all = {
	en: {
		title: 'Submissions',
		note: 'What visitors sent with the forms, the newest first.',
		allForms: 'All forms',
		form: 'Form',
		date: 'Date',
		site: 'Site',
		content: 'Content',
		none: 'No submissions yet.',
		more: 'Show more',
		delete: 'Delete',
		confirmDelete: 'Delete this submission for good?',
		empty: '–',
		page: 'Page',
		pages: 'Pages',
		addPage: 'Add page',
		pageName: 'Name of the page',
		pageNameNote: 'The title of this step on the website.',
		deletePage: 'Delete page',
		confirmDeletePage: 'Delete this page and its fields?',
		moveTo: 'Move to page',
		noFields: 'No fields on this page yet.',
	},
	de: {
		title: 'Einsendungen',
		note: 'Was Besucher mit den Formularen geschickt haben, die neusten zuerst.',
		allForms: 'Alle Formulare',
		form: 'Formular',
		date: 'Datum',
		site: 'Website',
		content: 'Inhalt',
		none: 'Noch keine Einsendungen.',
		more: 'Mehr zeigen',
		delete: 'Löschen',
		confirmDelete: 'Diese Einsendung endgültig löschen?',
		empty: '–',
		page: 'Seite',
		pages: 'Seiten',
		addPage: 'Seite hinzufügen',
		pageName: 'Name der Seite',
		pageNameNote: 'Der Titel dieses Schritts auf der Website.',
		deletePage: 'Seite löschen',
		confirmDeletePage: 'Diese Seite und ihre Felder löschen?',
		moveTo: 'Auf Seite verschieben',
		noFields: 'Noch keine Felder auf dieser Seite.',
	},
};

/** The texts in the studio's language, English for any other. */
export function texts() {
	return all[document.documentElement.lang] ?? all.en;
}
