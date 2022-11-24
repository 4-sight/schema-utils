use crate::TryFromI64;

pub trait TryIntoInteger<T> {
    type Error;
    fn try_into_integer(self) -> Result<T, Self::Error>;
}

impl<T> TryIntoInteger<T> for i64
where
    T: TryFromI64,
{
    type Error = <T as TryFromI64>::Error;
    fn try_into_integer(self) -> Result<T, Self::Error> {
        T::try_from_i64(self)
    }
}
