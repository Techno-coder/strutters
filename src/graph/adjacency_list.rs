use collections::BTreeMap;
use super::Edge;
use super::Graph;
use Vec;

pub struct AdjacencyList<'g, E> where E: Edge + 'g {
	edges: BTreeMap<E::Node, Vec<E>>,
	_empty: Vec<E>,
	_lifetime: ::core::marker::PhantomData<&'g AdjacencyList<'g, E>>,
}

impl<'g, E> AdjacencyList<'g, E> where E: Edge, E::Node: Ord {
	pub fn new() -> AdjacencyList<'g, E> {
		AdjacencyList {
			edges: BTreeMap::new(),
			_empty: Vec::new(),
			_lifetime: Default::default(),
		}
	}

	pub fn add_edge(&mut self, start: E::Node, edge: E) {
		self.edges.entry(start).or_insert(Vec::new()).push(edge);
	}
}

impl<'g, E> Graph<'g> for AdjacencyList<'g, E> where E: Edge + 'g, E::Node: Ord {
	type Edge = E;
	type NodeIterator = ::collections::btree_map::Keys<'g, E::Node, Vec<E>>;
	type EdgeIterator = ::core::slice::Iter<'g, E>;

	fn nodes(&'g self) -> <Self as Graph>::NodeIterator {
		self.edges.keys()
	}

	fn neighbours(&'g self, node: &E::Node) -> <Self as Graph>::EdgeIterator {
		self.edges.get(node).or(Some(&self._empty)).unwrap().iter()
	}
}