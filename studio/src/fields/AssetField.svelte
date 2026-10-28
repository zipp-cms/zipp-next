<script lang="ts">
	import {
		assetUrl,
		listAssets,
		uploadAssets,
		type Asset,
		type Field,
		type Values,
	} from '../api';
	import AssetGrid from './AssetGrid.svelte';

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
			{#if a.kind === 'image'}
				<img src={assetUrl(a, true)} alt={a.alt} />
			{/if}
			<span>{a.filename}</span>
			<button class="icon" title="Remove" onclick={() => toggle(a)}>
				×
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
		gap: 0.5rem;
		align-items: center;
	}

	.chosen {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.25rem 0.25rem 0.25rem 0.25rem;
		border: 1px solid var(--line);
		border-radius: 0.3rem;

		img {
			width: 3rem;
			height: 3rem;
			object-fit: cover;
			border-radius: 0.2rem;
		}

		span {
			font-size: 0.85rem;
			max-width: 12rem;
			overflow: hidden;
			text-overflow: ellipsis;
			white-space: nowrap;
		}
	}

	.overlay {
		position: fixed;
		inset: 0;
		z-index: 10;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.35);
	}

	.panel {
		width: min(52rem, 94vw);
		height: min(40rem, 90vh);
		overflow: auto;
		padding: 1rem;
		background: white;
		border-radius: 0.5rem;

		header {
			display: flex;
			gap: 0.5rem;
			margin-bottom: 1rem;
		}
	}

	.upload {
		cursor: pointer;
		padding: 0.45rem 0.9rem;
		border: 1px solid var(--line);
		border-radius: 0.3rem;
		white-space: nowrap;

		input {
			display: none;
		}
	}
</style>
