extern crate num;
use self::num::traits::{ PrimInt, One, Zero };

pub fn factors_for<T: PrimInt>( n: T ) -> Vec<T> {
	let mut vec: Vec<T> = Vec::new();
	if n == One::one() {
		vec.push(One::one());
	}
	let mut limit = n;
	let mut i: T = One::one();
	while i < limit {
		if n % i == Zero::zero() {
			limit = n / i;
			if limit != i {
				vec.push(limit);
			}
			vec.push(i);
		}
		i = i + One::one();
	}
	vec.sort_by(|a, b| a.cmp(b));
	vec
}

pub fn number_of_factors<T: PrimInt>(n: T) -> T {
	if n == One::one() {
		return One::one();
	}
	let mut factor_count: T = Zero::zero();
	let mut limit = n;
	let mut i: T = One::one();
	while i < limit {
		if n % i == Zero::zero() {
			limit = n / i;
			if limit != i {
				factor_count = factor_count + One::one();
			}
			factor_count = factor_count + One::one();
		}
		i = i + One::one();
	}
	factor_count
}
