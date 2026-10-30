<script lang="ts">
	import type { Schema } from '../core/schema';
	import { roleName } from '../core/access';
	import { session } from '../core/session.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import { displayName } from '../users/api';

	/** Who is logged in, with the role, and the logout. */
	let { schema, onlogout }: { schema: Schema; onlogout: () => void } =
		$props();

	const name = $derived(session.user ? displayName(session.user) : '?');
</script>

<div class="me">
	<a href="/admin/users/{session.user?.id}" title="Your account">
		<span class="avatar">{name[0].toUpperCase()}</span>
		<span class="who">
			<strong class="truncate">{name}</strong>
			<small>{roleName(schema)}</small>
		</span>
	</a>
	<button class="icon" onclick={onlogout} title="Log out">
		<Icon name="logout" />
	</button>
</div>

<style lang="scss">
	.me {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		margin-top: auto;
		padding-top: 0.75rem;
		border-top: 1px solid var(--line);
	}

	a {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		flex: 1;
		min-width: 0;
		padding: 0.25rem 0.375rem;
		border-radius: var(--radius);
		color: inherit;
		text-decoration: none;
	}

	@include hover {
		a:hover {
			background: var(--hover);
		}
	}

	.avatar {
		display: grid;
		place-items: center;
		width: 1.75rem;
		height: 1.75rem;
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

	@include max-tablet {
		.me {
			margin-top: 0;
		}
	}
</style>
