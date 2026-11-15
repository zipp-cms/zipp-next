<script>
	import { getSite } from 'crelte';
	import { getTranslations } from 'crelte/translations';

	// the contact form of zipp-forms: the server proxies /forms/contact
	// to the cms, which mails it to who is set in Forms
	let { title } = $props();
	const t = getTranslations();
	const site = getSite();

	let values = $state({
		name: '',
		email: '',
		phone: '',
		message: '',
		website: '',
	});
	// the inputs besides the message
	/** @type {{ key: 'name' | 'email' | 'phone', type: string, required: boolean }[]} */
	const INPUTS = [
		{ key: 'name', type: 'text', required: true },
		{ key: 'email', type: 'email', required: true },
		{ key: 'phone', type: 'tel', required: false },
	];

	/** @type {Record<string, string>} */
	let errors = $state({});
	let sent = $state('');
	let failed = $state('');
	let sending = $state(false);

	async function submit(e) {
		e.preventDefault();
		sending = true;
		errors = {};
		failed = '';
		try {
			const resp = await fetch(`/forms/contact?site=${$site.handle}`, {
				method: 'POST',
				headers: { 'content-type': 'application/json' },
				body: JSON.stringify(values),
			});
			const body = await resp.json().catch(() => ({}));
			if (body.success) sent = body.message;
			else if (body.errors)
				errors = Object.fromEntries(
					body.errors.map(e => [e.path, e.message]),
				);
			else failed = body.message ?? $t('contact.failed');
		} catch {
			failed = $t('contact.failed');
		} finally {
			sending = false;
		}
	}
</script>

<section class="contact">
	{#if title}<h2>{title}</h2>{/if}
	{#if sent}
		<p class="sent" role="status">{sent}</p>
	{:else}
		<form onsubmit={submit}>
			{#each INPUTS as { key, type, required } (key)}
				<label>
					{$t(`contact.${key}`)}{required ? '*' : ''}
					<input
						{type}
						name={key}
						bind:value={values[key]}
						{required}
					/>
					{#if errors[key]}<span class="error">
							{errors[key]}
						</span>{/if}
				</label>
			{/each}
			<label>
				{$t('contact.message')}*
				<textarea
					name="message"
					rows="5"
					bind:value={values.message}
					required
				></textarea>
				{#if errors.message}<span class="error">
						{errors.message}
					</span>{/if}
			</label>
			<!-- people don't see it, bots fill it -->
			<input
				class="honeypot"
				name="website"
				tabindex="-1"
				autocomplete="off"
				aria-hidden="true"
				bind:value={values.website}
			/>
			{#if failed}<p class="error">{failed}</p>{/if}
			<button disabled={sending}>{$t('contact.send')}</button>
		</form>
	{/if}
</section>

<style>
	.contact {
		max-width: 32rem;
		margin: 2rem 0;
	}

	form {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	label {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	input,
	textarea {
		padding: 0.5rem;
		font: inherit;
	}

	.honeypot {
		position: absolute;
		left: -10000px;
	}

	.error {
		color: #b00020;
	}

	button {
		align-self: flex-start;
		padding: 0.5rem 1rem;
		border: 0;
		border-radius: 0.3rem;
		background: #222;
		color: white;
	}
</style>
