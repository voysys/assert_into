#![no_std]

use core::{any::type_name, convert::TryFrom, fmt::Debug};

pub trait AssertInto<T>: Sized {
    fn assert_into(self) -> T;
}

impl<T, U> AssertInto<U> for T
where
    U: TryFrom<T>,
    T: Debug,
    T: Copy,
    <U as TryFrom<T>>::Error: Debug,
{
    #[inline]
    #[track_caller]
    fn assert_into(self) -> U {
        let v = U::try_from(self);

        match v {
            Ok(v) => v,
            Err(e) => {
                panic!(
                    "{:?} is out of range for type {}: {:?}",
                    self,
                    type_name::<U>(),
                    e
                );
            }
        }
    }
}
