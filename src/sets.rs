use super::infinite_set::InfiniteSet;

/// Infinite set of positive ints (excludes zero)
#[derive(Default)]
pub struct InfinitePositiveInts {
    /// Defaults to zero via derivation of Default.
    current: u128
}

impl InfinitePositiveInts {
    pub fn new() -> Self {
        Default::default()
    }
}

impl InfiniteSet for InfinitePositiveInts {
    fn contains(&self, x: &u128) -> bool {
        *x > 0
    }
}

impl Iterator for InfinitePositiveInts {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        // increments the current number and returns it
        self.current += 1;

        Some(self.current)
    }
}

/// Infinite set of prime numbers
pub struct InfinitePrimes {
    primes: primal::Primes,
}

impl InfinitePrimes {
    pub fn new() -> Self {
        Self {
            primes: primal::Primes::all(),
        }
    }
}

impl InfiniteSet for InfinitePrimes {
    fn contains(&self, x: &u128) -> bool {
        primal::is_prime(*x as u64)
    }
}

impl Iterator for InfinitePrimes {
    type Item = u128;
    fn next(&mut self) -> Option<Self::Item> {
        self.primes.next().map(|u| u as u128)
    }
}

/// Infinite set of positive even numbers (excludes zero)
#[derive(Default)]
pub struct InfiniteEvens {
    /// Defaults to 0 via derivation of Default
    current: u128,
}

impl InfiniteEvens {
    pub fn new() -> Self {
        Default::default()
    }
}

impl InfiniteSet for InfiniteEvens {
    fn contains(&self, x: &u128) -> bool {
        *x > 0 && x % 2 == 0
    }
}

impl Iterator for InfiniteEvens {
    type Item = u128;
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 2;

        Some(self.current)
    }
}

/// Infinite set of positive odd numbers
pub struct InfiniteOdds {
    current: u128,
}

impl InfiniteOdds {
    pub fn new() -> Self {
        Self {
            current: 1,
        }
    }
}

impl InfiniteSet for InfiniteOdds {
    fn contains(&self, x: &u128) -> bool {
        *x > 0 && x % 2 == 1
    }
}

impl Iterator for InfiniteOdds {
    type Item = u128;
    fn next(&mut self) -> Option<Self::Item> {
        // save current number before raising it
        let result = Some(self.current);

        // advance to next odd number
        self.current += 2;

        // return the odd number we saved
        result
    }
}

/// Infinite set for powers of two
pub struct InfiniteTwoPowers {
    current: u128,
}

impl InfiniteTwoPowers {
    pub fn new() -> Self {
        Self {
            current: 1,
        }
    }
}

impl InfiniteSet for InfiniteTwoPowers {
    fn contains(&self, x: &u128) -> bool {
        let log = (*x as f64).log2();

        // checks if the log2 is an int. if it is, that means that x is a power of 2
        *x > 0 && log.fract() != 0.0
    }
}

impl Iterator for InfiniteTwoPowers {
    type Item = u128;
    fn next(&mut self) -> Option<Self::Item> {
        // save current number before raising it
        let result = Some(self.current);

        // advance to next power of 2 by multiplying by 2
        self.current *= 2;

        // return the number we saved
        result
    }
}
