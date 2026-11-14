import { locale, t } from '../i18n/index.svelte';

/** Latin letters as ASCII, like the server's `entries::ascii`. */
const ASCII: Record<string, string> = {
	ä: 'ae',
	æ: 'ae',
	ö: 'oe',
	œ: 'oe',
	ü: 'ue',
	ß: 'ss',
	þ: 'th',
};
for (const [a, cs] of Object.entries({
	a: 'àáâãåāăą',
	c: 'çćĉċč',
	d: 'ďđð',
	e: 'èéêëēĕėęě',
	g: 'ĝğġģ',
	h: 'ĥħ',
	i: 'ìíîïĩīĭįı',
	j: 'ĵ',
	k: 'ķ',
	l: 'ĺļľŀł',
	n: 'ñńņň',
	o: 'òóôõøōŏő',
	r: 'ŕŗř',
	s: 'śŝşšș',
	t: 'ţťŧț',
	u: 'ùúûũūŭůűų',
	w: 'ŵ',
	y: 'ýÿŷ',
	z: 'źżž',
})) {
	for (const c of cs) ASCII[c] = a;
}

/**
 * A slug of a title like the server's (`entries::slugify`): lowercase,
 * Latin letters as ASCII (ü is ue, é is e, other scripts stay) and
 * digits, anything else one `-`. Text in NFD (macOS: `u` and a
 * combining `¨`) is composed first.
 */
export function slugify(s: string): string {
	return [...s.normalize('NFC').toLowerCase()]
		.map(c => ASCII[c] ?? c)
		.join('')
		.replace(/[^\p{L}\p{N}]+/gu, '-')
		.replace(/^-+|-+$/g, '');
}

/** Lower case without accents, for searches: "uber" finds "Über". */
export function plain(s: string): string {
	return s
		.normalize('NFD')
		.replace(/\p{Diacritic}/gu, '')
		.toLowerCase();
}

/** A date and time in the user's language. */
export function time(ms: number): string {
	return new Date(ms).toLocaleString(locale());
}

/** A date and time without seconds, like the date picker shows them. */
export function shortTime(ms: number): string {
	return new Date(ms).toLocaleString(locale(), {
		dateStyle: 'medium',
		timeStyle: 'short',
	});
}

/** A date in the user's language. */
export function date(ms: number): string {
	return new Date(ms).toLocaleDateString(locale());
}

/** `publish` becomes `Publish`. */
export function capitalized(s: string): string {
	return s.charAt(0).toUpperCase() + s.slice(1);
}

/** A size of a file in the user's language: `27 KB`, `1,4 MB`. */
export function fileSize(bytes: number): string {
	if (bytes < 1024 * 1024) {
		const kb = Math.max(1, Math.round(bytes / 1024));
		return t('common.kb', { n: kb.toLocaleString(locale()) });
	}
	const mb = new Intl.NumberFormat(locale(), {
		minimumFractionDigits: 1,
		maximumFractionDigits: 1,
	});
	return t('common.mb', { n: mb.format(bytes / 1024 / 1024) });
}

/** How long a video plays: `0:42`, `1:02:05`. */
export function duration(seconds: number): string {
	const s = Math.round(seconds);
	const two = (n: number) => String(n).padStart(2, '0');
	const h = Math.floor(s / 3600);
	const m = Math.floor((s % 3600) / 60);
	return h ? `${h}:${two(m)}:${two(s % 60)}` : `${m}:${two(s % 60)}`;
}
