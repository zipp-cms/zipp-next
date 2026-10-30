import { sitePreviewToken, type Schema } from '../api';

/**
 * The website in a frame with every draft (the AI page). A site with
 * zipp's preview bridge (see the crelte example) tells which page it
 * shows and opens pages without a reload; only pages of the website may
 * talk to the studio, the frame could have navigated anywhere.
 */
export class SitePreview {
	/** the page the frame shows, without the token */
	view = $state('');
	/** the src of the frame, only set to open the site or without bridge */
	src = $state('');
	error = $state('');
	private token = '';
	private bridge = false;
	private frame: HTMLIFrameElement | null = null;
	private readonly bases: string[];
	private readonly origins: Set<string>;

	constructor(schema: Schema, site: string, view: string | null) {
		const home = schema.siteUrls[site] ?? '';
		this.bases = Object.values(schema.siteUrls).filter(Boolean);
		this.origins = new Set(this.bases.map(b => new URL(b).origin));
		// from before, it may be of a site that changed
		this.view = view && this.ofSite(view) ? view : home;
	}

	/** Gets the token and listens to the frame; the result stops it. */
	start(frame: HTMLIFrameElement): () => void {
		this.frame = frame;
		sitePreviewToken()
			.then(t => {
				this.token = t;
				this.src = this.withToken(this.view);
			})
			.catch(e => (this.error = e.message));

		const onMessage = (e: MessageEvent) => {
			if (e.source !== this.frame?.contentWindow) return;
			if (!this.origins.has(e.origin)) return;
			if (e.data?.type === 'zipp:location' && this.ofSite(e.data.url)) {
				this.bridge = true;
				this.view = e.data.url;
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
		if (!url) return '';
		const u = new URL(url);
		u.searchParams.set('token', this.token);
		u.searchParams.set('x-craft-live-preview', '1');
		return u.href;
	}

	/** Opens a page of the website. */
	open(url: string) {
		if (!this.ofSite(url)) return;
		this.view = url;
		if (!this.bridge) return void (this.src = this.withToken(url));
		// to the website only, the url has the token
		this.frame?.contentWindow?.postMessage(
			{ type: 'zipp:open', url: this.withToken(url) },
			new URL(url).origin,
		);
	}

	/** Shows the changes: setting src loads the page again. */
	refresh() {
		if (this.frame) this.frame.src = this.withToken(this.view);
	}
}
