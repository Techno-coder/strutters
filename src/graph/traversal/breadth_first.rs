use collections::BTreeSet;
use collections::VecDeque;
use graph::Edge;
use graph::Graph;

pub struct BreadthFirst<'g, G, E> where G: 'g + Graph<'g, Edge=E>, E: 'g + Edge {
	graph: &'g G,
	queue: VecDeque<&'g E::Node>,
	visited: BTreeSet<&'g E::Node>,
}

impl<'g, G, E> BreadthFirst<'g, G, E> where G: Graph<'g, Edge=E>, E: Edge {
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
		for edge in self.graph.neighbours(next) {
			if !self.visited.contains(edge.end_node()) {
				self.queue.push_back(edge.end_node());
				self.visited.insert(edge.end_node());
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
		use graph::HalfEdge;
		use graph::MutableGraph;
		use Vec;

		let graph = ::graph::AdjacencyList::new();
		let mut graph = ::graph::UndirectedGraph::new(graph);
		graph.add_edge('a', HalfEdge::new_weightless('b'));
		graph.add_edge('a', HalfEdge::new_weightless('d'));
		graph.add_edge('a', HalfEdge::new_weightless('e'));
		graph.add_edge('b', HalfEdge::new_weightless('c'));
		graph.add_edge('b', HalfEdge::new_weightless('e'));
		graph.add_edge('c', HalfEdge::new_weightless('f'));
		graph.add_edge('d', HalfEdge::new_weightless('g'));
		graph.add_edge('e', HalfEdge::new_weightless('g'));
		graph.add_edge('g', HalfEdge::new_weightless('h'));
		graph.add_edge('g', HalfEdge::new_weightless('i'));
		let traversal = BreadthFirst::new(&graph, &'a');
		let traversal: Vec<char> = traversal.cloned().collect();
		assert_eq!(&traversal, &['a', 'b', 'd', 'e', 'c', 'g', 'f', 'h', 'i']);
	}
}
