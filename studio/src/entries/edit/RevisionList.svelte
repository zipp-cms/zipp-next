<script lang="ts">
	import type { Component } from '../../core/schema';
	import type { Revision } from '../drafts';
	import { time } from '../../core/format';
	import { changeText } from './revisions';

	/** The revisions, newest first: the first is live, older ones restore. */
	let {
		revisions,
		comp,
		multiSite,
		onview,
		onrestore,
	}: {
		revisions: Revision[];
		comp: Component;
		multiSite: boolean;
		onview: (rev: Revision) => void;
		/** none for a role that may not edit */
		onrestore: ((rev: Revision) => void) | null;
	} = $props();
</script>

<ul class="revisions">
	{#each revisions as rev, i (rev.id)}
		<li>
			<button class="link rev" onclick={() => onview(rev)}>
				<span>
					{time(rev.createdAt)}
					{#if rev.user}<span class="muted by">{rev.user}</span>{/if}
					{#if i === 0}<span class="muted by">live</span>{/if}
				</span>
				<span class="what">{changeText(rev, comp, multiSite)}</span>
			</button>
			{#if i > 0 && onrestore}
				<button class="link" onclick={() => onrestore(rev)}>
					Restore
				</button>
			{/if}
		</li>
	{/each}
</ul>

<style lang="scss">
	.revisions {
		margin: 0;
		padding: 0;
		list-style: none;

		li {
			display: flex;
			justify-content: space-between;
			align-items: center;
			padding: 6px 0;
			border-bottom: 1px solid var(--line);
		}
	}

	.rev {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		gap: 2px;
		padding: 0;
		color: var(--text);
		text-align: left;
	}

	.what {
		font-size: var(--fs-md);
		color: var(--muted);
	}

	.by {
		margin-left: 8px;
	}
</style>
