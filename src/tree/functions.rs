use super::DefaultGenerator;
use Vec;

pub fn extend_inclusive<T, D>(vector: &mut Vec<T>, index: usize, default: &mut D) where D: DefaultGenerator<T> {
	extend_until(vector, index + 1, default);
}

pub fn extend_until<T, D>(vector: &mut Vec<T>, index: usize, default: &mut D) where D: DefaultGenerator<T> {
	for _ in vector.len()..index {
		vector.push(default());
	}
}

pub fn split_range(left: usize, right: usize) -> (usize, usize) {
	let middle_left = (left + right) / 2;
	let middle_right = middle_left + 1;
	(middle_left, middle_right)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_extend_until() {
		let mut vector = vec![30, 15];
		extend_until(&mut vector, 10, &mut || 1337);
		assert_eq!(vector[0], 30);
		assert_eq!(vector[1], 15);
		for index in 2..10 {
			assert_eq!(vector[index], 1337);
		}
		assert_eq!(vector.len(), 10);
	}

	#[test]
	fn test_split_range() {
		assert_eq!(split_range(0, 7), (3, 4));
		assert_eq!(split_range(10, 19), (14, 15))
	}
}