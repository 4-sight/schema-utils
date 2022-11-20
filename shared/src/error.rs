use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("Invalid field :{0}")]
	InvalidField(String),

	#[error("Field not found :{0}")]
	FieldNotFound(String),

	#[error("Value not of type :{0}")]
	ValueNotOfType(String),
}
