use collections::BTreeMap;
use core::cell::RefCell;
use core::ops::Deref;
use graph::Graph;
use graph::SignedWeight;
use graph::Weight;
use graph::WeightedEdge;
use OwnedRef;
use provider::Provider;

pub struct Floyd<'a, E> where E: 'a + WeightedEdge {
	distances: BTreeMap<&'a E::Node, BTreeMap<&'a E::Node, OwnedRef<'a, E::Weight>>>,
	next: BTreeMap<&'a E::Node, BTreeMap<&'a E::Node, &'a E::Node>>,
	has_negative_cycle: RefCell<Option<bool>>,
}

impl<'a, E> Floyd<'a, E> where E: 'a + WeightedEdge, E::Node: Ord {
	fn new() -> Floyd<'a, E> {
		Floyd {
			distances: BTreeMap::new(),
			next: BTreeMap::new(),
			has_negative_cycle: RefCell::new(None),
		}
	}

	/// Calculates the shortest distance between every pair of nodes
	///
	/// # Arguments
	///
	/// - `default` specifies the default distance between a node and itself.
	/// This should be used when the graph has edges that start and end at the same node.
	pub fn compute<'g, G, P>(graph: &'g G, default: Option<P>) -> Floyd<'g, E>
		where G: Graph<'g, Edge=E>, E: WeightedEdge, E::Node: Ord, P: Provider<E::Weight> {
		let mut store = Floyd::new();
		Self::populate_defaults(graph, &mut store, default);

		for middle in graph.nodes() {
			for start in graph.nodes() {
				for end in graph.nodes() {
					let new_distance = (|| {
						let start_to_middle = store.distance(&start, &middle)?;
						let middle_to_end = store.distance(&middle, &end)?;
						Some(E::Weight::combine(&start_to_middle, &middle_to_end))
					})();

					if let Some(new_distance) = new_distance {
						let prefer_new = store.distance(&start, &end)
						                      .and_then(|distance| Some(&new_distance < distance))
						                      .unwrap_or(true);
						if prefer_new {
							store.distances.entry(&start).or_insert(BTreeMap::new())
							     .insert(&end, new_distance.into());
							let next = store.next[start.deref()][middle.deref()];
							store.next.entry(&start).or_insert(BTreeMap::new())
							     .insert(&end, next);
						}
					}
				}
			}
		}

		store
	}

	fn populate_defaults<'g, G, P>(graph: &'g G, store: &mut Floyd<'g, E>, default: Option<P>)
		where G: Graph<'g, Edge=E>, E: WeightedEdge, P: Provider<E::Weight> {
		for node in graph.nodes() {
			for edge in graph.neighbours(&node) {
				store.distances.entry(&node).or_insert(BTreeMap::new())
				     .insert(&edge.end_node(), edge.weight().into());
				store.next.entry(&node).or_insert(BTreeMap::new())
				     .insert(&edge.end_node(), &edge.end_node());
			}

			if let Some(ref default) = default {
				let default = default.create();
				let prefer_default = store.distance(&node, &node)
				                          .and_then(|distance| Some(&default < &distance))
				                          .unwrap_or(true);
				if prefer_default {
					store.distances.entry(&node).or_insert(BTreeMap::new())
					     .insert(&node, default.into());
				}
			}
		}
	}

	pub fn distance(&self, start: &E::Node, end: &E::Node) -> Option<&E::Weight> {
		Some(self.distances.get(start)?.get(end)?.deref())
	}

	pub fn next(&self, node: &E::Node, end: &E::Node) -> Option<&E::Node> {
		self.next.get(node)?.get(end).cloned()
	}
}

impl<'a, E> Floyd<'a, E> where E: 'a + WeightedEdge, E::Node: Ord, E::Weight: SignedWeight {
	pub fn has_negative_cycle(&self) -> bool {
		if self.has_negative_cycle.borrow().is_none() {
			for node in self.distances.keys() {
				if let Some(distance) = self.distance(node, node) {
					if distance.negative() {
						self.has_negative_cycle.replace(Some(true));
						return true;
					}
				}
			}
			self.has_negative_cycle.replace(Some(false));
		}
		self.has_negative_cycle.borrow().unwrap()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		use graph::MutableGraph;
		use graph::HalfEdge;

		let mut graph = ::graph::AdjacencyList::new();
		graph.add_edge(1, HalfEdge::new(4, 3));
		graph.add_edge(2, HalfEdge::new(1, 3));
		graph.add_edge(3, HalfEdge::new(4, 2));
		graph.add_edge(4, HalfEdge::new(2, 1));
		graph.add_edge(4, HalfEdge::new(3, 1));
		graph.add_edge(5, HalfEdge::new(4, 2));

		let floyd = Floyd::compute(&graph, Some(&0));
		assert_eq!(floyd.has_negative_cycle(), false);
		for node in 1..=5 {
			assert_eq!(floyd.distance(&node, &node), Some(&0));
		}

		assert_eq!(floyd.distance(&1, &2), Some(&4));
		assert_eq!(floyd.distance(&1, &3), Some(&4));
		assert_eq!(floyd.distance(&1, &4), Some(&3));
		assert_eq!(floyd.distance(&1, &5), None);
		assert_eq!(floyd.distance(&2, &1), Some(&3));
		assert_eq!(floyd.distance(&2, &3), Some(&7));
		assert_eq!(floyd.distance(&2, &4), Some(&6));
		assert_eq!(floyd.distance(&2, &5), None);

		assert_eq!(floyd.next(&1, &3), Some(&4));
		assert_eq!(floyd.next(&2, &1), Some(&1));
		assert_eq!(floyd.next(&5, &1), Some(&4));
		assert_eq!(floyd.next(&1, &1), None);
		assert_eq!(floyd.next(&1, &5), None);
	}
}
