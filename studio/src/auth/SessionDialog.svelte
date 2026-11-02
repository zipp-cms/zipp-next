<script lang="ts">
	import { session } from '../core/session.svelte';
	import { endSession } from '../core/router.svelte';
	import { displayName } from '../users/api';
	import PasswordInput from './PasswordInput.svelte';
	import { Busy } from '../core/busy.svelte';
	import { t } from '../i18n/index.svelte';

	/**
	 * The session ended while the studio was open: the login again over
	 * the page, so what was typed stays and is saved after it. Another
	 * user loads the studio new, their rights differ. Logged in as
	 * someone else there is no password to ask for: back to the own
	 * account or to the login.
	 */
	let email = $state(session.user?.email ?? '');
	let password = $state('');
	const action = new Busy();

	let dialog: HTMLDialogElement;
	// modal: the page under it can't be used, Tab stays in it
	$effect(() => dialog.showModal());

	function onsubmit(e: SubmitEvent) {
		e.preventDefault();
		action.run(async () => {
			if (!(await session.renew(email, password)))
				location.assign('/admin');
		});
	}

	// the page loads new, it stays busy until then
	const back = () => action.run(() => session.backToOwnAccount());
</script>

<!-- Escape doesn't close it, the session is still gone. Chrome lets a
second Escape through the cancel event, so closedby and a close opens
it again. -->
<dialog
	bind:this={dialog}
	aria-labelledby="session-ended"
	closedby="none"
	oncancel={e => e.preventDefault()}
	onclose={() => dialog.isConnected && dialog.showModal()}
>
	{#if session.impersonatedBy && session.user}
		<div class="form">
			<div class="intro">
				<h2 id="session-ended">{t('auth.sessionEnded')}</h2>
				<p class="muted">
					{t('auth.wereLoggedInAs', {
						name: displayName(session.user),
					})}
				</p>
			</div>
			<div class="actions">
				<button type="button" onclick={() => endSession()}>
					{t('auth.logIn')}
				</button>
				<button class="primary" onclick={back} disabled={action.busy}>
					{t('common.backToAccount')}
				</button>
			</div>
		</div>
	{:else}
		<form class="form" {onsubmit}>
			<div class="intro">
				<h2 id="session-ended">{t('auth.sessionEnded')}</h2>
				<p class="muted">{t('auth.logInAgain')}</p>
			</div>

			<label class="stacked">
				{t('common.email')}
				<input
					type="email"
					autocomplete="username"
					bind:value={email}
					required
				/>
			</label>

			<label class="stacked">
				{t('common.password')}
				<PasswordInput
					autocomplete="current-password"
					bind:value={password}
					autofocus
				/>
			</label>

			{#if action.error}<p class="error">{action.error}</p>{/if}

			<div class="actions">
				<button type="button" onclick={() => endSession()}>
					{t('common.logOut')}
				</button>
				<button class="primary" disabled={action.busy}>
					{t('auth.logIn')}
				</button>
			</div>
		</form>
	{/if}
</dialog>

<style lang="scss">
	dialog {
		width: min(24rem, calc(100vw - 2rem));
		padding: 1.5rem;
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
		background: var(--surface);
		box-shadow: var(--shadow-pop);
		color: var(--text);

		&::backdrop {
			background: var(--backdrop);
		}
	}

	.form {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.intro {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;

		h2 {
			margin: 0;
			font-size: var(--fs-xl);
			line-height: 1.3;
		}

		p {
			margin: 0;
		}
	}

	.actions {
		display: flex;
		justify-content: flex-end;
		gap: 0.5rem;
	}
</style>
