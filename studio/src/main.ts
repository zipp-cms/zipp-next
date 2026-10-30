import { mount } from 'svelte';
import App from './shell/App.svelte';
import { startRouter } from './core/router.svelte';
import { session } from './core/session.svelte';
import { registerRoutes } from './shell/routes';
// bundled, the studio loads nothing from other servers
import '@fontsource/inter/latin-400.css';
import '@fontsource/inter/latin-500.css';
import '@fontsource/inter/latin-600.css';
import '@fontsource/inter/latin-700.css';
import '@fontsource/jetbrains-mono/latin-400.css';
import './app.scss';

registerRoutes();
// who is logged in, before the first page; the app shows once it is loaded
session
	.init()
	.then(() =>
		startRouter(
			{
				notFound: () => import('./shell/NotFound.svelte'),
				error: () => import('./shell/ErrorPage.svelte'),
			},
			() => mount(App, { target: document.getElementById('app')! }),
		),
	)
	// the server can't be reached: the session stays for the next try
	.catch(e => {
		const p = document.createElement('p');
		p.className = 'error';
		p.style.padding = '24px';
		p.textContent = `zipp can't be reached (${e.message}), reload the page to try again.`;
		document.getElementById('app')!.append(p);
	});
