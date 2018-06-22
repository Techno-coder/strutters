use core::mem::replace;
use core::ops::Deref;
use core::ops::DerefMut;
use OwnedRef;
use super::AssociativeOperator;
use super::BackingTree;
use super::DeltaSifter;
use super::DeltaWrapper;
use super::FixedDataSource;
use super::ImplicitTree;

pub struct LazySegmentTree<T, B, O, S, D> where B: BackingTree {
	tree: B,
	operator: O,
	length: usize,
	sifter: S,
	_type: ::core::marker::PhantomData<T>,
	_delta_type: ::core::marker::PhantomData<D>,
}

impl<T, B, O, S, D> LazySegmentTree<T, B, O, S, D>
	where O: AssociativeOperator<T>, B: BackingTree<Value=DeltaWrapper<T, D>>,
	      S: DeltaSifter<T, D>, D: Clone {
	fn construct_recursively(values: &mut impl FixedDataSource<DeltaWrapper<T, D>>, operator: &mut O,
	                         tree: &mut ImplicitTree<DeltaWrapper<T, D>>, range_left: usize,
	                         range_right: usize, node: usize) {
		// Same as SegmentTree construct_recursively
		if range_right == range_left {
			unsafe { tree.insert_unchecked(node, values.next_back().unwrap()) };
			return;
		}

		let (middle_left, middle_right) = super::functions::split_range(range_left, range_right);
		let left_child = tree.child(&node, 0);
		let right_child = tree.child(&node, 1);
		Self::construct_recursively(values, operator, tree, middle_right, range_right, right_child);
		Self::construct_recursively(values, operator, tree, range_left, middle_left, left_child);
		let value = operator(&tree.get(&left_child).unwrap(), &tree.get(&right_child).unwrap());
		unsafe { tree.insert_unchecked(node, DeltaWrapper::new(value)) };
	}

	pub fn query(&self, left: usize, right: usize) -> OwnedRef<T> {
		assert!(left <= right);
		assert!(right < self.length);
		let root = self.tree.root();
		let last = self.length - 1;
		self.query_recursively(root, left, right, 0, last).unwrap()
	}

	fn query_recursively(&self, node: B::Identifier, left: usize, right: usize,
	                     current_left: usize, current_right: usize) -> Option<OwnedRef<T>> {
		self.sift_node(&node, current_left, current_right);

		// Same as SegmentTree query_recursively
		let in_range = left <= current_left && current_right <= right;
		if in_range { return self.tree.get(&node).and_then(|reference| Some(reference.deref().into())); }

		let (middle_left, middle_right) = super::functions::split_range(current_left, current_right);
		let mut left_value = None;
		let mut right_value = None;
		if middle_left >= left {
			let left_child = self.tree.child(&node, 0);
			left_value = self.query_recursively(left_child, left, right, current_left, middle_left);
		}
		if middle_right <= right {
			let right_child = self.tree.child(&node, 1);
			right_value = self.query_recursively(right_child, left, right, middle_right, current_right);
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

	pub fn update_range(&mut self, left: usize, right: usize, delta: &D) {
		assert!(left <= right);
		assert!(right < self.length);
		let root = self.tree.root();
		let last = self.length - 1;
		self.update_range_recursively(root, left, right, 0, last, delta);
	}

	fn update_range_recursively(&mut self, node: B::Identifier, left: usize, right: usize,
	                            current_left: usize, current_right: usize, delta: &D) {
		self.sift_node(&node, current_left, current_right);

		let in_range = left <= current_left && current_right <= right;
		if in_range {
			self.tree.get_mut(&node).unwrap().delta.replace(Some(delta.clone()));
			self.sift_node(&node, current_left, current_right);
			return;
		}

		let (middle_left, middle_right) = super::functions::split_range(current_left, current_right);
		if middle_left >= left {
			let left_child = self.tree.child(&node, 0);
			self.update_range_recursively(left_child, left, right, current_left, middle_left, delta);
		}
		if middle_right <= right {
			let right_child = self.tree.child(&node, 1);
			self.update_range_recursively(right_child, left, right, middle_right, current_right, delta);
		}

		let value = {
			let left_child = self.tree.get(&self.tree.child(&node, 0)).unwrap();
			let right_child = self.tree.get(&self.tree.child(&node, 1)).unwrap();
			(self.operator)(left_child, right_child)
		};
		replace(self.tree.get_mut(&node).unwrap().deref_mut(), value);
	}

	fn sift_node(&self, node: &B::Identifier, current_left: usize, current_right: usize) {
		let range_length = (current_right - current_left) + 1;
		let node_delta;
		{
			let current_node = self.tree.get(node).unwrap();
			if let Some(ref delta) = current_node.delta.borrow().deref() {
				let new_value = (self.sifter)(&current_node, delta, range_length);
				replace(unsafe { current_node.object.get().as_mut() }.unwrap(), new_value);
				node_delta = delta.clone();
			} else {
				return;
			}
			current_node.delta.replace(None);
		}
		if range_length == 1 { return; }

		let (middle_left, middle_right) = super::functions::split_range(current_left, current_right);
		let left_child_node = self.tree.child(node, 0);
		if let Some(left_child) = self.tree.get(&left_child_node) {
			self.sift_node(&left_child_node, current_left, middle_left);
			left_child.delta.replace(Some(node_delta.clone()));
		}
		let right_child_node = self.tree.child(node, 1);
		if let Some(right_child) = self.tree.get(&right_child_node) {
			self.sift_node(&right_child_node, middle_right, current_right);
			right_child.delta.replace(Some(node_delta.clone()));
		}
	}
}

impl<T, O, S, D> LazySegmentTree<T, ImplicitTree<DeltaWrapper<T, D>>, O, S, D>
	where O: AssociativeOperator<T>, S: DeltaSifter<T, D>, D: Clone {
	pub fn construct_implicit(values: impl FixedDataSource<T>, mut operator: O, sifter: S)
	                          -> LazySegmentTree<T, ImplicitTree<DeltaWrapper<T, D>>, O, S, D> {
		let mut tree = ImplicitTree::new(super::BINARY_WIDTH);
		let mut values = values.map(|value| DeltaWrapper::new(value));
		let root = tree.root();
		let length = values.len();
		Self::construct_recursively(&mut values, &mut operator, &mut tree, 0, length - 1, root);
		LazySegmentTree {
			tree,
			operator,
			length,
			sifter,
			_type: Default::default(),
			_delta_type: Default::default(),
		}
	}
}

impl<T, B, O, S, D> ::core::fmt::Debug for LazySegmentTree<T, B, O, S, D>
	where B: ::core::fmt::Debug + BackingTree {
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
		let sifter = |current: &i32, delta: &i32, length: usize| *current + (*delta * length as i32);
		let mut tree = LazySegmentTree::construct_implicit(data.into_iter(), ::tree::summation(), sifter);
		assert_eq!(*tree.query(0, 3).deref(), 1 + 2 + 3 + 4);
		assert_eq!(*tree.query(0, 6).deref(), 1 + 2 + 3 + 4 + 5 + 6 + 7);
		assert_eq!(*tree.query(4, 6).deref(), 5 + 6 + 7);
		assert_eq!(*tree.query(0, 0).deref(), 1);
		assert_eq!(*tree.query(2, 3).deref(), 3 + 4);
		tree.update_range(0, 6, &5);
		assert_eq!(*tree.query(6, 6).deref(), 12);
		assert_eq!(*tree.query(4, 6).deref(), 10 + 11 + 12);
		assert_eq!(*tree.query(0, 6).deref(), 6 + 7 + 8 + 9 + 10 + 11 + 12);
		assert_eq!(*tree.query(2, 3).deref(), 8 + 9);
		tree.update_range(0, 6, &-2);
		tree.update_range(0, 6, &-3);
		assert_eq!(*tree.query(0, 3).deref(), 1 + 2 + 3 + 4);
		assert_eq!(*tree.query(0, 6).deref(), 1 + 2 + 3 + 4 + 5 + 6 + 7);
		assert_eq!(*tree.query(4, 6).deref(), 5 + 6 + 7);
		assert_eq!(*tree.query(0, 0).deref(), 1);
		assert_eq!(*tree.query(2, 3).deref(), 3 + 4);
	}

	#[test]
	fn test_minimum() {
		use core::ops::Deref;
		let data = vec![1, 2, 3, 4, 5, 6, 7];
		let sifter = |_: &u32, delta: &u32, _| *delta;
		let mut tree = LazySegmentTree::construct_implicit(data.into_iter(), ::tree::minimum(), sifter);
		assert_eq!(*tree.query(0, 3).deref(), 1);
		assert_eq!(*tree.query(0, 6).deref(), 1);
		assert_eq!(*tree.query(4, 6).deref(), 5);
		assert_eq!(*tree.query(0, 0).deref(), 1);
		assert_eq!(*tree.query(2, 3).deref(), 3);
		tree.update_range(6, 6, &0);
		assert_eq!(*tree.query(6, 6).deref(), 0);
		assert_eq!(*tree.query(4, 6).deref(), 0);
		assert_eq!(*tree.query(0, 5).deref(), 1);
		tree.update_range(4, 5, &1000);
		assert_eq!(*tree.query(6, 6).deref(), 0);
		assert_eq!(*tree.query(0, 3).deref(), 1);
		assert_eq!(*tree.query(0, 5).deref(), 1);
		assert_eq!(*tree.query(5, 5).deref(), 1000);
	}
}
