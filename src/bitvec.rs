use ::std::fmt;

#[derive(Clone)]
/// A fixed size bit vec
pub struct BitVec32(u32);

impl BitVec32 {
    pub const NO_SET_BITS: usize = 32;
    pub const ALL: BitVec32 = BitVec32(u32::max_value());

    pub fn new() -> BitVec32 {
        BitVec32(0)
    }

    pub fn get(&self, i: usize) -> bool {
        let mask = 1 << i;
        self.0 & mask == mask
    }

    pub fn set(&mut self, i: usize) {
        let mask = 1 << i;
        self.0 |= mask
    }

    pub fn clear(&mut self, i: usize) {
        let mask = !(1 << i);
        self.0 &= mask
    }

    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn iter(&self) -> BitVec32Iter {
        BitVec32Iter(self.clone())
    }

    /// The first set bit. A value of [BitVec32::NO_SET_BITS] indicates no bits set.
    pub fn first(&self) -> usize {
        self.0.trailing_zeros() as usize
    }

    /// The last set bit. A value of [BitVec32::NO_SET_BITS] indicates no bits set.
    pub fn last(&self) -> usize {
        let lz = self.0.leading_zeros();
        match lz {
            32 => BitVec32::NO_SET_BITS,
            _ => (31 - lz) as usize
        }
    }

    pub fn to_u32(&self) -> u32 {
        self.0
    }
}

impl fmt::Debug for BitVec32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BitSet32({:032b})", self.0)
    }
}

struct BitVec32Iter(BitVec32);

impl Iterator for BitVec32Iter {
    type Item = usize;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let lsb = self.0.first();
        match lsb {
            32 => None,
            _ => {
                self.0.clear(lsb);
                Some(lsb)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        assert_eq!(true, BitVec32(1).get(0));
        assert_eq!(false, BitVec32(0).get(0));
        assert_eq!(false, BitVec32(0x80000000).get(0));
        assert_eq!(true, BitVec32(0x80000000).get(31));
    }

    #[test]
    fn set() {
        let mut bs = BitVec32::new();

        bs.set(0);
        assert_eq!(bs.0, 1);

        bs.set(1);
        assert_eq!(bs.0, 3);
    }

    #[test]
    fn clear() {
        let mut bs = BitVec32(1);
        bs.clear(0);
        assert_eq!(bs.0, 0);

        let mut bs = BitVec32(0x80000001);
        bs.clear(31);
        assert_eq!(bs.0, 1);
    }

    #[test]
    fn len() {
        let mut bs = BitVec32(1);
        assert_eq!(1, bs.len());

        bs.set(31);
        assert_eq!(2, bs.len());

        let bs = BitVec32(u32::max_value());
        assert_eq!(32, bs.len());
    }

    #[test]
    fn iter() {
        let bs = BitVec32(0);
        let actual: Vec<usize> = bs.iter().collect();
        let expected: Vec<usize> = vec![];
        assert_eq!(expected, actual);

        let bs = BitVec32(1);
        let actual: Vec<usize> = bs.iter().collect();
        let expected = vec![0usize];
        assert_eq!(expected, actual);

        let bs = BitVec32(0x80000001);

        let actual: Vec<usize> = bs.iter().collect();
        let expected: Vec<usize> = vec![0, 31];
        assert_eq!(expected, actual);
    }
}