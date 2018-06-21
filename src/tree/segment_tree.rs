use OwningRef;
use super::AssociativeOperator;
use super::BackingTree;
use super::FixedDataSource;
use super::ImplicitTree;

pub struct SegmentTree<T, B, O> where B: BackingTree {
	tree: B,
	operator: O,
	length: usize,
	_type: ::core::marker::PhantomData<T>,
}

impl<T, B, O> SegmentTree<T, B, O> where O: AssociativeOperator<T>, B: BackingTree<Value=T> {
	fn construct_recursively(values: &mut impl FixedDataSource<T>, operator: &mut O, tree: &mut ImplicitTree<T>,
	                         range_left: usize, range_right: usize, node: usize) {
		if range_right == range_left {
			unsafe { tree.insert_unchecked(node, values.next_back().unwrap()) };
			return;
		}

		let (middle_left, middle_right) = super::functions::split_range(range_left, range_right);
		let left_child = tree.child(&node, 0);
		let right_child = tree.child(&node, 1);
		Self::construct_recursively(values, operator, tree, middle_right, range_right, right_child);
		Self::construct_recursively(values, operator, tree, range_left, middle_left, left_child);
		let value = operator(tree.get(&left_child).unwrap(), tree.get(&right_child).unwrap());
		unsafe { tree.insert_unchecked(node, value) };
	}

	/// Get the value for a range
	///
	/// # Panics
	///
	/// `left` cannot be greater than `right`
	/// `right` must be less than the length of the input array
	pub fn query(&self, left: usize, right: usize) -> OwningRef<T> {
		assert!(left <= right);
		assert!(right < self.length);
		self.query_recursively(self.tree.root(), left, right, 0, self.length - 1).unwrap()
	}

	fn query_recursively(&self, node: B::Identifier, left: usize, right: usize,
	                     current_left: usize, current_right: usize) -> Option<OwningRef<T>> {
		let in_range = left <= current_left && current_right <= right;
		if in_range { return self.tree.get(&node).and_then(|reference| Some(reference.into())); }

		let (middle_left, middle_right) = super::functions::split_range(current_left, current_right);
		let mut left_value = None;
		let mut right_value = None;
		if middle_left >= left {
			let left_child = self.tree.child(&node, 0);
			left_value = self.query_recursively(left_child, left, right, current_left, middle_left);
		}
		if middle_right <= right {
			let right_child = self.tree.child(&node, 1);
			right_value = self.query_recursively(right_child, left, right, middle_right, right);
		}

		if let Some(left_value) = left_value {
			if let Some(right_value) = right_value {
				Some((self.operator)(&left_value, &right_value).into())
			} else {
				Some(left_value)
			}
		} else if let Some(right_value) = right_value {
			Some(right_value.into())
		} else {
			unreachable!()
		}
	}

	/// Updates a value
	///
	/// # Panics
	///
	/// `index` must be less than the length of the input array
	pub fn update(&mut self, index: usize, value: T) {
		assert!(index < self.length);
		let root = self.tree.root();
		let last = self.length - 1;
		self.update_recursively(root, index, value, 0, last);
	}

	fn update_recursively(&mut self, node: B::Identifier, index: usize, value: T,
	                      current_left: usize, current_right: usize) {
		use core::mem::replace;
		if index == current_left && current_left == current_right {
			replace(self.tree.get_mut(&node).unwrap(), value);
			return;
		}

		let (middle_left, middle_right) = super::functions::split_range(current_left, current_right);
		if index <= middle_left {
			let left_child = self.tree.child(&node, 0);
			self.update_recursively(left_child, index, value, current_left, middle_left);
		} else {
			let right_child = self.tree.child(&node, 1);
			self.update_recursively(right_child, index, value, middle_right, current_right);
		}

		let value = {
			let left_child = self.tree.get(&self.tree.child(&node, 0)).unwrap();
			let right_child = self.tree.get(&self.tree.child(&node, 1)).unwrap();
			(self.operator)(left_child, right_child)
		};
		replace(self.tree.get_mut(&node).unwrap(), value);
	}
}

impl<T, O> SegmentTree<T, ImplicitTree<T>, O> where O: AssociativeOperator<T> {
	pub fn construct_implicit(mut values: impl FixedDataSource<T>, mut operator: O) -> SegmentTree<T, ImplicitTree<T>, O> {
		let mut tree = ImplicitTree::new(super::BINARY_WIDTH);
		let root = tree.root();
		let length = values.len();
		Self::construct_recursively(&mut values, &mut operator, &mut tree, 0, length - 1, root);
		SegmentTree {
			tree,
			operator,
			length,
			_type: Default::default(),
		}
	}
}

impl<T, B, O> ::core::fmt::Debug for SegmentTree<T, B, O> where B: ::core::fmt::Debug + BackingTree {
	fn fmt(&self, f: &mut ::core::fmt::Formatter) -> Result<(), ::core::fmt::Error> {
		writeln!(f, "{:?}", self.tree)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_summation() {
		use core::ops::Deref;
		let data = vec![1, 2, 3, 4, 5, 6, 7];
		let mut tree = SegmentTree::construct_implicit(data.into_iter(), ::tree::summation());
		assert_eq!(*tree.query(0, 3).deref(), 1 + 2 + 3 + 4);
		assert_eq!(*tree.query(0, 6).deref(), 1 + 2 + 3 + 4 + 5 + 6 + 7);
		assert_eq!(*tree.query(4, 6).deref(), 5 + 6 + 7);
		assert_eq!(*tree.query(0, 0).deref(), 1);
		assert_eq!(*tree.query(2, 3).deref(), 3 + 4);
		tree.update(6, 10);
		assert_eq!(*tree.query(6, 6).deref(), 10);
		assert_eq!(*tree.query(4, 6).deref(), 5 + 6 + 10);
		assert_eq!(*tree.query(0, 6).deref(), 1 + 2 + 3 + 4 + 5 + 6 + 10);
	}

	#[test]
	fn test_minimum() {
		use core::ops::Deref;
		let data = vec![1, 2, 3, 4, 5, 6, 7];
		let mut tree = SegmentTree::construct_implicit(data.into_iter(), ::tree::minimum());
		assert_eq!(*tree.query(0, 3).deref(), 1);
		assert_eq!(*tree.query(0, 6).deref(), 1);
		assert_eq!(*tree.query(4, 6).deref(), 5);
		assert_eq!(*tree.query(0, 0).deref(), 1);
		assert_eq!(*tree.query(2, 3).deref(), 3);
		tree.update(6, 10);
		assert_eq!(*tree.query(6, 6).deref(), 10);
		assert_eq!(*tree.query(4, 6).deref(), 5);
		assert_eq!(*tree.query(0, 6).deref(), 1);
	}
}
