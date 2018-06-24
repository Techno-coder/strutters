use collections::BTreeMap;
use core::cell::RefCell;
use Vec;

/// `DisjointSet` allows fast checking of whether two values are grouped together
///
/// Two values can be grouped together by using the union operation. If **a** is
/// grouped with **b** and **b** is grouped with **c** then **a** is grouped with **c**.
/// Each value has a *parent* and if two values' parents are the same then they
/// are considered grouped together.
///
/// Typical applications of the `DisjointSet` include the minimum spanning
/// tree algorithm, *Kruskal's algorithm*.
pub struct DisjointSet<T> where T: Ord {
	indexes: BTreeMap<T, usize>,
	parents: Vec<RefCell<usize>>,
}

impl<T> DisjointSet<T> where T: Ord {
	pub fn new() -> DisjointSet<T> {
		DisjointSet {
			indexes: BTreeMap::new(),
			parents: Vec::new(),
		}
	}

	pub fn make_set(&mut self, node: T) {
		let index = self.parents.len();
		self.indexes.insert(node, index);
		self.parents.push(RefCell::new(index));
	}

	pub fn is_set(&self, node: &T) -> bool {
		self.indexes.contains_key(node)
	}

	pub fn connected(&self, a: &T, b: &T) -> bool {
		self.find_parent(a) == self.find_parent(b)
	}

	pub fn find_parent(&self, node: &T) -> usize {
		assert!(self.is_set(node));
		self.find_parent_index(self.indexes[node])
	}

	fn find_parent_index(&self, mut node: usize) -> usize {
		while *self.parents[node].borrow() != node {
			let parent = *self.parents[node].borrow();
			self.parents[node].replace(*self.parents[parent].borrow());
			node = parent;
		}
		node
	}

	pub fn union(&mut self, a: &T, b: &T) {
		assert!(self.is_set(a) && self.is_set(b));
		let a_index = self.indexes[a];
		let b_index = self.indexes[b];
		self.union_indexes(a_index, b_index);
	}

	pub fn union_indexes(&mut self, a: usize, b: usize) {
		let a_parent = self.find_parent_index(a);
		let b_parent = self.find_parent_index(b);
		self.parents[a_parent].replace(b_parent);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let mut set = DisjointSet::new();
		for index in 0..=3 {
			set.make_set(index);
		}
		set.union(&0, &1);
		set.union(&1, &2);
		assert!(set.connected(&0, &2));
		assert!(!set.connected(&0, &3));
	}

	#[test]
	fn test_reference() {
		let mut set = DisjointSet::new();
		set.make_set(&0);
		set.make_set(&1);
		set.make_set(&2);
		set.make_set(&3);
		set.union(&&0, &&1);
		set.union(&&1, &&2);
		assert!(set.connected(&&0, &&2));
		assert!(!set.connected(&&0, &&3));
	}
}