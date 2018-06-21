use super::Weight;

macro_rules! define_weight {
    ($data_type: ty) => {
		impl Weight for $data_type {
			fn combine(left: &Self, right: &Self) -> Self {
				left + right
			}
		}
    };
}

define_weight!(i8);
define_weight!(i16);
define_weight!(i32);
define_weight!(i64);
define_weight!(isize);

define_weight!(u8);
define_weight!(u16);
define_weight!(u32);
define_weight!(u64);
define_weight!(usize);
