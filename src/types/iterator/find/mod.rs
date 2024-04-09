mod sorted;
mod unsorted;

pub use sorted::{FindIterSorted, FindIterSortedOwned};
pub use unsorted::{FindIter, FindIterOwned};

#[cfg(test)]
mod testing {
    use crate::{BaseInterval, Coordinates};

    #[test]
    fn test_f_len_a() {
        let iv = BaseInterval::new(0, 100);
        let frac = 0.5;
        let len = iv.f_len(frac);
        assert_eq!(len, 50);
    }

    #[test]
    fn test_f_len_b() {
        let iv = BaseInterval::new(0, 100);
        let frac = 0.3;
        let len = iv.f_len(frac);
        assert_eq!(len, 30);
    }

    #[test]
    fn test_f_len_c() {
        let iv = BaseInterval::new(0, 100);
        let frac = 0.301;
        let len = iv.f_len(frac);
        assert_eq!(len, 30);
    }

    #[test]
    fn test_f_len_d() {
        let iv = BaseInterval::new(0, 100);
        let frac = 0.299;
        let len = iv.f_len(frac);
        assert_eq!(len, 30);
    }
}
