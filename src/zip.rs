use crate::hlist::{Cons, Nil};

pub type Zipped<T, U> = <T as Zip<U>>::Output;
pub trait Zip<O> {
    type Output: UnZip<Left = Self, Right = O>;

    fn zip(self, other: O) -> Self::Output;
}

pub trait UnZip {
    type Left: Zip<Self::Right, Output = Self>;
    type Right: Zip<Self::Left, Output = Self::Flipped>;
    type Flipped: UnZip<Left = Self::Right, Right = Self::Left, Flipped = Self>;

    fn unzip(self) -> (Self::Left, Self::Right);
    fn flip(self) -> Self::Flipped;
}

impl Zip<Nil> for Nil {
    type Output = Self;

    fn zip(self, Nil: Self) -> Self::Output { Self }
}

impl UnZip for Nil {
    type Left = Self;
    type Right = Self;
    type Flipped = Self;

    fn unzip(self) -> (Self::Left, Self::Right) { (Self, Self) }

    fn flip(self) -> Self::Flipped { Self }
}

impl<T, U, R: Zip<S>, S> Zip<Cons<U, S>> for Cons<T, R> {
    type Output = Cons<(T, U), R::Output>;

    fn zip(self, other: Cons<U, S>) -> Self::Output {
        Cons {
            value: (self.value, other.value),
            rest: self.rest.zip(other.rest),
        }
    }
}

impl<T, U, R: UnZip> UnZip for Cons<(T, U), R> {
    type Left = Cons<T, R::Left>;
    type Right = Cons<U, R::Right>;
    type Flipped = Cons<(U, T), R::Flipped>;

    fn unzip(self) -> (Self::Left, Self::Right) {
        let (left, right) = self.rest.unzip();
        (
            Cons {
                value: self.value.0,
                rest: left,
            },
            Cons {
                value: self.value.1,
                rest: right,
            },
        )
    }

    fn flip(self) -> Self::Flipped {
        Cons {
            value: (self.value.1, self.value.0),
            rest: self.rest.flip(),
        }
    }
}
