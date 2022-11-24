use crate::Error;
use surrealdb::sql::{Array, Object, Value};

pub struct NewTypeWrapper<T>(pub T);

impl TryFrom<NewTypeWrapper<Value>> for Object {
    type Error = Error;
    fn try_from(val: NewTypeWrapper<Value>) -> Result<Object, Error> {
        let Value::Object(obj) = val.0 else {
            return Err(Error::ValueNotOfType("Object".into()));
        };
        Ok(obj)
    }
}

impl TryFrom<NewTypeWrapper<Value>> for Array {
    type Error = Error;
    fn try_from(val: NewTypeWrapper<Value>) -> Result<Array, Error> {
        let Value::Array(arr) = val.0 else {
            return Err(Error::ValueNotOfType("Array".into()));
        };
        Ok(arr)
    }
}

impl TryFrom<NewTypeWrapper<Value>> for i64 {
    type Error = Error;
    fn try_from(val: NewTypeWrapper<Value>) -> Result<i64, Error> {
        let Value::Number(num) = val.0 else {
            return Err(Error::ValueNotOfType("Number".into()));
        };
        Ok(num.as_int())
    }
}

impl TryFrom<NewTypeWrapper<Value>> for u64 {
    type Error = Error;
    fn try_from(val: NewTypeWrapper<Value>) -> Result<u64, Error> {
        let Value::Number(num) = val.0 else {
            return Err(Error::ValueNotOfType("Number".into()));
        };
        let int = num.as_int();
        let output = int
            .try_into()
            .map_err(|_| Error::ValueNotOfType("u64".into()))?;
        Ok(output)
    }
}

impl TryFrom<NewTypeWrapper<Value>> for u32 {
    type Error = Error;
    fn try_from(val: NewTypeWrapper<Value>) -> Result<u32, Error> {
        let Value::Number(num) = val.0 else {
            return Err(Error::ValueNotOfType("Number".into()));
        };
        let int = num.as_int();
        let output = int
            .try_into()
            .map_err(|_| Error::ValueNotOfType("u32".into()))?;
        Ok(output)
    }
}

impl TryFrom<NewTypeWrapper<Value>> for u16 {
    type Error = Error;
    fn try_from(val: NewTypeWrapper<Value>) -> Result<u16, Error> {
        let Value::Number(num) = val.0 else {
            return Err(Error::ValueNotOfType("Number".into()));
        };
        let int = num.as_int();
        let output = int
            .try_into()
            .map_err(|_| Error::ValueNotOfType("u16".into()))?;
        Ok(output)
    }
}

impl TryFrom<NewTypeWrapper<Value>> for u8 {
    type Error = Error;
    fn try_from(val: NewTypeWrapper<Value>) -> Result<u8, Error> {
        let Value::Number(num) = val.0 else {
            return Err(Error::ValueNotOfType("Number".into()));
        };
        let int = num.as_int();
        let output = int
            .try_into()
            .map_err(|_| Error::ValueNotOfType("u8".into()))?;
        Ok(output)
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

impl TryFrom<NewTypeWrapper<Value>> for Option<u8> {
    type Error = Error;
    fn try_from(val: NewTypeWrapper<Value>) -> Result<Option<u8>, Error> {
        match val.0 {
            Value::None => Ok(None),
            Value::Number(obj) => {
                let int = obj.as_int();
                let out = int
                    .try_into()
                    .map_err(|_| Error::ValueNotOfType("u8".into()))?;
                Ok(Some(out))
            }
            _ => Err(Error::ValueNotOfType("Option".into())),
        }
    }
}

impl TryFrom<NewTypeWrapper<Value>> for Option<u16> {
    type Error = Error;
    fn try_from(val: NewTypeWrapper<Value>) -> Result<Option<u16>, Error> {
        match val.0 {
            Value::None => Ok(None),
            Value::Number(obj) => {
                let int = obj.as_int();
                let out = int
                    .try_into()
                    .map_err(|_| Error::ValueNotOfType("u16".into()))?;
                Ok(Some(out))
            }
            _ => Err(Error::ValueNotOfType("Option".into())),
        }
    }
}

impl TryFrom<NewTypeWrapper<Value>> for Option<u32> {
    type Error = Error;
    fn try_from(val: NewTypeWrapper<Value>) -> Result<Option<u32>, Error> {
        match val.0 {
            Value::None => Ok(None),
            Value::Number(obj) => {
                let int = obj.as_int();
                let out = int
                    .try_into()
                    .map_err(|_| Error::ValueNotOfType("u32".into()))?;
                Ok(Some(out))
            }
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

//===========================================================

pub trait IntoValue {
    fn into_value(self) -> Value;
}

impl IntoValue for String {
    fn into_value(self) -> Value {
        self.into()
    }
}
