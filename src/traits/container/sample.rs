use crate::{
    traits::{IntervalBounds, SetError, ValueBounds},
    Container,
};
use rand::{seq::SliceRandom, RngCore, SeedableRng};
use rand_chacha::ChaChaRng;

/// Utility functions for random sampling within a container.
pub trait Sample<T, I>: Container<T, I>
where
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    /// Shuffles the elements of the container in place using the given random number generator.
    ///
    /// # Example
    /// ```
    /// use bedrs::{Container, Interval, IntervalSet, Sample};
    /// use rand::thread_rng;
    ///
    /// let intervals = vec![
    ///     Interval::new(10, 100),
    ///     Interval::new(20, 200),
    ///     Interval::new(30, 300),
    ///     Interval::new(40, 400),
    /// ];
    /// let mut set = IntervalSet::from_sorted(intervals).unwrap();
    /// set.shuffle_rng(&mut thread_rng());
    /// ```
    fn shuffle_rng(&mut self, rng: &mut impl RngCore) {
        self.records_mut().shuffle(rng);
        self.set_unsorted();
    }

    /// Shuffles the elements of the container in place.
    ///
    /// # Example
    /// ```
    /// use bedrs::{Container, Interval, IntervalSet, Sample};
    ///
    /// let intervals = vec![
    ///    Interval::new(10, 100),
    ///    Interval::new(20, 200),
    ///    Interval::new(30, 300),
    ///    Interval::new(40, 400),
    /// ];
    /// let mut set = IntervalSet::from_sorted(intervals).unwrap();
    /// set.shuffle();
    /// ```
    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.shuffle_rng(&mut rng);
    }

    /// Shuffles the elements of the container in place using the given seed.
    ///
    /// # Example
    /// ```
    /// use bedrs::{Container, Interval, IntervalSet, Sample};
    ///
    /// let intervals = vec![
    ///    Interval::new(10, 100),
    ///    Interval::new(20, 200),
    ///    Interval::new(30, 300),
    ///    Interval::new(40, 400),
    /// ];
    /// let mut set = IntervalSet::from_sorted(intervals).unwrap();
    /// set.shuffle_seed(42);
    /// ```
    fn shuffle_seed(&mut self, seed: u64) {
        let mut rng = ChaChaRng::seed_from_u64(seed);
        self.shuffle_rng(&mut rng);
    }

    /// Returns a new container with the elements of the container in random order using the given random number generator.
    ///
    /// # Example
    /// ```
    /// use bedrs::{Container, Interval, IntervalSet, Sample};
    ///
    /// let intervals = vec![
    ///     Interval::new(10, 100),
    ///     Interval::new(20, 200),
    ///     Interval::new(30, 300),
    ///     Interval::new(40, 400),
    /// ];
    /// let set = IntervalSet::from_sorted(intervals).unwrap();
    /// let mut rng = rand::thread_rng();
    /// let shuffled_set = set.sample_rng(2, &mut rng).unwrap();
    /// assert_eq!(shuffled_set.len(), 2);
    /// ```
    fn sample_rng(&self, n: usize, rng: &mut impl RngCore) -> Result<Self, SetError> {
        if n > self.records().len() {
            return Err(SetError::SampleSizeTooLarge);
        }
        let mut records = self.records().clone();
        records.shuffle(rng);
        records.truncate(n);
        Ok(Self::new(records))
    }

    /// Returns a new container with the elements of the container in random order.
    ///
    /// # Example
    /// ```
    /// use bedrs::{Container, Interval, IntervalSet, Sample};
    ///
    /// let intervals = vec![
    ///     Interval::new(10, 100),
    ///     Interval::new(20, 200),
    ///     Interval::new(30, 300),
    ///     Interval::new(40, 400),
    /// ];
    /// let set = IntervalSet::from_sorted(intervals).unwrap();
    /// let shuffled_set = set.sample(2).unwrap();
    /// assert_eq!(shuffled_set.len(), 2);
    /// ```
    fn sample(&self, n: usize) -> Result<Self, SetError> {
        if n > self.records().len() {
            return Err(SetError::SampleSizeTooLarge);
        }
        let mut rng = rand::thread_rng();
        self.sample_rng(n, &mut rng)
    }

    /// Returns a new container with the elements of the container in random order using the given seed.
    ///
    /// # Example
    /// ```
    /// use bedrs::{Container, Interval, IntervalSet, Sample};
    ///
    /// let intervals = vec![
    ///     Interval::new(10, 100),
    ///     Interval::new(20, 200),
    ///     Interval::new(30, 300),
    ///     Interval::new(40, 400),
    /// ];
    /// let set = IntervalSet::from_sorted(intervals).unwrap();
    /// let shuffled_set_a = set.sample_seed(2, 42).unwrap();
    /// let shuffled_set_b = set.sample_seed(2, 42).unwrap();
    /// assert_eq!(shuffled_set_a.len(), 2);
    /// assert_eq!(shuffled_set_b.len(), 2);
    /// ```
    fn sample_seed(&self, n: usize, seed: u64) -> Result<Self, SetError> {
        if n > self.records().len() {
            return Err(SetError::SampleSizeTooLarge);
        }
        let mut rng = ChaChaRng::seed_from_u64(seed);
        self.sample_rng(n, &mut rng)
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use crate::{Coordinates, Interval, IntervalSet};

    #[test]
    fn shuffle_rng() {
        let intervals = vec![
            Interval::new(10, 100),
            Interval::new(20, 200),
            Interval::new(30, 300),
            Interval::new(40, 400),
            Interval::new(10, 100),
            Interval::new(20, 200),
            Interval::new(30, 300),
            Interval::new(40, 400),
        ];
        let set = IntervalSet::new(intervals);
        let mut shuffled_set = set.clone();
        shuffled_set.shuffle();
        set.records()
            .iter()
            .zip(shuffled_set.records())
            .all(|(a, b)| !a.eq(b));
    }

    #[test]
    fn shuffle_rng_seed() {
        let intervals = vec![
            Interval::new(10, 100),
            Interval::new(20, 200),
            Interval::new(30, 300),
            Interval::new(40, 400),
            Interval::new(10, 100),
            Interval::new(20, 200),
            Interval::new(30, 300),
            Interval::new(40, 400),
        ];
        let set = IntervalSet::new(intervals);
        let mut shuffled_set_a = set.clone();
        let mut shuffled_set_b = set.clone();
        shuffled_set_a.shuffle_seed(0);
        shuffled_set_b.shuffle_seed(0);
        shuffled_set_a
            .records()
            .iter()
            .zip(shuffled_set_b.records())
            .all(|(a, b)| a.eq(b));
    }

    #[test]
    fn shuffle_sorted_toggle() {
        let intervals = vec![
            Interval::new(10, 100),
            Interval::new(20, 200),
            Interval::new(30, 300),
            Interval::new(40, 400),
        ];
        let mut set = IntervalSet::from_sorted(intervals).unwrap();
        assert_eq!(set.is_sorted(), true);
        set.shuffle();
        assert_eq!(set.is_sorted(), false);
    }

    #[test]
    fn sample() {
        let intervals = vec![
            Interval::new(10, 100),
            Interval::new(20, 200),
            Interval::new(30, 300),
            Interval::new(40, 400),
        ];
        let set = IntervalSet::from_sorted(intervals).unwrap();
        let sampled_set = set.sample(4).unwrap();
        assert_eq!(sampled_set.records().len(), 4);
        assert!(!sampled_set.is_sorted());
    }

    #[test]
    fn sample_seed() {
        let intervals = vec![
            Interval::new(10, 100),
            Interval::new(20, 200),
            Interval::new(30, 300),
            Interval::new(40, 400),
        ];
        let set = IntervalSet::from_sorted(intervals).unwrap();
        let sampled_set_a = set.sample_seed(4, 0).unwrap();
        let sampled_set_b = set.sample_seed(4, 0).unwrap();
        for (a, b) in sampled_set_a.records().iter().zip(sampled_set_b.records()) {
            assert!(a.eq(b));
        }
    }

    #[test]
    fn sample_oversized() {
        let intervals = vec![
            Interval::new(10, 100),
            Interval::new(20, 200),
            Interval::new(30, 300),
            Interval::new(40, 400),
        ];
        let set = IntervalSet::from_sorted(intervals).unwrap();
        let sampled_set = set.sample(5);
        assert!(sampled_set.is_err());
    }
}
