import type { Texts } from '../index.svelte';
import type en from '../en/trash';

export default {
	'trash.confirmDelete':
		'{name} endgültig löschen? Dies kann nicht wiederhergestellt werden.',
	'trash.title': 'Papierkorb',
	'trash.note': 'Gelöschte Einträge und Dateien, für {days} Tage',
	'trash.empty': 'Papierkorb leeren',
	'trash.restored': 'Wiederhergestellt:',
	'trash.isEmpty': 'Der Papierkorb ist leer.',
	'trash.entries': 'Einträge',
	'trash.entry': 'Eintrag',
	'trash.section': 'Abschnitt',
	'trash.deleted': 'Gelöscht',
	'trash.assets': 'Dateien',
	'trash.asset': 'Datei',
	'trash.size': 'Grösse',
	'trash.below': {
		one: 'und {n} Eintrag darunter',
		other: 'und {n} Einträge darunter',
	},
	'trash.deleteForGood': 'Endgültig löschen',
	'trash.deleteNameForGood': '{name} endgültig löschen',
	'trash.confirmEntryChildren': {
		one: '{name} endgültig löschen? Und {n} Eintrag darunter. Dies kann nicht wiederhergestellt werden.',
		other: '{name} endgültig löschen? Und {n} Einträge darunter. Dies kann nicht wiederhergestellt werden.',
	},
	'trash.confirmAll': {
		one: '{n} Element endgültig löschen? Es kann nicht wiederhergestellt werden.',
		other: '{n} Elemente endgültig löschen? Sie können nicht wiederhergestellt werden.',
	},
} satisfies Texts<typeof en>;
