use super::Edge;

pub trait Graph<'g> {
	type Edge: Edge + 'g;
	type NodeIterator: Iterator<Item=&'g <Self::Edge as Edge>::Node>;
	type EdgeIterator: Iterator<Item=&'g Self::Edge>;

	fn add_edge(&mut self, start: <Self::Edge as Edge>::Node, edge: Self::Edge);
	fn nodes(&'g mut self) -> Self::NodeIterator;
	fn neighbours(&'g self, node: &<Self::Edge as Edge>::Node) -> Option<Self::EdgeIterator>;
}