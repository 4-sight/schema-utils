use schema_utils::*;

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
    let _ = TestSchemaUpdateInput::default();
    let update = TestSchemaUpdateInput::new();

    assert_eq!(update.id, None);
    assert_eq!(update.date, None);
    assert_eq!(update.notes, None);
    assert_eq!(update.option, None);
    assert_eq!(update.is_true, None);
}
