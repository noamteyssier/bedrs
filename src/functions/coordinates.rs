pub trait Coordinates<T> {
    fn start(&self) -> &T;
    fn end(&self) -> &T;
}
