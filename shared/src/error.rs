#[derive(Debug)]
pub enum Error {
    InvalidField(String),
    FieldNotFound(String),
    ValueNotOfType(String),
}
