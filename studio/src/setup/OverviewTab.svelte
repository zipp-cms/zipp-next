<script lang="ts">
	import type { Component, Schema } from '../core/schema';
	import { componentOf } from '../core/schema';
	import { isAdmin } from '../core/access';
	import { plural } from '../core/format';
	import SchemaIcon from '../core/SchemaIcon.svelte';
	import {
		componentHref,
		fieldUsage,
		holders,
		sectionsByGroup,
		usedIn,
		visibleEntryTypes,
		visibleSections,
	} from './setup';

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
	const blocks = $derived(
		schema.components.filter(c => !c.entry && !c.part_of),
	);
	// blocks no field holds, code that can go
	const unused = $derived(blocks.filter(b => !usedIn(schema, b).length));
	const sections = $derived(visibleSections(schema));
	const stats = $derived([
		{ n: sections.length, label: 'Sections', href: 'sections' },
		{
			n: visibleEntryTypes(schema).length,
			label: 'Entry types',
			href: 'types',
		},
		{ n: blocks.length, label: 'Blocks', href: 'blocks' },
		{ n: fieldUsage(schema).length, label: 'Fields', href: 'fields' },
		{
			n: Object.values(counts).reduce((a, b) => a + b, 0),
			label: 'Entries',
			href: 'sections',
		},
		{ n: schema.sites.length, label: 'Sites', href: 'sites' },
	]);

	// a plugin's handles as links of Setup
	function pluginParts(p: Schema['pluginsAdded'][number]) {
		const comps = p.components
			.map(h => componentOf(schema, h))
			.filter(c => c !== undefined);
		return [
			{
				name: 'Sections',
				links: sections
					.filter(s => p.sections.includes(s.handle))
					.map(s => ({
						text: s.name,
						href: `/admin/setup/sections/${s.handle}`,
					})),
			},
			{
				name: 'Types and blocks',
				links: comps.map(c => ({
					text: c.name,
					href: componentHref(c),
				})),
			},
			{
				name: 'Fields',
				links: p.fields.map(f => {
					const [c, field] = f.split('.');
					const comp = componentOf(schema, c);
					const href = comp
						? `${componentHref(comp)}#field-${field}`
						: undefined;
					return { text: f, href };
				}),
			},
			{
				name: 'Field types',
				links: p.fieldTypes.map(t => ({ text: t, href: undefined })),
			},
			{
				name: 'Transforms',
				links: p.transforms.map(t => ({
					text: t,
					href: '/admin/setup/sites',
				})),
			},
		].filter(x => x.links.length);
	}
</script>

{#snippet parts(c: Component, seen: string[])}
	{@const held = holders(schema, c)}
	{#if held.length}
		<ul class="plain tree">
			{#each held as h (h.field.handle)}
				<li>
					<a
						class="field"
						href="{componentHref(c)}#field-{h.field.handle}"
					>
						<code>{h.field.handle}</code>
					</a>
					{#each h.parts as p, i (p.handle)}
						{i ? ', ' : ''}
						<a href={componentHref(p)}>{p.name}</a>
					{/each}
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
							<a href="/admin/setup/sections/{s.handle}">
								<strong>{s.name}</strong>
							</a>
							<span class="muted">
								{s.kind}{s.handle in counts
									? `, ${plural(counts[s.handle], 'entry', 'entries')}`
									: ''}
							</span>
						</div>
						<ul class="plain types">
							{#each s.types as t (t)}
								{@const c = componentOf(schema, t)}
								<li>
									{#if c}
										<a class="type" href={componentHref(c)}>
											<SchemaIcon
												icon={c.icon}
												size={14}
											/>
											{c.name}
										</a>
										<span class="muted">
											{plural(
												c.fields.length,
												'field',
												'fields',
											)}
										</span>
										{@render parts(c, [c.handle])}
									{:else}
										{t}
									{/if}
								</li>
							{/each}
						</ul>
					</div>
				{/each}
			</div>
		{:else}
			<div class="card">
				<p class="empty">No sections you may see.</p>
			</div>
		{/each}
		{#if unused.length && isAdmin(schema)}
			<div class="card">
				<h2>Blocks used nowhere</h2>
				<p>
					{#each unused as b, i (b.handle)}
						{i ? ', ' : ''}
						<a href={componentHref(b)}>{b.name}</a>
					{/each}
				</p>
			</div>
		{/if}
	</div>

	<aside class="cards">
		{#if schema.pluginsAdded.length}
			<div class="card">
				<h2>Plugins</h2>
				{#each schema.pluginsAdded as p (p.plugin)}
					{@const added = pluginParts(p)}
					<div class="plugin">
						<code>{p.plugin}</code>
						{#if added.length}
							<dl>
								{#each added as a (a.name)}
									<dt class="muted">{a.name}</dt>
									<dd>
										{#each a.links as l, i (l.text)}
											{i ? ', ' : ''}
											{#if l.href}
												<a href={l.href}>{l.text}</a>
											{:else}
												{l.text}
											{/if}
										{/each}
									</dd>
								{/each}
							</dl>
						{:else}
							<p class="muted">Nothing in the content model.</p>
						{/if}
					</div>
				{/each}
			</div>
		{/if}

		<div class="card">
			<h2>More</h2>
			<ul class="plain links">
				{#if isAdmin(schema)}
					<li>
						<a href="/admin/graphql">GraphQL</a>
						<span class="muted">tokens of frontends, GraphiQL</span>
					</li>
				{/if}
				<li>
					<a href="/admin/setup/sites#roles">Roles</a>
					<span class="muted">who may do what</span>
				</li>
				<li>
					<a href="/admin/setup/sites">Sites and transforms</a>
				</li>
				{#if isAdmin(schema)}
					<li>
						<a href="/admin/setup/sites#mail">Mail</a>
						<span class="muted">how mails are sent</span>
					</li>
					<li>
						<a href="/admin/setup/requests">Requests</a>
						<span class="muted">
							{openRequests
								? `${openRequests} open`
								: 'changes editors asked for'}
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
