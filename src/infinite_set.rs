use std::cmp::Ordering;
use std::hash::Hash;

/// The InfiniteSet trait. Uses an Iterator design to return an infinite set of types. The trait
/// requires an implementation of Iterator, but users should be careful not to attempt to collect
/// for iterate over the entire set. It is infinite after all!
///
/// Important note: Since Iterator requires a starting point, the infinite set must have a starting
/// point as well. In other words, this cannot fairly represent a "double-ended", ordered infinite
/// set (such as the set of all positive and negative integers).
pub trait InfiniteSet: Iterator {
    /// A function to determine if `x` could exist in the infinite set. `x` is an item that could
    /// be in the set. This function is probably impossible to call with an incompatible type.
    fn contains(&self, x: &<Self as Iterator>::Item) -> bool;

    /// Returns a InfiniteUnion between this set and another.
    fn union<I>(self, other: I) -> InfiniteUnion<Self::Item>
    where
        <Self as Iterator>::Item: Ord + Eq + Hash,
        Self: Sized + 'static,
        I: InfiniteSet<Item = Self::Item> + 'static,
    {
        InfiniteUnion::from_sets(self, other)
    }

    /// Returns an InfiniteIntersection between this set and another.
    fn intersect<I>(self, other: I) -> InfiniteIntersection<Self::Item>
    where
        Self: Sized + 'static,
        I: InfiniteSet<Item = Self::Item> + 'static,
    {
        InfiniteIntersection::from_sets(self, other)
    }
}

/// A union between two infinite sets. InfiniteUnion is also an InfiniteSet.
///
/// first_next and second_next are the stored next values in the iterators. We store them because
/// simply comparing the results of next() on each set would unfairly throw away a value from one
/// of the sets and exclude the value from the union.
pub struct InfiniteUnion<T>
where
    T: Ord,
{
    first_set: Box<dyn InfiniteSet<Item = T>>,
    second_set: Box<dyn InfiniteSet<Item = T>>,

    first_next: T,
    second_next: T,

    /// The value to be returned upon a call to next()
    next_value: T,
}

impl<T: Ord> InfiniteUnion<T> {
    pub fn from_sets(
        mut first_set: impl InfiniteSet<Item = T> + 'static,
        mut second_set: impl InfiniteSet<Item = T> + 'static,
    ) -> Self {
        let mut first_next = first_set
            .next()
            .expect("first infinite set in union didn't have a next value");

        let mut second_next = loop {
            let n = second_set
                .next()
                .expect("second infinite set in union didn't have a next value");
            if n != first_next {
                break n;
            }
        };

        // get the next value by finding the lesser of the two next values from the infinite set
        let next_value = loop {
            match first_next.cmp(&second_next) {
                Ordering::Less => {
                    // if the first next value is lesser, then use it and advance the first set
                    let tmp = first_next;
                    first_next = first_set
                        .next()
                        .expect("first infinite set in union didn't have a next value");
                    break tmp;
                }
                Ordering::Equal => {
                    // if both of the next values are equal, advance the first set.
                    first_next = first_set
                        .next()
                        .expect("first infinite set in union didn't have a next value");

                    // continue looking for a lesser value
                    continue;
                }
                Ordering::Greater => {
                    // if the first next value is greater, then use the second value and advance the second set
                    let tmp = second_next;
                    second_next = second_set
                        .next()
                        .expect("second infinite set in union didn't have a next value");
                    break tmp;
                }
            }
        };

        Self {
            first_set: Box::new(first_set),
            second_set: Box::new(second_set),
            first_next,
            second_next,
            next_value,
        }
    }
}

impl<T: Ord + Clone> InfiniteSet for InfiniteUnion<T> {
    fn contains(&self, x: &T) -> bool {
        self.first_set.contains(x) || self.second_set.contains(x)
    }
}

impl<T: Ord + Clone> Iterator for InfiniteUnion<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.next_value.clone();

        self.next_value = loop {
            match self.first_next.cmp(&self.second_next) {
                Ordering::Less => {
                    // if the first next value is lesser, then use it and advance the first set
                    let tmp = self.first_next.clone();
                    self.first_next = self
                        .first_set
                        .next()
                        .expect("first infinite set in union didn't have a next value");
                    break tmp;
                }
                Ordering::Equal => {
                    // if both of the next values are equal, advance the first set.
                    self.first_next = self
                        .first_set
                        .next()
                        .expect("first infinite set in union didn't have a next value");

                    // continue looking for a lesser value
                    continue;
                }
                Ordering::Greater => {
                    // if the first next value is greater, then use the second value and advance the second set
                    let tmp = self.second_next.clone();
                    self.second_next = self
                        .second_set
                        .next()
                        .expect("second infinite set in union didn't have a next value");
                    break tmp;
                }
            }
        };

        Some(tmp)
    }
}

/// A intersection between two infinite sets. InfiniteIntersection is also an InfiniteSet.
///
/// first_next and second_next are the stored next values in the iterators. We store them because
/// simply comparing the results of next() on each set could unfairly throw away a value from one
/// of the sets and exclude the value from the union.
///
/// WARNING: InfiniteIntersection currently does not check for empty intersections. Calling next()
/// on an empty intersection will stall the program!
pub struct InfiniteIntersection<T> {
    first: Box<dyn InfiniteSet<Item = T>>,
    second: Box<dyn InfiniteSet<Item = T>>,
}

impl<T> InfiniteIntersection<T> {
    pub fn from_sets<I, J>(first: I, second: J) -> Self
    where
        I: InfiniteSet<Item = T> + 'static,
        J: InfiniteSet<Item = T> + 'static,
    {
        Self {
            first: Box::new(first),
            second: Box::new(second),
        }
    }
}

impl<T> InfiniteSet for InfiniteIntersection<T> {
    fn contains(&self, x: &<Self as Iterator>::Item) -> bool {
        self.first.contains(x) && self.second.contains(x)
    }
}

impl<T> Iterator for InfiniteIntersection<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // we find the next value by advancing the first set until its value can also be found in
        // the second set
        let next = loop {
            let x = self
                .first
                .next()
                .expect("first infinite set in intersection didn't have a next value");
            if self.second.contains(&x) {
                break x;
            } else {
                // not needed obviously, but is a reminder that we'll continue looping if the
                // element from the first set isn't also in the second
                continue;
            }
        };

        Some(next)
    }
}
