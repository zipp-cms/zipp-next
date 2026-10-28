import { request } from './client';

export type LinkKind = 'entry' | 'url' | 'email' | 'phone' | 'asset';

export type Link = {
	type: LinkKind;
	value: string | number | null;
	label?: string;
	newTab?: boolean;
	/** a file to download instead of opening it */
	download?: boolean;
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
	| { kind: 'assets'; images: boolean; max: number | null }
	| { kind: 'link'; to: LinkKind[]; labels: boolean; newTab: boolean }
	| { kind: 'group'; component: string }
	| { kind: 'markdown' }
	/** the SEO of single entries, see zipp's seo */
	| { kind: 'seo' }
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

export type Component = {
	handle: string;
	name: string;
	entry: boolean;
	fields: Field[];
};

export type Site = { handle: string; name: string; language: string };

export type Section = {
	handle: string;
	name: string;
	kind: 'single' | 'channel' | 'structure';
	types: string[];
	uri: string | null;
	/** the uri of an entry type that differs from `uri` */
	type_uris?: Record<string, string>;
};

/** A named image size, only these exist. */
export type Transform = {
	handle: string;
	width: number | null;
	height: number | null;
	crop: boolean;
};

export type NavGroup = { name: string; sections: string[] };

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
	/** SEO defaults of sections and entry types, null without SEO */
	seo: Seo | null;
	me: Me;
	/** the models of the AI chat, empty if it is off */
	aiModels: { id: string; name: string }[];
	/** markdown files of the code, read only, as html */
	docs: { handle: string; name: string; html: string }[];
};

export type Block = { id?: string; type: string; fields: Values };

export type Values = Record<string, any>;

export function getSchema(): Promise<Schema> {
	return request('GET', '/schema');
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

/** The logged in user, `role: null` is an admin. */
export type Me = { id: number; email: string; role: string | null };

/** SEO defaults of a section or an entry type, from the code. */
export type SeoCodeDefaults = {
	titlePattern: string | null;
	description: string | null;
	robots: string | null;
	descriptionFrom: string | null;
	imageFrom: string | null;
	sitemap: boolean;
};

export type Seo = {
	sections: Record<string, SeoCodeDefaults>;
	types: Record<string, SeoCodeDefaults>;
	sitemap_page_size: number;
};
