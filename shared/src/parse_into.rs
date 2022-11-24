use crate::TryFromValue;
use surrealdb::sql::Value;

pub trait ParseInto<T>
where
    T: TryFromValue,
{
    type Error;
    fn parse_into(self) -> Result<T, Self::Error>;
}

impl<T> ParseInto<T> for Value
where
    T: TryFromValue,
{
    type Error = <T as TryFromValue>::Error;
    fn parse_into(self) -> Result<T, Self::Error> {
        T::try_from_value(self)
    }
}
