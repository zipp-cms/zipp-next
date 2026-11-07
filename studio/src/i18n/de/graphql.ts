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
	'graphql.sites': 'Websites',
	'graphql.drafts': 'Entwürfe',
	'graphql.users': 'Benutzer',
	'graphql.all': 'Alle',
	'graphql.confirmRevoke':
		'{name} widerrufen? Frontends, die ihn senden, erhalten keine Inhalte mehr.',
	'graphql.graphiqlTitle': 'GraphiQL mit Ihrer Sitzung',
	'graphql.tryIt': 'Ausprobieren',
	'graphql.intro':
		'Frontends senden einen Token als {header} an {path}. Jeder liest, was sein Schema sagt.',
	'graphql.token': 'Token',
	'graphql.lastUsed': 'Zuletzt verwendet',
	'graphql.noTokens': 'Noch keine Tokens',
	'graphql.expired': 'Abgelaufen',
	'graphql.revoke': 'Token widerrufen',
	'graphql.schema': 'Schema',
	'graphql.schemas': 'Schemas',
	'graphql.schemasNote':
		'Was Tokens lesen, ist Code wie das Inhaltsmodell: {code}. Mehrere Tokens können eines teilen.',
	'graphql.noSchemas':
		'Der Code hat noch keine Schemas, nur der Token des Servers liest.',
	'graphql.assets': 'Dateien',
	'graphql.allSites': 'allen Websites',
	'graphql.reads': 'Liest {sections} in {sites}{extras}.',
	'graphql.serverToken': 'Server',
	'graphql.everything': 'Alles',
	'graphql.unknownSchema': 'Unbekanntes Schema {handle}',
	'graphql.unknownSchemaNote':
		'Der Code hat dieses Schema nicht mehr: der Token liest nichts.',
} satisfies Texts<typeof en>;
