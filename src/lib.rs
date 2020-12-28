#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(feature = "extreme_tuples", recursion_limit = "256")]
#![cfg_attr(feature = "nightly", feature(unsized_local))]

#[cfg(feature = "alloc")]
extern crate alloc as std;

#[doc(hidden)]
pub use {core, macros};

#[cfg(feature = "macros")]
pub mod anon;
pub mod call;
pub mod coprod;
pub mod hlist;
pub mod peano;

pub mod as_ref;
pub mod cmp;
pub mod fold;
pub mod map;
pub mod zip;

pub mod convert;

use seal::Seal;
mod seal {
    pub trait Seal {}
}
