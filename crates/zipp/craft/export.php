<?php
// Exports the content of a Craft 5 site as JSON for `import-craft` of zipp:
// sites, entries of all sections (structures in their order), categories,
// global sets and assets, every field value in a plain shape. Craft runs it
// as a statement block, from the Craft project:
//
//   ddev craft exec "if (true) { $(sed 1d export.php) }"
//
// It writes storage/zipp-export.json. The values per field class:
// text, number, boolean as they are; dropdowns the value, checkboxes a list
// of values; tables rows by column handle; json as it is; entries,
// categories and assets a list of ids; links { type, id | value, label,
// target, download }; matrix a list of nested entries { id, type, title,
// enabled, fields }; content blocks { fields }; CKEditor { html, entries }
// with the raw html (`<craft-entry data-entry-id>`, `{entry:1@1:url}`);
// meierlabs-seo's field in zipp-seo's shape with asset ids; TableMaker
// { columns: [{ heading, width, align, type }], rows } with each row a list
// in the order of the columns. Assets have the fields of their volume.

$sites = \Craft::$app->getSites()->getAllSites();

$value = null;
$fieldsOf = function (\craft\base\ElementInterface $el) use (&$value): array {
	$out = [];
	foreach ($el->getFieldLayout()?->getCustomFields() ?? [] as $field) {
		$out[$field->handle] = $value($el, $field);
	}
	return $out;
};

$nested = function (\craft\elements\Entry $e) use ($fieldsOf): array {
	return [
		'id' => $e->id,
		'type' => $e->getType()->handle,
		'title' => $e->title,
		'enabled' => $e->enabled && $e->getEnabledForSite(),
		'fields' => $fieldsOf($e),
	];
};

$value = function (\craft\base\ElementInterface $el, \craft\base\FieldInterface $field) use ($fieldsOf, $nested) {
	$v = $el->getFieldValue($field->handle);
	$ids = fn($q) => $q instanceof \craft\elements\db\ElementQueryInterface
		? (clone $q)->status(null)->siteId($el->siteId)->ids()
		: array_map(fn($e) => $e->id, iterator_to_array($q));

	return match (true) {
		$v === null => null,
		$field instanceof \craft\fields\Matrix => array_map(
			$nested,
			(clone $v)->status(null)->siteId($el->siteId)->all(),
		),
		$field instanceof \craft\fields\ContentBlock => ['fields' => $fieldsOf($v)],
		$field instanceof \craft\ckeditor\Field => [
			'html' => $v->getRawContent(),
			'entries' => array_map(
				$nested,
				\craft\elements\Entry::find()->ownerId($el->id)->fieldId($field->id)
					->siteId($el->siteId)->status(null)->all(),
			),
		],
		$field instanceof \craft\fields\Link => [
			'type' => $v->getType(),
			'id' => $v->getElement()?->id,
			'value' => $v->getElement() ? null : $v->getValue(),
			'label' => $v->getLabel(true),
			'target' => $v->target,
			'download' => $v->download,
		],
		$field instanceof \craft\fields\BaseRelationField => $ids($v),
		// meierlabs-seo, the plugin may not be installed
		$v instanceof \soerenmeier\meierlabsseo\models\SeoFieldData => [
			'title' => $v->title,
			'description' => $v->description,
			'image' => array_filter([$v->imageId]),
			'socialTitle' => $v->socialTitle,
			'socialDescription' => $v->socialDescription,
			'socialImage' => array_filter([$v->socialImageId]),
			'robots' => $v->robots,
			'canonicalUrl' => $v->canonicalUrl,
		],
		// the columns are the entry's own, keyed by an id like col1
		get_class($field) === 'verbb\\tablemaker\\fields\\TableMakerField' => [
			'columns' => array_values(array_map(fn($c) => [
				'heading' => $c['heading'] ?? '',
				'width' => $c['width'] ?? '',
				'align' => $c['align'] ?? '',
				'type' => $c['type'] ?? 'singleline',
			], $v['columns'] ?? [])),
			'rows' => array_values(array_map(
				fn($row) => array_map(
					fn($key) => $row[$key] ?? null,
					array_keys($v['columns'] ?? []),
				),
				$v['rows'] ?? [],
			)),
		],
		$v instanceof \craft\fields\data\SingleOptionFieldData => $v->value,
		$v instanceof \craft\fields\data\MultiOptionsFieldData => array_map(
			fn($o) => $o->value,
			iterator_to_array($v),
		),
		$field instanceof \craft\fields\Table => array_map(
			function ($row) use ($field) {
				$out = [];
				foreach ($field->columns as $id => $col) {
					$out[$col['handle']] = $row[$col['handle']] ?? $row[$id] ?? null;
				}
				return $out;
			},
			$v,
		),
		$v instanceof \craft\fields\data\JsonData => $v->getValue(),
		$v instanceof \DateTime => $v->format(\DateTime::ATOM),
		is_scalar($v) || is_array($v) => $v,
		// SEOmatic: zipp's SEO plugin has its own settings, the import
		// leaves the field out
		get_class($field) === 'nystudio107\\seomatic\\fields\\SeoSettings' => null,
		default => throw new \Exception("{$field->handle}: can't export " . get_class($field)),
	};
};

