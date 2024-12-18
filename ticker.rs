// TODO: allowing optional target iteration and show ETA

/// a helper struct to print iteration count and the elapsed time
#[cfg(feature = "std")]
pub struct Ticker {
    iter_per_print: usize,

    iter_count: usize,
    start_time: std::time::Instant,
    last_tick: std::time::Instant
}

#[cfg(feature = "std")]
impl Ticker {
    pub fn new(iter_per_print: usize) -> Self {
        let now = std::time::Instant::now();
        Ticker { iter_count: 0, iter_per_print, start_time: now, last_tick: now }
    }

    pub fn tick(&mut self) -> core::time::Duration {
        let now = std::time::Instant::now();
        let elapsed = now - self.last_tick;
        self.last_tick = now;

        self.iter_count += 1;
        if self.iter_per_print != 0 && self.iter_count % self.iter_per_print == 0 {
            eprintln!("{self}")
        }

        elapsed
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
        if self.iter_per_print != 0 {
            eprintln!("{self}")
        }
    }
}
