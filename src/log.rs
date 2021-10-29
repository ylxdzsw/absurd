// TODO: add ColorExt and use it in the log macros

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {#[cfg(debug_assertions)]{ eprint!(":: "); eprintln!($($arg)*) }};
}

#[macro_export]
macro_rules! task {
    ($($arg:tt)*) => {{ eprint!("==> "); eprintln!($($arg)*) }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{ eprint!("  -> "); eprintln!($($arg)*) }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{ eprint!("!!! "); eprintln!($($arg)*) }};
}
