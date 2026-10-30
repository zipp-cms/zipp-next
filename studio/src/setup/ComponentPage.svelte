<script lang="ts">
	import type { Component, Schema } from '../core/schema';
	import { describeField } from '../fields/fieldKind';
	import { gqlType, usedIn } from './setup';
	import TypePreview from './TypePreview.svelte';

	/**
	 * An entry type or block: its editor as editors see it, its fields
	 * with handles and kinds, where it is used.
	 */
	let { schema, component: c }: { schema: Schema; component: Component } =
		$props();

	const tabOf = (field: string) =>
		c.tabs?.find(t => t.fields.includes(field))?.name;
</script>

<div class="with-aside">
	<div class="cards">
		<div class="card">
			<h2>
				Editor
				<span class="note">how editors see it, read only</span>
			</h2>
			{#key c.handle}
				<TypePreview {schema} component={c} />
			{/key}
		</div>
		<div class="card">
			<h2>
				Fields <code class="gql">{gqlType(c)}</code>
			</h2>
			{#if c.fields.length}
				<table class="table">
					<thead>
						<tr>
							<th>Label</th>
							<th>Handle</th>
							{#if c.tabs?.length}<th>Tab</th>{/if}
							<th>Kind</th>
							<th>Sites</th>
							<th></th>
						</tr>
					</thead>
					<tbody>
						{#each c.fields as f (f.handle)}
							<tr id="field-{f.handle}">
								<td>{f.label}</td>
								<td><code>{f.handle}</code></td>
								{#if c.tabs?.length}<td>
										{tabOf(f.handle)}
									</td>{/if}
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
							</tr>
						{/each}
					</tbody>
				</table>
			{:else}
				<p class="muted">No fields, only a title.</p>
			{/if}
		</div>
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

<style>
	tr:global(.target) td {
		background: var(--hover);
		transition: background 0.3s;
	}

	h2 {
		display: flex;
		align-items: baseline;
		gap: 0.625rem;
	}

	.gql {
		margin-left: auto;
	}

	li {
		padding: 0.3125rem 0;
		border-top: 1px solid var(--line);
	}
</style>
