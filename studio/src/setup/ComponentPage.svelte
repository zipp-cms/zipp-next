<script lang="ts">
	import type { Component, Field, Schema } from '../core/schema';
	import { describeField } from './fieldKind';
	import { describeCondition, fieldOptions } from './fieldOptions';
	import { gqlType, usedIn } from './setup';
	import Table from 'zipp-ui/Table.svelte';
	import { t } from '../i18n/index.svelte';

	/**
	 * An entry type or block: its fields by tab, a click on one shows all
	 * its options; where it is used.
	 */
	let { schema, component: c }: { schema: Schema; component: Component } =
		$props();

	// the tabs of the editor, else one card of all fields
	const groups = $derived.by(() => {
		if (!c.tabs?.length)
			return [
				{
					name: t('setup.fields'),
					fields: c.fields,
					when: undefined,
					card: c.card,
				},
			];
		const byTab = c.tabs.map(tab => ({
			name: tab.name,
			fields: c.fields.filter(f => tab.fields.includes(f.handle)),
			when: tab.when,
			card: c.card || tab.card,
		}));
		const rest = c.fields.filter(
			f => !c.tabs!.some(tab => tab.fields.includes(f.handle)),
		);
		return rest.length
			? [
					...byTab,
					{
						name: t('setup.other'),
						fields: rest,
						when: undefined,
						card: false,
					},
				]
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
					{#if g.when}<span class="muted when">
							{t('setup.shownWhen', {
								condition: describeCondition(c, g.when),
							})}
						</span>{/if}
					{#if g.card}<span class="muted when">
							{t('setup.inACard')}
						</span>{/if}
					{#if i === 0}<code class="gql">{gqlType(c)}</code>{/if}
				</h2>
				{#if g.fields.length}
					<Table
						rows={g.fields}
						key={f => f.handle}
						id={f => `field-${f.handle}`}
						columns={[
							t('setup.label'),
							t('setup.handle'),
							t('setup.kind'),
							t('setup.sites'),
							'',
						]}
						widths={['20%', '18%', '36%', '12%', '14%']}
						isOpen={f => open.includes(f.handle)}
						ontoggle={toggle}
					>
						{#snippet row(f)}
							<td>{f.label}</td>
							<td><code>{f.handle}</code></td>
							<td>{describeField(f)}</td>
							<td>
								{c.entry && f.kind !== 'note'
									? f.translatable
										? t('setup.perSite')
										: t('setup.allSites')
									: ''}
							</td>
							<td class="muted">
								{f.width ? `${f.width} %` : ''}
								{f.required ? t('setup.required') : ''}
								{f.when ? t('setup.conditional') : ''}
								{f.indexed ? t('setup.indexed') : ''}
								{f.handle === c.title_field
									? t('setup.title')
									: ''}
							</td>
						{/snippet}
						{#snippet detail(f)}
							<dl class="options">
								{#each fieldOptions(schema, f, c) as o (o.name)}
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
					<p class="muted">{t('setup.noFieldsHere')}</p>
				{/if}
			</div>
		{:else}
			<div class="card">
				<p class="muted">{t('setup.onlyTitle')}</p>
			</div>
		{/each}
	</div>
	<aside class="card">
		<h2>{t('setup.usedIn')}</h2>
		<ul class="plain">
			{#each usedIn(schema, c) as u (u.text)}
				<li><a href={u.href}>{u.text}</a></li>
			{:else}
				<li class="muted">{t('setup.nothing')}</li>
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

	.when {
		font-size: var(--fs-md);
		font-weight: 400;
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
