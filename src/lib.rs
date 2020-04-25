mod io;
mod mem;
mod structs;
mod syntax;
mod sys;
mod terminal;
mod utils;

pub use mem::*;
pub use structs::*;
pub use syntax::*;
pub use sys::*;

pub use io::ReadExt;
impl<R: std::io::Read + ?Sized> ReadExt for R {}

pub use utils::MonadExt;
impl<T> MonadExt for T {}

pub use utils::SizedMonadExt;
impl<T: Sized> SizedMonadExt for T {}

pub use utils::ResultExt;
impl<X, F> ResultExt for Result<X, F> {
    type S = X;
    fn msg<T>(self, x: T) -> Result<X, T> {
        self.map_err(|_| x)
    }
}
impl<X> ResultExt for Option<X> {
    type S = X;
    fn msg<T>(self, x: T) -> Result<X, T> {
        self.ok_or(x)
    }
}

pub use utils::PrintableResultExt;
impl<T, E: std::fmt::Debug> PrintableResultExt for Result<T, E> {
    fn warn(self) -> Self {
        if let Err(e) = &self {
            warn!("{:?}", e)
        }
        self
    }
}

pub use utils::UnwrapUnchecked;
impl<T, E> UnwrapUnchecked for Result<T, E> {
    type O = T;
    unsafe fn unwrap_unchecked(self) -> T {
        self.unwrap_or_else(|_| core::hint::unreachable_unchecked())
    }
}
impl<T> UnwrapUnchecked for Option<T> {
    type O = T;
    unsafe fn unwrap_unchecked(self) -> T {
        self.unwrap_or_else(|| core::hint::unreachable_unchecked())
    }
}

pub use terminal::*;
