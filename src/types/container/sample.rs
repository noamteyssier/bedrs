use crate::{
    traits::{ChromBounds, IntervalBounds, SetError, ValueBounds},
    IntervalContainer,
};
use rand::{seq::SliceRandom, RngCore, SeedableRng};
use rand_chacha::ChaChaRng;

/// Utility functions for random sampling within a container.
impl<I, C, T> IntervalContainer<I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    /// Shuffles the elements of the container in place using the given random number generator.
    ///
    /// # Example
    /// ```
    /// use bedrs::{BaseInterval, IntervalContainer};
    /// use rand::thread_rng;
    ///
    /// let intervals = vec![
    ///     BaseInterval::new(10, 100),
    ///     BaseInterval::new(20, 200),
    ///     BaseInterval::new(30, 300),
    ///     BaseInterval::new(40, 400),
    /// ];
    /// let mut set = IntervalContainer::from_sorted(intervals).unwrap();
    /// set.shuffle_rng(&mut thread_rng());
    /// ```
    pub fn shuffle_rng(&mut self, rng: &mut impl RngCore) {
        self.records_mut().shuffle(rng);
        self.set_unsorted();
    }

    /// Shuffles the elements of the container in place.
    ///
    /// # Example
    /// ```
    /// use bedrs::{BaseInterval, IntervalContainer};
    ///
    /// let intervals = vec![
    ///    BaseInterval::new(10, 100),
    ///    BaseInterval::new(20, 200),
    ///    BaseInterval::new(30, 300),
    ///    BaseInterval::new(40, 400),
    /// ];
    /// let mut set = IntervalContainer::from_sorted(intervals).unwrap();
    /// set.shuffle();
    /// ```
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.shuffle_rng(&mut rng);
    }

    /// Shuffles the elements of the container in place using the given seed.
    ///
    /// # Example
    /// ```
    /// use bedrs::{BaseInterval, IntervalContainer};
    ///
    /// let intervals = vec![
    ///    BaseInterval::new(10, 100),
    ///    BaseInterval::new(20, 200),
    ///    BaseInterval::new(30, 300),
    ///    BaseInterval::new(40, 400),
    /// ];
    /// let mut set = IntervalContainer::from_sorted(intervals).unwrap();
    /// set.shuffle_seed(42);
    /// ```
    pub fn shuffle_seed(&mut self, seed: u64) {
        let mut rng = ChaChaRng::seed_from_u64(seed);
        self.shuffle_rng(&mut rng);
    }

    /// Returns a new container with the elements of the container in random order using the given random number generator.
    ///
    /// # Example
    /// ```
    /// use bedrs::{BaseInterval, IntervalContainer};
    ///
    /// let intervals = vec![
    ///     BaseInterval::new(10, 100),
    ///     BaseInterval::new(20, 200),
    ///     BaseInterval::new(30, 300),
    ///     BaseInterval::new(40, 400),
    /// ];
    /// let set = IntervalContainer::from_sorted(intervals).unwrap();
    /// let mut rng = rand::thread_rng();
    /// let shuffled_set = set.sample_rng(2, &mut rng).unwrap();
    /// assert_eq!(shuffled_set.len(), 2);
    /// ```
    pub fn sample_rng(&self, n: usize, rng: &mut impl RngCore) -> Result<Self, SetError> {
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
    /// use bedrs::{BaseInterval, IntervalContainer};
    ///
    /// let intervals = vec![
    ///     BaseInterval::new(10, 100),
    ///     BaseInterval::new(20, 200),
    ///     BaseInterval::new(30, 300),
    ///     BaseInterval::new(40, 400),
    /// ];
    /// let set = IntervalContainer::from_sorted(intervals).unwrap();
    /// let shuffled_set = set.sample(2).unwrap();
    /// assert_eq!(shuffled_set.len(), 2);
    /// ```
    pub fn sample(&self, n: usize) -> Result<Self, SetError> {
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
    /// use bedrs::{BaseInterval, IntervalContainer};
    ///
    /// let intervals = vec![
    ///     BaseInterval::new(10, 100),
    ///     BaseInterval::new(20, 200),
    ///     BaseInterval::new(30, 300),
    ///     BaseInterval::new(40, 400),
    /// ];
    /// let set = IntervalContainer::from_sorted(intervals).unwrap();
    /// let shuffled_set_a = set.sample_seed(2, 42).unwrap();
    /// let shuffled_set_b = set.sample_seed(2, 42).unwrap();
    /// assert_eq!(shuffled_set_a.len(), 2);
    /// assert_eq!(shuffled_set_b.len(), 2);
    /// ```
    pub fn sample_seed(&self, n: usize, seed: u64) -> Result<Self, SetError> {
        if n > self.records().len() {
            return Err(SetError::SampleSizeTooLarge);
        }
        let mut rng = ChaChaRng::seed_from_u64(seed);
        self.sample_rng(n, &mut rng)
    }

    /// Returns a new iterator over the elements of the container in random order using the
    /// given random number generator.
    ///
    /// # Example
    /// ```
    /// use bedrs::{BaseInterval, IntervalContainer};
    ///
    /// let intervals = vec![
    ///    BaseInterval::new(10, 100),
    ///    BaseInterval::new(20, 200),
    ///    BaseInterval::new(30, 300),
    ///    BaseInterval::new(40, 400),
    /// ];
    /// let set = IntervalContainer::from_sorted(intervals).unwrap();
    /// let mut rng = rand::thread_rng();
    /// let shuffled_iter = set.sample_iter_rng(2, &mut rng).unwrap();
    /// assert_eq!(shuffled_iter.count(), 2);
    /// ```
    pub fn sample_iter_rng<'a>(
        &'a self,
        n: usize,
        rng: &mut impl RngCore,
    ) -> Result<Box<dyn Iterator<Item = &I> + 'a>, SetError> {
        if n > self.records().len() {
            return Err(SetError::SampleSizeTooLarge);
        }
        let iter = self.records().choose_multiple(rng, n);
        Ok(Box::new(iter))
    }

    /// Returns a new iterator over the elements of the container in random order
    ///
    /// # Example
    /// ```
    /// use bedrs::{BaseInterval, IntervalContainer};
    ///
    /// let intervals = vec![
    ///    BaseInterval::new(10, 100),
    ///    BaseInterval::new(20, 200),
    ///    BaseInterval::new(30, 300),
    ///    BaseInterval::new(40, 400),
    /// ];
    /// let set = IntervalContainer::from_sorted(intervals).unwrap();
    /// let shuffled_iter = set.sample_iter(2).unwrap();
    /// assert_eq!(shuffled_iter.count(), 2);
    pub fn sample_iter<'a>(
        &'a self,
        n: usize,
    ) -> Result<Box<dyn Iterator<Item = &I> + 'a>, SetError> {
        if n > self.records().len() {
            return Err(SetError::SampleSizeTooLarge);
        }
        let mut rng = rand::thread_rng();
        self.sample_iter_rng(n, &mut rng)
    }

    /// Returns a new iterator over the elements of the container in random order using the
    /// given seed.
    ///
    /// # Example
    /// ```
    /// use bedrs::{BaseInterval, IntervalContainer};
    ///
    /// let intervals = vec![
    ///    BaseInterval::new(10, 100),
    ///    BaseInterval::new(20, 200),
    ///    BaseInterval::new(30, 300),
    ///    BaseInterval::new(40, 400),
    /// ];
    /// let set = IntervalContainer::from_sorted(intervals).unwrap();
    /// let shuffled_iter = set.sample_iter_seed(2, 42).unwrap();
    /// assert_eq!(shuffled_iter.count(), 2);
    /// ```
    pub fn sample_iter_seed<'a>(
        &'a self,
        n: usize,
        seed: u64,
    ) -> Result<Box<dyn Iterator<Item = &I> + 'a>, SetError> {
        if n > self.records().len() {
            return Err(SetError::SampleSizeTooLarge);
        }
        let mut rng = ChaChaRng::seed_from_u64(seed);
        self.sample_iter_rng(n, &mut rng)
    }
}

