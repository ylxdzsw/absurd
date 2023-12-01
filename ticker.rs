// TODO: allowing optional target iteration and show ETA

/// a helper struct to print iteration count and the elapsed time
#[cfg(feature = "std")]
pub struct Ticker {
    iter_count: usize,
    iter_per_print: usize,
    start_time: std::time::Instant,
}

#[cfg(feature = "std")]
impl Ticker {
    pub fn new(iter_per_print: usize) -> Self {
        Ticker { iter_count: 0, iter_per_print, start_time: std::time::Instant::now() }
    }

    pub fn tick(&mut self) {
        self.iter_count += 1;
        if self.iter_count % self.iter_per_print == 0 {
            eprintln!("{self}")
        }
    }
}

#[cfg(feature = "std")]
impl core::fmt::Display for Ticker {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "iter count: {}, speed: {} iter/s",
            self.iter_count,
            self.iter_count as f64 / self.start_time.elapsed().as_secs_f64()
        )
    }
}

#[cfg(feature = "std")]
impl Drop for Ticker {
    fn drop(&mut self) {
        eprintln!("{self}")
    }
}
