use collections::BTreeMap;
use collections::BTreeSet;
use graph::Graph;
use graph::Weight;
use graph::WeightedEdge;

pub struct Floyd<'a, E> where E: 'a + WeightedEdge {
	distances: BTreeMap<&'a E::Node, BTreeMap<&'a E::Node, E::Weight>>,
	has_negative_cycle: bool,
}

impl<'a, E> Floyd<'a, E> where E: 'a + WeightedEdge, E::Node: Ord {
	fn new() -> Floyd<'a, E> {
		Floyd {
			distances: BTreeMap::new(),
			has_negative_cycle: false,
		}
	}

	pub fn distance(&self, start: &E::Node, end: &E::Node) -> Option<&E::Weight> {
		self.distances.get(start)?.get(end)
	}

	pub fn has_negative_cycle(&self) -> bool {
		self.has_negative_cycle
	}
}

pub fn floyd<'g, G, E>(graph: &'g G) -> Floyd<'g, E> where G: Graph<'g>, E: WeightedEdge {
//	let mut store = Floyd::new();
	unimplemented!()
}
