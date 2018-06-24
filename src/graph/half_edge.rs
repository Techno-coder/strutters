use super::Edge;
use super::ReversibleEdge;
use super::Weight;
use super::WeightedEdge;

pub struct HalfEdge<N, W> {
	end_node: N,
	weight: W,
}

impl<N, W> HalfEdge<N, W> {
	pub fn new(end_node: N, weight: W) -> HalfEdge<N, W> {
		HalfEdge {
			end_node,
			weight,
		}
	}
}

impl<N> HalfEdge<N, ()> {
	pub fn new_weightless(end_node: N) -> HalfEdge<N, ()> {
		HalfEdge {
			end_node,
			weight: (),
		}
	}
}

impl<N, W> Edge for HalfEdge<N, W> where N: Ord {
	type Node = N;

	fn end_node(&self) -> &<Self as Edge>::Node {
		&self.end_node
	}
}

impl<N, W> WeightedEdge for HalfEdge<N, W> where N: Ord, W: Weight + Ord {
	type Weight = W;

	fn weight(&self) -> &<Self as WeightedEdge>::Weight {
		&self.weight
	}
}

impl<N, W> ReversibleEdge for HalfEdge<N, W> where N: Ord + Clone, W: Clone {
	fn reverse_with(&self, start: &N) -> (N, Self) {
		let end_node = self.end_node();
		let weight = self.weight.clone();
		(end_node.clone(), HalfEdge::new(start.clone(), weight))
	}
}
