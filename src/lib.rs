mod io;
mod utils;

pub use io::ReadExt;
impl<R: std::io::Read + ?Sized> ReadExt for R {}

pub use utils::MonadExt;
impl<T> MonadExt for T {}

pub use utils::SizedMonadExt;
impl<T: Sized> SizedMonadExt for T {}