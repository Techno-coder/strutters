use collections::BTreeSet;
use graph::Edge;
use graph::Graph;
use Vec;

pub struct DepthFirst<'g, G, E> where G: 'g + Graph<'g>, E: 'g + Edge {
	graph: &'g G,
	stack: Vec<&'g E::Node>,
	visited: BTreeSet<&'g E::Node>,
}

impl<'g, G, E> DepthFirst<'g, G, E> where G: Graph<'g>, E: Edge {
	pub fn new(graph: &'g G, start: &'g E::Node) -> DepthFirst<'g, G, E> {
		let mut stack = Vec::new();
		stack.push(start);
		let visited = BTreeSet::new();
		DepthFirst {
			graph,
			stack,
			visited,
		}
	}
}

impl<'g, G, E> Iterator for DepthFirst<'g, G, E> where G: Graph<'g, Edge=E>, E: Edge {
	type Item = &'g E::Node;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		let next = self.stack.pop()?;
		self.visited.insert(next);

		for edge in self.graph.neighbours(next) {
			if !self.visited.contains(edge.end_node()) {
				self.stack.push(edge.end_node());
			}
		}
		Some(next)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		use graph::MutableGraph;
		use graph::WeightlessEdge;

		let mut graph = ::graph::AdjacencyList::new();
		graph.add_edge(1, WeightlessEdge::new(2));
		graph.add_edge(1, WeightlessEdge::new(5));
		graph.add_edge(2, WeightlessEdge::new(3));
		graph.add_edge(2, WeightlessEdge::new(4));
		graph.add_edge(5, WeightlessEdge::new(6));
		graph.add_edge(5, WeightlessEdge::new(7));
		let traversal = DepthFirst::new(&graph, &1);
		let traversal: Vec<u32> = traversal.cloned().collect();
		assert_eq!(&traversal, &[1, 5, 7, 6, 2, 4, 3]);
	}
}
