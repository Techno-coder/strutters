pub use self::delta::DeltaSifter;
pub use self::delta::DeltaWrapper;
pub use self::implicit_tree::ImplicitTree;
pub use self::lazy_segment_tree::LazySegmentTree;
pub use self::segment_tree::SegmentTree;
pub use self::tree::BackingTree;

pub mod implicit_tree;
pub mod segment_tree;
pub mod lazy_segment_tree;
pub mod tree;
pub mod functions;
pub mod delta;

pub const BINARY_WIDTH: usize = 2;

pub trait FixedDataSource<T>: DoubleEndedIterator<Item=T> + ExactSizeIterator<Item=T> {}

impl<T, I> FixedDataSource<T> for I where I: DoubleEndedIterator<Item=T> + ExactSizeIterator<Item=T> {}

pub trait AssociativeOperator<T>: Fn(&T, &T) -> T {}

impl<F, T> AssociativeOperator<T> for F where F: ?Sized + Fn(&T, &T) -> T {}

pub fn summation<T>() -> impl AssociativeOperator<T> where T: ::core::ops::Add<Output=T> + Clone {
	|a: &T, b: &T| a.clone() + b.clone()
}

pub fn minimum<T>() -> impl AssociativeOperator<T> where T: Ord + Clone {
	|a: &T, b: &T| a.clone().min(b.clone())
}

pub fn maximum<T>() -> impl AssociativeOperator<T> where T: Ord + Clone {
	|a: &T, b: &T| a.clone().max(b.clone())
}
