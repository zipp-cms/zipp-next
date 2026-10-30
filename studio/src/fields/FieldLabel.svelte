<script lang="ts">
	import type { Component, Field, Schema } from '../core/schema';
	import { isAdmin } from '../core/access';
	import Icon from 'zipp-ui/Icon.svelte';

	/**
	 * The label of a field: a dot when the draft changed it, required,
	 * translated per site, and for admins its handle linking to Setup.
	 */
	let {
		schema,
		component,
		field,
		path,
		changed,
	}: {
		schema: Schema;
		component: Component;
		field: Field;
		path: string;
		changed: boolean;
	} = $props();

	// entry fields with their own value in each site get an icon, like
	// Craft's; the others are the same in all sites
	const translated = $derived(
		component.entry && field.translatable && schema.sites.length > 1,
	);
</script>

<label for={path}>
	<span class="name">
		{#if changed}<span
				class="changed-dot"
				title="Changed"
			></span>{/if}{field.label}{#if field.required}<span class="req">
				*
			</span>{/if}
	</span>
	{#if translated}
		<span class="translated" title="Translated: each site has its own">
			<Icon name="translate" size={13} />
		</span>
	{/if}
	<!-- for admins the handle, like Craft's "show field handles" -->
	{#if isAdmin(schema)}
		<a
			class="handle-code"
			href="/admin/setup/{component.entry
				? 'types'
				: 'blocks'}/{component.handle}#field-{field.handle}"
			title="The field in Setup"
		>
			{field.handle}
		</a>
	{/if}
</label>

<style lang="scss">
	label {
		display: flex;
		align-items: baseline;
		gap: 0.375rem;
		flex-wrap: wrap;
		font-size: var(--fs-md);
		font-weight: 600;
		color: var(--text);
	}

	.changed-dot {
		margin-right: 0.25rem;
	}

	.translated {
		display: inline-flex;
		align-self: center;
		color: var(--subtle);
	}

	.req {
		margin-left: 2px;
		color: var(--danger);
	}

	/* the handle for developers, links to the field in Setup */
	.handle-code {
		text-decoration: none;

		@include hover {
			&:hover {
				color: var(--accent);
			}
		}
	}
</style>
