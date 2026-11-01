<script>
	import { getTranslations } from 'crelte/translations';
	import Img from '@/components/Img.svelte';
	import Genre from '@/components/Genre.svelte';

	const t = getTranslations();

	let { entry } = $props();
</script>

{#each entry.image as img (img.url)}
	<Img {img} />
{/each}
<h1>{entry.title}</h1>
<Genre genre={entry.genre} />
<p class="meta">
	{entry.date} in {entry.city}{entry.spokenIn
		? ` · ${entry.spokenIn.toUpperCase()}`
		: ''}{entry.tickets?.price
		? ` · CHF ${entry.tickets.price.toFixed(2)}`
		: ''}{entry.tickets?.soldOut ? ` · ${$t('events.soldOut')}` : ''}
</p>
{#if entry.tickets?.categories.length}
	<table class="prices">
		{#each entry.tickets.categories as c, i (i)}
			<tr class:sold={c.soldOut}>
				<td>{c.name ?? ''}</td>
				<td>{c.price != null ? `CHF ${c.price.toFixed(2)}` : ''}</td>
				<td>{c.soldOut ? $t('events.soldOut') : ''}</td>
			</tr>
		{/each}
	</table>
{/if}
{#if entry.description}{@html entry.description.html}{/if}
{#each entry.page as p (p.url)}
	<a href={p.url}>{p.title}</a>
{/each}

<style>
	.meta {
		color: #666;
	}

	.prices {
		margin-bottom: 1rem;
	}

	.prices td {
		padding: 0.2rem 1rem 0.2rem 0;
	}

	.sold {
		color: #999;
	}
</style>
