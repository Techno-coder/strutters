use collections::BinaryHeap;
use collections::BTreeMap;
use core::cmp::Ordering;
use graph::Graph;
use graph::Weight;
use graph::WeightedEdge;

pub struct Dijkstra<'a, E> where E: 'a + WeightedEdge {
	distances: BTreeMap<&'a E::Node, E::Weight>,
	parents: BTreeMap<&'a E::Node, &'a E::Node>,
}

impl<'a, E> Dijkstra<'a, E> where E: WeightedEdge, E::Node: Ord {
	fn new() -> Dijkstra<'a, E> {
		Dijkstra {
			distances: BTreeMap::new(),
			parents: BTreeMap::new(),
		}
	}

	/// Calculates the shortest distance to any node from a single source
	///
	/// # Arguments
	///
	/// - `start_weight` specifies the distance to the source node
	pub fn compute<'g, G>(graph: &'g G, start: &'g E::Node, start_weight: E::Weight)
	                      -> Dijkstra<'g, E> where G: Graph<'g, Edge=E>, E: WeightedEdge, E::Node: Ord {
		let mut store = Dijkstra::new();
		let mut queue: BinaryHeap<DijkstraElement<E>> = BinaryHeap::new();

		queue.push(DijkstraElement {
			parent: None,
			node: &start,
			weight: start_weight,
		});

		while let Some(next) = queue.pop() {
			if let Some(current_weight) = store.distance(next.node) {
				if &next.weight > current_weight { continue; }
			}

			for edge in graph.neighbours(next.node) {
				let new_weight = E::Weight::combine(&edge.weight(), &next.weight);
				let prefer_new = store.distance(next.node)
				                      .and_then(|weight| Some(&new_weight < weight))
				                      .unwrap_or(true);
				if prefer_new {
					queue.push(DijkstraElement {
						parent: Some(next.node),
						node: edge.end_node(),
						weight: new_weight,
					});
				}
			}

			store.distances.insert(next.node, next.weight);
			if let Some(parent) = next.parent {
				store.parents.insert(next.node, parent);
			}
		}

		store
	}

	pub fn distance(&self, node: &E::Node) -> Option<&E::Weight> {
		self.distances.get(node)
	}

	pub fn parent(&self, node: &E::Node) -> Option<&E::Node> {
		self.parents.get(node).cloned()
	}
}

struct DijkstraElement<'a, E> where E: 'a + WeightedEdge {
	parent: Option<&'a E::Node>,
	node: &'a E::Node,
	weight: E::Weight,
}

impl<'a, E> PartialEq for DijkstraElement<'a, E> where E: WeightedEdge {
	fn eq(&self, other: &DijkstraElement<E>) -> bool {
		self.weight.eq(&other.weight)
	}
}

impl<'a, E> Eq for DijkstraElement<'a, E> where E: WeightedEdge {}

impl<'a, E> PartialOrd for DijkstraElement<'a, E> where E: WeightedEdge {
	fn partial_cmp(&self, other: &DijkstraElement<E>) -> Option<Ordering> {
		other.weight.partial_cmp(&self.weight)
	}
}

impl<'a, E> Ord for DijkstraElement<'a, E> where E: WeightedEdge {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

#[cfg(test)]
mod tests {
	use graph::AdjacencyList;
	use graph::HalfEdge;
	use graph::MutableGraph;
	use super::*;

	#[test]
	fn test() {
		let mut graph = AdjacencyList::new();
		graph.add_edge('a', HalfEdge::new('b', 1));
		graph.add_edge('b', HalfEdge::new('c', 1));
		graph.add_edge('b', HalfEdge::new('d', 3));
		graph.add_edge('a', HalfEdge::new('c', 5));

		let store = Dijkstra::compute(&graph, &'a', 0);
		assert_eq!(store.distance(&'a'), Some(&0));
		assert_eq!(store.distance(&'b'), Some(&1));
		assert_eq!(store.distance(&'c'), Some(&2));
		assert_eq!(store.distance(&'d'), Some(&4));

		assert_eq!(store.parent(&'a'), None);
		assert_eq!(store.parent(&'b'), Some(&'a'));
		assert_eq!(store.parent(&'c'), Some(&'b'));
		assert_eq!(store.parent(&'d'), Some(&'b'));
	}

	#[test]
	fn test_another() {
		let mut graph = AdjacencyList::new();
		graph.add_edge(1, HalfEdge::new(4, 3));
		graph.add_edge(2, HalfEdge::new(1, 3));
		graph.add_edge(3, HalfEdge::new(4, 2));
		graph.add_edge(4, HalfEdge::new(2, 1));
		graph.add_edge(4, HalfEdge::new(3, 1));
		graph.add_edge(5, HalfEdge::new(4, 2));

		let store = Dijkstra::compute(&graph, &1, 0);
		assert_eq!(store.distance(&2), Some(&4));
		assert_eq!(store.distance(&3), Some(&4));
		assert_eq!(store.distance(&4), Some(&3));
		assert_eq!(store.distance(&5), None);

		let store = Dijkstra::compute(&graph, &2, 0);
		assert_eq!(store.distance(&1), Some(&3));
		assert_eq!(store.distance(&3), Some(&7));
		assert_eq!(store.distance(&4), Some(&6));
		assert_eq!(store.distance(&5), None);
	}
}