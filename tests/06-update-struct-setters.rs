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
    let new_id = "1234".to_string();
    let new_date = 2134_i64;
    let new_notes = vec![];
    let new_option = None;
    let new_is_true = false;

    let mut update = TestSchemaUpdateInput::new();
    update
        .id(new_id.clone())
        .date(new_date)
        .notes(new_notes.clone())
        .option(new_option.clone())
        .is_true(new_is_true);

    assert_eq!(update.id, Some(new_id));
    assert_eq!(update.date, Some(new_date));
    assert_eq!(update.notes, Some(new_notes));
    assert_eq!(update.option, Some(new_option));
    assert_eq!(update.is_true, Some(new_is_true));
}
