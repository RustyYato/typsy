use crate::hlist::{Cons, HList, Nil, Shuffle};

pub struct Field<T, const NAME: &'static str>(pub T);

#[macro_export]
macro_rules! Anon {
    (
        $($($field_name:ident: $field_ty:ty),+ $(, @$rest:ty)? $(,)?)?
    ) => {
        $crate::HList!($($($crate::anon::Field::<$field_ty, {stringify!($field_name)}>),* $(, @$rest)?)?)
    };
}

#[macro_export]
macro_rules! anon {
    (
        $($($field_name:ident = $field_val:expr),+ $(,)?)?
    ) => {
        $crate::hlist!($($($crate::anon::Field::<_, {stringify!($field_name)}>($field_val)),*)?)
    };
}

pub trait AnonType: HList {}
impl AnonType for Nil {}
impl<T, R: AnonType, const NAME: &'static str> AnonType for Cons<Field<T, NAME>, R> {}

impl<T> Access for T {}
pub trait Access {
    fn field<N, T, const NAME: &'static str>(&self) -> &T
    where
        Self: crate::hlist::Access<Field<T, NAME>, N>,
    {
        &self.get().0
    }

    fn field_mut<N, T, const NAME: &'static str>(&mut self) -> &mut T
    where
        Self: crate::hlist::Access<Field<T, NAME>, N>,
    {
        &mut self.get_mut().0
    }

    fn take_field<N, T, const NAME: &'static str>(self) -> (T, Self::Remainder)
    where
        Self: crate::hlist::Access<Field<T, NAME>, N>,
    {
        let (field, rest) = self.take();
        (field.0, rest)
    }
}

pub trait FromAnon<T: AnonType> {
    fn from_anon(anon: T) -> Self;
}

pub trait IntoAnon<T: AnonType> {
    fn into_anon<U: AnonType, N>(self) -> U
    where
        T: Shuffle<U, N>;
}

impl<T: AnonType> IntoNamed for T {}
pub trait IntoNamed: Sized + AnonType {
    fn into_named<T, This: AnonType, N>(self) -> T
    where
        Self: Shuffle<This, N>,
        T: FromAnon<This>,
    {
        T::from_anon(self.shuffle())
    }
}