#[cfg(test)]
mod testing {
    use crate::{BaseInterval, Coordinates, IntervalContainer};

    #[test]
    fn shuffle_rng() {
        let intervals = vec![
            BaseInterval::new(10, 100),
            BaseInterval::new(20, 200),
            BaseInterval::new(30, 300),
            BaseInterval::new(40, 400),
            BaseInterval::new(10, 100),
            BaseInterval::new(20, 200),
            BaseInterval::new(30, 300),
            BaseInterval::new(40, 400),
        ];
        let set = IntervalContainer::new(intervals);
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
            BaseInterval::new(10, 100),
            BaseInterval::new(20, 200),
            BaseInterval::new(30, 300),
            BaseInterval::new(40, 400),
            BaseInterval::new(10, 100),
            BaseInterval::new(20, 200),
            BaseInterval::new(30, 300),
            BaseInterval::new(40, 400),
        ];
        let set = IntervalContainer::new(intervals);
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
            BaseInterval::new(10, 100),
            BaseInterval::new(20, 200),
            BaseInterval::new(30, 300),
            BaseInterval::new(40, 400),
        ];
        let mut set = IntervalContainer::from_sorted(intervals).unwrap();
        assert!(set.is_sorted());
        set.shuffle();
        assert!(!set.is_sorted());
    }

    #[test]
    fn sample() {
        let intervals = vec![
            BaseInterval::new(10, 100),
            BaseInterval::new(20, 200),
            BaseInterval::new(30, 300),
            BaseInterval::new(40, 400),
        ];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let sampled_set = set.sample(4).unwrap();
        assert_eq!(sampled_set.records().len(), 4);
        assert!(!sampled_set.is_sorted());
    }

    #[test]
    fn sample_seed() {
        let intervals = vec![
            BaseInterval::new(10, 100),
            BaseInterval::new(20, 200),
            BaseInterval::new(30, 300),
            BaseInterval::new(40, 400),
        ];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let sampled_set_a = set.sample_seed(4, 0).unwrap();
        let sampled_set_b = set.sample_seed(4, 0).unwrap();
        for (a, b) in sampled_set_a.records().iter().zip(sampled_set_b.records()) {
            assert!(a.eq(b));
        }
    }

    #[test]
    fn sample_oversized() {
        let intervals = vec![
            BaseInterval::new(10, 100),
            BaseInterval::new(20, 200),
            BaseInterval::new(30, 300),
            BaseInterval::new(40, 400),
        ];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let sampled_set = set.sample(5);
        assert!(sampled_set.is_err());
    }

    #[test]
    fn sample_iter() {
        let intervals = vec![
            BaseInterval::new(10, 100),
            BaseInterval::new(20, 200),
            BaseInterval::new(30, 300),
            BaseInterval::new(40, 400),
        ];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let sampled_iter = set.sample_iter(2).unwrap();
        assert_eq!(sampled_iter.count(), 2);
    }

    #[test]
    fn sample_iter_seed() {
        let intervals = vec![
            BaseInterval::new(10, 100),
            BaseInterval::new(20, 200),
            BaseInterval::new(30, 300),
            BaseInterval::new(40, 400),
        ];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let sampled_iter_a = set.sample_iter_seed(2, 0).unwrap();
        let sampled_iter_b = set.sample_iter_seed(2, 0).unwrap();
        for (a, b) in sampled_iter_a.zip(sampled_iter_b) {
            assert!(a.eq(b));
        }
    }

    #[test]
    fn sample_iter_oversized() {
        let intervals = vec![
            BaseInterval::new(10, 100),
            BaseInterval::new(20, 200),
            BaseInterval::new(30, 300),
            BaseInterval::new(40, 400),
        ];
        let set = IntervalContainer::from_sorted(intervals).unwrap();
        let sampled_iter = set.sample_iter(5);
        assert!(sampled_iter.is_err());

        let sampled_iter_seed = set.sample_iter_seed(5, 0);
        assert!(sampled_iter_seed.is_err());
    }
}
