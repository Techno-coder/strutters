//! The `Provider` trait abstracts over different methods of providing values.
//!
//! Due to Rust's coherence requirement, `Provider` cannot normally be implemented for
//! both cloneable types and closures that return the required type. However, we can use
//! a "hack" to allow it. `Provider` is implemented for **references** to types that
//! implement Clone and is implemented for **mutable references** to closures.
//!
//! If these are not sufficient or you want to pass the provider by value instead of by
//! reference, `ClosureProvider` and `ObjectProvider` hold the value they are given.
//! Additionally, you can implement the trait manually for any given type.
//!
//! # Example
//!
//! ```rust
//! use strutters::provider::Provider;
//! use strutters::provider::ClosureProvider;
//! use strutters::provider::ObjectProvider;
//!
//! fn use_provider<P>(provider: P) where P: Provider<u32> {}
//!
//! use_provider(&1337);
//! use_provider(&mut || 1337);
//! use_provider(ClosureProvider::from(|| 1337));
//! use_provider(ObjectProvider::from(1337));
//! ```

pub trait Provider<T> {
	fn create(&self) -> T;
}

impl<'a, T> Provider<T> for &'a T where T: Clone {
	fn create(&self) -> T {
		(*self).clone()
	}
}

impl<'a, F, T> Provider<T> for &'a mut F where F: ?Sized + Fn() -> T {
	fn create(&self) -> T {
		(*self)()
	}
}

pub struct ClosureProvider<F, T> {
	closure: F,
	_type: ::core::marker::PhantomData<T>,
}

impl<F, T> From<F> for ClosureProvider<F, T> where F: Sized + Fn() -> T {
	fn from(closure: F) -> ClosureProvider<F, T> {
		ClosureProvider {
			closure,
			_type: Default::default(),
		}
	}
}

impl<F, T> Provider<T> for ClosureProvider<F, T> where F: Fn() -> T {
	fn create(&self) -> T {
		(self.closure)()
	}
}

pub struct ObjectProvider<T> {
	object: T
}

impl<T> From<T> for ObjectProvider<T> {
	fn from(object: T) -> ObjectProvider<T> {
		ObjectProvider {
			object,
		}
	}
}

impl<T> Provider<T> for ObjectProvider<T> where T: Clone {
	fn create(&self) -> T {
		self.object.clone()
	}
}
