use FixedDataSource;
use graph::CompleteEdge;
use graph::Graph;
use graph::MutableGraph;
use graph::ReversibleEdge;
use graph::UndirectedGraph;
use graph::WeightedEdge;
use query::DisjointSet;
use std::vec::Vec;

pub struct Kruskals<'g, E> where E: 'g + WeightedEdge {
	set: DisjointSet<&'g E::Node>,
	edges: Vec<&'g E>,
	next: usize,
}

impl<'g, E> Kruskals<'g, E> where E: WeightedEdge {
	pub fn new<S>(edges: S) -> Kruskals<'g, E> where S: FixedDataSource<&'g E> {
		let mut edges: Vec<&'g E> = edges.collect();
		edges.sort_unstable_by(|a, b| a.weight().cmp(b.weight()));
		Kruskals {
			set: DisjointSet::new(),
			edges,
			next: 0,
		}
	}
}

impl<'g, E> Kruskals<'g, E>
	where E: WeightedEdge + CompleteEdge + ReversibleEdge {
	/// Takes all the remaining edges and adds them to the graph `base`
	pub fn construct<G>(self, base: G) -> UndirectedGraph<G>
		where G: MutableGraph<'g> + Graph<'g, Edge=E> {
		let mut graph = UndirectedGraph::new(base);
		for edge in self {
			let (node, new_edge) = edge.reverse();
			graph.add_edge(node, new_edge);
		}
		graph
	}
}

impl<'g, E> Iterator for Kruskals<'g, E>
	where E: WeightedEdge + CompleteEdge {
	type Item = &'g E;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		loop {
			let edge = self.edges.get(self.next)?;
			self.next += 1;

			if !self.set.is_set(&edge.start_node()) { self.set.make_set(edge.start_node()); }
			if !self.set.is_set(&edge.end_node()) { self.set.make_set(edge.end_node()); }

			if !self.set.connected(&edge.start_node(), &edge.end_node()) {
				self.set.union(&edge.start_node(), &edge.end_node());
				return Some(edge);
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		use graph::AdjacencyList;
		use graph::FullEdge;
		use graph::Edge;
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

		let kruskals = Kruskals::new(edges.into_iter());
		let graph = kruskals.construct(base);
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