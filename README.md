# Zipp

Tries to be the best cms on the planet.

## Best, what does that mean?

Best is defined by the following terms:
- Performance, a cms should be fast to respond to requests and able to edit content with a blink of an eye
- flexible, fields and layouts should be easely changeable, some might even be changed on the flight, like a form builder
- localized, since the planet speaks a lot of different languages, the cms should support those
- extendable, a good cms, should be extenable, from new backend sites to field types and new datastructures
- diverse, the cms should be language and framework agnostic
- Secure, this cms should be able to be used safely by news outlets and goverment authorities
- Accessible, the cms should be accessable to any human with a computer (with a recent browser)
- Free, the cms in it's basic form should be open source and accesable to any body
- Maintanance, the cms should be easely deployed, and be able to auto update itself

## Target group

Who is the best cms for.
This cms should be able to be used from a small website for you're neighbour to a million views news outlet.
It might even be used for a online shop with a few thousand orthers per day, but let's not get ahead of ourself.

## What this requires

Since the web knows many technologies and framework the best cms should be able to be used by any of them.
Having a web api is crucial to achieve this goal.
I see two options, either a custom api or a graphql api, the graphql seems to be the better option.

To build a fast and secure cms the fundamentals need to be correct, there is no better language to facilitate this than rust.

## The architecture

The best cms might be organized in 4 different layers

### 1. Database
To achieve a loosely schemed database a new database wrapper might be required (maybe it could interface with postgres and cassandra)

### 2. Backend
This layer contains the graphql endpoint which allows editors and viewers to access their content and edit fields...

### 3. Admin UI
The be able to edit content not just for machines, a user interface is needed.

### 4. Frontend Framework
To delivery the best and fastest developer experience a new library and or frameworks needs to be built.
This layers needs to be optional as to our goals.
