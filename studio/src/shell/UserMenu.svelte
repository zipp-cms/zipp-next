<script lang="ts">
	import type { Schema } from '../core/schema';
	import { roleName } from '../core/access';
	import { session } from '../core/session.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import { displayName } from '../users/api';
	import Avatar from '../users/Avatar.svelte';
	import { t } from '../i18n/index.svelte';

	/** Who is logged in, with the role, and the logout. */
	let { schema, onlogout }: { schema: Schema; onlogout: () => void } =
		$props();

	const name = $derived(session.user ? displayName(session.user) : '?');
</script>

<div class="me">
	<a href="/admin/users/{session.user?.id}" title={t('common.yourAccount')}>
		{#if session.user}<Avatar user={session.user} small />{/if}
		<span class="who">
			<strong class="truncate">{name}</strong>
			<small>{roleName(schema)}</small>
		</span>
	</a>
	<button class="icon" onclick={onlogout} title={t('common.logOut')}>
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
</style>
