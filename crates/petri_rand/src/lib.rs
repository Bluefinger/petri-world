use fastrand::*;

const SCALE: f64 = 2.0 * (1u64 << 63) as f64;

#[derive(Debug, Default, Clone)]
pub struct PetriRand {
    rng: Rng,
}

impl PetriRand {
    #[inline]
    pub fn new() -> Self {
        Self { rng: Rng::new() }
    }

    #[inline]
    pub fn with_seed(seed: u64) -> Self {
        Self { rng: Rng::with_seed(seed) }
    }

    #[inline]
    pub fn get_u64(&self) -> u64 {
        self.rng.u64(..)
    }

    #[inline]
    pub fn get_f32(&self) -> f32 {
        self.rng.f32()
    }

    #[inline]
    pub fn get_f32_normalised(&self) -> f32 {
        self.rng.f32() * 2.0 - 1.0
    }

    #[inline]
    pub fn bool(&self) -> bool {
        self.rng.bool()
    }

    #[inline]
    pub fn chance(&self, rate: f64) -> bool {
        debug_assert!((0.0..=1.0).contains(&rate));

        let rate_int = (rate * SCALE) as u64;

        match rate_int {
            u64::MAX => true,
            _ => self.get_u64() < rate_int
        }
    }

    #[inline]
    pub fn sample<'a, T>(&self, list: &'a [T]) -> Option<&'a T> {
        match list.len() {
            0 => None,
            // SOUND: Length already known to be 1, therefore index 0 will yield an item
            1 => unsafe { Some(list.get_unchecked(0)) },
            // SOUND: Range is exclusive, so yielded random values will always be a valid index and within bounds
            _ => unsafe { Some(list.get_unchecked(self.rng.usize(..list.len()))) }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
