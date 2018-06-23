use Vec;

/// `IndexSparseTable` allows efficient range queries against immutable data
///
/// `IndexSparseTable` is similar to `SparseTable` but stores indexes
/// to data rather than values. Queries return the maximum in a range but
/// this can be changed by changing the implementation of Ord. Query
/// operation is executed in O(`1`) time compared to `SparseTable`'s time
/// of O(`log n`). However, `SparseTable` is more flexible in the type
/// of operations that are allowed (`IndexSparseTable` only allows idempotent
/// operations).
pub struct IndexSparseTable<'t, T> where T: 't {
	data: &'t [T],
	table: Vec<Vec<usize>>,
}

impl<'t, T> IndexSparseTable<'t, T> where T: Ord {
	pub fn compute(data: &[T]) -> IndexSparseTable<T> {
		let mut table = Vec::new();
		let length = data.len();
		table.push((0..length).collect());

		for level in 1..=(length as f32).log2() as _ {
			table.push(Vec::new());
			for left_interval in 0..=(length - (1 << level)) {
				let previous_level = level - 1;
				let previous_range_length = 1 << previous_level;
				let right_interval = left_interval + previous_range_length;

				let left_range_index = table[level - 1][left_interval];
				let right_range_index = table[level - 1][right_interval];

				if data[left_range_index] > data[right_range_index] {
					table[level].push(left_range_index);
				} else {
					table[level].push(right_range_index);
				}
			}
		}

		IndexSparseTable {
			data,
			table,
		}
	}

	pub fn query_index(&self, left: usize, right: usize) -> usize {
		assert!(left <= right && right < self.table[0].len());
		let range_length = right - left;
		let level = ((range_length + 1) as f32).log2() as usize;
		let right_range_index = (right + 1) - (1 << level);

		let left_range_index = self.table[level][left];
		let right_range_index = self.table[level][right_range_index];

		if self.data[left_range_index] > self.data[right_range_index] {
			left_range_index
		} else {
			right_range_index
		}
	}

	pub fn query(&self, left: usize, right: usize) -> &T {
		&self.data[self.query_index(left, right)]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let data = vec![1, 2, 3, 4, 5, 6, 7];
		let table = IndexSparseTable::compute(&data);
		for right in 0..=6 {
			for left in 0..=right {
				assert_eq!(table.query(left, right), &(right + 1));
			}
		}
	}
}