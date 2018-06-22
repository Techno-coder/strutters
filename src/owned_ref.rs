use core::ops::Deref;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum OwnedRef<'a, T: 'a> {
	Reference(&'a T),
	Concrete(T),
}

impl<'a, T> OwnedRef<'a, T> where T: Clone {
	pub fn concrete(self) -> T {
		match self {
			OwnedRef::Reference(reference) => T::clone(reference),
			OwnedRef::Concrete(object) => object,
		}
	}
}

impl<'a, T> Deref for OwnedRef<'a, T> {
	type Target = T;

	fn deref(&self) -> &<Self as Deref>::Target {
		match self {
			OwnedRef::Reference(reference) => reference,
			OwnedRef::Concrete(object) => &object,
		}
	}
}

impl<'a, T> From<T> for OwnedRef<'a, T> {
	fn from(object: T) -> Self {
		OwnedRef::Concrete(object)
	}
}

impl<'a, T> From<&'a T> for OwnedRef<'a, T> {
	fn from(object: &'a T) -> Self {
		OwnedRef::Reference(object)
	}
}

impl<'a, T> Clone for OwnedRef<'a, T> where T: Clone {
	fn clone(&self) -> Self {
		OwnedRef::Concrete(self.deref().clone())
	}
}
