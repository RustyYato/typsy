use core::{convert::Infallible, marker::PhantomData};

use crate::{hlist::Cons, peano};

pub trait CallOnce<Args, Tag = ()> {
    type Output;

    fn call_once(self, args: Args) -> Self::Output;
}

pub trait CallMut<Args, Tag = ()>: CallOnce<Args, Tag> {
    fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait Call<Args, Tag = ()>: CallMut<Args, Tag> {
    fn call(&self, args: Args) -> Self::Output;
}

//
// Forwarding implementations
//

impl<C: ?Sized + CallMut<Args, Tag>, Args, Tag> CallOnce<Args, Tag> for &mut C {
    type Output = C::Output;

    fn call_once(self, args: Args) -> Self::Output { C::call_mut(self, args) }
}

impl<C: ?Sized + CallMut<Args, Tag>, Args, Tag> CallMut<Args, Tag> for &mut C {
    fn call_mut(&mut self, args: Args) -> Self::Output { C::call_mut(self, args) }
}

impl<C: ?Sized + Call<Args, Tag>, Args, Tag> CallOnce<Args, Tag> for &C {
    type Output = C::Output;

    fn call_once(self, args: Args) -> Self::Output { C::call(self, args) }
}

impl<C: ?Sized + Call<Args, Tag>, Args, Tag> CallMut<Args, Tag> for &C {
    fn call_mut(&mut self, args: Args) -> Self::Output { C::call(self, args) }
}

impl<C: ?Sized + Call<Args, Tag>, Args, Tag> Call<Args, Tag> for &C {
    fn call(&self, args: Args) -> Self::Output { C::call(self, args) }
}

//
// Ad hoc callable types (using an hlist of closures)
//

impl<Args, F: FnOnce(Args) -> O, O, R> CallOnce<Args, peano::Zero> for Cons<F, R> {
    type Output = O;

    fn call_once(self, args: Args) -> Self::Output { (self.value)(args) }
}

impl<Args, N, F, R: CallOnce<Args, N>> CallOnce<Args, peano::Succ<N>> for Cons<F, R> {
    type Output = R::Output;

    #[inline(always)]
    fn call_once(self, args: Args) -> Self::Output { self.rest.call_once(args) }
}

impl<Args, F: FnMut(Args) -> O, O, R> CallMut<Args, peano::Zero> for Cons<F, R> {
    fn call_mut(&mut self, args: Args) -> Self::Output { (self.value)(args) }
}

impl<Args, N, F, R: CallMut<Args, N>> CallMut<Args, peano::Succ<N>> for Cons<F, R> {
    #[inline(always)]
    fn call_mut(&mut self, args: Args) -> Self::Output { self.rest.call_mut(args) }
}

impl<Args, F: Fn(Args) -> O, O, R> Call<Args, peano::Zero> for Cons<F, R> {
    fn call(&self, args: Args) -> Self::Output { (self.value)(args) }
}

impl<Args, N, F, R: Call<Args, N>> Call<Args, peano::Succ<N>> for Cons<F, R> {
    #[inline(always)]
    fn call(&self, args: Args) -> Self::Output { self.rest.call(args) }
}

//
// combinators - baked tag
//

impl<T> Bake for T {}
pub trait Bake: Sized {
    fn bake<TagList>(self) -> Baked<Self, TagList> { Baked::bake(self) }
}

pub struct Simple<F>(pub F);

impl<Args, F: CallOnce<Args>> CallOnce<Args> for Simple<F> {
    type Output = F::Output;

    fn call_once(self, args: Args) -> Self::Output { self.0.call_once(args) }
}

impl<Args, F: CallMut<Args>> CallMut<Args> for Simple<F> {
    fn call_mut(&mut self, args: Args) -> Self::Output { self.0.call_mut(args) }
}

impl<Args, F: Call<Args>> Call<Args> for Simple<F> {
    fn call(&self, args: Args) -> Self::Output { self.0.call(args) }
}

pub struct Baked<F, TagList>(F, PhantomData<TagList>);

impl<F, TagList> Baked<F, TagList> {
    pub const fn bake(f: F) -> Self { Self(f, PhantomData) }

    pub fn into_inner(self) -> F {
        let Self(f, PhantomData) = self;
        f
    }

    pub fn rebake<T>(self) -> Baked<F, T> { Baked::bake(self.into_inner()) }
}

impl<Args, Tag, F: CallOnce<Args, Tag>, TagList> CallOnce<Args, Tag> for Baked<F, TagList> {
    type Output = F::Output;

    fn call_once(self, args: Args) -> Self::Output { self.0.call_once(args) }
}

impl<Args, Tag, F: CallMut<Args, Tag>, TagList> CallMut<Args, Tag> for Baked<F, TagList> {
    fn call_mut(&mut self, args: Args) -> Self::Output { self.0.call_mut(args) }
}

impl<Args, Tag, F: Call<Args, Tag>, TagList> Call<Args, Tag> for Baked<F, TagList> {
    fn call(&self, args: Args) -> Self::Output { self.0.call(args) }
}

//
// combinators - not
//

pub struct Not<F>(pub F);
impl<Args, Tag, F: CallOnce<Args, Tag, Output = bool>> CallOnce<Args, Tag> for Not<F> {
    type Output = bool;

