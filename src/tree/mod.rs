pub use self::implicit_tree::ImplicitTree;
pub use self::segment_tree::SegmentTree;
pub use self::tree::BackingTree;

pub mod implicit_tree;
pub mod segment_tree;
pub mod tree;
pub mod functions;

pub const BINARY_WIDTH: usize = 2;

pub trait DefaultGenerator<T>: FnMut() -> T {}

impl<F, T> DefaultGenerator<T> for F where F: ?Sized + FnMut() -> T {}

pub trait AssociativeOperator<T>: Fn(&T, &T) -> T {}

impl<F, T> AssociativeOperator<T> for F where F: ?Sized + Fn(&T, &T) -> T {}

pub fn summation<T>() -> impl AssociativeOperator<T> where T: ::core::ops::Add<Output=T> + Clone {
	|a: &T, b: &T| a.clone() + b.clone()
}
