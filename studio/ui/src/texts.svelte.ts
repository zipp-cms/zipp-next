/**
 * The texts the components write themselves, English, and the locale of
 * their dates. The studio sets them in its user's language (`ui.texts =
 * …`, `ui.locale = 'de-CH'`). A plugin bundles its own zipp-ui, so the
 * one object lives on `globalThis`: the copy of a plugin reads what the
 * studio set. It reads it when it renders, a change of the language
 * shows in a plugin's view when it opens again.
 */
const en = {
	actions: 'Actions',
	back: 'Back',
	breadcrumbs: 'Breadcrumbs',
	cantMove: "It can't move",
	changed: 'Changed',
	clear: 'Clear',
	close: 'Close',
	collapse: 'Collapse',
	date: 'Date',
	disabled: 'Disabled',
	dragOrClick: 'drag, or click for the moves',
	done: 'Done',
	duplicate: 'Duplicate',
	expand: 'Expand',
	forward: 'Forward',
	hasErrors: 'Has errors',
	hide: 'Hide it on the website',
	loading: 'Loading…',
	monthAfter: 'Month after',
	monthBefore: 'Month before',
	move: 'Move',
	moveDown: 'Move down',
	moveIn: 'Move under the one above',
	moveOut: 'Move out a level',
	moveUp: 'Move up',
	noDate: 'No date',
	nothingFound: 'Nothing found',
	nothingYet: 'Nothing yet',
	pickDay: 'Pick a day',
	pickMonth: 'Pick a month',
	pickYear: 'Pick a year',
	remove: 'Remove',
	resize: 'Drag to change the width, double click for the default',
	search: 'Search',
	show: 'Show it on the website',
	time: 'time',
	today: 'Today',
};

export type UiTexts = { [K in keyof typeof en]: string };

export type Ui = {
	texts: UiTexts;
	/** of dates, `undefined` is the browser's */
	locale: string | undefined;
};

const KEY = Symbol.for('zipp-ui');
const own: Ui = $state({ texts: en, locale: undefined });
const global = globalThis as { [KEY]?: Ui };

export const ui: Ui = (global[KEY] ??= own);
