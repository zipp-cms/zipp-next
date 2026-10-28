<script lang="ts">
	import { gql, url, type Link } from './api';

	let { city }: { city: string | null } = $props();

	// upcoming events, filtered and sorted by their indexed fields; like
	// crelte's loadData of a block
	const EVENTS = `
		query ($site: String, $where: [Where!]) {
			entries(type: "event", site: $site, where: $where, order: "date") {
				title uri
				... on event_Entry { date city }
			}
		}
	`;

	type Event = Link & { date: string; city: string };
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
			<a href={url(e)}>{e.title}</a>
			{e.date}, {e.city}
		</li>
	{:else}
		<li>No upcoming events</li>
	{/each}
</ul>
