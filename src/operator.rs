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
