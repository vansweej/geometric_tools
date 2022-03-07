use std::hash::Hash;

pub trait ConvertFp {
    type Item: Hash;

    fn to_bits1(&self) -> Self::Item;
}

impl ConvertFp for f32 {
    type Item = u32;

    #[inline(always)]
    fn to_bits1(&self) -> Self::Item
    {
        self.to_bits()
    }
}

impl ConvertFp for f64 {
    type Item = u64;

    #[inline(always)]
    fn to_bits1(&self) -> Self::Item
    {
        self.to_bits()
    }
}


#[cfg(test)]
mod tests {
     use super::*;

    #[test]
    fn test_convert_fp() {
        let a: f32 = 0.432;
        let x = a.to_bits1();
        println!("{:?}", x);

        let b: f64 = 0.47867867;
        let y = b.to_bits1();
        println!("{:?}", y);
    }
}