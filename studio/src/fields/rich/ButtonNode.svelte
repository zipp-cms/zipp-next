<script lang="ts">
	import type { Link } from '../../core/schema';
	import type { NodeProps } from './extensions.svelte';
	import LinkInput from '../link/LinkInput.svelte';
	import NodeFrame from './NodeFrame.svelte';
	import DragHandle from 'zipp-ui/DragHandle.svelte';
	import { BUTTON_LINKS } from '../link/link';
	import { t } from '../../i18n/index.svelte';

	/**
	 * A button in a rich text: a link with a label, like a link field;
	 * buttons next to each other are one row on the website.
	 */
	let {
		attrs,
		schema,
		refs,
		sections,
		editable,
		update,
		remove,
		insertLine,
		sortable,
		index,
	}: NodeProps<{ link: Link }> = $props();
</script>

<NodeFrame {editable} oninsert={insertLine}>
	<fieldset disabled={!editable}>
		<LinkInput
			{schema}
			{refs}
			link={attrs.link}
			to={BUTTON_LINKS}
			labels
			{sections}
			onchange={link => update({ link })}
			onremove={remove}
			removeLabel={t('fields.deleteButton')}
		>
			{#snippet handle()}
				{#if editable}
					<DragHandle
						{sortable}
						{index}
						label={t('fields.moveButton')}
					/>
				{/if}
			{/snippet}
		</LinkInput>
	</fieldset>
</NodeFrame>
