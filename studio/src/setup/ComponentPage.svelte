<script lang="ts">
	import type { Component, Field, Schema } from '../core/schema';
	import { describeField } from '../fields/fieldKind';
	import { fieldOptions } from '../fields/fieldOptions';
	import { gqlType, usedIn } from './setup';
	import Table from 'zipp-ui/Table.svelte';

	/**
	 * An entry type or block: its fields by tab, a click on one shows all
	 * its options; where it is used.
	 */
	let { schema, component: c }: { schema: Schema; component: Component } =
		$props();

	// the tabs of the editor, else one card of all fields
	const groups = $derived.by(() => {
		if (!c.tabs?.length) return [{ name: 'Fields', fields: c.fields }];
		const byTab = c.tabs.map(t => ({
			name: t.name,
			fields: c.fields.filter(f => t.fields.includes(f.handle)),
		}));
		const rest = c.fields.filter(
			f => !c.tabs!.some(t => t.fields.includes(f.handle)),
		);
		return rest.length
			? [...byTab, { name: 'Other', fields: rest }]
			: byTab;
	});

	// the open fields, the one of the url's anchor too (#field-intro)
	let open = $state<string[]>(
		location.hash.startsWith('#field-') ? [location.hash.slice(7)] : [],
	);
	const toggle = (f: Field) =>
		(open = open.includes(f.handle)
			? open.filter(h => h !== f.handle)
			: [...open, f.handle]);
</script>

<div class="with-aside">
	<div class="cards">
		{#each groups as g, i (g.name)}
			<div class="card">
				<h2>
					{g.name}
					{#if i === 0}<code class="gql">{gqlType(c)}</code>{/if}
				</h2>
				{#if g.fields.length}
					<Table
						rows={g.fields}
						key={f => f.handle}
						id={f => `field-${f.handle}`}
						columns={['Label', 'Handle', 'Kind', 'Sites', '']}
						widths={['20%', '18%', '36%', '12%', '14%']}
						isOpen={f => open.includes(f.handle)}
						ontoggle={toggle}
					>
						{#snippet row(f)}
							<td>{f.label}</td>
							<td><code>{f.handle}</code></td>
							<td>{describeField(f)}</td>
							<td>
								{c.entry
									? f.translatable
										? 'per site'
										: 'all sites'
									: ''}
							</td>
							<td class="muted">
								{f.required ? 'required' : ''}
								{f.indexed ? 'indexed' : ''}
							</td>
						{/snippet}
						{#snippet detail(f)}
							<dl class="options">
								{#each fieldOptions(schema, f, c.entry) as o (o.name)}
									<dt>{o.name}</dt>
									<dd>
										{#if o.links?.length}
											{#each o.links as l, k (l.href)}
												{k ? ', ' : ''}
												<a href={l.href}>{l.text}</a>
											{/each}
										{:else if o.name === 'GraphQL'}
											<code>{o.value}</code>
										{:else}
											{o.value}
										{/if}
									</dd>
								{/each}
							</dl>
						{/snippet}
					</Table>
				{:else}
					<p class="muted">No fields here.</p>
				{/if}
			</div>
		{:else}
			<div class="card">
				<p class="muted">No fields, only a title.</p>
			</div>
		{/each}
	</div>
	<aside class="card">
		<h2>Used in</h2>
		<ul class="plain">
			{#each usedIn(schema, c) as u (u.text)}
				<li><a href={u.href}>{u.text}</a></li>
			{:else}
				<li class="muted">nothing</li>
			{/each}
		</ul>
	</aside>
</div>

<style lang="scss">
	h2 {
		display: flex;
		align-items: baseline;
		gap: 0.625rem;
	}

	.gql {
		margin-left: auto;
	}

	// all options of an open field, a name and its value a line
	.options {
		display: grid;
		grid-template-columns: max-content 1fr;
		gap: 0.25rem 1.5rem;
		margin: 0;
		padding: 0.5rem 0.75rem;
		border-left: 2px solid var(--accent);
		font-size: var(--fs-md);
	}

	dt {
		color: var(--muted);
	}

	dd {
		margin: 0;
	}

	li {
		padding: 0.3125rem 0;
		border-top: 1px solid var(--line);
	}
</style>
