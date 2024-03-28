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





### Schema example request


```
Example schema request
{
  name: "entries"
  fields: [
    {
      name: "postDate",
      type: "DateTime",
      index: "hard"
    },
    {
      name: "title",
      type: "text"
    },
    {
      name: "slug",
      type: "text",
      index: "unique"
  ]
}


entry type request
{
  name: "event",
  parent: "entries",
  fields: [
    {
      name: "posterImage",
      type: "UniqueId",
    }
  ]
}



components (matrix)
{
  name: "components",
  fields: [
    {
      name: "type",
      type: "text",
      index: "soft"
    }
  ]
}


image component
{
  name: "image",
  parent: "components",
  fields: [
    {
      name: "asset",
      type: "UniqueId"
    }
  ]
}

```

### Query
components and entries can be queried, as well as like components(type: "")
