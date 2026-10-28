import { request } from './client';

/** site → namespace → key → saved text, defaults are in the schema */
export async function getTranslations(): Promise<
	Record<string, Record<string, Record<string, string>>>
> {
	return (await request('GET', '/translations')).saved;
}

export function putTranslation(
	site: string,
	namespace: string,
	key: string,
	value: string,
): Promise<void> {
	return request('PUT', '/translations', { site, namespace, key, value });
}
