use rand::{ RngCore, SeedableRng };
use rand::Error as RandError;

// A linear congruential pseudo-random number generator, using the parameters
// found in Java's implementation. More on `java.util.Random` and glib's 
// `rand48()` (both are identical).
#[derive(Clone)]
pub struct Random {
    state: u64,
}

impl Random {
    #[inline]
    fn next_bits(&mut self, bit: usize) -> u64 {
        self.state = (self.state.wrapping_mul(0x5DEECE66D) + 0xB) & ((1 << 48) - 1);
        self.state >> (48 - bit)
    }
}

impl RngCore for Random {
    fn next_u32(&mut self) -> u32 {
        self.next_bits(32) as u32
    }

    fn next_u64(&mut self) -> u64 {
        (self.next_bits(32) << 32) + self.next_bits(32)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut current = 0;

        for x in 0..(dest.len() / 4) {
            current = self.next_bits(32);
            let current = current.to_le_bytes();
            let i = x * 4;
            dest[i..i + 4].copy_from_slice(&current[0..4])
        }

        if dest.len() % 4 > 0 {
            current = self.next_bits(32);
        }

        for x in 0..(dest.len() % 4) {
            dest[dest.len() / 4 * 4 + x] = current as u8;
            current >>= 8;
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), RandError> {
        Ok(self.fill_bytes(dest))
    }
}

impl SeedableRng for Random {
    type Seed = [u8; 8];

    fn from_seed(seed: Self::Seed) -> Self {
        let seed = u64::from_le_bytes(seed);
        Self::seed_from_u64(seed)
    }

    fn seed_from_u64(seed: u64) -> Self {
        Self {
            state: (seed.wrapping_mul(0x5DEECE66D) + 0xB) & ((1 << 48) - 1)
        }
    }
}
