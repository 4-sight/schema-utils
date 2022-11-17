use schema_utils::Schema;
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

    let _: Object = t.into();
}
