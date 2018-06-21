use super::BackingTree;
use Vec;

pub struct ImplicitTree<T> {
	values: Vec<Option<T>>,
	width: usize,
}

impl<T> ImplicitTree<T> {
	pub fn new(width: usize) -> ImplicitTree<T> {
		ImplicitTree {
			values: vec![None],
			width,
		}
	}

	/// Inserts a node into the tree ignoring checks
	///
	/// # Safety
	///
	/// Inserting a node can break the invariant that every valid child has
	/// a valid parent
	pub unsafe fn insert_unchecked(&mut self, node: usize, value: T) {
		super::functions::extend_inclusive(&mut self.values, node, &mut || None);
		self.values[node] = Some(value);
	}
}

impl<T> BackingTree for ImplicitTree<T> {
	type Value = T;
	type Identifier = usize;

	fn root(&self) -> usize {
		0
	}

	fn set_root(&mut self, value: T) {
		let root = self.root();
		self.values[root] = Some(value);
	}

	fn parent(&self, node: &usize) -> usize {
		(node - 1) / 2
	}

	fn child(&self, node: &usize, child_index: usize) -> usize {
		assert!(child_index < self.width);
		(self.width * node) + (child_index + 1)
	}

	fn get(&self, node: &usize) -> Option<&T> {
		self.values.get(*node).and_then(|node| node.as_ref())
	}

	fn get_mut(&mut self, node: &usize) -> Option<&mut T> {
		self.values.get_mut(*node).and_then(|node| node.as_mut())
	}

	fn insert_child(&mut self, node: usize, offset: usize, value: T) -> Option<usize> {
		self.get(&node)?;
		let child = self.child(&node, offset);
		super::functions::extend_inclusive(&mut self.values, child, &mut || None);
		self.values[child] = Some(value);
		Some(child)
	}
}

impl<T> ::core::fmt::Debug for ImplicitTree<T> where T: ::core::fmt::Debug {
	fn fmt(&self, f: &mut ::core::fmt::Formatter) -> Result<(), ::core::fmt::Error> {
		write!(f, "ImplicitTree: [")?;
		for value in self.values.iter() {
			write!(f, "{:?}, ", value)?;
		}
		writeln!(f, "]")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let mut tree = ImplicitTree::new(2);
		tree.set_root(0);
		assert_eq!(*tree.get(&tree.root()).unwrap(), 0);
		assert_eq!(tree.child(&tree.root(), 1), 2);
		assert!(tree.insert_child(0, 0, 3).is_some());
		assert!(tree.insert_child(0, 1, 6).is_some());
		assert_eq!(*tree.get(&tree.child(&tree.root(), 0)).unwrap(), 3);
		assert_eq!(*tree.get(&tree.child(&tree.root(), 1)).unwrap(), 6);
		assert!(tree.insert_child(1, 0, 9).is_some());
		assert!(tree.insert_child(2, 1, 12).is_some());
		assert_eq!(*tree.get(&tree.child(&1, 0)).unwrap(), 9);
		assert_eq!(*tree.get(&tree.child(&2, 1)).unwrap(), 12);
		assert_eq!(tree.insert_child(8, 0, 0), None);
		assert_eq!(tree.parent(&2), 0);
		assert_eq!(*tree.get(&tree.parent(&2)).unwrap(), 0);
		unsafe { tree.insert_unchecked(3, 4); }
		assert_eq!(*tree.get(&3).unwrap(), 4);
	}
}
