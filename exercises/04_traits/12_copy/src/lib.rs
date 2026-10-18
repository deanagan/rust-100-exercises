// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.

use std::ops::Add;

// Derive PartialEq and Debug so assert_eq! works
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

impl Add for WrappingU32 {
    // For WrappingU32, 
    // the Output type is Self (which means WrappingU32 itself)
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let sum = self.value.wrapping_add(other.value);
        WrappingU32::new(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
