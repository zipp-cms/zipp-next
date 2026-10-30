<script module lang="ts">
	/** Anyone may open the login. */
	export const isPublic = true;
</script>

<script lang="ts">
	import { session } from '../core/session.svelte';
	import { router } from '../core/router.svelte';
	import Icon from 'zipp-ui/Icon.svelte';
	import LogoIcon from 'zipp-ui/LogoIcon.svelte';
	import logo from '../logo/full.svg';

	/**
	 * The login: the form on the left, the icon big on the accent on the
	 * right (only where there is room).
	 */
	let email = $state('');
	let password = $state('');
	let remember = $state(true);
	let showPassword = $state(false);
	let error = $state('');
	let busy = $state(false);

	async function onsubmit(e: SubmitEvent) {
		e.preventDefault();
		error = '';
		busy = true;
		try {
			await session.login(email, password, remember);
			// back to the page that wanted the login
			const url = new URLSearchParams(location.search).get('url');
			const back =
				url?.startsWith('/admin') && !url.startsWith('/admin/login');
			router.open(back ? url! : '/admin', {}, { history: 'replace' });
		} catch (e: any) {
			error = e.message;
		} finally {
			busy = false;
		}
	}
</script>

<div class="login">
	<main>
		<header>
			<img class="logo" src={logo} alt="zipp" />
		</header>

		<form {onsubmit}>
			<div class="intro">
				<h1>Log in</h1>
				<p class="muted">Welcome back. Log in with your account.</p>
			</div>

			<label class="stacked">
				Email
				<input
					type="email"
					autocomplete="username"
					placeholder="name@example.com"
					bind:value={email}
					required
				/>
			</label>

			<label class="stacked">
				Password
				<span class="password">
					<input
						type={showPassword ? 'text' : 'password'}
						autocomplete="current-password"
						bind:value={password}
						required
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

			<label class="check">
				<input type="checkbox" bind:checked={remember} />
				Stay logged in
			</label>

			{#if error}<p class="error">{error}</p>{/if}

			<button class="primary submit" disabled={busy}>
				Log in
				<Icon name="arrow-right" />
			</button>

			<p class="note">
				No access or forgot your password? Ask the administration of
				your website.
			</p>
		</form>

		<footer class="note">zipp CMS · v{__ZIPP_VERSION__}</footer>
	</main>

	<aside aria-hidden="true">
		<div class="mark"><LogoIcon /></div>
	</aside>
</div>

<style lang="scss">
	.login {
		position: fixed;
		inset: 0;
		display: grid;
		grid-template-columns: 1fr 1fr;
		background: var(--surface);
		overflow-y: auto;

		@include max-tablet {
			grid-template-columns: 1fr;
		}
	}

	main {
		display: flex;
		flex-direction: column;
		min-height: 100%;
		padding: 2rem;
	}

	.logo {
		display: block;
		width: 5rem;
	}

	form {
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
		width: min(22rem, 100%);
		margin: auto;
		padding: 3rem 0;
	}

	.intro {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;

		h1 {
			font-size: var(--fs-title);
			line-height: 1.2;
			font-weight: 700;
		}

		p {
			margin: 0;
			font-size: var(--fs-lg);
		}
	}

	input:not([type='checkbox']) {
		padding: 0.5625rem 0.75rem;
		font-size: var(--fs-lg);
	}

	// the eye inside the input, on its right
	.password {
		position: relative;
		display: block;

		input {
			padding-right: 2.75rem;
		}

		button {
			position: absolute;
			top: 50%;
			right: 0.375rem;
			transform: translateY(-50%);
		}
	}

	.check {
		font-size: var(--fs-lg);
	}

	.submit {
		justify-content: center;
		gap: 0.5rem;
		padding: 0.625rem;
		font-size: var(--fs-lg);
		font-weight: 600;
	}

	form > .note {
		margin: 0;
		color: var(--muted);
		text-align: center;
	}

	footer {
		text-align: center;
		font-family: var(--mono);
		font-size: var(--fs-xs);
	}

	aside {
		display: grid;
		place-items: center;
		background: var(--accent);

		@include max-tablet {
			display: none;
		}
	}

	.mark {
		--ring: var(--accent-ring);
		--bolt: var(--on-accent);
		width: min(28rem, 60%);
	}
</style>
