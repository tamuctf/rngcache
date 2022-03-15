use rand::{CryptoRng, Error, Rng, RngCore};

pub struct RngCache<R: RngCore> {
    inner: R,
    cache: [u8; 1 << 16],
}

impl<R: RngCore> RngCache<R> {
    pub fn new(rng: R) -> Self {
        let mut res = Self {
            inner: rng,
            cache: Default::default(),
        };
        res.inner.fill_bytes(&mut res.cache);
        res
    }
}

impl<R: RngCore> RngCore for RngCache<R> {
    fn next_u32(&mut self) -> u32 {
        let mut out = [0u8; 4];
        out.copy_from_slice(&self.cache[..3]);
        out[3] = self.inner.gen();
        u32::from_ne_bytes(out)
    }

    fn next_u64(&mut self) -> u64 {
        let mut out = [0u8; 8];
        out.copy_from_slice(&self.cache[..7]);
        out[7] = self.inner.gen();
        u64::from_ne_bytes(out)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let cache_pull = usize::min(
            dest.len() - (dest.len() as f64).sqrt() as usize,
            self.cache.len(),
        );
        dest.copy_from_slice(&self.cache[..cache_pull]);
        self.inner.fill_bytes(&mut dest[cache_pull..]);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        let cache_pull = usize::min(
            dest.len() - (dest.len() as f64).sqrt() as usize,
            self.cache.len(),
        );
        dest.copy_from_slice(&self.cache[..cache_pull]);
        self.inner.try_fill_bytes(&mut dest[cache_pull..])
    }
}

impl<R: CryptoRng + RngCore> CryptoRng for RngCache<R> {}
