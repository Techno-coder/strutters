#![no_std]
#![cfg_attr(not(feature = "std"), feature(alloc))]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc as collections;
#[cfg(feature = "std")]
#[macro_use]
extern crate std;

#[cfg(not(feature = "std"))]
use collections::Vec;
pub use self::owned_ref::OwnedRef;
#[cfg(feature = "std")]
use std::collections as collections;
#[cfg(feature = "std")]
use std::vec::Vec;

pub mod operator;
pub mod query;
pub mod tree;
pub mod graph;
pub mod provider;
mod owned_ref;

pub trait FixedDataSource<T>: DoubleEndedIterator<Item=T> + ExactSizeIterator<Item=T> {}

impl<T, I> FixedDataSource<T> for I where I: DoubleEndedIterator<Item=T> + ExactSizeIterator<Item=T> {}
