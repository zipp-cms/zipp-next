<script lang="ts">
	import { entries, url } from './api';
	import type { Event as EventEntry } from './content';

	let { city }: { city?: string } = $props();

	// upcoming events, filtered and sorted by their indexed fields
	let events = $state<EventEntry[]>([]);
	$effect(() => {
		const today = new Date().toISOString().slice(0, 10);
		const params: Record<string, string> = {
			type: 'event',
			'f.date.gte': today,
			order: 'date',
		};
		if (city) params['f.city'] = city;
		entries<EventEntry>(params).then(r => (events = r.entries));
	});
</script>

<ul>
	{#each events as e (e.id)}
		<li>
			<a href={url(e)}>{e.title}</a>
			{e.fields.date}, {e.fields.city}
		</li>
	{:else}
		<li>No upcoming events</li>
	{/each}
</ul>
