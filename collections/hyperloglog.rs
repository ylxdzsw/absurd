use core::hash::{BuildHasher, Hash, Hasher};

// we use 30 bits of the hash output
// 14 bits are used for indexing 16384 registers, providing 1.04/sqrt(16384) = 0.8125% standard errors
// 15 bits are used for estimate the numbers in a 4 bit register
// todo: 15 bits are a bit insufficient, but more bits makes packing more challenging. Need a bitvec helper.
// todo: making them configurable after generic const expr stablized?

pub struct HyperLogLog<H: BuildHasher> {
    build_hasher: H,
    registers: [u8; 8192] // 2 registers are packed into a u8
}

#[cfg(feature = "std")]
impl Default for HyperLogLog<std::hash::RandomState> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<H: BuildHasher> HyperLogLog<H> {
    pub fn new(build_hasher: H) -> Self {
        Self {
            build_hasher,
            registers: [0; 8192]
        }
    }

    pub fn insert(&mut self, v: impl Hash) {
        let x = {
            let mut hasher = self.build_hasher.build_hasher();
            v.hash(&mut hasher);
            hasher.finish()
        };

        let j = (x >> 15) & 0b0011_1111_1111_1111;
        let w = x as u16 & 0b0111_1111_1111_1111; // 15 bits
        let rho = w.leading_zeros() as u8; // at least one for more accurate linear counting

        let register = &mut self.registers[(j >> 1) as usize];
        let shift = if j & 1 == 1 { 0 } else { 4 };
        let mask = 0b1111 << shift;
        let old = (*register & mask) >> shift;
        let new = old.max(rho);
        *register = (*register & !mask) | (new << shift);
    }

    pub fn count(&self) -> usize {
        let alpha = 0.7213 / (1.0 + 1.079 / 16384.);

        let z = (0..16384).map(|j| {
            let register = &self.registers[(j >> 1) as usize];
            let shift = if j & 1 == 1 { 0 } else { 4 };
            let mask = 0b1111 << shift;
            let r = (*register & mask) >> shift;
            1. / 2i32.pow(r as _) as f32
        }).sum::<f32>();

        let mut raw = alpha * 16384. * 16384. / z;

        #[cfg(feature = "std")]
        if raw <= 2.5 * 16384. { // linear counting for small cardinality.
            let zeros = self.registers.iter().map(|&r| {
                (r & 0b0000_1111 == 0) as i32 + (r & 0b1111_0000 == 0) as i32
            }).sum::<i32>();
            raw = 16384. * (16384. / zeros as f32).ln()
        }

        raw as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "std")]
    #[test]
    fn test_hyperloglog() {
        let mut hll: HyperLogLog<_> = Default::default();
        assert!(hll.count() == 0);

        for i in 0..1000 {
            hll.insert(i);
        }
        assert!((hll.count() as i64 - 1000).abs() < 25);

        for i in 0..10000 {
            hll.insert(i);
        }
        assert!((hll.count() as i64 - 10000).abs() < 244);

        for i in 0..100000 {
            hll.insert(i);
        }
        assert!((hll.count() as i64 - 100000).abs() < 2438);

        for i in 0..100000 {
            hll.insert(i);
        }
        assert!((hll.count() as i64 - 100000).abs() < 2438);
    }
}
