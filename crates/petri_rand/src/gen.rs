use std::{
    cell::Cell,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    thread,
};

#[cfg(target_arch = "wasm32")]
use instant::Instant;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

pub trait GenRand {
    type Output;

    fn rand(&self) -> Self::Output;
}

#[inline]
fn generate_entropy() -> u64 {
    let mut hasher = DefaultHasher::new();
    Instant::now().hash(&mut hasher);
    thread::current().id().hash(&mut hasher);
    let hash = hasher.finish();
    (hash << 1) | 1
}

#[derive(Debug)]
pub struct WyRand {
    state: Cell<u64>,
}

impl WyRand {
    pub fn new() -> Self {
        Self {
            state: Cell::new(generate_entropy()),
        }
    }

    pub fn with_seed(seed: u64) -> Self {
        Self {
            state: Cell::new(seed << 1 | 1),
        }
    }

    pub fn reseed(&self, seed: u64) {
        self.state.set(seed << 1 | 1);
    }
}

impl GenRand for WyRand {
    type Output = [u8; core::mem::size_of::<u64>()];

    fn rand(&self) -> Self::Output {
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
        Self::new()
    }
}
