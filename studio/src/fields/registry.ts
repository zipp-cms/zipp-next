import type { Component } from 'svelte';
import type { Field, FieldOf, Schema, Values } from '../core/schema';
import type { Refs } from '../entries/api';
import AssetField from './AssetField.svelte';
import BlocksField from './BlocksField.svelte';
import CheckboxesField from './CheckboxesField.svelte';
import CustomField from './CustomField.svelte';
import DateField from './DateField.svelte';
import DropdownField from './DropdownField.svelte';
import GroupField from './GroupField.svelte';
import LinkField from './LinkField.svelte';
import MarkdownField from './MarkdownField.svelte';
import NavigationField from './NavigationField.svelte';
import RelationField from './RelationField.svelte';
import RichTextField from './RichTextField.svelte';
import SimpleField from './SimpleField.svelte';
import TableField from './TableField.svelte';
import TextsField from './TextsField.svelte';

/**
 * What every field component gets; each takes what it needs. `values`
 * is the object the field's value is in, `values[field.handle]`.
 */
export type FieldProps<F extends Field = Field> = {
	schema: Schema;
	field: F;
	values: Values;
	errors: Record<string, string>;
	refs: Refs;
	/** the path of the value, like `body.0.text`, for errors and ids */
	path: string;
	changed: Set<string>;
};

/** The component of each kind of field, it gets the field of its kind. */
const FIELDS: { [K in Field['kind']]: Component<FieldProps<FieldOf<K>>> } = {
	text: SimpleField,
	number: SimpleField,
	boolean: SimpleField,
	relation: RelationField,
	assets: AssetField,
	link: LinkField,
	group: GroupField,
	markdown: MarkdownField,
	date: DateField,
	richText: RichTextField,
	texts: TextsField,
	dropdown: DropdownField,
	checkboxes: CheckboxesField,
	table: TableField,
	navigation: NavigationField,
	blocks: BlocksField,
	custom: CustomField,
};

/** The component of a field. */
export function fieldComponent(field: Field): Component<FieldProps> {
	// the kind of the field picks the component that takes it
	return FIELDS[field.kind] as Component<FieldProps<any>>;
}

/** What a field in a plugin's module gets, there is no entry around it. */
export type HostedProps<F extends Field = Field> = Pick<
	FieldProps<F>,
	'field' | 'values' | 'path'
>;

/** The kinds a plugin's module may mount, they need nothing else. */
const HOSTED: {
	[K in Field['kind']]?: Component<HostedProps<FieldOf<K>>>;
} = {
	text: SimpleField,
	assets: AssetField,
	dropdown: DropdownField,
	checkboxes: CheckboxesField,
	table: TableField,
	markdown: MarkdownField,
	date: DateField,
};

/** The component of a field in a plugin's module, null if it can't be. */
export function hostedComponent(field: Field): Component<HostedProps> | null {
	return (HOSTED[field.kind] as Component<HostedProps<any>>) ?? null;
}
