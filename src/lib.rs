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
pub use self::provider::Provider;
#[cfg(feature = "std")]
use std::collections as collections;
#[cfg(feature = "std")]
use std::vec::Vec;

pub mod tree;
pub mod graph;
pub mod provider;
pub mod owned_ref;
