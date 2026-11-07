import type { Texts } from '../index.svelte';
import type en from '../en/graphql';

export default {
	'graphql.newToken': 'Neuer Token',
	'graphql.copyNow':
		'Kopieren Sie den Token jetzt, er wird nicht mehr angezeigt. Ein Frontend sendet ihn als {header}; crelte liest ihn als {env}.',
	'graphql.theToken': 'Der Token',
	'graphql.namePlaceholder': 'Website',
	'graphql.expires': 'Läuft ab',
	'graphql.sections': 'Abschnitte',
	'graphql.allSections': 'Alle Abschnitte',
	'graphql.sites': 'Websites',
	'graphql.allSites': 'Alle Websites',
	'graphql.drafts': 'Entwürfe',
	'graphql.previewsShowDrafts': 'Vorschau-Links zeigen Entwürfe',
	'graphql.previewsNote':
		'Für die Website, die die Vorschau des Studios zeigt; andere Frontends lesen nur, was live ist.',
	'graphql.users': 'Benutzer',
	'graphql.usersPublic': 'Liest die öffentlichen Profile der Benutzer',
	'graphql.usersNote':
		'Name, Slug, Seite und Benutzerfelder der Benutzer mit einem veröffentlichten Eintrag, nie die E-Mail oder die Rolle. Ohne haben die Autoren von Einträgen nur ihren Namen.',
	'graphql.all': 'Alle',
	'graphql.confirmRevoke':
		'{name} widerrufen? Frontends, die ihn senden, erhalten keine Inhalte mehr.',
	'graphql.graphiqlTitle': 'GraphiQL mit Ihrer Sitzung',
	'graphql.tryIt': 'Ausprobieren',
	'graphql.intro':
		'Frontends senden einen Token als {header} an {path}. Jeder liest nur seine Abschnitte und Websites.',
	'graphql.envToken': 'Der Token von {env} liest alles.',
	'graphql.token': 'Token',
	'graphql.lastUsed': 'Zuletzt verwendet',
	'graphql.noTokens': 'Noch keine Tokens',
	'graphql.expired': 'Abgelaufen',
	'graphql.revoke': 'Token widerrufen',
} satisfies Texts<typeof en>;
