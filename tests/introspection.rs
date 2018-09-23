#[macro_use]
extern crate graphql_client;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/introspection/introspection_query.graphql",
    schema_path = "tests/introspection/introspection_schema.graphql",
    response_derives = "Debug,PartialEq"
)]
pub struct IntrospectionQuery;

#[test]
fn introspection_schema() {
    ()
}

const INTROSPECTION_RESPONSE: &'static str =
    include_str!("./introspection/introspection_response.json");

#[test]
fn leading_underscores_are_preserved() {
    let deserialized: graphql_client::Response<introspection_query::ResponseData> =
        serde_json::from_str(INTROSPECTION_RESPONSE).unwrap();
    assert!(deserialized.data.is_some());
    assert!(deserialized.data.unwrap().schema.is_some());
}
