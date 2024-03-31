use std::collections::BTreeMap;

use apollo_compiler::{
	ast::{
		Definition, DirectiveList, Document, FieldDefinition, Name,
		ObjectTypeDefinition, OperationType, SchemaDefinition,
		Type as ApolloType,
	},
	validation::Valid,
	ExecutableDocument, Node, NodeStr, Schema,
};

// 1. schema (files)

// api
// 2. types (output types)
// 3. graphQl Types

/*
events(id: "123", filters: { or: [{ id: { gt: "2" } }, { title: { contains: "title" } }] } ) {
	artists(order: "123") {
		name

		... on ArtistGroup {
			names
		}
	}
}


*/

#[derive(Debug, Clone)]
pub struct Property {
	name: String,
	arguments: Vec<Argument>,
	ty: Type,
}

#[derive(Debug, Clone)]
pub struct Argument {
	name: String,
	ty: Type,
}

#[derive(Debug, Clone)]
pub enum Type {
	Id,
	String,
	Int,
	Float,
	Boolean,
	Array(Vec<Type>),
	Object {
		type_name: String,
		fields: Vec<Property>,
		fragments: Vec<Property>,
	},
}

// https://docs.rs/apollo-compiler/1.0.0-beta.14/apollo_compiler/ast/struct.SchemaDefinition.html

const ID: Name = Name::new_unchecked(NodeStr::from_static(&"ID"));
const STRING: Name = Name::new_unchecked(NodeStr::from_static(&"String"));
const INT: Name = Name::new_unchecked(NodeStr::from_static(&"Int"));
const FLOAT: Name = Name::new_unchecked(NodeStr::from_static(&"Float"));
const BOOLEAN: Name = Name::new_unchecked(NodeStr::from_static(&"Boolean"));

fn type_to_definitions(ty: &Type, defs: &mut Vec<Definition>) -> ApolloType {
	match ty {
		Type::Id => ApolloType::Named(ID),
		Type::String => ApolloType::Named(STRING),
		Type::Int => ApolloType::Named(INT),
		Type::Float => ApolloType::Named(FLOAT),
		Type::Boolean => ApolloType::Named(BOOLEAN),
		Type::Object {
			type_name,
			fields,
			fragments,
		} => {
			let type_name = Name::new(type_name).unwrap();

			let fields = fields
				.iter()
				.map(|f| Node::new(property_to_definitions(f, defs)))
				.collect();

			// converts this property into a definition
			let def = ObjectTypeDefinition {
				description: None,
				name: type_name.clone(),
				implements_interfaces: vec![],
				directives: DirectiveList::new(),
				fields,
			};

			defs.push(Definition::ObjectTypeDefinition(Node::new(def)));

			ApolloType::Named(type_name)
		}

		_ => todo!(),
	}
}

fn property_to_definitions(
	prop: &Property,
	defs: &mut Vec<Definition>,
) -> FieldDefinition {
	let name = Name::new(&prop.name).unwrap();

	for arg in &prop.arguments {
		let name = Name::new(&arg.name).unwrap();

		let arg_ty = type_to_definitions(&arg.ty, defs);

		// defs.push(Definition::InputObjectTypeDefinition(Node::new(
		// 	InputObjectTypeDefinition {
		// 		description: None,
		// 		name: Name::new("User").unwrap(),
		// 		directives: DirectiveList::new(),
		// 		fields: args,
		// 	},
		// )));
	}

	FieldDefinition {
		description: None,
		name: name.clone(),
		arguments: vec![],
		ty: type_to_definitions(&prop.ty, defs),
		directives: DirectiveList::new(),
	}
}

fn create_schema(props: Vec<Property>) -> Valid<Schema> {
	let mut document = Document::new();
	let defs = &mut document.definitions;

	let mut query_fields: Vec<Node<FieldDefinition>> = vec![];

	for prop in &props {
		query_fields.push(Node::new(property_to_definitions(prop, defs)));
	}

	defs.push(Definition::ObjectTypeDefinition(Node::new(
		ObjectTypeDefinition {
			description: None,
			name: Name::new("Query").unwrap(),
			implements_interfaces: vec![],
			directives: DirectiveList::new(),
			fields: query_fields,
		},
	)));

	defs.push(Definition::SchemaDefinition(Node::new(SchemaDefinition {
		description: None,
		directives: DirectiveList::new(),
		root_operations: vec![Node::new((
			OperationType::Query,
			Name::new("Query").unwrap(),
		))],
	})));

	let mut schema = Schema::builder().add_ast(&document).build().unwrap();

	// let schema_input = r#"
	// type User {
	//   id: ID
	//   name: String
	//   profilePic(size: Int): URL
	// }

	// schema { query: User }

	// scalar URL @specifiedBy(url: "https://tools.ietf.org/html/rfc3986")
	// "#;
	// let schema =
	// 	Schema::parse_and_validate(schema_input, "schema.graphql").unwrap();

	Valid::assume_valid(schema)
}

#[cfg(test)]
mod tests {

	use apollo_compiler::executable;

	use super::*;

	#[test]
	fn test() {
		let query_input = r#"
    query getUser {
        user {
            ... vipCustomer
        }
    }

    #fragment definition where we want to know the field types.
    fragment vipCustomer on User {
      id
      name
      profilePic(size: 50) {
        url
      }
    }
    "#;

		let schema = create_schema(vec![Property {
			name: "user".to_string(),
			arguments: vec![],
			ty: Type::Object {
				type_name: "User".into(),
				fields: vec![
					Property {
						name: "id".to_string(),
						arguments: vec![],
						ty: Type::Id,
					},
					Property {
						name: "name".to_string(),
						arguments: vec![],
						ty: Type::String,
					},
					Property {
						name: "profilePic".to_string(),
						arguments: vec![Argument {
							name: "size".to_string(),
							ty: Type::Int,
						}],
						ty: Type::Object {
							type_name: "ProfilePic".into(),
							fields: vec![Property {
								name: "url".to_string(),
								arguments: vec![],
								ty: Type::String,
							}],
							fragments: vec![],
						},
					},
				],
				fragments: vec![],
			},
		}]);
		let document = ExecutableDocument::parse_and_validate(
			&schema,
			query_input,
			"query.graphql",
		)
		.unwrap();

		let op = document
			.get_operation(Some("getUser"))
			.expect("getUser query does not exist");
		let fragment_in_op = op
			.selection_set
			.selections
			.iter()
			.filter_map(|sel| match sel {
				executable::Selection::FragmentSpread(spread) => Some(
					document.fragments.get(&spread.fragment_name)?.as_ref(),
				),
				_ => None,
			})
			.collect::<Vec<&executable::Fragment>>();
		let fragment_fields = fragment_in_op
			.iter()
			.flat_map(|frag| frag.selection_set.fields())
			.collect::<Vec<&Node<executable::Field>>>();
		let field_ty = fragment_fields
			.iter()
			.map(|f| f.ty().inner_named_type().as_str())
			.collect::<Vec<&str>>();
		assert_eq!(field_ty, ["ID", "String", "URL"]);
	}
}
