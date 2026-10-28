<script module>
	import eventsQuery from '@/queries/events.graphql';

	// the block loads its own list, filtered and sorted by indexed fields
	/**
	 * @param {import('crelte').CrelteRequest} cr
	 * @param {{ city: string | null }} block
	 */
	export async function loadData(cr, block) {
		const today = new Date().toISOString().slice(0, 10);
		const where = [{ field: 'date', op: 'gte', value: today }];
		if (block.city) where.push({ field: 'city', op: 'eq', value: block.city });
		return await cr.query(eventsQuery, { where });
	}
</script>

<script>
	import { getTranslations } from 'crelte/translations';
	import Genre from '@/components/Genre.svelte';

	const t = getTranslations();
	let { events } = $props();
</script>

<ul>
	{#each events as e (e.url)}
		<li>
			{#each e.image as img (img.url)}<img src={img.url} alt={img.alt} />{/each}
			<a href={e.url}>{e.title}</a>
			<Genre genre={e.genre} />
			<span>{e.date}, {e.city}</span>
			{#if e.soldOut}<strong>{$t('events.soldOut')}</strong>{/if}
		</li>
	{:else}
		<li>{$t('events.none')}</li>
	{/each}
</ul>

<style>
	ul {
		margin: 1rem 0;
	}

	li {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 0.5rem;
	}

	img {
		width: 80px;
		height: 60px;
		border-radius: 0.2rem;
	}

	span {
		color: #666;
	}
</style>
