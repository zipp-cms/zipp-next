<script lang="ts">
	import { initials, type User } from './api';

	/** A user's initials in a circle; `small` in the sidebar. */
	let {
		user,
		small = false,
	}: { user: Pick<User, 'name' | 'email'>; small?: boolean } = $props();
</script>

<span class="avatar" class:small aria-hidden="true">{initials(user)}</span>

<style lang="scss">
	.avatar {
		display: grid;
		place-items: center;
		width: 2.75rem;
		height: 2.75rem;
		flex-shrink: 0;
		border-radius: 50%;
		background: var(--accent-soft);
		color: var(--accent);
		font-weight: 600;

		&.small {
			width: 1.75rem;
			height: 1.75rem;
			font-size: var(--fs-sm);
			font-weight: 700;
		}
	}
</style>
