use collections::BTreeMap;
use super::Graph;
use Vec;

pub struct AdjacencyList<N, E> {
	edges: BTreeMap<N, Vec<E>>,
}

impl<N, E> AdjacencyList<N, E> where N: Ord {
	pub fn new() -> AdjacencyList<N, E> {
		AdjacencyList {
			edges: BTreeMap::new(),
		}
	}
}

impl<N, E> Graph for AdjacencyList<N, E> where N: Ord {
	type Node = N;
	type Edge = E;

	fn add_edge(&mut self, start: N, edge: E) {
		self.edges.entry(start).or_insert(Vec::new()).push(edge);
	}

	fn neighbours(&self, node: &N) -> Option<&[E]> {
		self.edges.get(node).and_then(|vector| Some(vector.as_slice()))
	}
}