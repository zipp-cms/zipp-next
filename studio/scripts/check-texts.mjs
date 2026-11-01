// Checks the translations: every text of English in each language, no
// others, with the same {params}. TypeScript checks the keys too, this
// also the params, which a translator easily forgets.
import { readdirSync, readFileSync } from 'node:fs';
import ts from 'typescript';

const dir = new URL('../src/i18n/', import.meta.url);

async function load(lang, file) {
	const source = readFileSync(new URL(`${lang}/${file}`, dir), 'utf8');
	const js = ts.transpileModule(source, {
		compilerOptions: { module: ts.ModuleKind.ESNext },
	}).outputText;
	const url = `data:text/javascript,${encodeURIComponent(js)}`;
	return (await import(url)).default;
}

const params = text =>
	[...new Set(JSON.stringify(text).match(/\{\w+\}/g) ?? [])].sort().join();

const langs = readdirSync(dir, { withFileTypes: true })
	.filter(d => d.isDirectory() && d.name !== 'en')
	.map(d => d.name);
const files = readdirSync(new URL('en/', dir)).filter(
	f => f !== 'index.ts',
);

const problems = [];
for (const file of files) {
	const en = await load('en', file);
	for (const lang of langs) {
		const other = await load(lang, file);
		for (const [key, text] of Object.entries(en)) {
			if (!(key in other)) problems.push(`${lang}: ${key} is missing`);
			else if (params(text) !== params(other[key]))
				problems.push(`${lang}: ${key} has other params`);
		}
		for (const key of Object.keys(other))
			if (!(key in en)) problems.push(`${lang}: ${key} isn't in en`);
	}
}

if (problems.length) {
	console.error(problems.join('\n'));
	process.exit(1);
}
console.log(`texts: ${langs.join(', ')} have all of en`);
