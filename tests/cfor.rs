#![no_std]

use absurd::*;

mod cfor {
    use super::*;

    #[test]
    fn cfor() {
        let mut a = [0; 10];
        let mut i;

        cfor!(i = 0; i < 10; i += 1; {
            a[i] = i;
        });

        assert_eq!(a, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(i, 10);
    }
}