/** Where the Lucide icons are, see `lucideIcons` in vite.config.ts. */
const BASE = import.meta.env.DEV
	? '/node_modules/lucide-static/icons'
	: `/_studio/lucide-${__LUCIDE_VERSION__}`;

/** The ones the schema brought, see `knowIcons`. */
const known = new Map<string, string>();
const fetched = new Map<string, Promise<string | undefined>>();

/** The svgs of the icons the schema names, they come with it. */
export function knowIcons(icons: Record<string, string>) {
	for (const [name, svg] of Object.entries(icons)) known.set(name, svg);
}

/** A Lucide icon the schema brought, right away. */
export function knownIcon(name: string): string | undefined {
	return known.get(name);
}

/**
 * The svg of a Lucide icon by its name, for the icons of the code (entry
 * types, blocks, nav groups): the schema's, else fetched (in dev, or a
 * name the schema doesn't know); undefined for a name Lucide doesn't
 * have.
 */
export function lucide(name: string): Promise<string | undefined> {
	const svg = known.get(name);
	if (svg) return Promise.resolve(svg);
	if (!/^[a-z0-9-]+$/.test(name)) return Promise.resolve(undefined);
	let file = fetched.get(name);
	if (!file) {
		file = fetch(`${BASE}/${name}.svg`)
			.then(r => (r.ok ? r.text() : undefined))
			// a missing file can be the index page in dev
			.then(t => (t?.includes('<svg') ? t : undefined))
			.catch(() => undefined);
		fetched.set(name, file);
	}
	return file;
}
