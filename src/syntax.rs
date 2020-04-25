#[macro_export]
macro_rules! cfor {
    (; $($rest: tt)*) => { cfor!((); $($rest)*) };
    ($($init: stmt),+; ; $($rest: tt)*) => { cfor!($($init),+; true; $($rest)*) };
    ($($init: stmt),+; $cond: expr; ; $body: block) => { cfor!{$($init),+; $cond; (); $body} };

    ($($init: stmt),+; $cond: expr; $($step: expr),+; $body: block) => {{
        let mut _steped = true;
        $($init;)+
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