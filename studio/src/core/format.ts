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

/** `1 entry`, `3 entries`. */
export function plural(n: number, one: string, many: string): string {
	return `${n} ${n === 1 ? one : many}`;
}
