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
fn main() {
    let t = TestSchema {
        id: "1234".into(),
        date: 123456,
        notes: vec!["test".into()],
        option: None,
        is_true: false,
    };

    let o: Object = t.into();

    let _: TestSchema = o.try_into().unwrap();
}
