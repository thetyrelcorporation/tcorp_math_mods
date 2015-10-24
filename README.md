# TCorp Math Mods

This will be a growing collection of helpful math functions and algorythems all of which will be abstracted into generic types.

## Factors

Install and initialize with

```rust
extern crate tcorp_math_mods;
use tcorp_math_mods::factors;
```

Special thanks to Kendall at http://stackoverflow.com/questions/110344/algorithm-to-calculate-the-number-of-divisors-of-a-given-number for the fastest factorization algorithm ever. Both functions rely on a ported and slightly modified version.

There are two methods currently.

The first is number_of_factors<T>(n:T) which returns the number of factors for parameter n in the same type as parameter n.

```rust
let x: u32 = 7;
assert_eq!(factors::number_of_factors(x), 2); // Will pass and the return type will be u32
```

The second function is factors_for<T>(n:T) which returns a vector<T> of all factors for a given n.

```rust
let x: i64 = 100;
assert_eq!(factors::factors_for(x), ); // Will pass and the return type will be Vec<i64>
```
