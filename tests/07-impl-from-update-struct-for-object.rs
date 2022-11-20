use schema_utils::*;
use surrealdb::sql::Object;

#[allow(dead_code)]
#[derive(Updatable)]
pub struct TestSchema {
    id: String,
    date: i64,
    notes: Vec<String>,
    option: Option<String>,
    is_true: bool,
}

#[test]
fn main() {
    let update1 = TestSchemaUpdateInput::new();
    let mut update2 = TestSchemaUpdateInput::new();
    update2
        .id("1234".to_string())
        .date(2134)
        .notes(vec!["note".to_string()])
        .option(None)
        .is_true(true);
    let mut update3 = TestSchemaUpdateInput::new();
    update3.id("1234".to_string()).date(2134);

    let _: Object = update1.into();
    let _: Object = update2.into();
    let _: Object = update3.into();
}
