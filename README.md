# strutters
Rust library for data structures [WIP]

## Features
- `no_std` compatible. Enable with `default-features = false`. Requires `alloc` crate.
- No clone/copy required for the main data type that is to be stored (may change in the future!)

## Data structures
- Segment tree
	- Vanilla
	- Lazy propagation
- Graph
	- Adjacency list
	
## Algorithms
- Graph
	- Shortest path
		- Dijkstra
		- Floyd-Warshall

## Specific details
For segment trees, a closure needs to be implemented that implements `AssociativeOperator`. This closure produces the parent node of its child nodes. Some example associative operators are included in `tree/mod.rs`.

For the lazy propagation segment tree, a closure needs to be implemented that implements `DeltaSifter`. This closure produces the correct node value from its current value and its current delta value. Additionally, the node will be updated from its child nodes once this is executed.
