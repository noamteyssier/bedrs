use super::Coordinates;

pub trait Container<T, I: Coordinates<T>> {
    fn records(&self) -> &Vec<I>;
    fn len(&self) -> usize {
        self.records().len()
    }
}
