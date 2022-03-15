use rand::{CryptoRng, Error, RngCore};

pub struct RngCache<R: RngCore> {
    inner: R,
    cache: [u8; 1 << 16],
}

impl<R: RngCore> RngCache<R> {
    pub fn new(rng: R) -> Self {
        let mut res = Self {
            inner: rng,
            cache: [0u8; 1 << 16],
        };
        res.inner.fill_bytes(&mut res.cache);
        res
    }
}

impl<R: RngCore> RngCore for RngCache<R> {
    fn next_u32(&mut self) -> u32 {
        self.inner.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.inner.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let cache_pull = usize::min(
            (dest.len() / 2).checked_sub(2).unwrap_or(0),
            self.cache.len(),
        );
        dest[..cache_pull].copy_from_slice(&self.cache[..cache_pull]);
        self.inner.fill_bytes(&mut dest[cache_pull..]);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        let cache_pull = usize::min(
            (dest.len() / 2).checked_sub(2).unwrap_or(0),
            self.cache.len(),
        );
        dest[..cache_pull].copy_from_slice(&self.cache[..cache_pull]);
        self.inner.try_fill_bytes(&mut dest[cache_pull..])
    }
}

impl<R: CryptoRng + RngCore> CryptoRng for RngCache<R> {}