$element = fn(\craft\base\ElementInterface $e) => [
	'id' => $e->id,
	'type' => $e instanceof \craft\elements\Entry ? $e->getType()->handle : null,
	'title' => $e->title,
	'slug' => $e->slug,
	'uri' => $e->uri,
	'enabled' => $e->enabled && $e->getEnabledForSite(),
	'parentId' => $e->getParent()?->id,
	'postDate' => $e instanceof \craft\elements\Entry ? $e->postDate?->format(\DateTime::ATOM) : null,
	'dateCreated' => $e->dateCreated->format(\DateTime::ATOM),
	'fields' => $fieldsOf($e),
];

$out = [
	'sites' => array_map(fn($s) => [
		'id' => $s->id,
		'handle' => $s->handle,
		'name' => $s->name,
		'language' => $s->language,
		'enabled' => $s->enabled,
	], $sites),
	'sections' => [],
	'categories' => [],
	'globals' => [],
	'assets' => [],
];

foreach (\Craft::$app->getEntries()->getAllSections() as $section) {
	foreach ($sites as $site) {
		// a structure comes in its order, parents before their children
		$entries = \craft\elements\Entry::find()->sectionId($section->id)
			->siteId($site->id)->status(null)->all();
		$out['sections'][$section->handle]['type'] = $section->type;
		$out['sections'][$section->handle]['sites'][$site->handle] = array_map($element, $entries);
	}
}

foreach (\Craft::$app->getCategories()->getAllGroups() as $group) {
	foreach ($sites as $site) {
		$cats = \craft\elements\Category::find()->groupId($group->id)
			->siteId($site->id)->status(null)->all();
		$out['categories'][$group->handle][$site->handle] = array_map($element, $cats);
	}
}

foreach ($sites as $site) {
	foreach (\Craft::$app->getGlobals()->getAllSets() as $set) {
		$set = \Craft::$app->getGlobals()->getSetByHandle($set->handle, $site->id);
		$out['globals'][$set->handle][$site->handle] = ['fields' => $fieldsOf($set)];
	}
}

// the alt text can be translated, per site
$alts = [];
foreach ($sites as $site) {
	foreach (\craft\elements\Asset::find()->siteId($site->id)->status(null)->all() as $a) {
		if ($a->alt) $alts[$a->id][$site->handle] = $a->alt;
	}
}

foreach (\craft\elements\Asset::find()->status(null)->all() as $a) {
	$focal = $a->getHasFocalPoint() ? $a->getFocalPoint() : null;
	$out['assets'][] = [
		'id' => $a->id,
		'volume' => $a->getVolume()->handle,
		'volumeName' => $a->getVolume()->name,
		'path' => $a->getPath(),
		'title' => $a->title,
		'alt' => $a->alt,
		'alts' => $alts[$a->id] ?? [],
		'focalPoint' => $focal ? [$focal['x'], $focal['y']] : null,
		'fields' => $fieldsOf($a),
	];
}

$path = \Craft::getAlias('@storage/zipp-export.json');
file_put_contents($path, json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_UNICODE | JSON_UNESCAPED_SLASHES));
echo "wrote $path\n";
