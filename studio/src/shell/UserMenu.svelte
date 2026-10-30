<script lang="ts">
	import type { Schema } from '../core/schema';
	import { roleName } from '../core/access';
	import { session } from '../core/session.svelte';
	import Icon from '../ui/Icon.svelte';

	/** Who is logged in, with the role, and the logout. */
	let { schema, onlogout }: { schema: Schema; onlogout: () => void } =
		$props();

	const email = $derived(session.user?.email ?? '?');
</script>

<div class="me">
	<a href="/admin/setup/sites#roles" title="What you may do">
		<span class="avatar">{email[0].toUpperCase()}</span>
		<span class="who">
			<strong class="truncate">{email}</strong>
			<small>{roleName(schema)}</small>
		</span>
	</a>
	<button class="icon" onclick={onlogout} title="Log out">
		<Icon name="logout" />
	</button>
</div>

<style>
	.me {
		display: flex;
		align-items: center;
		gap: 4px;
		margin-top: auto;
		padding-top: 12px;
		border-top: 1px solid var(--line);
	}

	a {
		display: flex;
		align-items: center;
		gap: 10px;
		flex: 1;
		min-width: 0;
		padding: 4px 6px;
		border-radius: var(--radius);
		color: inherit;
		text-decoration: none;
	}

	a:hover {
		background: var(--hover);
	}

	.avatar {
		display: grid;
		place-items: center;
		width: 28px;
		height: 28px;
		flex-shrink: 0;
		border-radius: 50%;
		background: var(--accent-soft);
		color: var(--accent);
		font-size: var(--fs-sm);
		font-weight: 700;
	}

	.who {
		display: flex;
		flex-direction: column;
		min-width: 0;
	}

	strong {
		font-size: var(--fs-sm);
		font-weight: 600;
	}

	small {
		color: var(--muted);
		font-size: var(--fs-xs);
	}

	@media (max-width: 800px) {
		.me {
			margin-top: 0;
		}
	}
</style>
