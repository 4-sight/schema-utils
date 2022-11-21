use crate::Error;
use surrealdb::sql::{Array, Object, Value};

pub struct NewTypeWrapper<T>(pub T);

impl TryFrom<NewTypeWrapper<Value>> for Object {
    type Error = Error;
    fn try_from(val: NewTypeWrapper<Value>) -> Result<Object, Error> {
        match val.0 {
            Value::Object(obj) => Ok(obj),
            _ => Err(Error::ValueNotOfType("object".into())),
        }
    }
}

impl TryFrom<NewTypeWrapper<Value>> for Array {
    type Error = Error;
    fn try_from(val: NewTypeWrapper<Value>) -> Result<Array, Error> {
        match val.0 {
            Value::Array(obj) => Ok(obj),
            _ => Err(Error::ValueNotOfType("Array".into())),
        }
    }
}

impl TryFrom<NewTypeWrapper<Value>> for i64 {
    type Error = Error;
    fn try_from(val: NewTypeWrapper<Value>) -> Result<i64, Error> {
        match val.0 {
            Value::Number(obj) => Ok(obj.as_int()),
            _ => Err(Error::ValueNotOfType("i64".into())),
        }
    }
}

impl TryFrom<NewTypeWrapper<Value>> for bool {
    type Error = Error;
    fn try_from(val: NewTypeWrapper<Value>) -> Result<bool, Error> {
        match val.0 {
            Value::False => Ok(false),
            Value::True => Ok(true),
            _ => Err(Error::ValueNotOfType("bool".into())),
        }
    }
}

impl TryFrom<NewTypeWrapper<Value>> for String {
    type Error = Error;
    fn try_from(val: NewTypeWrapper<Value>) -> Result<String, Error> {
        match val.0 {
            Value::Strand(strand) => Ok(strand.as_string()),
            Value::Thing(thing) => Ok(thing.to_string()),
            _ => Err(Error::ValueNotOfType("String".into())),
        }
    }
}

impl TryFrom<NewTypeWrapper<Value>> for Option<i64> {
    type Error = Error;
    fn try_from(val: NewTypeWrapper<Value>) -> Result<Option<i64>, Error> {
        match val.0 {
            Value::None => Ok(None),
            Value::Number(obj) => Ok(Some(obj.as_int())),
            _ => Err(Error::ValueNotOfType("Option".into())),
        }
    }
}

impl TryFrom<NewTypeWrapper<Value>> for Option<String> {
    type Error = Error;
    fn try_from(val: NewTypeWrapper<Value>) -> Result<Option<String>, Error> {
        match val.0 {
            Value::None => Ok(None),
            Value::Strand(s) => Ok(Some(s.as_string())),
            _ => Err(Error::ValueNotOfType("Option".into())),
        }
    }
}

impl TryFrom<NewTypeWrapper<Value>> for Vec<String> {
    type Error = Error;
    fn try_from(val: NewTypeWrapper<Value>) -> Result<Vec<String>, Error> {
        match val.0 {
            Value::Array(a) => {
                let mut v = vec![];
                for val in a.0.into_iter() {
                    match val {
                        Value::Strand(s) => v.push(s.as_string()),
                        _ => return Err(Error::ValueNotOfType("string".into())),
                    }
                }

                Ok(v)
            }
            _ => Err(Error::ValueNotOfType("Option".into())),
        }
    }
}

impl From<NewTypeWrapper<String>> for Value {
    fn from(w: NewTypeWrapper<String>) -> Self {
        w.0.into()
    }
}

impl From<NewTypeWrapper<i64>> for Value {
    fn from(w: NewTypeWrapper<i64>) -> Self {
        w.0.into()
    }
}

impl From<NewTypeWrapper<u64>> for Value {
    fn from(w: NewTypeWrapper<u64>) -> Self {
        w.0.into()
    }
}

impl From<NewTypeWrapper<bool>> for Value {
    fn from(w: NewTypeWrapper<bool>) -> Self {
        w.0.into()
    }
}

impl<T> From<NewTypeWrapper<Vec<T>>> for Value
where
    T: Into<Value>,
{
    fn from(w: NewTypeWrapper<Vec<T>>) -> Self {
        w.0.into_iter()
            .map(|i| i.into())
            .collect::<Vec<Value>>()
            .into()
    }
}

impl<T> From<NewTypeWrapper<Option<T>>> for Value
where
    T: Into<Value>,
{
    fn from(w: NewTypeWrapper<Option<T>>) -> Self {
        match w.0 {
            None => Value::None,
            Some(val) => val.into(),
        }
    }
}
