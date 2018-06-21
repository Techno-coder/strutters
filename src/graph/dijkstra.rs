use collections::BinaryHeap;
use collections::BTreeMap;
use core::cmp::Ordering;
use super::Graph;
use super::Weight;
use super::WeightedEdge;

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

	pub fn distance_to(&self, node: &E::Node) -> Option<&E::Weight> {
		self.distances.get(node)
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
		let ordering = self.weight.partial_cmp(&other.weight)?;
		Some(match ordering {
			Ordering::Less => Ordering::Greater,
			Ordering::Equal => Ordering::Equal,
			Ordering::Greater => Ordering::Less,
		})
	}
}

impl<'a, E> Ord for DijkstraElement<'a, E> where E: WeightedEdge {
	fn cmp(&self, other: &Self) -> Ordering {
		match self.weight.cmp(&other.weight) {
			Ordering::Less => Ordering::Greater,
			Ordering::Equal => Ordering::Equal,
			Ordering::Greater => Ordering::Less,
		}
	}
}

pub fn dijkstra<'g, E>(graph: &'g Graph<Node=E::Node, Edge=E>, start: &'g E::Node, start_weight: E::Weight)
                       -> Dijkstra<'g, E> where E: WeightedEdge, E::Node: Ord {
	let mut store = Dijkstra::new();
	let mut queue: BinaryHeap<DijkstraElement<E>> = BinaryHeap::new();

	queue.push(DijkstraElement {
		parent: None,
		node: &start,
		weight: start_weight,
	});

	while let Some(next) = queue.pop() {
		if let Some(current_weight) = store.distances.get(next.node) {
			if &next.weight > current_weight { continue; }
		}

		for neighbour in graph.neighbours(next.node).or(Some(&[])).unwrap() {
			let new_weight = E::Weight::combine(&neighbour.weight(), &next.weight);
			if let Some(current_weight) = store.distances.get(next.node) {
				if &new_weight < current_weight {
					queue.push(DijkstraElement {
						parent: Some(next.node),
						node: neighbour.end_node(),
						weight: new_weight,
					});
				}
			} else {
				queue.push(DijkstraElement {
					parent: Some(next.node),
					node: neighbour.end_node(),
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		use graph::GenericEdge;

		let mut graph = ::graph::AdjacencyList::new();
		graph.add_edge('a', GenericEdge::new('b', 1));
		graph.add_edge('b', GenericEdge::new('c', 1));
		graph.add_edge('b', GenericEdge::new('d', 3));
		graph.add_edge('a', GenericEdge::new('c', 5));

		let store = dijkstra(&graph, &'a', 0);
		assert_eq!(store.distance_to(&'a').unwrap(), &0);
		assert_eq!(store.distance_to(&'b').unwrap(), &1);
		assert_eq!(store.distance_to(&'c').unwrap(), &2);
		assert_eq!(store.distance_to(&'d').unwrap(), &4);
	}
}