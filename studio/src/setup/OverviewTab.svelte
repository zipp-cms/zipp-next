<script lang="ts">
	import Links from './Links.svelte';
	import type { Component, Schema } from '../core/schema';
	import { componentOf } from '../core/schema';
	import { isAdmin } from '../core/access';
	import SchemaIcon from '../core/SchemaIcon.svelte';
	import {
		blocksOf,
		holders,
		overviewStats,
		pluginParts,
		sectionsByGroup,
		usedIn,
		componentLink,
	} from './setup';
	import { componentPath, fieldPath, setupPath } from '../core/paths';
	import { t } from '../i18n/index.svelte';

	/**
	 * What is where: the nav groups with their sections, each with its
	 * entries, entry types and the blocks and groups of their fields, a
	 * level deeper where blocks hold blocks. On the side the counts, what
	 * the plugins added and the links to GraphQL, roles and mails.
	 */
	let {
		schema,
		counts,
		openRequests,
	}: {
		schema: Schema;
		/** entries per section, see Setup's `loadProps` */
		counts: Record<string, number>;
		/** for admins, the requests not done yet */
		openRequests?: number;
	} = $props();

	const groups = $derived(sectionsByGroup(schema));
	const stats = $derived(overviewStats(schema, counts));
	// blocks no field holds, code that can go
	const unused = $derived(
		blocksOf(schema).filter(b => !usedIn(schema, b).length),
	);
</script>

{#snippet parts(c: Component, seen: string[])}
	{@const held = holders(schema, c)}
	{#if held.length}
		<ul class="plain tree">
			{#each held as h (h.field.handle)}
				<li>
					<a class="field" href={fieldPath(c, h.field.handle)}>
						<code>{h.field.handle}</code>
					</a>
					<Links links={h.parts.map(componentLink)} />
					{#each h.parts.filter(p => !seen.includes(p.handle)) as p (p.handle)}
						{#if holders(schema, p).length}
							<div class="deeper">
								<span class="muted">{p.name}:</span>
								{@render parts(p, [...seen, p.handle])}
							</div>
						{/if}
					{/each}
				</li>
			{/each}
		</ul>
	{/if}
{/snippet}

<div class="card stats">
	{#each stats as s (s.label)}
		<a href="/admin/setup/{s.href}">
			<strong>{s.n}</strong>
			<span class="muted">{s.label}</span>
		</a>
	{/each}
</div>

<div class="with-aside">
	<div class="cards">
		{#each groups as g (g.name)}
			<div class="card">
				<h2>{g.name}</h2>
				{#each g.sections as s (s.handle)}
					<div class="section">
						<div class="head">
							<a href={setupPath('sections', s.handle)}>
								<strong>{s.name}</strong>
							</a>
							<span class="muted">
								{s.kind}{s.handle in counts
									? `, ${t('common.entries', { n: counts[s.handle] })}`
									: ''}
							</span>
						</div>
						<ul class="plain types">
							{#each s.types as type (type)}
								{@const c = componentOf(schema, type)}
								<li>
									{#if c}
										<a class="type" href={componentPath(c)}>
											<SchemaIcon
												icon={c.icon}
												size={14}
											/>
											{c.name}
										</a>
										<span class="muted">
											{t('setup.nFields', {
												n: c.fields.length,
											})}
										</span>
										{@render parts(c, [c.handle])}
									{:else}
										{type}
									{/if}
								</li>
							{/each}
						</ul>
					</div>
				{/each}
			</div>
		{:else}
			<div class="card">
				<p class="empty">{t('setup.noSectionsVisible')}</p>
			</div>
		{/each}
		{#if unused.length && isAdmin(schema)}
			<div class="card">
				<h2>{t('setup.unusedBlocks')}</h2>
				<p>
					<Links links={unused.map(componentLink)} />
				</p>
			</div>
		{/if}
	</div>

	<aside class="cards">
		{#if schema.pluginsAdded.length}
			<div class="card">
				<h2>{t('setup.plugins')}</h2>
				{#each schema.pluginsAdded as p (p.plugin)}
					{@const added = pluginParts(schema, p)}
					<div class="plugin">
						<code>{p.plugin}</code>
						{#if added.length}
							<dl>
								{#each added as a (a.name)}
									<dt class="muted">{a.name}</dt>
									<dd>
										<Links links={a.links} />
									</dd>
								{/each}
							</dl>
						{:else}
							<p class="muted">{t('setup.nothingInModel')}</p>
						{/if}
					</div>
				{/each}
			</div>
		{/if}

		<div class="card">
			<h2>{t('setup.more')}</h2>
			<ul class="plain links">
				{#if isAdmin(schema)}
					<li>
						<a href="/admin/graphql">GraphQL</a>
						<span class="muted">{t('setup.graphqlNote')}</span>
					</li>
				{/if}
				<li>
					<a href="/admin/setup/sites#roles">{t('setup.roles')}</a>
					<span class="muted">{t('setup.rolesNote')}</span>
				</li>
				<li>
					<a href="/admin/setup/sites">
						{t('setup.sitesAndTransforms')}
					</a>
				</li>
				{#if isAdmin(schema)}
					<li>
						<a href="/admin/setup/sites#mail">{t('setup.mail')}</a>
						<span class="muted">{t('setup.mailNote')}</span>
					</li>
					<li>
						<a href="/admin/setup/requests">
							{t('shell.setupRequests')}
						</a>
						<span class="muted">
							{openRequests
								? t('setup.nOpen', { n: openRequests })
								: t('setup.requestsNote')}
						</span>
					</li>
				{/if}
			</ul>
		</div>
	</aside>
</div>

<style lang="scss">
	.stats {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem 2rem;
		margin-bottom: 1.25rem;

		a {
			display: flex;
			flex-direction: column;
			color: inherit;
			text-decoration: none;
		}

		strong {
			font-size: var(--fs-xl);
		}
	}

	.section {
		padding: 0.5rem 0;
		border-top: 1px solid var(--line);
	}

	.head {
		display: flex;
		align-items: baseline;
		gap: 0.5rem;
	}

	.types > li {
		padding: 0.25rem 0 0 1rem;
	}

	.type {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
	}

	// the fields with blocks and groups, under their type
	.tree {
		margin-left: 0.4375rem;
		padding-left: 0.875rem;
		border-left: 1px solid var(--line);
		font-size: var(--fs-md);

		li {
			padding: 0.125rem 0;
		}
	}

	.field {
		margin-right: 0.25rem;
	}

	.deeper {
		margin-top: 0.125rem;
	}

	.plugin + .plugin {
		margin-top: 0.75rem;
		padding-top: 0.75rem;
		border-top: 1px solid var(--line);
	}

	dl {
		display: grid;
		grid-template-columns: max-content 1fr;
		gap: 0.25rem 0.75rem;
		margin: 0.375rem 0 0;
		font-size: var(--fs-md);
	}

	dd {
		margin: 0;
		overflow-wrap: anywhere;
	}

	p {
		margin: 0;
	}

	.links li {
		padding: 0.3125rem 0;
		border-top: 1px solid var(--line);
	}
</style>
