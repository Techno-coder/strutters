use super::Edge;
use super::Weight;
use super::WeightedEdge;

pub struct GenericEdge<T, W> {
	end_node: T,
	weight: W,
}

impl<T, W> GenericEdge<T, W> {
	pub fn new(end_node: T, weight: W) -> GenericEdge<T, W> {
		GenericEdge {
			end_node,
			weight,
		}
	}
}

impl<T, W> Edge for GenericEdge<T, W> where T: Ord {
	type Node = T;

	fn end_node(&self) -> &<Self as Edge>::Node {
		&self.end_node
	}
}

impl<T, W> WeightedEdge for GenericEdge<T, W> where T: Ord, W: Weight + Ord {
	type Weight = W;

	fn weight(&self) -> &<Self as WeightedEdge>::Weight {
		&self.weight
	}
}

pub struct WeightlessEdge<T> {
	edge: GenericEdge<T, ()>,
}

impl<T> WeightlessEdge<T> {
	pub fn new(end_node: T) -> WeightlessEdge<T> {
		WeightlessEdge {
			edge: GenericEdge::new(end_node, ()),
		}
	}
}

impl<T> Edge for WeightlessEdge<T> where T: Ord {
	type Node = T;

	fn end_node(&self) -> &<Self as Edge>::Node {
		self.edge.end_node()
	}
}
