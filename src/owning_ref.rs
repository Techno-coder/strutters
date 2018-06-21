use core::ops::Deref;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum OwningRef<'a, T: 'a> {
	Reference(&'a T),
	Concrete(T),
}

impl<'a, T> OwningRef<'a, T> where T: Clone {
	pub fn concrete(self) -> T {
		match self {
			OwningRef::Reference(reference) => T::clone(reference),
			OwningRef::Concrete(object) => object,
		}
	}
}

impl<'a, T> Deref for OwningRef<'a, T> {
	type Target = T;

	fn deref(&self) -> &<Self as Deref>::Target {
		match self {
			OwningRef::Reference(reference) => reference,
			OwningRef::Concrete(object) => &object,
		}
	}
}

impl<'a, T> From<T> for OwningRef<'a, T> {
	fn from(object: T) -> Self {
		OwningRef::Concrete(object)
	}
}

impl<'a, T> From<&'a T> for OwningRef<'a, T> {
	fn from(object: &'a T) -> Self {
		OwningRef::Reference(object)
	}
}

impl<'a, T> Clone for OwningRef<'a, T> where T: Clone {
	fn clone(&self) -> Self {
		OwningRef::Concrete(self.deref().clone())
	}
}
