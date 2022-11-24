use crate::{Error, TryFromI64, TryIntoInteger};
use surrealdb::sql::{Array, Object, Value};

pub trait TryFromValue
where
    Self: Sized,
{
    type Error;
    fn try_from_value(val: Value) -> Result<Self, Self::Error>;
}

impl TryFromValue for Object {
    type Error = Error;
    fn try_from_value(val: Value) -> Result<Self, Self::Error> {
        let Value::Object(obj) = val else {
            return Err(Error::ValueNotOfType("Object".into()));
        };
        Ok(obj)
    }
}

impl TryFromValue for Array {
    type Error = Error;
    fn try_from_value(val: Value) -> Result<Array, Error> {
        let Value::Array(arr) = val else {
            return Err(Error::ValueNotOfType("Array".into()));
        };
        Ok(arr)
    }
}

impl TryFromValue for i64 {
    type Error = Error;
    fn try_from_value(val: Value) -> Result<i64, Error> {
        let Value::Number(num) = val else {
            return Err(Error::ValueNotOfType("Number".into()));
        };
        Ok(num.as_int())
    }
}

impl TryFromValue for u64 {
    type Error = Error;
    fn try_from_value(val: Value) -> Result<u64, Error> {
        let Value::Number(num) = val else {
            return Err(Error::ValueNotOfType("Number".into()));
        };
        let int = num.as_int();
        let output = int
            .try_into()
            .map_err(|_| Error::ValueNotOfType("u64".into()))?;
        Ok(output)
    }
}

impl TryFromValue for u32 {
    type Error = Error;
    fn try_from_value(val: Value) -> Result<u32, Error> {
        let Value::Number(num) = val else {
            return Err(Error::ValueNotOfType("Number".into()));
        };
        let int = num.as_int();
        let output = int
            .try_into()
            .map_err(|_| Error::ValueNotOfType("u32".into()))?;
        Ok(output)
    }
}

impl TryFromValue for u16 {
    type Error = Error;
    fn try_from_value(val: Value) -> Result<u16, Error> {
        let Value::Number(num) = val else {
            return Err(Error::ValueNotOfType("Number".into()));
        };
        let int = num.as_int();
        let output = int
            .try_into()
            .map_err(|_| Error::ValueNotOfType("u16".into()))?;
        Ok(output)
    }
}

impl TryFromValue for u8 {
    type Error = Error;
    fn try_from_value(val: Value) -> Result<u8, Error> {
        let Value::Number(num) = val else {
            return Err(Error::ValueNotOfType("Number".into()));
        };
        let int = num.as_int();
        let output = int
            .try_into()
            .map_err(|_| Error::ValueNotOfType("u8".into()))?;
        Ok(output)
    }
}

impl TryFromValue for bool {
    type Error = Error;
    fn try_from_value(val: Value) -> Result<bool, Error> {
        match val {
            Value::False => Ok(false),
            Value::True => Ok(true),
            _ => Err(Error::ValueNotOfType("bool".into())),
        }
    }
}

impl TryFromValue for String {
    type Error = Error;
    fn try_from_value(val: Value) -> Result<String, Error> {
        match val {
            Value::Strand(strand) => Ok(strand.as_string()),
            Value::Thing(thing) => Ok(thing.to_string()),
            _ => Err(Error::ValueNotOfType("String".into())),
        }
    }
}

impl<I> TryFromValue for Option<I>
where
    I: TryFromI64,
{
    type Error = Error;
    fn try_from_value(val: Value) -> Result<Option<I>, Error> {
        match val {
            Value::None => Ok(None),
            Value::Number(obj) => {
                let int = obj.as_int();
                let out = int
                    .try_into_integer()
                    .map_err(|_| Error::ValueNotOfType("integer".into()))?;
                Ok(Some(out))
            }
            _ => Err(Error::ValueNotOfType("Option".into())),
        }
    }
}

impl TryFromValue for Option<String> {
    type Error = Error;
    fn try_from_value(val: Value) -> Result<Option<String>, Error> {
        match val {
            Value::None => Ok(None),
            Value::Strand(s) => Ok(Some(s.as_string())),
            _ => Err(Error::ValueNotOfType("Option".into())),
        }
    }
}

impl TryFromValue for Vec<String> {
    type Error = Error;
    fn try_from_value(val: Value) -> Result<Vec<String>, Error> {
        match val {
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
