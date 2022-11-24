use surrealdb::sql::Value;

pub trait IntoValue {
    fn into_value(self) -> Value;
}

impl IntoValue for String {
    fn into_value(self) -> Value {
        self.into()
    }
}

impl IntoValue for bool {
    fn into_value(self) -> Value {
        self.into()
    }
}

impl IntoValue for i64 {
    fn into_value(self) -> Value {
        self.into()
    }
}

impl IntoValue for u64 {
    fn into_value(self) -> Value {
        self.into()
    }
}

impl<T: Into<Value>> IntoValue for Vec<T> {
    fn into_value(self) -> Value {
        self.into_iter()
            .map(|i| i.into())
            .collect::<Vec<Value>>()
            .into()
    }
}

impl<T: Into<Value>> IntoValue for Option<T> {
    fn into_value(self) -> Value {
        match self {
            None => Value::None,
            Some(val) => val.into(),
        }
    }
}
