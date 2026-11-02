import { session } from '../core/session.svelte';
import en from './en';
import de from './de';
import enUi from './en/ui';
import { ui, type UiTexts } from 'zipp-ui/texts';

/**
 * The studio in the user's language: `t('key', params)` gives a text of
 * `en/` or `de/`, one file per area. `{name}` in a text is a param; a
 * text with `one` and `other` is a plural, by the param `n`. What reads
 * `t` in a component changes with the language; a `.ts` calls it when
 * the text is shown, not once when it loads.
 */

/** The languages of the studio, also the server's `users::LANGUAGES`. */
export const LANGUAGES = { en: 'English', de: 'Deutsch' } as const;

export type Lang = keyof typeof LANGUAGES;

/** One text, or a plural by the param `n`. */
export type Text = string | { one: string; other: string };

/** The texts of a language: every key of English, nothing else. */
export type Texts<T> = { [K in keyof T]: Text };

export type Key = keyof typeof en;

const texts: Record<Lang, Texts<typeof en>> = { en, de };

function isLang(l: string | undefined): l is Lang {
	return !!l && l in LANGUAGES;
}

/** German if the browser wants German first, else English. */
export function browserLang(): Lang {
	const l = (navigator.languages?.[0] ?? navigator.language ?? '')
		.slice(0, 2)
		.toLowerCase();
	return isLang(l) ? l : 'en';
}

/** The user's choice, without one (and before the login) the browser's. */
export function lang(): Lang {
	const l = session.user?.language;
	return isLang(l) ? l : browserLang();
}

/**
 * For dates and numbers through `Intl`: the browser's own when it is the
 * language (de-CH writes 14.11.2026 like de-DE but numbers Swiss), else
 * the language.
 */
export function locale(): string {
	const l = lang();
	const own = navigator.language;
	return own.slice(0, 2).toLowerCase() === l ? own : l;
}

type Params = Record<string, string | number>;

/** The text of `key` in the language, the plural by `params.n`. */
function raw(key: Key, params: Params): string {
	const l = lang();
	const text: Text = texts[l][key] ?? en[key];
	if (typeof text === 'string') return text;
	const rule = new Intl.PluralRules(l).select(Number(params.n));
	return rule === 'one' ? text.one : text.other;
}

export function t(key: Key, params: Params = {}) {
	return raw(key, params).replace(/\{(\w+)\}/g, (all, name) =>
		name in params ? String(params[name]) : all,
	);
}

/** A part of a text: a param has its `name`, the text between none. */
export type Part = { text: string; name?: string };

/**
 * A text in parts, for markup around its params (a link, code), which
 * `Sentence.svelte` renders.
 */
export function tParts(key: Key, params: Params): Part[] {
	return raw(key, params)
		.split(/\{(\w+)\}/)
		.map((text, i) => {
			// the names are the odd parts of the split
			if (i % 2 === 0) return { text };
			const name = text;
			return name in params
				? { text: String(params[name]), name }
				: { text: `{${name}}` };
		})
		.filter(p => p.text);
}

/** The texts of zipp-ui's components in the language, for `ui.texts`. */
export function uiTexts(): UiTexts {
	// the keys of our file, not of `ui.texts`: an effect sets that
	const keys = Object.keys(enUi).map(k => k.slice(3) as keyof UiTexts);
	return Object.fromEntries(keys.map(k => [k, t(`ui.${k}`)])) as UiTexts;
}

/**
 * The language for what isn't the studio's: zipp-ui's texts and dates
 * (also in plugins) and `<html lang>`, which plugins can read for theirs.
 */
export function applyLanguage() {
	ui.texts = uiTexts();
	ui.locale = locale();
	document.documentElement.lang = lang();
}
