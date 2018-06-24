pub trait Edge {
	type Node: Ord;

	fn end_node(&self) -> &Self::Node;
}

pub trait WeightedEdge: Edge {
	type Weight: Weight + Ord;

	fn weight(&self) -> &Self::Weight;
}

pub trait ReversibleEdge: Edge {
	fn reverse_with(&self, start: &Self::Node) -> (Self::Node, Self);
}

pub trait CompleteEdge: Edge {
	fn start_node(&self) -> &Self::Node;

	fn reverse(&self) -> (Self::Node, Self) where Self: Sized + ReversibleEdge {
		ReversibleEdge::reverse_with(self, self.start_node())
	}
}

pub trait Weight {
	fn combine(left: &Self, right: &Self) -> Self;
}

pub trait SignedWeight: Weight {
	fn negative(&self) -> bool;
}
