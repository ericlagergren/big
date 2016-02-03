#![crate_type = "lib"]
#![crate_name = "big"]

extern crate num;

use num::bigint::BigInt;
use num::CheckedAdd;

#[derive(Clone, Debug, Hash)]
pub struct Decimal {
    compact: i64,
    scale: i32,
    ctx: Context,
    acc: Accuracy,
    mant: num::bigint::BigInt,
}

impl Decimal {
    fn isOverflown(&self) -> bool {
        self.compact == OVERFLOWN_COMPACT;
    }
}

impl<'a> Add<&'a Decimal> for Decimal {
    type Output = Decimal;

    fn add(&self, &other: Decimal) -> Decimal {
        if !self.isOverflown() {
            if !other.isOverflown() {
                return self.addCompact(other);
            }
            return self.addHalf(other);
        }
        if !other.isOverflown() {
            return other.addHalf(self);
        }
        return self.addBig(y);
    }
}

impl Decimal {

	fn addCompact(&self, &other: Decimal) -> Decimal {
		debug_assert!(!self.compact.isOverflown);
		debug_assert!(!other.compact.isOverflown);

		let mut z: Decimal;

		if self.scale == other.scale {
			z.scale = self.scale;
			let sum = self.compact.checked_add(&other.compact);
			if sum != None {
				z.compact = sum;
			} else {
				z.mant = self.compact.to_bigint().unwrap() + other.mant;
			}
			return z;
		}
	}
}