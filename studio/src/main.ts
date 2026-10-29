import { mount } from 'svelte';
import App from './App.svelte';
// bundled, the studio loads nothing from other servers
import '@fontsource/inter/latin-400.css';
import '@fontsource/inter/latin-500.css';
import '@fontsource/inter/latin-600.css';
import '@fontsource/inter/latin-700.css';
import '@fontsource/jetbrains-mono/latin-400.css';
import './app.scss';

mount(App, { target: document.getElementById('app')! });
