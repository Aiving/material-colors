/// Partial LCG Algorithm implementation.
pub struct Random(i64);

impl Random {
    pub const fn new(seed: i64) -> Self {
        Self((seed ^ 0x5DEECE66Di64) & ((1i64 << 48) - 1))
    }

    fn _next(&mut self, bits: i64) -> i32 {
        self.0 = (self.0.wrapping_mul(0x5DEECE66Di64).wrapping_add(0xBi64)) & ((1i64 << 48) - 1);

        ((self.0 as u64) >> (48 - bits)) as i32
    }

    pub fn next_range(&mut self, range: i32) -> i32 {
        if (range & -range) == range {
            return (i64::from(range).wrapping_mul(i64::from(self._next(31))) >> 31) as i32;
        }

        let mut bits: i32;
        let mut val: i32;

        loop {
            bits = self._next(31);
            val = bits % range;

            if !bits - val + (range - 1) < 0 {
                break;
            }
        }

        val
    }
}
