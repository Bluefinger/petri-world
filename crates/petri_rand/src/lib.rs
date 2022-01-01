use std::{
    ops::{Bound, RangeBounds},
    rc::Rc,
};

use crate::{entropy::generate_entropy, gen::WyRand};

mod entropy;
mod gen;

const SCALE: f64 = 2.0 * (1u64 << 63) as f64;

thread_local! {
    static PETRI: Rc<PetriRand> = Rc::new(PetriRand::with_seed(generate_entropy()));
}

#[derive(Debug, Default, Clone)]
#[repr(transparent)]
pub struct PetriRand {
    rng: WyRand,
}

macro_rules! index {
    ($bigger:tt) => {
        #[inline]
        pub fn index(&self, bounds: impl RangeBounds<usize>) -> usize {
            const BITS: $bigger = core::mem::size_of::<usize>() as $bigger * 8;
            let lower = match bounds.start_bound() {
                Bound::Included(lower) => *lower,
                Bound::Excluded(lower) => lower.saturating_add(1),
                Bound::Unbounded => usize::MIN,
            };
            let upper = match bounds.end_bound() {
                Bound::Included(upper) => upper.saturating_sub(lower).saturating_add(1),
                Bound::Excluded(upper) => upper.saturating_sub(lower),
                Bound::Unbounded => usize::MAX,
            };

            let mut value = usize::from_le_bytes(self.rng.rand());
            let mut m = (upper as $bigger).wrapping_mul(value as $bigger);
            if (m as usize) < upper {
                let t = (!upper + 1) % upper;
                while (m as usize) < t {
                    value = usize::from_le_bytes(self.rng.rand());
                    m = (upper as $bigger).wrapping_mul(value as $bigger);
                }
            }
            (m >> BITS) as usize + lower
        }
    };
}

impl PetriRand {
    #[inline]
    pub fn new() -> Self {
        Self {
            rng: WyRand::with_seed(PETRI.with(|t| t.get_u64())),
        }
    }

    #[inline]
    pub fn with_seed(seed: u64) -> Self {
        Self {
            rng: WyRand::with_seed(seed),
        }
    }

    #[inline]
    pub fn reseed(&self, seed: u64) {
        self.rng.reseed(seed);
    }

    #[inline]
    pub fn reseed_local(seed: u64) {
        PETRI.with(|t| t.reseed(seed));
    }

    #[inline]
    pub fn get_u64(&self) -> u64 {
        u64::from_le_bytes(self.rng.rand())
    }

    #[inline]
    pub fn get_u32(&self) -> u32 {
        let mut bytes = [0u8; core::mem::size_of::<u32>()];
        let random = self.rng.rand();
        let generated = random.len().min(core::mem::size_of::<u32>());
        bytes[..generated].copy_from_slice(&random[..generated]);
        u32::from_le_bytes(bytes)
    }

    #[inline]
    pub fn get_f32(&self) -> f32 {
        (self.get_u32() as f32) / (u32::MAX as f32)
    }

    #[inline]
    pub fn get_f32_normalised(&self) -> f32 {
        self.get_f32() * 2.0 - 1.0
    }

    #[inline]
    pub fn bool(&self) -> bool {
        self.rng.rand()[0] % 2 == 0
    }

    #[cfg(target_pointer_width = "16")]
    index!(u32);
    #[cfg(target_pointer_width = "32")]
    index!(u64);
    #[cfg(target_pointer_width = "64")]
    index!(u128);

    #[inline]
    pub fn chance(&self, rate: f64) -> bool {
        debug_assert!((0.0..=1.0).contains(&rate));

        let rate_int = (rate * SCALE) as u64;

        match rate_int {
            u64::MAX => true,
            _ => self.get_u64() < rate_int,
        }
    }

    #[inline]
    pub fn sample<'a, T>(&self, list: &'a [T]) -> Option<&'a T> {
        match list.len() {
            0 => None,
            // SOUND: Length already known to be 1, therefore index 0 will yield an item
            1 => unsafe { Some(list.get_unchecked(0)) },
            // SOUND: Range is exclusive, so yielded random values will always be a valid index and within bounds
            _ => unsafe { Some(list.get_unchecked(self.index(..list.len()))) },
        }
    }
}
