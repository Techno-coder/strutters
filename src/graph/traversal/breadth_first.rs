use collections::BTreeSet;
use collections::VecDeque;
use graph::Edge;
use graph::Graph;

pub struct BreadthFirst<'g, G, E> where G: 'g + Graph<'g>, E: 'g + Edge {
	graph: &'g G,
	queue: VecDeque<&'g E::Node>,
	visited: BTreeSet<&'g E::Node>,
}

impl<'g, G, E> BreadthFirst<'g, G, E> where G: Graph<'g>, E: Edge {
	pub fn new(graph: &'g G, start: &'g E::Node) -> BreadthFirst<'g, G, E> {
		let mut queue = VecDeque::new();
		queue.push_back(start);
		let mut visited = BTreeSet::new();
		visited.insert(start);
		BreadthFirst {
			graph,
			queue,
			visited,
		}
	}
}

impl<'g, G, E> Iterator for BreadthFirst<'g, G, E> where G: Graph<'g, Edge=E>, E: Edge {
	type Item = &'g E::Node;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		let next = self.queue.pop_front()?;
		for neighbour in self.graph.neighbours(next) {
			if !self.visited.contains(neighbour.end_node()) {
				self.queue.push_back(neighbour.end_node());
			}
		}
		Some(next)
	}
}
