pub fn integer_log2(value: u64) -> u32 {
	63 - value.leading_zeros()
}

pub fn greatest_common_divisor(a: u64, b: u64) -> u64 {
	if b == 0 {
		a
	} else {
		greatest_common_divisor(b, a % b)
	}
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

	#[test]
	fn test_greatest_common_divisor() {
		assert_eq!(greatest_common_divisor(252, 105), 21);
		assert_eq!(greatest_common_divisor(105, 252), 21);
	}
}
