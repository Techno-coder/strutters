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

		for neighbour in self.graph.neighbours(next) {
			if !self.visited.contains(neighbour.end_node()) {
				self.stack.push(neighbour.end_node());
			}
		}
		Some(next)
	}
}
