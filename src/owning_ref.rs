use core::ops::Deref;

#[derive(Debug)]
pub enum OwningRef<'a, T: 'a> {
	Reference(&'a T),
	Object(T)
}

impl<'a, T> Deref for OwningRef<'a, T> {
	type Target = T;

	fn deref(&self) -> &<Self as Deref>::Target {
		match self {
			OwningRef::Reference(reference) => reference,
			OwningRef::Object(object) => &object,
		}
	}
}

impl<'a, T> From<T> for OwningRef<'a, T> {
	fn from(object: T) -> Self {
		OwningRef::Object(object)
	}
}

impl<'a, T> From<&'a T> for OwningRef<'a, T> {
	fn from(object: &'a T) -> Self {
		OwningRef::Reference(object)
	}
}
