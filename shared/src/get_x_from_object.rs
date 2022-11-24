use super::{ParseInto, TryFromValue};
use crate::Error;
use surrealdb::sql::Object;

pub fn get_x_from_object<T>(obj: &Object, key: String) -> Result<T, Error>
where
    T: TryFromValue,
{
    match obj.0.get(&key).map(|v| v.clone().parse_into()) {
        None => std::result::Result::Err(Error::FieldNotFound(key)),
        Some(Ok(val)) => Ok(val),
        Some(Err(_)) => std::result::Result::Err(Error::InvalidField(key)),
    }
}
