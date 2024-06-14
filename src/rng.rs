use core::ops::Deref;

use esp_hal::rng::Rng;
use rand_core::{CryptoRng, RngCore};

pub struct EspRng(pub Rng);

impl RngCore for EspRng {
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.0.try_fill_bytes(dest)
    }
}

impl CryptoRng for EspRng {}

impl Deref for EspRng {
    type Target = Rng;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
