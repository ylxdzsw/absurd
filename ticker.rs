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


#[derive(Default)]
pub struct RunningAverage {
    count: usize,
    mean: f64,
    m2: f64
}

impl RunningAverage {
    pub fn new() -> Self {
        Self { count: 0, mean: 0., m2: 0. }
    }

    pub fn observe(&mut self, x: f64) {
        self.count += 1;
        let delta = x - self.mean;
        self.mean += delta / self.count as f64;
        let delta2 = x - self.mean;
        self.m2 += delta * delta2;
    }

    /// return mean and variance
    pub fn get(&self) -> (f64, f64) {
        (self.mean, self.m2 / self.count as f64)
    }

    /// return mean and variance, reset the statistic
    pub fn take(&mut self) -> (f64, f64) {
        core::mem::take(self).get()
    }
}
