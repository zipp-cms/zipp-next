# The example site in crelte

crelte's default template, pointed at the example cms instead of Craft.
Templates are `<sectionHandle>-<typeHandle>`, blocks live in
`components/content` and the events block loads its list with its own
`loadData`.

```sh
cp craft/.env.example craft/.env
cd ../cms && cargo run -- serve --site-url de=http://localhost:8080 \
	--site-url en=http://localhost:8080/en --public-url http://127.0.0.1:8090
cd ../crelte/svelte && npm install && npm run dev
```
