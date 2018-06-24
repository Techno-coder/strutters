use super::CompleteEdge;
use super::Edge;
use super::HalfEdge;
use super::ReversibleEdge;
use super::Weight;
use super::WeightedEdge;

pub struct FullEdge<N, W> {
	start_node: N,
	edge: HalfEdge<N, W>,
}

impl<N, W> FullEdge<N, W> {
	pub fn new(start_node: N, end_node: N, weight: W) -> FullEdge<N, W> {
		FullEdge {
			start_node,
			edge: HalfEdge::new(end_node, weight),
		}
	}
}

impl<N> FullEdge<N, ()> {
	pub fn new_weightless(start_node: N, end_node: N) -> FullEdge<N, ()> {
		FullEdge {
			start_node,
			edge: HalfEdge::new_weightless(end_node),
		}
	}
}

impl<N, W> Edge for FullEdge<N, W> where N: Ord {
	type Node = N;

	fn end_node(&self) -> &<Self as Edge>::Node {
		self.edge.end_node()
	}
}

impl<N, W> WeightedEdge for FullEdge<N, W> where N: Ord, W: Weight + Ord {
	type Weight = W;

	fn weight(&self) -> &<Self as WeightedEdge>::Weight {
		self.edge.weight()
	}
}

impl<N, W> ReversibleEdge for FullEdge<N, W> where N: Ord + Clone, W: Weight + Ord + Clone {
	fn reverse_with(&self, start: &N) -> (N, Self) {
		let end_node = self.end_node();
		let weight = self.weight().clone();
		(end_node.clone(), FullEdge::new(end_node.clone(), start.clone(), weight))
	}
}

impl<N, W> CompleteEdge for FullEdge<N, W> where N: Ord {
	fn start_node(&self) -> &<Self as Edge>::Node {
		&self.start_node
	}
}