## Goal

To achieve the most performant cms an optimized database is needed.

An administrator or developer defines a schema, this schema then get's propagated to this database.
The database is then obligated to update it's own layout to accommodate the new schema.

Read performance should be favored instead of write. Since in every case this cms is inteded reads will overweight writes.

The db provide an endpoint to define a schema of an element.

### Postgres

If we would create a dynamic schema using postgres.

Creating a table for each element seems not that much of a problem.
But i think when querying all elements or a few shared ones the performance might become a problem.

### Set Schema

// maybe instead of index we use filter?

Entry schema

```json
{
  "name": "entity",
  "fields": {
    "id": {
      "type": "id",
      "primary": true
    },
    "typeHandle": {
      "type": "text",
      "index": true,
      "not_null": true
    },
    // single, channel, what ever
    "type": {
      "type": "i8",
    },
    "order": {
      "type": "i64",
      "index": true
    }
  }
}

{
  "name": "entity_site",
  "fields": {
    "id": {
      "type": "id",
      "primary": true
    },
    "entryId": {
      "type": "id",
      "related": "entry.id",
      "index": true
    },
    "siteId": {
      "type": "id",
      "related": "site.id",
      "index": true
    },
    "state": {
      "type": "i8",
      "index": true
    },
    "updatedOn": {
      "type": "datetime",
      "index": true
    },
    "componentId": {
      "type": "componentId",
      "index": true
    }
  }
}
```

Component Schema

```json
{
  // name required to start with component
  "name": "component_event",
  "fields": {
    // field id required, as type id, and primary
    "id": {
      "type": "id",
      "primary": true
    },
    "eventDate": {
      "type": "datetime",
      "index": true
    },
    "artists": {
      "type": "component",
      "component": "component_artists",
      // should that be here?
      "amount": 10
    }
  }
},
{
  "name" : "component_record_label",
  "fields" : {
    "artists" : {
      "type": "component",
      "component": "component_artists",
    }
  }
}
```

Query get latest entry bySiteId
Query should probably be similar to graphQl

```json
{
  "schema": "entry",
  "fields": {

    "id": true,
    "typeHandle": true,
    "type": true,
    "site": {
      "id": true,
      "state": true,
      "updatedOn": true,
      "componentId": true
    }
  }
  "filter": {
    "type": "and",
    "values": [
      { "type": "eq", "key": "site.siteId", "value": "mySiteId" },
      { "type": "eq", "key": "site.state", "value": 5 }
    ]
  },
  "order": {
    "site.updatedOn": "desc"
  },
  "limit": 1
}
```

components query

```json
// get component by Id with fields

[
	"myField",
	"field2",
	{
		// this component is related to the parent via the
		// related component query
		"name": "myNestedField",
		"fields": ["otherField", "thirdField"]
	}
]

// how can the query be done??
```

db api

-   set Schema
-   delete Schema
-   set Component
-   delete Component

-   create Schema Data
-   read (with: filter) Schema Data
-   update Schema Data
-   delete Schema Data

-   create Component Data
-   read (wuth: filter) Component Data
-   update Component Data
-   delete Component Data
