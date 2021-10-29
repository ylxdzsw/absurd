use super::*;

#[cfg(feature = "std")]
pub trait ReadExt: std::io::Read {
    /// Allocate a Vec<u8> and read all bytes until EOF in this source into it.
    fn read_to_end_alloc(&mut self) -> std::io::Result<alloc::vec::Vec<u8>> {
        self.read_to_end_alloc_with_capacity(0)
    }

    /// Allocate a Vec<u8> with specified initial capacity and read all bytes until EOF in this source into it.
    fn read_to_end_alloc_with_capacity(&mut self, capacity: usize) -> std::io::Result<alloc::vec::Vec<u8>> {
        let mut buf = alloc::vec::Vec::with_capacity(capacity);
        self.read_to_end(&mut buf)?;
        Ok(buf)
    }

    /// Allocate a String and read all bytes until EOF in this source into it.
    fn read_to_string_alloc(&mut self) -> std::io::Result<alloc::string::String> {
        self.read_to_string_alloc_with_capacity(0)
    }

    /// Allocate a Vec<u8> with specified initial capacity and read all bytes until EOF in this source into it.
    fn read_to_string_alloc_with_capacity(&mut self, capacity: usize) -> std::io::Result<alloc::string::String> {
        let mut buf = alloc::string::String::with_capacity(capacity);
        self.read_to_string(&mut buf)?;
        Ok(buf)
    }

    /// Allocate a Vec<u8> and read exact `n` bytes in this source into it.
    fn read_exact_alloc(&mut self, n: usize) -> std::io::Result<alloc::vec::Vec<u8>> {
        let mut buf = alloc::vec::Vec::with_capacity(n);
        buf.set_len_uninit_primitive(n);
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
}

#[cfg(feature = "std")]
impl<R: std::io::Read + ?Sized> ReadExt for R {}
