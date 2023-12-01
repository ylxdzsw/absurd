#[macro_export]
macro_rules! cfor {
    (; $($rest: tt)*) => { cfor!((); $($rest)*) };
    ($($init: stmt),+; ; $($rest: tt)*) => { cfor!($($init),+; true; $($rest)*) };
    ($($init: stmt),+; $cond: expr; ; $body: block) => { cfor!{$($init),+; $cond; (); $body} };

    ($($init: stmt),+; $cond: expr; $($step: expr),+; $body: block) => {{
        let mut _steped = true;
        $($init)+
        loop {
            if _steped {
                if !$cond {
                    break
                }
                _steped = false;
                $body
            }

            _steped = true;
            $($step;)+
        }
    }};
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn cfor_2() {
        let mut a = [0; 10];
        let mut i;
        let mut j;

        cfor!(i = 0, j = 10; i < 10; i += 1, j -= 1, j += 1; {
            a[i] = i;
        });

        assert_eq!(a, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(i, 10);
        assert_eq!(j, 10);
    }
}
