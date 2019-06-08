pub trait ReadExt: std::io::Read {
    fn read_to_end_alloc(&mut self) -> std::io::Result<Vec<u8>> {
        let mut buf = Vec::new();
        let n = self.read_to_end(&mut buf)?;
        buf.truncate(n);
        Ok(buf)
    }
}
