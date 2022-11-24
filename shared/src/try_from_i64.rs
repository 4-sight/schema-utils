use crate::Error;

pub trait TryFromI64
where
    Self: Sized,
{
    type Error;
    fn try_from_i64(n: i64) -> Result<Self, Self::Error>;
}

macro_rules! impl_try_from_i64 {
    ($target_type:ty, $type_string: expr) => {
        impl TryFromI64 for $target_type {
            type Error = Error;
            fn try_from_i64(n: i64) -> Result<Self, Self::Error> {
                n.try_into()
                    .map_err(|_| Error::ValueNotOfType($type_string.to_string()))
            }
        }
    };
}

impl_try_from_i64!(u8, "u8");
impl_try_from_i64!(u16, "u16");
impl_try_from_i64!(u32, "u32");
impl_try_from_i64!(u64, "u64");
impl_try_from_i64!(i64, "i64");
