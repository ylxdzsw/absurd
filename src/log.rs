#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{ eprint!("[debug] "); eprintln!(format_args!($($arg)*)) }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{ eprint!("[info] "); eprintln!(format_args!($($arg)*)) }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{ eprint!("[warn] "); eprintln!(format_args!($($arg)*)) }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{ eprint!("[error] "); eprintln!(format_args!($($arg)*)) }};
}