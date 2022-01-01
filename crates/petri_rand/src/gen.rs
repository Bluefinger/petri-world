use std::cell::Cell;

#[derive(Debug)]
#[repr(transparent)]
pub struct WyRand {
    state: Cell<u64>,
}

impl WyRand {
    #[inline]
    pub fn with_seed(seed: u64) -> Self {
        Self {
            state: Cell::new(seed << 1 | 1),
        }
    }

    #[inline]
    pub fn reseed(&self, seed: u64) {
        self.state.set(seed << 1 | 1);
    }

    #[inline]
    pub fn rand(&self) -> [u8; core::mem::size_of::<u64>()] {
        let state = self.state.get().wrapping_add(0xa0761d6478bd642f);
        self.state.set(state);
        let t: u128 = (state as u128).wrapping_mul((state ^ 0xe7037ed1a0b428db) as u128);
        let ret = (t.wrapping_shr(64) ^ t) as u64;
        ret.to_le_bytes()
    }
}

impl Clone for WyRand {
    fn clone(&self) -> Self {
        Self {
            state: Cell::new(u64::from_le_bytes(self.rand())),
        }
    }
}

impl Default for WyRand {
    fn default() -> Self {
        Self::with_seed(Default::default())
    }
}
