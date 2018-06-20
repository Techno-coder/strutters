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

#[cfg(feature = "std")]
use std::vec::Vec;

pub use self::owning_ref::OwningRef;

pub mod tree;
pub mod owning_ref;
