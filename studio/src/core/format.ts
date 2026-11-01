/** A date and time like the browser writes it. */
/**
 * A slug of a title like the server's (`entries::slugify`): lowercase,
 * letters and digits (also ü), anything else one `-`.
 */
export function slugify(s: string): string {
	return s
		.toLowerCase()
		.replace(/[^\p{L}\p{N}]+/gu, '-')
		.replace(/^-+|-+$/g, '');
}

export function time(ms: number): string {
	return new Date(ms).toLocaleString();
}

/** A date and time without seconds, like the date picker shows them. */
export function shortTime(ms: number): string {
	return new Date(ms).toLocaleString(undefined, {
		dateStyle: 'medium',
		timeStyle: 'short',
	});
}

/** A date like the browser writes it. */
export function date(ms: number): string {
	return new Date(ms).toLocaleDateString();
}

/** `publish` becomes `Publish`. */
export function capitalized(s: string): string {
	return s.charAt(0).toUpperCase() + s.slice(1);
}

/** A size of a file: `27 KB`, `1.4 MB`. */
export function fileSize(bytes: number): string {
	if (bytes < 1024 * 1024)
		return `${Math.max(1, Math.round(bytes / 1024))} KB`;
	return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
}

/** How long a video plays: `0:42`, `1:02:05`. */
export function duration(seconds: number): string {
	const s = Math.round(seconds);
	const two = (n: number) => String(n).padStart(2, '0');
	const h = Math.floor(s / 3600);
	const m = Math.floor((s % 3600) / 60);
	return h ? `${h}:${two(m)}:${two(s % 60)}` : `${m}:${two(s % 60)}`;
}

/** `1 entry`, `3 entries`. */
export function plural(n: number, one: string, many: string): string {
	return `${n} ${n === 1 ? one : many}`;
}
