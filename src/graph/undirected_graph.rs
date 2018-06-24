use core::ops::Deref;
use super::Edge;
use super::Graph;
use super::MutableGraph;
use super::ReversibleEdge;

pub struct UndirectedGraph<G> {
	graph: G,
}

impl<G> UndirectedGraph<G> {
	pub fn new(graph: G) -> UndirectedGraph<G> {
		UndirectedGraph {
			graph,
		}
	}
}

impl<'g, G> Graph<'g> for UndirectedGraph<G> where G: Graph<'g> {
	type Edge = G::Edge;
	type NodeIterator = G::NodeIterator;
	type EdgeIterator = G::EdgeIterator;

	fn nodes(&'g self) -> <Self as Graph>::NodeIterator {
		self.deref().nodes()
	}

	fn neighbours(&'g self, node: &<G::Edge as Edge>::Node) -> <Self as Graph>::EdgeIterator {
		self.deref().neighbours(node)
	}
}

impl<'g, G> MutableGraph<'g> for UndirectedGraph<G>
	where G: MutableGraph<'g>, G::Edge: ReversibleEdge {
	fn add_edge(&mut self, start: <G::Edge as Edge>::Node, edge: G::Edge) {
		let (reverse_start, reverse_edge) = edge.reverse_with(&start);
		self.graph.add_edge(reverse_start, reverse_edge);
		self.graph.add_edge(start, edge);
	}
}

impl<'g, G> Deref for UndirectedGraph<G> {
	type Target = G;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.graph
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let graph = ::graph::AdjacencyList::new();
		let mut graph = UndirectedGraph::new(graph);
		graph.add_edge('a', ::graph::HalfEdge::new('b', 1337));
		assert!(graph.neighbours(&'a').find(|edge| edge.end_node() == &'b').is_some())
	}
}
