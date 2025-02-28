use num_bigint::BigUint;

#[derive(Clone)]
struct FibPair {
    a: BigUint,
    b: BigUint,
}

impl FibPair {
    #[inline(always)]
    fn new() -> Self {
        FibPair {
            a: BigUint::from(1u32),
            b: BigUint::from(0u32),
        }
    }

    #[inline(always)]
    fn square(&mut self) {
        let a_squared = &self.a * &self.a;
        let b_squared = &self.b * &self.b;
        let mut double_ab = &self.a * &self.b;
        double_ab <<= 1;

        self.a = a_squared + &b_squared;
        self.b = double_ab + &b_squared;
    }

    #[inline(always)]
    fn step(&mut self) {
        self.a = std::mem::take(&mut self.b);
        self.b += &self.a;
    }
}

pub fn fib(n: usize) -> BigUint {
    if n <= 1 {
        return BigUint::from(n);
    }

    let mut pair = FibPair::new();
    let bits = usize::BITS - n.leading_zeros();

    #[test]
    fn test_larger_values() {
        assert_eq!(fib_luc(10).0, BigInt::from(55));
        assert_eq!(fib_luc(15).0, BigInt::from(610));
        assert_eq!(fib_luc(20).0, BigInt::from(6765));
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(fib(0), BigUint::from(0u32));
        assert_eq!(fib(1), BigUint::from(1u32));
        assert_eq!(fib(2), BigUint::from(1u32));
        assert_eq!(fib(3), BigUint::from(2u32));
        assert_eq!(fib(4), BigUint::from(3u32));
        assert_eq!(fib(5), BigUint::from(5u32));
        assert_eq!(fib(6), BigUint::from(8u32));
        assert_eq!(fib(7), BigUint::from(13u32));
        assert_eq!(fib(8), BigUint::from(21u32));
        assert_eq!(fib(9), BigUint::from(34u32));
        assert_eq!(fib(10), BigUint::from(55u32));
        assert_eq!(fib(11), BigUint::from(89u32));
        assert_eq!(fib(12), BigUint::from(144u32));
        assert_eq!(fib(13), BigUint::from(233u32));
        assert_eq!(fib(14), BigUint::from(377u32));
        assert_eq!(fib(15), BigUint::from(610u32));
        assert_eq!(fib(16), BigUint::from(987u32));
        assert_eq!(fib(17), BigUint::from(1597u32));
        assert_eq!(fib(18), BigUint::from(2584u32));
        assert_eq!(fib(19), BigUint::from(4181u32));
        assert_eq!(fib(20), BigUint::from(6765u32));
        assert_eq!(fib(21), BigUint::from(10946u32));
        assert_eq!(fib(22), BigUint::from(17711u32));
        assert_eq!(fib(23), BigUint::from(28657u32));
        assert_eq!(fib(24), BigUint::from(46368u32));
        assert_eq!(fib(25), BigUint::from(75025u32));
        assert_eq!(fib(26), BigUint::from(121393u32));
        assert_eq!(fib(27), BigUint::from(196418u32));
        assert_eq!(fib(28), BigUint::from(317811u32));
        assert_eq!(fib(29), BigUint::from(514229u32));
        assert_eq!(fib(30), BigUint::from(832040u32));
    }
}