use super::Edge;
use super::ReversibleEdge;
use super::Weight;
use super::WeightedEdge;

pub struct GenericEdge<N, W> {
	end_node: N,
	weight: W,
}

impl<N, W> GenericEdge<N, W> {
	pub fn new(end_node: N, weight: W) -> GenericEdge<N, W> {
		GenericEdge {
			end_node,
			weight,
		}
	}
}

impl<N, W> Edge for GenericEdge<N, W> where N: Ord {
	type Node = N;

	fn end_node(&self) -> &<Self as Edge>::Node {
		&self.end_node
	}
}

impl<N, W> WeightedEdge for GenericEdge<N, W> where N: Ord, W: Weight + Ord {
	type Weight = W;

	fn weight(&self) -> &<Self as WeightedEdge>::Weight {
		&self.weight
	}
}

impl<N, W> ReversibleEdge for GenericEdge<N, W> where N: Ord + Clone, W: Clone {
	fn reverse(&self, start: &N) -> (N, Self) {
		(self.end_node.clone(), GenericEdge::new(start.clone(), self.weight.clone()))
	}
}

pub struct WeightlessEdge<N> {
	edge: GenericEdge<N, ()>,
}

impl<N> WeightlessEdge<N> {
	pub fn new(end_node: N) -> WeightlessEdge<N> {
		WeightlessEdge {
			edge: GenericEdge::new(end_node, ()),
		}
	}
}

impl<N> Edge for WeightlessEdge<N> where N: Ord {
	type Node = N;

	fn end_node(&self) -> &<Self as Edge>::Node {
		self.edge.end_node()
	}
}

impl<N> ReversibleEdge for WeightlessEdge<N> where N: Ord + Clone {
	fn reverse(&self, start: &N) -> (N, Self) {
		let (node, edge) = self.edge.reverse(start);
		(node, WeightlessEdge { edge })
	}
}
