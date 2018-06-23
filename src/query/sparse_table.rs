use FixedDataSource;
use math::integer_log2;
use operator::AssociativeOperator;
use OwnedRef;
use Vec;

/// `SparseTable` allows efficient range queries against immutable data
///
/// `SparseTable` is similar to `IndexSparseTable` but can use any associative
/// operator at the cost of a slower query operation. See `IndexSparseTable`
/// for more details.
pub struct SparseTable<T, O> {
	table: Vec<Vec<T>>,
	operator: O,
}

impl<T, O> SparseTable<T, O> where O: AssociativeOperator<T> {
	pub fn compute<S>(data: S, operator: O) -> SparseTable<T, O> where S: FixedDataSource<T> {
		let mut table = Vec::new();
		let data: Vec<T> = data.collect();
		let length = data.len();
		table.push(data);

		for level in 1..=integer_log2(length as u64) as _ {
			table.push(Vec::new());
			for left_interval in 0..=(length - (1 << level)) {
				let previous_level = level - 1;
				let previous_range_length = 1 << previous_level;
				let right_interval = left_interval + previous_range_length;

				let value = {
					let left_range = &table[level - 1][left_interval];
					let right_range = &table[level - 1][right_interval];
					operator(left_range, right_range)
				};

				table[level].push(value);
			}
		}

		SparseTable {
			table,
			operator,
		}
	}

	pub fn query(&self, mut left: usize, right: usize) -> OwnedRef<T> {
		assert!(left <= right && right < self.table[0].len());
		let mut value: Option<OwnedRef<T>> = None;
		let max_level = integer_log2(left as u64 + 1) as usize;
		for level in (0..=max_level).rev() {
			let interval_length = 1 << level;
			if left + interval_length - 1 <= right {
				let table_value = &self.table[level][left];
				let new_value: OwnedRef<T> = if let Some(value) = value {
					(self.operator)(&value, &table_value).into()
				} else {
					table_value.into()
				};

				value = Some(new_value);
				left += interval_length;
			}
		}
		value.unwrap()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		use core::ops::Deref;
		use math::greatest_common_divisor;

		let data = vec![6, 12, 24, 36, 36];
		let operator = |a: &u64, b: &u64| greatest_common_divisor(*a, *b);
		let table = SparseTable::compute(data.into_iter(), operator);
		assert_eq!(table.query(0, 1).deref(), &6);
		assert_eq!(table.query(0, 4).deref(), &6);
		assert_eq!(table.query(3, 4).deref(), &36);
		assert_eq!(table.query(2, 4).deref(), &12);
		assert_eq!(table.query(1, 3).deref(), &12);
	}
}
