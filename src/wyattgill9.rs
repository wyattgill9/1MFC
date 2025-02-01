use num_bigint::BigUint;

#[derive(Clone)]
struct FibPair {
    a: BigUint,
    b: BigUint,
}

impl FibPair {
    #[inline(always)]
    fn new(a: u32, b: u32) -> Self {
        FibPair {
            a: BigUint::from(a),
            b: BigUint::from(b),
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

    let mut pair = FibPair::new(1, 0);
    let bits = usize::BITS - n.leading_zeros();

    for i in (0..bits).rev() {
        pair.square();
        if (n & (1 << i)) != 0 {
            pair.step();
        }
    }

    pair.b
}
