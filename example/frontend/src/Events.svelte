<script lang="ts">
	import { gql, url, type Link } from './api';

	let { city }: { city: string | null } = $props();

	// upcoming events, filtered and sorted by their indexed fields; like
	// crelte's loadData of a block
	const EVENTS = `
		query ($site: String, $where: [Where!]) {
			entries(type: "event", site: $site, where: $where, order: "date") {
				title uri
				... on event_Entry { date city image { url(transform: "thumb") alt } }
			}
		}
	`;

	type Event = Link & {
		date: string;
		city: string;
		image: { url: string; alt: string }[];
	};
	let events = $state<Event[]>([]);

	$effect(() => {
		const today = new Date().toISOString().slice(0, 10);
		const where = [{ field: 'date', op: 'gte', value: today }];
		if (city) where.push({ field: 'city', op: 'eq', value: city });
		gql<{ entries: Event[] }>(EVENTS, { where }).then(
			r => (events = r.entries),
		);
	});
</script>

<ul>
	{#each events as e (e.uri)}
		<li>
			{#each e.image as img (img.url)}<img
					src={img.url}
					alt={img.alt}
				/>{/each}
			<a href={url(e)}>{e.title}</a>
			{e.date}, {e.city}
		</li>
	{:else}
		<li>No upcoming events</li>
	{/each}
</ul>

<style>
	ul {
		padding: 0;
		list-style: none;
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
</style>
