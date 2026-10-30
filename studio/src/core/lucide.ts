/** Where the Lucide icons are, see `lucideIcons` in vite.config.ts. */
const BASE = import.meta.env.DEV
	? '/node_modules/lucide-static/icons'
	: `/_studio/lucide-${__LUCIDE_VERSION__}`;

const cache = new Map<string, Promise<string | undefined>>();

/**
 * The svg of a Lucide icon by its name, for the icons of the code (entry
 * types, blocks, nav groups); undefined for a name Lucide doesn't have.
 */
export function lucide(name: string): Promise<string | undefined> {
	if (!/^[a-z0-9-]+$/.test(name)) return Promise.resolve(undefined);
	let svg = cache.get(name);
	if (!svg) {
		svg = fetch(`${BASE}/${name}.svg`)
			.then(r => (r.ok ? r.text() : undefined))
			// a missing file can be the index page in dev
			.then(t => (t?.includes('<svg') ? t : undefined))
			.catch(() => undefined);
		cache.set(name, svg);
	}
	return svg;
}
