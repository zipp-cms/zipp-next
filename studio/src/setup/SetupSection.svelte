<script lang="ts">
	import { gqlType, uriOf, visibleRoles } from './setup';
	import {
		componentPath,
		entryPath,
		sectionPath,
		sitesPath,
	} from '../core/paths';
	import { depths } from '../core/tree';
	import EntryBadges from '../entries/EntryBadges.svelte';
	import { entryUrl, navGroups, type Schema } from '../core/schema';
	import type { EntryList } from '../entries/api';
	import { accessOf, accessText, isAdmin } from '../core/access';
	import PluginCards from './PluginCards.svelte';
	import Table from 'zipp-ui/Table.svelte';
	import { componentOf, sectionOf } from '../core/schema';
	import Sentence from '../i18n/Sentence.svelte';
	import { t } from '../i18n/index.svelte';

	/**
	 * A section in the setup, read only: its settings in the main column
	 * (entry types, urls per site, who may do what, cards of plugins), its entries as a
	 * short list on the side, linking to the editor.
	 */
	let {
		schema,
		handle,
		list,
	}: { schema: Schema; handle: string; list: EntryList } = $props();

	const section = $derived(sectionOf(schema, handle));
	const types = $derived(
		(section?.types ?? []).map(
			type =>
				componentOf(schema, type) ?? {
					handle: type,
					name: type,
					entry: true,
					fields: [],
				},
		),
	);
	const group = $derived(
		navGroups(schema).find(g => g.sections.includes(handle))?.name,
	);
	// an admin sees every role, a role only itself
	const roles = $derived(visibleRoles(schema));
	// who may what here, admins first
	const access = $derived([
		...(isAdmin(schema)
			? [
					{
						name: t('setup.admins'),
						handle: null,
						may: t('setup.everything'),
					},
				]
			: []),
		...roles.map(r => ({
			name: r.name,
			handle: r.handle,
			may: accessText(r.sections[handle], t('setup.nothingHidden')),
		})),
	]);

	// the first entries in the primary site, see Setup's loadProps
	const { entries, published, drafts, total } = $derived(list);
	const entryDepths = $derived(depths(entries));

	const KINDS = {
		single: 'setup.single',
		channel: 'setup.channel',
		structure: 'setup.structure',
	} as const;
</script>

{#if section}
	<div class="with-aside">
		<div class="cards">
			<div class="card">
				<h2>{t('setup.general')}</h2>
				<dl>
					<dt>{t('common.name')}</dt>
					<dd>{section.name}</dd>
					<dt>{t('setup.handle')}</dt>
					<dd><code>{section.handle}</code></dd>
					<dt>{t('setup.kind')}</dt>
					<dd>{t(KINDS[section.kind])}</dd>
					{#if section.kind === 'structure'}
						<dt>{t('setup.maxLevel')}</dt>
						<dd>
							{section.max_level === 1
								? t('setup.maxLevelOne')
								: (section.max_level ??
									t('setup.maxLevelNone'))}
						</dd>
					{/if}
					{#if section.uri_if}
						<dt>{t('setup.pages')}</dt>
						<dd>
							<Sentence
								key="setup.pagesIf"
								params={{ field: section.uri_if }}
							/>
						</dd>
					{/if}
					<dt>{t('setup.navigation')}</dt>
					<dd>{group ?? '–'}</dd>
					<dt>GraphQL</dt>
					<dd>
						<code>
							{section.kind === 'single' && section.uri === null
								? `globalSet(handle: "${handle}")`
								: `entries(section: "${handle}")`}
						</code>
					</dd>
					<dt>{t('setup.youMay')}</dt>
					<dd>
						{accessText(
							accessOf(schema, handle),
							t('setup.nothing'),
						)}
					</dd>
				</dl>
			</div>

			<div class="card">
				<h2>{t('setup.entryTypes')}</h2>
				<Table
					rows={types}
					key={ty => ty.handle}
					columns={[
						t('common.name'),
						t('setup.handle'),
						t('setup.fields'),
						'URI',
						'GraphQL',
					]}
					href={componentPath}
				>
					{#snippet row(ty)}
						{@const uri = uriOf(section, ty.handle)}
						<td>
							<a href={componentPath(ty)}>
								{ty.name}
							</a>
						</td>
						<td><code>{ty.handle}</code></td>
						<td>{ty.fields.length}</td>
						<td>
							<code>
								{uri === null
									? t('setup.noPages')
									: uri || t('setup.home')}
							</code>
						</td>
						<td><code>{gqlType(ty)}</code></td>
					{/snippet}
				</Table>
			</div>

			<div class="card">
				<h2>{t('setup.sites')}</h2>
				<Table
					rows={schema.sites}
					key={s => s.handle}
					columns={[
						t('setup.site'),
						t('setup.language'),
						t('setup.urls'),
						t('setup.status'),
					]}
				>
					{#snippet row(s)}
						<td>{s.name}</td>
						<td>{s.language}</td>
						<td>
							{#each types as ty (ty.handle)}
								{@const uri = uriOf(
									section,
									ty.handle,
									s.handle,
								)}
								<div>
									{#if types.length > 1}
										<span class="muted">
											{ty.name}:
										</span>
									{/if}
									<code>
										{uri === null
											? t('setup.noPages')
											: (entryUrl(
													schema,
													s.handle,
													uri,
												) ?? '/' + uri)}
									</code>
								</div>
							{/each}
						</td>
						<td>
							{schema.offline.includes(s.handle)
								? t('setup.offline')
								: t('setup.online')}
						</td>
					{/snippet}
				</Table>
			</div>

			<div class="card">
				<h2>{t('setup.access')}</h2>
				<Table
					rows={access}
					key={a => a.name}
					columns={[t('setup.role'), t('setup.may')]}
				>
					{#snippet row(a)}
						<td>
							{a.name}
							{#if a.handle}<code>{a.handle}</code>{/if}
						</td>
						<td>{a.may}</td>
					{/snippet}
				</Table>
				<p class="muted">
					<a href={sitesPath('roles')}>{t('setup.allRoles')}</a>
				</p>
			</div>

			<PluginCards {schema} {section} />
		</div>

		<aside class="card">
			<h2>
				{t('setup.entries')}
				<span class="muted">{total}</span>
			</h2>
			<ul class="plain">
				{#each entries as e, i (e.id)}
					<li style:padding-left="{entryDepths[i] * 0.75}rem">
						<a href={entryPath(handle, e.id)}>{e.title}</a>
						<EntryBadges
							{schema}
							published={published[e.id] ?? {}}
							draft={drafts.includes(e.id)}
						/>
					</li>
				{/each}
			</ul>
			{#if total > entries.length}
				<p class="muted">
					{t('setup.theFirst', { n: entries.length })}
				</p>
			{/if}
			<p class="muted">
				<a href={sectionPath(handle)}>{t('setup.openInContent')}</a>
			</p>
		</aside>
	</div>
{:else}
	<p class="error">{t('setup.noSection', { handle })}</p>
{/if}

<style lang="scss">
	dl {
		display: grid;
		grid-template-columns: 8.75rem 1fr;
		gap: 0.5rem 1rem;
		margin: 0;
	}

	dt {
		color: var(--muted);
	}

	dd {
		margin: 0;
	}

	li {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.3125rem 0;
		border-top: 1px solid var(--line);
	}

	li a {
		flex: 1;
		min-width: 0;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		color: inherit;
	}
</style>
