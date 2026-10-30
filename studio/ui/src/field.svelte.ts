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
};

/** What the studio lends a module: its api and some of its fields. */
export type Host = {
	/** a request to `/api<path>` as the logged in user */
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
 * reactive: `export const mount = fieldModule(ButtonGroup);`. What the
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
