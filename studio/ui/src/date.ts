/**
 * The days of the date picker, with the browser's `Date` and `Intl` only:
 * a day is `2026-11-14`, the calendar's weeks start on Monday.
 */

/** The day of a `2026-11-14`, null if it isn't one of the calendar. */
export function parseDay(s: string | null | undefined): Date | null {
	const m = /^(\d{4})-(\d{2})-(\d{2})/.exec(s ?? '');
	if (!m) return null;
	const [y, mo, d] = [Number(m[1]), Number(m[2]), Number(m[3])];
	const date = new Date(y, mo - 1, d);
	const same =
		date.getFullYear() === y &&
		date.getMonth() === mo - 1 &&
		date.getDate() === d;
	return same ? date : null;
}

const pad = (n: number) => String(n).padStart(2, '0');

/** `2026-11-14` of a day. */
export function formatDay(d: Date): string {
	return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;
}

/** The first of the month of a day. */
export function monthOf(d: Date): Date {
	return new Date(d.getFullYear(), d.getMonth(), 1);
}

/** The days of a month in weeks from Monday, null before the first. */
export function monthDays(month: Date): (Date | null)[] {
	const [y, m] = [month.getFullYear(), month.getMonth()];
	const blank = (new Date(y, m, 1).getDay() + 6) % 7;
	const count = new Date(y, m + 1, 0).getDate();
	return [
		...Array<null>(blank).fill(null),
		...Array.from({ length: count }, (_, i) => new Date(y, m, i + 1)),
	];
}

/** Mo, Tu, … in the browser's language, from Monday. */
export function weekdays(locale?: string): string[] {
	const f = new Intl.DateTimeFormat(locale, { weekday: 'short' });
	// 2024-01-01 was a Monday
	return Array.from({ length: 7 }, (_, i) =>
		f.format(new Date(2024, 0, 1 + i)),
	);
}

/** January, February, … in the browser's language. */
export function monthNames(locale?: string): string[] {
	const f = new Intl.DateTimeFormat(locale, { month: 'long' });
	return Array.from({ length: 12 }, (_, i) => f.format(new Date(2000, i, 1)));
}
