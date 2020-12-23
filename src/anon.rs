use crate::hlist::{Cons, HList, Nil, Shuffle};

pub use core::marker::PhantomData;

pub use macros::Transform;

pub struct Named<T, Name: 'static>(pub T, PhantomData<Name>);
pub struct Unnamed<T>(pub T);

impl<T, Name: 'static> Named<T, Name> {
    pub const fn new(value: T) -> Self { Self(value, PhantomData) }
}

#[macro_export]
macro_rules! field {
    ($field_name:ident) => {
        $crate::macros::name! {$crate $field_name}
    };
}

#[macro_export]
macro_rules! Anon {
    (
        $($($field_name:ident: $field_ty:ty),+ $(, @$rest:ty)? $(,)?)?
    ) => {
        $crate::HList!($($($crate::anon::Named::<$field_ty, $crate::field!($field_name)>),* $(, @$rest)?)?)
    };
    (
        $($($field_ty:ty),+ $(, @$rest:ty)? $(,)?)?
    ) => {
        $crate::HList!($($($crate::anon::Unnamed::<$field_ty>),* $(, @$rest)?)?)
    };
}

#[macro_export]
macro_rules! anon {
    (
        $($($field_name:ident = $field_val:expr),+ $(,)?)?
    ) => {
        $crate::hlist!($($($crate::anon::Named::<_, $crate::field!($field_name)>::new($field_val)),*)?)
    };
    (
        $($($field_val:expr),+ $(,)?)?
    ) => {
        $crate::hlist!($($($crate::anon::Unnamed($field_val)),*)?)
    };
}

pub trait AnonType: HList {}
impl AnonType for Nil {}
impl<T, R: AnonType, Name> AnonType for Cons<Named<T, Name>, R> {}
impl<T, R: AnonType> AnonType for Cons<Unnamed<T>, R> {}

impl<T> Access for T {}
pub trait Access {
    fn field<N, T, Name: 'static>(&self) -> &T
    where
        Self: crate::hlist::Access<Named<T, Name>, N>,
    {
        &self.get().0
    }

    fn field_mut<N, T, Name: 'static>(&mut self) -> &mut T
    where
        Self: crate::hlist::Access<Named<T, Name>, N>,
    {
        &mut self.get_mut().0
    }

    fn take_field<N, T, Name>(self) -> (T, Self::Remainder)
    where
        Self: crate::hlist::Access<Named<T, Name>, N>,
    {
        let (field, rest) = self.take();
        (field.0, rest)
    }
}

pub trait Transform: Sized {
    type Canon: AnonType;

    fn from_canon(anon: Self::Canon) -> Self;

    fn into_canon(self) -> Self::Canon;

    fn transform<O: Transform, N>(self) -> O
    where
        Self::Canon: Shuffle<O::Canon, N>,
    {
        O::from_canon(self.into_canon().shuffle())
    }
}

impl<T: AnonType> IntoNamed for T {}
pub trait IntoNamed: Sized + AnonType {
    fn into_named<T, N>(self) -> T
    where
        Self: Shuffle<T::Canon, N>,
        T: Transform,
    {
        T::from_canon(self.shuffle())
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! character {
    ($($c:ident,)*) => {
        $crate::HList!($($crate::anon::character::$c),*)
    };
}

#[allow(non_camel_case_types)]
pub mod character {
    macro_rules! build_character {
        ($($c:ident)*) => {$(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub enum $c {}
        )*};
    }

    build_character! {
        a b c d e f g h i j k l m n o p q r s t u v w x y z
        A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
        _0 _1 _2 _3 _4 _5 _6 _7 _8 _9 __
    }
}
