use crate::newtype_wrapper::NewTypeWrapper;
use crate::Error;
use surrealdb::sql::{Object, Value};

pub fn get_x_from_object<T>(obj: &Object, key: String) -> Result<T, Error>
where
    T: TryFrom<NewTypeWrapper<Value>>,
{
    match obj
        .0
        .get(&key)
        .map(|v| NewTypeWrapper(v.clone()).try_into())
    {
        None => std::result::Result::Err(Error::FieldNotFound(key)),
        Some(Ok(val)) => Ok(val),
        Some(Err(_)) => std::result::Result::Err(Error::InvalidField(key)),
    }
}
