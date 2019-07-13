#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        #[cfg(debug)]
        {
            eprint!("[debug] ");
            eprintln!($($arg)*)
        }
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{ eprint!("[info] "); eprintln!($($arg)*) }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{ eprint!("[warn] "); eprintln!($($arg)*) }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{ eprint!("[error] "); eprintln!($($arg)*) }};
}