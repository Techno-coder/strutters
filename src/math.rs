pub fn integer_log2(value: u64) -> u32 {
	63 - value.leading_zeros()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_integer_log2() {
		assert_eq!(integer_log2(3), 1);
		assert_eq!(integer_log2(63), 5);
		assert_eq!(integer_log2(64), 6);
	}
}
