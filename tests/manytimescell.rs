#![no_std]

use absurd::*;

mod cfor {
    use super::*;

    #[test]
    fn manytimescell() {
        let cell = ManyTimesCell::new([1,2,3,4]);

        cell.get_mut().iter_mut().for_each(|x| *x += 1);
        cell.get_mut().iter_mut().for_each(|x| *x += 1);

        assert_eq!(cell.get(), &[3,4,5,6]);
    }

    #[test]
    #[should_panic]
    fn manytimescell_concurrent_mut() {
        let cell = ManyTimesCell::new([1,2,3,4]);

        let mut mutref_a = cell.get_mut();
        let mut mutref_b = cell.get_mut();

        mutref_a.iter_mut().for_each(|x| *x += 1);
        mutref_b.iter_mut().for_each(|x| *x += 1);
    }

    #[test]
    #[should_panic]
    fn manytimescell_mut_after_use() {
        let cell = ManyTimesCell::new([1,2,3,4]);

        let _ = cell.get();
        let mut mutref = cell.get_mut();

        mutref.iter_mut().for_each(|x| *x += 1);
    }
}
