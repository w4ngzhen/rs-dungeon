use std::ops::Range;
use rand::{Rng, RngCore, SeedableRng};
use rand_pcg::{Lcg128Xsl64, Pcg64};

pub struct RandGen {
    rng: Lcg128Xsl64,
}

impl RandGen {
    pub fn new(seed: Option<u64>) -> RandGen {
        let rng_seed = if let Some(seed) = seed {
            seed
        } else {
            0
        };
        let rng = Pcg64::seed_from_u64(rng_seed);
        RandGen {
            rng
        }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    pub fn range(&mut self, from: u64, to: u64) -> u64 {
        let range: Range<u64> = if from <= to {
            from..to
        } else {
            to..from
        };
        self.rng.gen_range(range)
    }

    pub fn roll_dice(&mut self, dice_count: u64, dice_size: u64) -> u64 {
        let mut result = 0_u64;
        for _ in 0..dice_count {
            result += self.rng.gen_range(1..dice_size + 1) as u64;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::rand_gen::RandGen;

    #[test]
    fn test_range() {
        let mut rand_gen = RandGen::new(None);
        let result = rand_gen.range(0, 99);
        println!("result = {}", result);
    }

    #[test]
    fn test_dice() {
        let mut rand_gen = RandGen::new(None);
        let result = rand_gen.roll_dice(8, 6);
        println!("result = {}", result);
    }
}