import { mount } from 'svelte';
import App from './shell/App.svelte';
import { startRouter } from './core/router.svelte';
import { registerRoutes } from './shell/routes';
// bundled, the studio loads nothing from other servers
import '@fontsource/inter/latin-400.css';
import '@fontsource/inter/latin-500.css';
import '@fontsource/inter/latin-600.css';
import '@fontsource/inter/latin-700.css';
import '@fontsource/jetbrains-mono/latin-400.css';
import './app.scss';

registerRoutes();
// the app shows once the first page is loaded
startRouter(() => mount(App, { target: document.getElementById('app')! }));
