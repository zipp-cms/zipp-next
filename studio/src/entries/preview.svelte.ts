import { type Schema } from '../core/schema';

/** A token is good for a day, a new one well before. */
const TOKEN_FOR_MS = 1000 * 60 * 60 * 20;

/**
 * The website in a frame with drafts: every draft on the AI page, the
 * entry's in the editor's live preview, `getToken` gives the token. A
 * site with zipp's preview bridge (see the crelte example) tells which
 * page it shows and opens pages without a reload; only pages of the
 * website may talk to the studio, the frame could have navigated
 * anywhere.
 */
export class SitePreview {
	/** the page the frame shows, without the token */
	view = $state('');
	error = $state('');
	/** without it the website shows no drafts, nothing loads before */
	token = $state('');
	private tokenAt = 0;
	private bridge = false;
	private frame: HTMLIFrameElement | null = null;
	private readonly bases: string[];
	private readonly origins: Set<string>;

	constructor(
		schema: Schema,
		site: string,
		view: string | null,
		private readonly getToken: () => Promise<string>,
	) {
		const home = schema.siteUrls[site] ?? '';
		this.bases = Object.values(schema.siteUrls).filter(Boolean);
		this.origins = new Set(
			this.bases.map(b => new URL(b, location.origin).origin),
		);
		// from before, it may be of a site that changed
		this.view = view && this.ofSite(view) ? view : home;
	}

	/** Gets the token and listens to the frame; the result stops it. */
	start(frame: HTMLIFrameElement): () => void {
		this.frame = frame;
		this.newToken();

		const onMessage = (e: MessageEvent) => {
			if (e.source !== this.frame?.contentWindow) return;
			if (!this.origins.has(e.origin)) return;
			if (e.data?.type === 'zipp:location' && this.ofSite(e.data.url)) {
				this.bridge = true;
				this.view = withoutToken(e.data.url);
			}
		};
		window.addEventListener('message', onMessage);
		return () => window.removeEventListener('message', onMessage);
	}

	ofSite(url: unknown): url is string {
		return (
			typeof url === 'string' &&
			this.bases.some(
				b => url === b || url.startsWith(b.replace(/\/?$/, '/')),
			)
		);
	}

	/** A page with the token, in crelte's live preview mode. */
	withToken(url: string): string {
		if (!url || !this.token) return '';
		const u = new URL(url);
		u.searchParams.set('token', this.token);
		u.searchParams.set('x-craft-live-preview', '1');
		return u.href;
	}

	/** Opens a page of the website. */
	open(url: string) {
		if (!this.ofSite(url)) return;
		this.view = url;
		if (!this.bridge || this.stale()) return this.load(url);
		// to the website only, the url has the token
		this.frame?.contentWindow?.postMessage(
			{ type: 'zipp:open', url: this.withToken(url) },
			new URL(url).origin,
		);
	}

	/** Shows the changes: the page loads again. */
	refresh() {
		this.load(this.view);
	}

	/**
	 * Shows a saved draft. The page loads again, like Craft's: crelte
	 * caches every query of a page in the browser, so only a load shows
	 * new content; its live preview mode keeps the scroll position.
	 */
	update() {
		// the first load comes with the token
		if (!this.token) return;
		this.load(this.view);
	}

	/** Whether the token is old: the studio may be open for days. */
	private stale() {
		return !!this.token && Date.now() - this.tokenAt > TOKEN_FOR_MS;
	}

	private newToken() {
		this.getToken()
			.then(t => {
				this.token = t;
				this.tokenAt = Date.now();
				this.error = '';
				this.load(this.view);
			})
			.catch(e => (this.error = e.message));
	}

	/**
	 * Loads a page in the frame, the only place that does. It replaces
	 * the frame's page: a new src would add a step to the history, and
	 * the studio's Back button would go back in the frame. The page may
	 * have no bridge, until it says so.
	 */
	private load(url: string) {
		// a new token first, it loads the page then
		if (this.stale()) return this.newToken();
		const src = this.withToken(url);
		const win = this.frame?.contentWindow;
		if (!win || !src) return;
		this.bridge = false;
		win.location.replace(src);
	}
}

/**
 * The url without the preview's token: it shows in the bar, goes to the
 * AI and into the session storage; a site's bridge may leave it on.
 */
function withoutToken(url: string): string {
	try {
		const u = new URL(url);
		u.searchParams.delete('token');
		u.searchParams.delete('x-craft-live-preview');
		return u.toString();
	} catch {
		return url;
	}
}
