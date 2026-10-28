<script lang="ts">
	import {
		listAssets,
		uploadAssets,
		type Asset,
		type Field,
		type Values,
	} from '../api';
	import AssetGrid from './AssetGrid.svelte';
	import Thumb from './Thumb.svelte';
	import Icon from '../lib/Icon.svelte';

	let { field, values }: { field: Field; values: Values } = $props();

	const images = $derived(field.kind === 'assets' && field.images);
	const max = $derived(field.kind === 'assets' ? field.max : null);
	const ids = $derived<number[]>(values[field.handle] ?? []);
	const full = $derived(max !== null && ids.length >= max);

	let chosen = $state<Asset[]>([]);
	let picking = $state(false);
	let library = $state<Asset[]>([]);
	let search = $state('');

	// the assets of the field, in its order
	$effect(() => {
		if (!ids.length) return void (chosen = []);
		listAssets({ ids: ids.join(',') }).then(a => (chosen = a));
	});

	$effect(() => {
		if (!picking) return;
		const params: Record<string, string> = { search };
		if (images) params.kind = 'image';
		listAssets(params).then(a => (library = a));
	});

	function toggle(a: Asset) {
		if (ids.includes(a.id)) {
			values[field.handle] = ids.filter(i => i !== a.id);
		} else if (!full) {
			values[field.handle] = [...ids, a.id];
		}
		// with a single one there is nothing more to pick
		if (max === 1) picking = false;
	}

	async function upload(files: FileList | null) {
		if (!files?.length) return;
		const added = await uploadAssets(files);
		for (const a of added) toggle(a);
		library = [...added, ...library];
	}
</script>

<div class="assets">
	{#each chosen as a (a.id)}
		<div class="chosen">
			<div class="tile"><Thumb asset={a} /></div>
			<button class="icon" title="Remove" onclick={() => toggle(a)}>
				<Icon name="delete" />
			</button>
		</div>
	{/each}
	{#if !full}
		<button onclick={() => (picking = true)}>
			Choose {images ? 'image' : 'file'}
		</button>
	{/if}
</div>

{#if picking}
	<div class="overlay" role="presentation" onclick={() => (picking = false)}>
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<div
			class="panel"
			role="dialog"
			tabindex="-1"
			onclick={e => e.stopPropagation()}
		>
			<header>
				<input placeholder="Search files" bind:value={search} />
				<label class="upload">
					Upload
					<input
						type="file"
						multiple={max !== 1}
						accept={images ? 'image/*' : undefined}
						onchange={e => upload(e.currentTarget.files)}
					/>
				</label>
				<button class="primary" onclick={() => (picking = false)}>
					Done
				</button>
			</header>
			<AssetGrid assets={library} selected={ids} onpick={toggle} />
		</div>
	</div>
{/if}

<style lang="scss">
	.assets {
		display: flex;
		flex-wrap: wrap;
		gap: 16px;
		align-items: center;
	}

	.chosen {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.tile {
		width: 117px;
	}

	.overlay {
		position: fixed;
		inset: 0;
		z-index: 10;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.6);
	}

	.panel {
		width: min(52rem, 94vw);
		height: min(40rem, 90vh);
		overflow: auto;
		padding: 1rem;
		background: white;
		box-shadow: var(--shadow);

		header {
			display: flex;
			gap: 0.5rem;
			margin-bottom: 1rem;
		}
	}

	.upload {
		cursor: pointer;
		padding: 0.45rem 0.9rem;
		border: 1px solid var(--dark);
		border-radius: 2px;
		font-size: 14px;
		white-space: nowrap;

		input {
			display: none;
		}
	}
</style>
