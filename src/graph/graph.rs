pub trait Graph {
	type Node;
	type Edge;

	fn add_edge(&mut self, start: Self::Node, edge: Self::Edge);
	fn neighbours(&self, node: &Self::Node) -> Option<&[Self::Edge]>;
}