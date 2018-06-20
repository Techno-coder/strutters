use core::cell::RefCell;
use core::cell::UnsafeCell;
use core::fmt::Debug;
use core::ops::Deref;

/// Represents a closure that produces an updated node value from it and its delta
///
/// # Arguments
/// * The current node value
/// * The current node delta
/// * The current range length
pub trait DeltaSifter<T, D>: Fn(&T, &D, usize) -> T {}

impl<F, T, D> DeltaSifter<T, D> for F where F: ?Sized + Fn(&T, &D, usize) -> T {}

pub struct DeltaWrapper<T, D> {
	pub(crate) object: UnsafeCell<T>,
	pub(crate) delta: RefCell<Option<D>>,
}

impl<T, D> DeltaWrapper<T, D> {
	pub fn new(object: T) -> DeltaWrapper<T, D> {
		DeltaWrapper {
			object: UnsafeCell::new(object),
			delta: RefCell::new(None),
		}
	}
}

impl<T, D> Deref for DeltaWrapper<T, D> {
	type Target = T;

	fn deref(&self) -> &<Self as Deref>::Target {
		unsafe { self.object.get().as_ref() }.unwrap()
	}
}

impl<T, D> ::core::ops::DerefMut for DeltaWrapper<T, D> {
	fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
		unsafe { self.object.get().as_mut() }.unwrap()
	}
}

impl<T, D> Debug for DeltaWrapper<T, D> where T: Debug, D: Debug {
	fn fmt(&self, f: &mut ::core::fmt::Formatter) -> Result<(), ::core::fmt::Error> {
		writeln!(f, "DeltaWrapper {{")?;
		writeln!(f, "   object: {:?}", self.deref())?;
		writeln!(f, "   delta: {:?}", self.delta)?;
		writeln!(f, "}}")
	}
}