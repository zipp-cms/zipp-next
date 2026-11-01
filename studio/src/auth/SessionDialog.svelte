<script lang="ts">
	import { session } from '../core/session.svelte';
	import { endSession } from '../core/router.svelte';
	import { displayName } from '../users/api';
	import Icon from 'zipp-ui/Icon.svelte';

	/**
	 * The session ended while the studio was open: the login again over
	 * the page, so what was typed stays and is saved after it. Another
	 * user loads the studio new, their rights differ. Logged in as
	 * someone else there is no password to ask for: back to the own
	 * account or to the login.
	 */
	let email = $state(session.user?.email ?? '');
	let password = $state('');
	let showPassword = $state(false);
	let error = $state('');
	let busy = $state(false);

	let dialog: HTMLDialogElement;
	// modal: the page under it can't be used, Tab stays in it
	$effect(() => dialog.showModal());

	async function onsubmit(e: SubmitEvent) {
		e.preventDefault();
		error = '';
		busy = true;
		try {
			if (!(await session.renew(email, password)))
				location.assign('/admin');
		} catch (e: any) {
			error = e.message;
		} finally {
			busy = false;
		}
	}

	async function back() {
		busy = true;
		const ok = await session.endLoginAs();
		location.assign(ok ? '/admin/users' : '/admin/login');
	}
</script>

<!-- Escape doesn't close it, the session is still gone -->
<dialog
	bind:this={dialog}
	aria-labelledby="session-ended"
	oncancel={e => e.preventDefault()}
>
	{#if session.impersonatedBy && session.user}
		<div class="form">
			<div class="intro">
				<h2 id="session-ended">The session ended</h2>
				<p class="muted">
					You were logged in as {displayName(session.user)}. Go back
					to your account or log in.
				</p>
			</div>
			<div class="actions">
				<button type="button" onclick={() => endSession()}>
					Log in
				</button>
				<button class="primary" onclick={back} disabled={busy}>
					Back to your account
				</button>
			</div>
		</div>
	{:else}
		<form class="form" {onsubmit}>
			<div class="intro">
				<h2 id="session-ended">The session ended</h2>
				<p class="muted">
					Log in again to go on, what you changed stays.
				</p>
			</div>

			<label class="stacked">
				Email
				<input
					type="email"
					autocomplete="username"
					bind:value={email}
					required
				/>
			</label>

			<label class="stacked">
				Password
				<span class="password">
					<!-- svelte-ignore a11y_autofocus -->
					<input
						type={showPassword ? 'text' : 'password'}
						autocomplete="current-password"
						bind:value={password}
						required
						autofocus
					/>
					<button
						type="button"
						class="icon"
						title={showPassword
							? 'Hide the password'
							: 'Show the password'}
						onclick={() => (showPassword = !showPassword)}
					>
						<Icon name={showPassword ? 'hide' : 'view'} />
					</button>
				</span>
			</label>

			{#if error}<p class="error">{error}</p>{/if}

			<div class="actions">
				<button type="button" onclick={() => endSession()}>
					Log out
				</button>
				<button class="primary" disabled={busy}>Log in</button>
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

	// the eye inside the input, on its right
	.password {
		position: relative;
		display: block;

		input {
			width: 100%;
			padding-right: 2.75rem;
		}

		button {
			position: absolute;
			top: 50%;
			right: 0.375rem;
			transform: translateY(-50%);
		}
	}

	.actions {
		display: flex;
		justify-content: flex-end;
		gap: 0.5rem;
	}
</style>
