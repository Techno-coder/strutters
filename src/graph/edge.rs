pub trait Edge {
	type Node: Ord;

	fn end_node(&self) -> &Self::Node;
}

pub trait WeightedEdge: Edge {
	type Weight: Weight + Ord;

	fn weight(&self) -> &Self::Weight;
}

pub trait ReversibleEdge: Edge {
	fn reverse(&self, start: &Self::Node) -> (Self::Node, Self);
}

pub trait Weight {
	fn combine(left: &Self, right: &Self) -> Self;
}

pub trait SignedWeight: Weight {
	fn negative(&self) -> bool;
}
