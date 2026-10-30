<script lang="ts">
	import { session } from '../core/session.svelte';
	import { displayName } from '../users/api';

	/**
	 * While an admin is logged in as someone else: who, and the way back
	 * to the own account. The page reloads, the schema is the role's.
	 */
	async function back() {
		const id = session.user?.id;
		const ok = await session.endLoginAs();
		location.assign(ok ? `/admin/users/${id}` : '/admin/login');
	}
</script>

{#if session.impersonatedBy && session.user}
	<div class="login-as" role="status">
		<span>
			Logged in as <strong>{displayName(session.user)}</strong>
			by
			{session.impersonatedBy}
		</span>
		<button class="light" onclick={back}>Back to your account</button>
	</div>
{/if}

<style lang="scss">
	// floats at the bottom, over every page and the flyout
	.login-as {
		position: fixed;
		bottom: 1rem;
		left: 50%;
		z-index: var(--z-overlay);
		display: flex;
		align-items: center;
		gap: 1rem;
		max-width: calc(100vw - 2rem);
		padding: 0.5rem 0.5rem 0.5rem 1rem;
		border-radius: var(--radius);
		background: var(--accent);
		box-shadow: var(--shadow-pop);
		color: var(--on-accent);
		font-size: var(--fs-md);
		transform: translateX(-50%);

		button {
			padding: 0.25rem 0.625rem;
		}
	}
</style>
