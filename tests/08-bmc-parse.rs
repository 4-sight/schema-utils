mod utils;

use schema_utils::*;
use utils::*;

#[allow(dead_code)]
#[derive(Default, Schema, Updatable, Bmc)]
pub struct TestSchema {
	id: String,
	date: i64,
	notes: Vec<String>,
	option: Option<String>,
	is_true: bool,
}

#[allow(dead_code)]
#[derive(Default, Schema, Updatable, Bmc)]
#[entity(override_entity)]
pub struct TestSchema2 {
	title: String,
	name: String,
	age: i64,
}

#[test]
fn main() {}
