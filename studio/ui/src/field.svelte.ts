import { mount as svelteMount, unmount, type Component } from 'svelte';

/**
 * The contract between the studio and the editor of a field type of
 * another crate (see docs/extending.md): its module exports
 * `mount(target, props)` and gets these props, again with `update`
 * when they change.
 */
export type FieldProps<V = unknown> = {
	/** the field of the schema: handle, label, type, options, … */
	field: { handle: string; label: string; type: string; options: unknown };
	/**
	 * null while there is none. Changed elsewhere (a restore) it comes
	 * with `update`; what the editor sends with `onchange` it shows
	 * itself, the studio doesn't send it back (`fieldModule` does that).
	 */
	value: V | null;
	onchange(value: V | null): void;
	/** the entry it belongs to, null for one not saved yet */
	entryId: number | null;
	site: string;
	/** a revision, or the role may not edit: show, don't change */
	readonly: boolean;
	/** what the server says is wrong with the value, if anything */
	error: string | null;
	host: Host;
	/**
	 * For a field whose items hold blocks (`Field::item_blocks`), null
	 * for others: new blocks and the studio's editor of one.
	 */
	blocks: ItemBlocks | null;
};

/** A block in an item of the value, `{ "block": … }`. */
export type ItemBlock = {
	id: string;
	type: string;
	fields: Record<string, unknown>;
	/** switched off, the website doesn't show it */
	enabled?: false;
};

/** What the studio lends the editor of a field whose items hold blocks. */
export type ItemBlocks = {
	/** the types of blocks the field takes, in its order */
	types: {
		handle: string;
		name: string;
		/** a Lucide icon by its name, or an svg of its own */
		icon: { lucide: string } | { svg: string } | null;
	}[];
	/** a new block of one of the types, with its defaults */
	create(type: string): ItemBlock;
	/**
	 * The studio's editor of the block with this id in the value: its
	 * fields (with their errors and tabs), the image picker and editor,
	 * rich text. Its changes are in the value the module gets with
	 * `update`; a block that left the value shows nothing.
	 */
	mount(target: HTMLElement, id: string): { destroy(): void };
};

/** What the studio lends a module: its api and some of its fields. */
export type Host = {
	/**
	 * A request to `/api<path>` as the logged in user. A failed one
	 * throws an `ApiErrorLike`, a plain `Error` when the network is down.
	 */
	request<T = unknown>(
		method: string,
		path: string,
		body?: unknown,
	): Promise<T>;
	/** one of the studio's own fields, like the image picker */
	mountField(
		target: HTMLElement,
		props: HostFieldProps,
	): {
		update(props: Partial<HostFieldProps>): void;
		destroy(): void;
	};
};

/**
 * What `host.request` throws when the server says no: its message (in
 * the user's language) and `Error` of zipp's error.rs.
 */
export type ApiErrorLike = Error & {
	/** `INVALID`, `FORBIDDEN`, `NOT_FOUND`, `CONFLICT`, `REQUEST`, … */
	type: string;
	/** the values `INVALID` names, `path` like `seo.title` */
	fields: { path: string; message: string }[];
	/** the HTTP status */
	status: number;
};

/** A field of the studio a module mounts with `host.mountField`. */
export type HostFieldProps = {
	/** like a field of the schema: `{ kind: 'assets', images: true, … }` */
	field: unknown;
	value: unknown;
	onchange(value: unknown): void;
};

export type Mounted = {
	update(props: Partial<FieldProps>): void;
	destroy(): void;
};

/** The module of a field type, what `/_fields/<type>.js` exports. */
export type FieldModule = {
	mount(target: HTMLElement, props: FieldProps): Mounted;
};

/**
 * The `mount` of a field module from a Svelte component, its props
 * reactive: `export const mount = fieldModule(SeoField);`. What the
 * editor sends with `onchange` is its `value` at once: the studio sends
 * the value back only when it changes elsewhere (a restore, another
 * site), not what the editor sent itself.
 */
export function fieldModule(component: Component<any>): FieldModule['mount'] {
	return (target, initial) => {
		// the studio's onchange, the editor gets one that shows the value
		let send = initial.onchange;
		const props = $state({
			...initial,
			onchange: (v: unknown) => {
				props.value = v;
				send(v);
			},
		});
		const app = svelteMount(component, { target, props });
		return {
			update({ onchange, ...next }) {
				if (onchange) send = onchange;
				Object.assign(props, next);
			},
			destroy() {
				unmount(app);
			},
		};
	};
}
