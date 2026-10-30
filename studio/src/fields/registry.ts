import type { Component } from 'svelte';
import type { Field, Refs, Schema, Values } from '../api';
import AssetField from './AssetField.svelte';
import BlocksField from './BlocksField.svelte';
import CheckboxesField from './CheckboxesField.svelte';
import CustomField from './CustomField.svelte';
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
export type FieldProps = {
	schema: Schema;
	field: Field;
	values: Values;
	errors: Record<string, string>;
	refs: Refs;
	/** the path of the value, like `body.0.text`, for errors and ids */
	path: string;
	changed: Set<string>;
};

/** The component of each kind of field. */
export const FIELDS: Record<Field['kind'], Component<any>> = {
	text: SimpleField,
	number: SimpleField,
	boolean: SimpleField,
	relation: RelationField,
	assets: AssetField,
	link: LinkField,
	group: GroupField,
	markdown: MarkdownField,
	richText: RichTextField,
	texts: TextsField,
	dropdown: DropdownField,
	checkboxes: CheckboxesField,
	table: TableField,
	navigation: NavigationField,
	blocks: BlocksField,
	custom: CustomField,
};

/** The kinds a plugin's module may mount, they need no entry around them. */
export const HOSTED: Field['kind'][] = [
	'text',
	'assets',
	'dropdown',
	'checkboxes',
	'table',
	'markdown',
];
