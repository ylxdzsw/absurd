mod io;
mod mem;
mod structs;
mod terminal;
mod utils;

pub use mem::*;
pub use structs::*;

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

pub use terminal::*;
