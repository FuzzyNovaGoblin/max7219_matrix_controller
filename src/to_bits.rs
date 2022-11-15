pub trait ToBits {
    fn to_bits(&self) -> Vec<bool>;
}

impl ToBits for u8 {
    fn to_bits(&self) -> Vec<bool> {
        (0..8)
            .rev()
            .map(|n| 1u8 << n)
            .map(|n| self & n != 0)
            .collect()
    }
}
impl ToBits for u16 {
    fn to_bits(&self) -> Vec<bool> {
        (0..16)
            .rev()
            .map(|n| 1u16 << n)
            .map(|n| self & n != 0)
            .collect()
    }
}

impl ToBits for u32 {
    fn to_bits(&self) -> Vec<bool> {
        (0..32)
            .rev()
            .map(|n| 1u32 << n)
            .map(|n| self & n != 0)
            .collect()
    }
}

impl ToBits for u64 {
    fn to_bits(&self) -> Vec<bool> {
        (0..64)
            .rev()
            .map(|n| 1u64 << n)
            .map(|n| self & n != 0)
            .collect()
    }
}

impl ToBits for u128 {
    fn to_bits(&self) -> Vec<bool> {
        (0..128)
            .rev()
            .map(|n| 1u128 << n)
            .map(|n| self & n != 0)
            .collect()
    }
}
impl ToBits for usize {
    fn to_bits(&self) -> Vec<bool> {
        (0..usize::BITS)
            .rev()
            .map(|n| 1usize << n)
            .map(|n| self & n != 0)
            .collect()
    }
}
