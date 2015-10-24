extern crate tcorp_math_mods;
use tcorp_math_mods::factors;

#[test]
fn number_of_factors_returns_correct_results() {
	assert_eq!(factors::number_of_factors(1), 1);
	assert_eq!(factors::number_of_factors(2), 2);
	assert_eq!(factors::number_of_factors(3), 2);
	assert_eq!(factors::number_of_factors(4), 3);
	assert_eq!(factors::number_of_factors(5), 2);
	assert_eq!(factors::number_of_factors(6), 4);
	let seven: u64 = 7;
	assert_eq!(factors::number_of_factors(seven), 2);
	assert_eq!(factors::number_of_factors(8), 4);
	assert_eq!(factors::number_of_factors(9), 3);
	let ten: usize = 10;
	assert_eq!(factors::number_of_factors(ten), 4);
	assert_eq!(factors::number_of_factors(90), 12);
	assert_eq!(factors::number_of_factors(100), 9);
}

#[test]
fn factors_returns_correct_results() {
	assert_eq!( factors::factors_for(1), vec![1]);
	assert_eq!( factors::factors_for(2), vec![1,2]);
	assert_eq!( factors::factors_for(4), vec![1,2,4]);
	assert_eq!( factors::factors_for(77), vec![1,7,11,77]);
	assert_eq!( factors::factors_for(84), vec![1, 2, 3, 4, 6, 7, 12, 14, 21, 28, 42, 84]);
}
