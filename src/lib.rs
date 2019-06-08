mod io;

pub use io::ReadExt;
impl<R: std::io::Read + ?Sized> ReadExt for R {}
