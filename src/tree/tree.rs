pub trait BackingTree {
	type Value;
	type Identifier;
	type ChildIterator: Iterator<Item=Self::Identifier>;

	fn root(&self) -> Self::Identifier;

	fn set_root(&mut self, value: Self::Value);

	fn parent(&self, node: &Self::Identifier) -> Self::Identifier;

	/// Gets an identifier for a child of this node
	///
	/// # Panics
	///
	/// `child_index` must not be greater than the node width
	fn child(&self, node: &Self::Identifier, child_index: usize) -> Self::Identifier;

	fn children(&self, node: &Self::Identifier) -> Self::ChildIterator;

	fn get(&self, node: &Self::Identifier) -> Option<&Self::Value>;

	fn get_mut(&mut self, node: &Self::Identifier) -> Option<&mut Self::Value>;

	/// Inserts a child node at the node referenced by `node`
	/// Returns an identifier for the newly inserted node if successful
	///
	/// # Errors
	///
	/// Returns `None` if `node` does not exist
	fn insert_child(&mut self, node: Self::Identifier, offset: usize, value: Self::Value) -> Option<Self::Identifier>;
}
