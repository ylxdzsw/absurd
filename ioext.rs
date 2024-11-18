#[cfg(feature = "std")]
pub trait ReadExt: std::io::Read {
    /// Allocate a `Vec<u8>` and read all bytes until EOF in this source into it.
    fn read_to_end_alloc(&mut self) -> std::io::Result<Vec<u8>> {
        self.read_to_end_alloc_with_capacity(0)
    }

    /// Allocate a `Vec<u8>` with specified initial capacity and read all bytes until EOF in this source into it.
    fn read_to_end_alloc_with_capacity(&mut self, capacity: usize) -> std::io::Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(capacity);
        self.read_to_end(&mut buf)?;
        Ok(buf)
    }

    /// Allocate a `String` and read all bytes until EOF in this source into it.
    fn read_to_string_alloc(&mut self) -> std::io::Result<String> {
        self.read_to_string_alloc_with_capacity(0)
    }

    /// Allocate a `Vec<u8>` with specified initial capacity and read all bytes until EOF in this source into it.
    fn read_to_string_alloc_with_capacity(&mut self, capacity: usize) -> std::io::Result<String> {
        let mut buf = String::with_capacity(capacity);
        self.read_to_string(&mut buf)?;
        Ok(buf)
    }

    /// Allocate a `Box<[u8]>` and read exactly `n` bytes in this source into it.
    fn read_exact_alloc(&mut self, n: usize) -> std::io::Result<Box<[u8]>> {
        // (read_buf #78485)
        // let buf: Box<[_]> = Box::new_uninit_slice(n);
        let mut buf = vec![0; n].into_boxed_slice();
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
}

#[cfg(feature = "std")]
impl<R: std::io::Read + ?Sized> ReadExt for R {}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_TEXT: &[u8] = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod.".as_bytes();

    #[test]
    fn read_to_end_alloc() {
        let mut source = SAMPLE_TEXT;
        let result = source.read_to_end_alloc().unwrap();
        assert_eq!(&result, SAMPLE_TEXT)
    }

    #[test]
    fn read_to_string_alloc() {
        let mut source = SAMPLE_TEXT;
        let result = source.read_to_string_alloc().unwrap();
        assert_eq!(result.as_bytes(), SAMPLE_TEXT)
    }

    #[test]
    fn read_exact_alloc() {
        let mut source = SAMPLE_TEXT;
        let result = source.read_exact_alloc(32).unwrap();
        assert_eq!(result[..], SAMPLE_TEXT[..32]);
        assert_eq!(source[..], SAMPLE_TEXT[32..])
    }
}
