<script lang="ts">
	import PageTop from 'zipp-ui/PageTop.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import Switch from 'zipp-ui/Switch.svelte';
	import CheckGroup from 'zipp-ui/CheckGroup.svelte';
	import DatePicker from 'zipp-ui/DatePicker.svelte';
	import type { Schema } from '../core/schema';
	import { go } from '../core/router.svelte';
	import { TokenForm } from './tokenForm.svelte';

	/**
	 * A new token for a frontend: a name, the sections and sites it reads,
	 * whether preview tokens show drafts, until when. The token shows once
	 * after creating, only its hash is kept.
	 */
	let { schema }: { schema: Schema } = $props();

	const form = new TokenForm();
	const today = new Date().toISOString().slice(0, 10);

	function save(e: SubmitEvent) {
		e.preventDefault();
		form.save();
	}
</script>

<PageTop
	crumbs={[
		{ label: 'GraphQL', href: '/admin/graphql' },
		{ label: 'New token' },
	]}
>
	{#snippet actions()}
		{#if form.created}
			<button class="primary" onclick={() => go('/admin/graphql')}>
				Done
			</button>
		{:else}
			<button class="primary" form="token" disabled={form.busy}>
				Create
			</button>
		{/if}
	{/snippet}
</PageTop>

{#if form.error && !Object.keys(form.fields).length}
	<p class="error">{form.error}</p>
{/if}

{#if form.created}
	<div class="card">
		<h2>{form.name}</h2>
		<p>
			Copy the token now, it doesn't show again. A frontend sends it as
			<code>Authorization: Bearer …</code>
			; crelte reads it as
			<code>ENDPOINT_TOKEN</code>
			.
		</p>
		<div class="token">
			<input readonly value={form.created} aria-label="The token" />
			<button onclick={() => form.copy()}>
				<Icon name={form.copied ? 'check' : 'copy'} />
				{form.copied ? 'Copied' : 'Copy'}
			</button>
		</div>
	</div>
{:else}
	<form id="token" class="cards" onsubmit={save}>
		<div class="card">
			<label class="stacked">
				Name
				<input bind:value={form.name} placeholder="Website" required />
				{#if form.fields.name}
					<span class="error">{form.fields.name}</span>
				{/if}
			</label>
			<label class="stacked">
				Expires
				<DatePicker
					value={form.expires}
					onchange={v => (form.expires = v)}
					placeholder="Never"
					min={today}
				/>
			</label>
		</div>

		<div class="card">
			<h2>Sections</h2>
			<Switch bind:checked={form.allSections}>All sections</Switch>
			{#if !form.allSections}
				<CheckGroup
					label="Sections"
					options={schema.sections.map(s => ({
						value: s.handle,
						label: s.name,
					}))}
					picked={form.sections}
					ontoggle={(v, on) => form.toggle('sections', v, on)}
				/>
			{/if}
			{#if form.fields.sections}
				<span class="error">{form.fields.sections}</span>
			{/if}
		</div>

		<div class="card">
			<h2>Sites</h2>
			<Switch bind:checked={form.allSites}>All sites</Switch>
			{#if !form.allSites}
				<CheckGroup
					label="Sites"
					options={schema.sites.map(s => ({
						value: s.handle,
						label: s.name,
					}))}
					picked={form.sites}
					ontoggle={(v, on) => form.toggle('sites', v, on)}
				/>
			{/if}
			{#if form.fields.sites}
				<span class="error">{form.fields.sites}</span>
			{/if}
		</div>

		<div class="card">
			<h2>Drafts</h2>
			<Switch bind:checked={form.previews}>
				Preview links show drafts
			</Switch>
			<p class="note">
				For the website that shows the studio's previews; other
				frontends only read what is live.
			</p>
		</div>
	</form>
{/if}

<style lang="scss">
	.token {
		display: flex;
		gap: 0.5rem;

		input {
			flex: 1;
			font-family: var(--mono);
		}
	}
</style>
