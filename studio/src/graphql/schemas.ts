import type { Schema } from '../core/schema';
import { t } from '../i18n/index.svelte';
import type { GraphqlSchema } from './api';

/** The names of some handles, or all. */
export function names(
	handles: string[] | null,
	all: { handle: string; name: string }[],
) {
	if (handles === null) return t('graphql.all');
	if (!handles.length) return t('common.none');
	return handles
		.map(h => all.find(x => x.handle === h)?.name ?? h)
		.join(', ');
}

/** What a GraphQL schema reads, in a line. */
export function reads(s: GraphqlSchema, schema: Schema) {
	const extras = [
		s.assets && t('graphql.assets'),
		s.users && t('graphql.users'),
		s.previews && t('graphql.drafts'),
	].filter(Boolean);
	return t('graphql.reads', {
		sections: names(s.sections, schema.sections),
		sites:
			s.sites === null
				? t('graphql.allSites')
				: names(s.sites, schema.sites),
		extras: extras.length ? ', ' + extras.join(', ') : '',
	});
}
