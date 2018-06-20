pub trait BackingTree<T> {
	type Identifier;

	fn root(&self) -> Self::Identifier;

	fn set_root(&mut self, value: T);

	fn parent(&self, node: &Self::Identifier) -> Self::Identifier;

	/// Gets an identifier for a child of this node
	///
	/// # Panics
	///
	/// `child_index` must not be greater than the node width
	fn child(&self, node: &Self::Identifier, child_index: usize) -> Self::Identifier;

	fn get(&self, node: &Self::Identifier) -> Option<&T>;

	fn get_mut(&mut self, node: &Self::Identifier) -> Option<&mut T>;

	/// Inserts a child node at the node referenced by `node`
	/// Returns an identifier for the newly inserted node if successful
	///
	/// # Errors
	///
	/// Returns `None` if `node` does not exist
	fn insert_child(&mut self, node: Self::Identifier, offset: usize, value: T) -> Option<Self::Identifier>;
}

// TODO
//pub struct Node<'a, T: 'a, I: 'a> {
//	tree: &'a mut Tree<T, I>,
//	identifier: I,
//}
//
//impl<'a, T, I> Node<'a, T, I> {
//	pub fn parent(self) -> Node<'a, T, I> {
//		self.tree.parent(self.identifier)
//	}
//
//	pub fn child(self) -> Node<'a, T, I> {
//		self.tree.parent(self.identifier)
//	}
//}
