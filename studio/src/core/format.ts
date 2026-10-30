
/** A date and time like the browser writes it. */
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
