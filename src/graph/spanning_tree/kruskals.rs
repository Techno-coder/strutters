use FixedDataSource;
use graph::CompleteEdge;
use graph::Edge;
use graph::MutableGraph;
use graph::ReversibleEdge;
use graph::UndirectedGraph;
use graph::WeightedEdge;
use query::DisjointSet;
use std::vec::Vec;

/// Finds a minimum spanning tree for the given edges
///
/// # Arguments
///
/// `base` is a graph to build the tree onto
pub fn kruskals<'g, S, G>(edges: S, base: G) -> UndirectedGraph<G>
	where G: MutableGraph<'g>, G::Edge: CompleteEdge + WeightedEdge + ReversibleEdge,
	      S: FixedDataSource<&'g G::Edge> {
	let mut set = DisjointSet::new();
	let mut graph = UndirectedGraph::new(base);

	let mut edges: Vec<&G::Edge> = edges.collect();
	edges.sort_unstable_by(|a, b| a.weight().cmp(b.weight()));

	for edge in edges {
		if !set.is_set(&edge.start_node()) { set.make_set(edge.start_node()); }
		if !set.is_set(&edge.end_node()) { set.make_set(edge.end_node()); }

		if !set.connected(&edge.start_node(), &edge.end_node()) {
			let (node, new_edge) = edge.reverse();
			graph.add_edge(node, new_edge);
			set.union(&edge.start_node(), &edge.end_node());
		}
	}
	graph
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		use graph::AdjacencyList;
		use graph::FullEdge;
		use graph::Graph;

		let base = AdjacencyList::new();
		let edges = [
			FullEdge::new(0, 1, 2),
			FullEdge::new(0, 5, 3),
			FullEdge::new(1, 2, 11),
			FullEdge::new(1, 6, 12),
			FullEdge::new(2, 3, 9),
			FullEdge::new(2, 6, 1),
			FullEdge::new(3, 4, 6),
			FullEdge::new(3, 6, 4),
			FullEdge::new(4, 5, 5),
			FullEdge::new(4, 6, 13),
			FullEdge::new(5, 6, 7),
		];

		let graph = kruskals(edges.into_iter(), base);
		let neighbours = |node: u32| -> Vec<u32> {
			graph.neighbours(&node).map(|edge| *edge.end_node()).collect()
		};
		assert_eq!(neighbours(0), &[1, 5]);
		assert_eq!(neighbours(1), &[0]);
		assert_eq!(neighbours(2), &[6]);
		assert_eq!(neighbours(3), &[6, 4]);
		assert_eq!(neighbours(4), &[5, 3]);
		assert_eq!(neighbours(5), &[0, 4]);
		assert_eq!(neighbours(6), &[2, 3]);
	}
}