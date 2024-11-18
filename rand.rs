pub struct Xorshift32 {
    state: u32,
}

impl Xorshift32 {
    pub fn new(seed: u32) -> Self {
        Self { state: seed }
    }

    #[cfg(feature = "std")]
    pub fn new_with_system_timestamp() -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        Self::new(timestamp as u32)
    }

    pub fn gen_u32(&mut self) -> u32 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
        self.state
    }

    #[cfg(target_pointer_width = "32")]
    pub fn gen_usize(&mut self) -> usize {
        self.gen_u32() as usize
    }

    #[cfg(target_pointer_width = "64")]
    pub fn gen_usize(&mut self) -> usize {
        let a = self.gen_u32() as usize;
        let b = self.gen_u32() as usize;
        (a << 32) | b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xorshift32() {
        let mut rng = Xorshift32::new(39393);
        let a = rng.gen_u32();
        let b = rng.gen_u32();
        assert_ne!(a, b);
    }

    #[test]
    fn test_xorshift32_usize() {
        let x = Xorshift32::new(39393).gen_usize();
        assert!(x > u32::MAX as usize);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_xorshift32_with_system_timestamp() {
        let a = Xorshift32::new_with_system_timestamp();
        std::thread::sleep(std::time::Duration::from_millis(2));
        let b = Xorshift32::new_with_system_timestamp();
        assert_ne!(a.state, b.state);
    }
}
