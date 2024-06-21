use super::subtree::Subtree;
use crate::traits::{ChromBounds, IntervalBounds, SetError};
use hashbrown::HashMap;

type Map<I, C> = HashMap<C, Subtree<I, C>>;

#[derive(Debug, Clone, Default)]
pub struct IntervalTree<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    map: Map<I, C>,
    is_sorted: bool,
}
impl<I, C> FromIterator<I> for IntervalTree<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    fn from_iter<It: IntoIterator<Item = I>>(iter: It) -> Self {
        let mut map = Map::new();
        for iv in iter {
            if !map.contains_key(iv.chr()) {
                map.insert(iv.chr().clone(), Subtree::empty());
            }
            map.get_mut(iv.chr()).unwrap().insert(iv);
        }
        Self {
            map,
            is_sorted: false,
        }
    }
}

impl<I, C> IntervalTree<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    pub fn new() -> Self {
        Self {
            map: Map::new(),
            is_sorted: false,
        }
    }

    pub fn from_map(map: Map<I, C>) -> Self {
        Self {
            map,
            is_sorted: false,
        }
    }

    pub fn map(&self) -> &Map<I, C> {
        &self.map
    }

    pub fn mut_map(&mut self) -> &mut Map<I, C> {
        &mut self.map
    }

    pub fn is_sorted(&self) -> bool {
        self.is_sorted
    }

    pub fn set_unsorted(&mut self) {
        self.is_sorted = false;
    }

    pub fn set_sorted(&mut self) {
        self.map.values_mut().for_each(Subtree::set_sorted);
        self.is_sorted = true;
    }

    /// Number of total intervals in all subtrees
    pub fn len(&self) -> usize {
        self.map.values().map(|x| x.len()).sum()
    }

    /// Retrieve a specific subtree
    pub fn subtree(&self, name: &C) -> Option<&Subtree<I, C>> {
        self.map.get(name)
    }

    /// Retrieve a mutable reference to a specific subtree
    pub fn subtree_mut(&mut self, name: &C) -> Option<&mut Subtree<I, C>> {
        self.map.get_mut(name)
    }

    /// Transfer ownership of a subtree
    pub fn subtree_owned(&mut self, name: &C) -> Option<Subtree<I, C>> {
        self.map.remove(name)
    }

    /// Number of subtrees in the tree
    pub fn num_subtrees(&self) -> usize {
        self.map.len()
    }

    pub fn subtree_names(&self) -> Vec<&C> {
        self.map.keys().collect()
    }

    pub fn subtree_names_sorted(&self) -> Vec<&C> {
        let mut names = self.subtree_names();
        names.sort_unstable();
        names
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Sorts the internal interval vector on the chromosome and start position of the intervals.
    pub fn sort(&mut self) {
        self.map.values_mut().for_each(Subtree::sort);
        self.set_sorted();
    }

    /// Sorts the internal interval vector on the chromosome and start position of the intervals.
    /// but parallelizes the sorting.
    #[cfg(feature = "rayon")]
    pub fn par_sort(&mut self) {
        self.map.values_mut().for_each(Subtree::par_sort);
        self.set_sorted();
    }

    /// Inserts an interval into the tree
    ///
    /// 1. checks if the chromosome key exists
    /// 2. if not, creates an empty subtree
    /// 3. inserts the interval into the subtree
    pub fn insert_interval(&mut self, record: I) {
        if !self.map.contains_key(record.chr()) {
            self.map.insert(record.chr().clone(), Subtree::empty());
        }
        self.map.get_mut(record.chr()).unwrap().insert(record);
    }

    pub fn insert_subtree(&mut self, name: C, subtree: Subtree<I, C>) {
        self.map.insert(name, subtree);
    }

    /// Applies a mutable function to each interval in the container
    pub fn apply_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut I),
    {
        for tree in self.map.values_mut() {
            tree.apply_mut(&f);
        }
    }

    /// Applies a mutable function to each subtree in the container
    pub fn apply_subtree_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut Subtree<I, C>),
    {
        for tree in self.map.values_mut() {
            f(tree);
        }
    }

    pub fn values(&self) -> impl Iterator<Item = &Subtree<I, C>> {
        self.map.values()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut Subtree<I, C>> {
        self.map.values_mut()
    }

    pub fn span(&self, name: &C) -> Option<Result<I, SetError>> {
        let subtree = self.map.get(name)?;
        Some(subtree.span())
    }
}
