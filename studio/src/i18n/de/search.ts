import type { Texts } from '../index.svelte';
import type en from '../en/search';

export default {
	'search.placeholder': 'Einträge und Dateien suchen',
	'search.searching': 'Sucht…',
	'search.notPublished': 'Nicht veröffentlicht',
	'search.assets': 'Dateien',
} satisfies Texts<typeof en>;
