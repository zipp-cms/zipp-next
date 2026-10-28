<script>
	import { getTranslations } from 'crelte/translations';
	import Img from '@/components/Img.svelte';

	const t = getTranslations();

	let { entry } = $props();
</script>

{#each entry.image as img (img.url)}
	<Img {img} />
{/each}
<h1>{entry.title}</h1>
<p class="meta">
	{entry.date} in {entry.city}{entry.price ? ` · CHF ${entry.price}` : ''}{entry.soldOut
		? ` · ${$t('events.soldOut')}`
		: ''}
</p>
{#if entry.description}<p>{entry.description}</p>{/if}
{#each entry.page as p (p.url)}
	<a href={p.url}>{p.title}</a>
{/each}

<style>
	.meta {
		color: #666;
	}
</style>
