import { marked, type Tokens } from 'marked';

/**
 * Markdown to html for the studio: raw html is dropped, links only go to
 * the web or the site and images aren't loaded, so text from the content
 * or an AI answer can't run code or send data anywhere.
 */
const safe = (href: string) =>
	/^(https?:\/\/|mailto:|#)/i.test(href) ||
	(href.startsWith('/') && !href.startsWith('//'));

const escape = (s: string) =>
	s.replace(/[&<>"]/g, c => `&#${c.charCodeAt(0)};`);

marked.use({
	renderer: {
		html: () => '',
		image: ({ text }: Tokens.Image) => escape(text),
		link(token: Tokens.Link) {
			const inner = this.parser.parseInline(token.tokens);
			if (!safe(token.href)) return inner;
			return `<a href="${escape(token.href)}" target="_blank" rel="noreferrer">${inner}</a>`;
		},
	},
});

export function markdown(md: string): string {
	return marked.parse(md, { async: false }) as string;
}
