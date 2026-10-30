import { request } from './client';
import { knowIcons } from './lucide';

export type LinkKind = 'entry' | 'url' | 'email' | 'phone' | 'asset';

export type Link = {
	type: LinkKind;
	value: string | number | null;
	label?: string;
	newTab?: boolean;
	/** a file to download instead of opening it */
	download?: boolean;
	/** a section of the target page, `kontakt` for `#kontakt` */
	anchor?: string;
};

/** An item of a navigation: a link and the items under it. */
export type NavItem = Link & { children?: NavItem[] };

export type Column = {
	handle: string;
	label: string;
	kind: 'text' | 'url' | 'email';
};

export type FieldKind =
	| { kind: 'text'; max: number | null; multiline: boolean }
	| { kind: 'number'; min: number | null; max: number | null }
	| { kind: 'boolean' }
	| { kind: 'relation'; types: string[]; max: number | null }
	| {
			kind: 'assets';
			images: boolean;
			max: number | null;
			/** where new files go and the picker opens, `Photos/Events` */
			folder?: string;
			/** only these, lowercase without the dot */
			extensions?: string[];
	  }
	| { kind: 'link'; to: LinkKind[]; labels: boolean; newTab: boolean }
	| { kind: 'group'; component: string }
	| { kind: 'markdown' }
	/** texts of the frontend by key, [key, text of the code] */
	| { kind: 'texts'; keys: [string, string][] }
	| { kind: 'dropdown'; options: { value: string; label: string }[] }
	| { kind: 'checkboxes'; options: { value: string; label: string }[] }
	| { kind: 'table'; columns: Column[] }
	| { kind: 'navigation'; to: LinkKind[]; depth: number }
	| {
			kind: 'richText';
			headings: number[];
			lists: boolean;
			links: boolean;
			buttons: boolean;
			blocks: string[];
	  }
	| {
			kind: 'blocks';
			types: string[];
			min: number | null;
			max: number | null;
	  }
	/** a field type of another crate, its editor is a module */
	| { kind: 'custom'; type: string; options: unknown };

export type Field = FieldKind & {
	handle: string;
	label: string;
	required: boolean;
	indexed: boolean;
	translatable: boolean;
};

/** A field of one kind, what the component of that kind gets. */
export type FieldOf<K extends Field['kind']> = Extract<Field, { kind: K }>;

export type Component = {
	handle: string;
	name: string;
	entry: boolean;
	fields: Field[];
	/** the tabs of the editor, none is one tab */
	tabs?: Tab[];
	icon?: SchemaIcon;
};

/** An icon of the code: a Lucide icon by its name, or an svg of its own. */
export type SchemaIcon = { lucide: string } | { svg: string };

/** A tab of the editor with the handles of its fields. */
export type Tab = { name: string; fields: string[] };

export type Site = { handle: string; name: string; language: string };

export type Section = {
	handle: string;
	name: string;
	kind: 'single' | 'channel' | 'structure';
	types: string[];
	uri: string | null;
	/** the uri of an entry type that differs from `uri` */
	type_uris?: Record<string, string>;
	/** the uri in a site that differs from `uri` */
	site_uris?: Record<string, string>;
	/** a boolean field, only entries with it on have a page */
	uri_if?: string;
};

/** A named image size, only these exist. */
export type Transform = {
	handle: string;
	width: number | null;
	height: number | null;
	crop: boolean;
};

export type NavGroup = {
	name: string;
	sections: string[];
	icon?: SchemaIcon;
};

export type Schema = {
	sites: Site[];
	sections: Section[];
	components: Component[];
	nav: NavGroup[];
	/** the url of each site, by handle */
	siteUrls: Record<string, string>;
	transforms: Transform[];
	/** sites not on the website yet */
	offline: string[];
	roles: Role[];
	/** development mode, see `serve --dev` */
	dev: boolean;
	/** the svgs of the Lucide icons it names, by name */
	icons: Record<string, string>;
	/** the plugins with a studio module, `/_plugins/<name>.js` */
	plugins: { name: string; data: unknown }[];
	/** the models of the AI chat, empty if it is off */
	aiModels: { id: string; name: string }[];
	/** markdown files of the code, read only, as html */
	docs: { handle: string; name: string; html: string }[];
};

export type Block = { id?: string; type: string; fields: Values };

/** The values of fields by their handle, of an entry, block or group. */
export type Values = Record<string, unknown>;

/** The value of a field of each kind, as the content keeps it. */
export type ValueOf = {
	text: string;
	number: number;
	boolean: boolean;
	relation: number[];
	assets: number[];
	link: Link;
	group: Values;
	markdown: string;
	/** the own texts by key */
	texts: Record<string, string>;
	dropdown: string;
	checkboxes: string[];
	table: Values[];
	navigation: NavItem[];
	/** a TipTap document */
	richText: Record<string, unknown>;
	blocks: Block[];
	/** what the plugin keeps */
	custom: unknown;
};

/** The value of a field in `values`, undefined while it has none. */
export function valueOf<K extends Field['kind']>(
	values: Values,
	field: FieldOf<K>,
): ValueOf[K] | undefined {
	// the server checks the values against the schema
	return values[field.handle] as ValueOf[K] | undefined;
}

export async function getSchema(): Promise<Schema> {
	const schema = await request<Schema>('GET', '/schema');
	knowIcons(schema.icons ?? {});
	return schema;
}

/** The groups of the top bar, all sections in one without a nav. */
export function navGroups(schema: Schema): NavGroup[] {
	if (schema.nav.length) return schema.nav;
	return [{ name: 'Entries', sections: schema.sections.map(s => s.handle) }];
}

/** The url of an entry on the website, if the site has one. */
export function entryUrl(
	schema: Schema,
	site: string,
	uri: string | null,
): string | null {
	const base = schema.siteUrls[site];
	if (uri === null || !base) return null;
	return `${base.replace(/\/$/, '')}/${uri}`;
}

/** What a role may do, each level includes the ones before. */
export type Access = 'view' | 'edit' | 'publish' | 'delete';

export type Role = {
	handle: string;
	name: string;
	sections: Record<string, Access>;
	assets: Access | null;
	ai: boolean;
};

/** Lookups in the schema by handle. */

export function componentOf(
	schema: Schema,
	handle: string,
): Component | undefined {
	return schema.components.find(c => c.handle === handle);
}

/** The name of an entry type or block, its handle if it is unknown. */
export function componentName(schema: Schema, handle: string): string {
	return componentOf(schema, handle)?.name ?? handle;
}

export function sectionOf(schema: Schema, handle: string): Section | undefined {
	return schema.sections.find(s => s.handle === handle);
}

export function sectionName(schema: Schema, handle: string): string {
	return sectionOf(schema, handle)?.name ?? handle;
}
