use std::vec::Vec;

use super::{Named, Unnamed};
use crate::hlist::*;

pub trait DeepTransformFrom<T, I> {
    fn deep_transform_from(value: T) -> Self;
}

pub trait DeepTransform<T, I> {
    fn deep_transform(self) -> T;
}

impl<T: DeepTransformFrom<U, I>, U, I> DeepTransform<T, I> for U {
    fn deep_transform(self) -> T { T::deep_transform_from(self) }
}

macro_rules! primitive {
    ($($primitive:ty),* $(,)?) => {
        $(
            impl DeepTransformFrom<$primitive, ()> for $primitive {
                fn deep_transform_from(prim: $primitive) -> Self { prim }
            }
        )*
    };
}

primitive! { (), u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64, bool, char }

impl<T, U: DeepTransformFrom<T, I>, I> DeepTransformFrom<Vec<T>, I> for Vec<U> {
    fn deep_transform_from(value: Vec<T>) -> Self { value.into_iter().map(U::deep_transform_from).collect() }
}

impl DeepTransformFrom<Nil, ()> for Nil {
    fn deep_transform_from(Nil: Self) -> Self { Self }
}

impl<T, R> DeepTransformFrom<Cons<T, R>, ()> for Nil {
    fn deep_transform_from(_: Cons<T, R>) -> Self { Self }
}

impl<T, Rt, L, I, Ti, Ri, Name> DeepTransformFrom<L, (I, Ti, Ri)> for Cons<Named<T, Name>, Rt>
where
    L: super::RemoveField<Name, I>,
    L::Value: DeepTransform<T, Ti>,
    L::Remainder: DeepTransform<Rt, Ri>,
{
    fn deep_transform_from(list: L) -> Self {
        let (value, rest) = list.remove_field();
        Self {
            value: Named::new(value.deep_transform()),
            rest: rest.deep_transform(),
        }
    }
}

impl<T, Rt, L: DeepTransform<Rt, Ri>, I, Ri> DeepTransformFrom<Cons<Unnamed<T>, L>, (I, Ri)> for Cons<Unnamed<T>, Rt> {
    fn deep_transform_from(list: Cons<Unnamed<T>, L>) -> Self {
        Self {
            value: list.value,
            rest: list.rest.deep_transform(),
        }
    }
}

impl<T, U: DeepTransform<T, I>, I, N> DeepTransformFrom<Named<U, N>, I> for Named<T, N> {
    fn deep_transform_from(value: Named<U, N>) -> Self {
        let value = value.0;
        let value = value.deep_transform();
        Self::new(value)
    }
}
