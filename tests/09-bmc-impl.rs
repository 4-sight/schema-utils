mod utils;

use std::sync::Arc;

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
fn main() {
	let ctx = Arc::new(Ctx());

	let _ = TestSchemaBmc::get(ctx.clone(), "1234");
	let _ = TestSchemaBmc::create(ctx.clone(), TestSchema::default());
	let _ = TestSchemaBmc::delete(ctx.clone(), "1234");
	let mut update = TestSchemaUpdateInput::new();
	update.date(123456.into());
	let _ = TestSchemaBmc::update(ctx.clone(), "1234", update);
	let _ = TestSchemaBmc::select(ctx.clone(), None, None);

	let _ = TestSchema2Bmc::get(ctx.clone(), "1234");
	let _ = TestSchema2Bmc::create(ctx.clone(), TestSchema2::default());
	let _ = TestSchema2Bmc::delete(ctx.clone(), "1234");
	let mut update = TestSchema2UpdateInput::new();
	update.age(23);
	let _ = TestSchema2Bmc::update(ctx.clone(), "1234", update);
	let _ = TestSchema2Bmc::select(ctx, None, None);
}
