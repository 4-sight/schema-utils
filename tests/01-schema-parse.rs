use schema_utils::*;
#[allow(unused_imports)]
use surrealdb::sql::{Object, Value};

#[derive(Schema)]
pub struct TestSchema {
    id: String,
    date: i64,
    notes: Vec<String>,
    option: Option<String>,
    is_true: bool,
}

#[test]
fn main() {}