    fn call_once(self, args: Args) -> Self::Output { !self.0.call_once(args) }
}

impl<Args, Tag, F: CallMut<Args, Tag, Output = bool>> CallMut<Args, Tag> for Not<F> {
    fn call_mut(&mut self, args: Args) -> Self::Output { !self.0.call_mut(args) }
}

impl<Args, Tag, F: Call<Args, Tag, Output = bool>> Call<Args, Tag> for Not<F> {
    fn call(&self, args: Args) -> Self::Output { !self.0.call(args) }
}

//
// combinators - always Ok
//

pub struct AlwaysOk<F>(pub F);
impl<Args, Tag, F: CallOnce<Args, Tag>> CallOnce<Args, Tag> for AlwaysOk<F> {
    type Output = Result<F::Output, Infallible>;

    fn call_once(self, args: Args) -> Self::Output { Ok(self.0.call_once(args)) }
}

impl<Args, Tag, F: CallMut<Args, Tag>> CallMut<Args, Tag> for AlwaysOk<F> {
    fn call_mut(&mut self, args: Args) -> Self::Output { Ok(self.0.call_mut(args)) }
}

impl<Args, Tag, F: Call<Args, Tag>> Call<Args, Tag> for AlwaysOk<F> {
    fn call(&self, args: Args) -> Self::Output { Ok(self.0.call(args)) }
}

#[doc(hidden)]
#[macro_export]
macro_rules! return_type {
    () => {
        ()
    };
    ($output:ty) => {
        $output
    };
}

#[macro_export]
macro_rules! call {
    () => {};
    (
        fn $( [ $($generics:tt)* ] )? ( $self:ident : $Self:ty $( $(, $args:ident : $args_ty:ty)+ $(,)?)? ) $(-> $output:ty)?
        $(where ($($where_clause:tt)*))?
        {
            $($body:tt)*
        }

        $($rest:tt)*
    ) => {
        #[allow(unused_parens)]
        impl $(<$($generics)*>)? $crate::call::CallOnce<($($($args_ty,)*)?)> for $Self
        $(where $($where_clause)*)?
        {
            type Output = $crate::return_type!($($output)?);

            fn call_once($self, ($($($args,)*)?): ($($($args_ty,)*)?)) -> Self::Output {
                #[warn(unused_parens)]
                {
                    $($body)*
                }
            }
        }

        $crate::call!{$($rest)*}
    };
    (
        fn $( [ $($generics:tt)* ] )? (&mut $self:ident : $Self:ty $( $(, $args:ident : $args_ty:ty)+ $(,)?)? ) $(-> $output:ty)?
        $(where ($($where_clause:tt)*))?
        {
            $($body:tt)*
        }

        $($rest:tt)*
    ) => {
        #[allow(unused_parens)]
        impl $(<$($generics)*>)? $crate::call::CallOnce<($($($args_ty,)*)?)> for $Self
        $(where $($where_clause)*)?
        {
            type Output = $crate::return_type!($($output)?);

            fn call_once(mut self, args: ($($($args_ty,)*)?)) -> Self::Output {
                $crate::call::CallMut::<($($($args_ty,)*)?)>::call_mut(&mut self, args)
            }
        }
        #[allow(unused_parens)]
        impl $(<$($generics)*>)? $crate::call::CallMut<($($($args_ty,)*)?)> for $Self
        $(where $($where_clause)*)?
        {
            fn call_mut(&mut $self, ($($($args,)*)?): ($($($args_ty,)*)?)) -> Self::Output {
                #[warn(unused_parens)]
                {
                    $($body)*
                }
            }
        }

        $crate::call!{$($rest)*}
    };
    (
        fn $( [ $($generics:tt)* ] )? (&$self:ident : $Self:ty $( $(, $args:ident : $args_ty:ty)+ $(,)?)? ) $(-> $output:ty)?
        $(where ($($where_clause:tt)*))?
        {
            $($body:tt)*
        }

        $($rest:tt)*
    ) => {
        #[allow(unused_parens)]
        impl $(<$($generics)*>)? $crate::call::CallOnce<($($($args_ty,)*)?)> for $Self
        $(where $($where_clause)*)?
        {
            type Output = $crate::return_type!($($output)?);

            fn call_once(self, args: ($($($args_ty,)*)?)) -> Self::Output {
                $crate::call::Call::<($($($args_ty,)*)?)>::call(&self, args)
            }
        }
        #[allow(unused_parens)]
        impl $(<$($generics)*>)? $crate::call::CallMut<($($($args_ty,)*)?)> for $Self
        $(where $($where_clause)*)?
        {
            fn call_mut(&mut self, args: ($($($args_ty,)*)?)) -> Self::Output {
                $crate::call::Call::<($($($args_ty,)*)?)>::call(&*self, args)
            }
        }
        #[allow(unused_parens)]
        impl $(<$($generics)*>)? $crate::call::Call<($($($args_ty,)*)?)> for $Self
        $(where $($where_clause)*)?
        {
            fn call(&$self, ($($($args,)*)?): ($($($args_ty,)*)?)) -> Self::Output {
                #[warn(unused_parens)]
                {
                    $($body)*
                }
            }
        }

        $crate::call!{$($rest)*}
    };
}
