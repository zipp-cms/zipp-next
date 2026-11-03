import type { Texts } from '../index.svelte';
import type en from '../en/entries';

export default {
	// the list
	'entries.noSection': 'Es gibt keinen Abschnitt {section}',
	'entries.viewOnWebsite': 'Auf der Website ansehen',
	'entries.confirmTrash': '{title} in den Papierkorb legen?',
	'entries.confirmTrashBelow': {
		one: '{title} in den Papierkorb legen? Der untergeordnete Eintrag auch.',
		other: '{title} in den Papierkorb legen? Die {n} untergeordneten Einträge auch.',
	},
	'entries.new': 'Neuer Eintrag',
	'entries.search': 'Einträge suchen',
	'entries.status': 'Status',
	'entries.type': 'Typ',
	'entries.updated': 'Geändert',
	'entries.move': '{title} verschieben',
	'entries.unpublishedChanges': 'Nicht veröffentlichte Änderungen',
	'entries.liveFrom': 'Live ab {time}',
	'entries.pending': 'Geplant',
	'entries.expiredAt': 'Abgelaufen am {time}',
	'entries.expired': 'Abgelaufen',
	'entries.actionsFor': 'Aktionen für {title}',
	'entries.noneYet': 'Noch keine Einträge',
	'entries.shownOf': '{shown} von {total}',

	// the picker
	'entries.chooseType': '{type} auswählen',
	'entries.chooseEntry': 'Eintrag auswählen',
	'entries.showBelow': 'Untergeordnete Einträge zeigen',
	'entries.hideBelow': 'Untergeordnete Einträge ausblenden',
	'entries.cantChoose': '{type} kann hier nicht ausgewählt werden',
	'entries.alreadyChosen': 'Bereits ausgewählt',
	'entries.searchTypes': '{types} suchen',
	'entries.didntLoad': 'Die Einträge konnten nicht geladen werden.',
	'entries.sections': 'Abschnitte',
	'entries.fieldFull': 'Das Feld ist voll, zuerst einen entfernen',
	'entries.chosen': '{n} ausgewählt',
	'entries.chosenOf': '{n} von {max} ausgewählt',
	'entries.loadMore': 'Mehr laden',
	'entries.firstFound': 'Die ersten {n}, mehr Wörter grenzen die Suche ein.',
	'entries.openInFlyout': 'Im Seitenfenster öffnen',

	// the sites
	'entries.sitePublished': '{site}: veröffentlicht',
	'entries.siteNotPublished': '{site}: nicht veröffentlicht',
	'entries.siteOpen': '{title}, öffnen',

	// the preview
	'entries.desktop': 'Desktop',
	'entries.tablet': 'Tablet',
	'entries.mobile': 'Mobil',
	'entries.reload': 'Neu laden',
	'entries.websiteWidth': 'Breite der Website',
	'entries.website': 'Website',

	// the editor
	'entries.confirmDiscard':
		'Die nicht veröffentlichten Änderungen verwerfen?',
	'entries.confirmCopy': 'Den Inhalt durch den von {site} ersetzen?',
	'entries.fieldLayout': 'Feldlayout «{name}»',
	'entries.draftBy': 'von {user}, {time}',
	'entries.live': 'Live',
	'entries.notPublished': 'Nicht veröffentlicht',
	'entries.notSaved': 'Nicht gespeichert',
	'entries.savedAsDraft': 'Als Entwurf gespeichert, noch nicht live',
	'entries.moveToTrash': 'In den Papierkorb legen',
	'entries.discardTitle': 'Die nicht veröffentlichten Änderungen verwerfen',
	'entries.liveTitle': 'Die Website neben dem Editor, mit dem Entwurf',
	'entries.livePreview': 'Live-Vorschau',
	'entries.previewTitle': 'Den Entwurf auf der Website öffnen',
	'entries.viewTitle': 'Die Seite auf der Website öffnen',
	'entries.mayNotPublish':
		'Ihre Rolle kann Entwürfe bearbeiten, veröffentlichen tut sie jemand anderes',
	'entries.conflict':
		'während Sie bearbeitet haben. Ihre letzten Änderungen sind noch nicht gespeichert.',
	'entries.loadTheirs': 'Deren Änderungen laden',
	'entries.keepMine': 'Meine behalten',
	'entries.confirmType':
		'Den Eintragstyp zu {type} ändern? Er hat kein Feld wie {fields}: deren Inhalt wird in jeder Website entfernt.',

	// status and sidebar
	'entries.publishedIn': 'Veröffentlicht in {site}',
	'entries.notLiveYet': 'Noch nicht live, ab {time}',
	'entries.notLiveAnymore': 'Nicht mehr live, abgelaufen am {time}',
	'entries.postDate': 'Veröffentlichungsdatum',
	'entries.whenPublished': 'Beim Veröffentlichen',
	'entries.expiryDate': 'Ablaufdatum',
	'entries.fieldsInSetup': 'Seine Felder in der Einrichtung',
	'entries.parent': 'Übergeordneter Eintrag',
	'entries.noParent': 'Keiner, auf oberster Ebene',
	'entries.slug': 'Slug',
	'entries.fromTitle': 'aus dem Titel',
	'entries.outlineOf': 'Übersicht von {field}',
	'entries.outline': 'Übersicht · {field}',
	'entries.missing': 'Fehlt',
	'entries.errors': { one: '{n} Fehler', other: '{n} Fehler' },
	'entries.copyContent': 'Inhalt kopieren',
	'entries.fromOtherSite': 'Von einer anderen Website…',

	// revisions
	'entries.revisions': 'Versionen',
	'entries.revisionsCount': 'Versionen · {n}',
	'entries.revisionOf': 'Version vom {time}, nur lesen',
	'entries.revisionOfBy': 'Version vom {time} von {user}, nur lesen',
	'entries.changedList': 'Geändert: {changes}',
	'entries.backToEditing': 'Zurück zum Bearbeiten',
	'entries.noContentHere':
		'Diese Version hat keinen Inhalt für diese Website.',
	'entries.revisionLive': 'live',
	'entries.created': 'Erstellt',
	'entries.confirmRestore': 'Diese Version wiederherstellen?',
	'entries.confirmRestoreDraft':
		'Diese Version wiederherstellen? Der Entwurf wird ersetzt.',
} satisfies Texts<typeof en>;
