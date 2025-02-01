use num_bigint::BigUint;
use num_traits::{One, Zero};

pub fn fib(n: usize) -> BigUint {
    let (f_n, _) = fib_pair(n);
    f_n
}

fn fib_pair(n: usize) -> (BigUint, BigUint) {
    let mut a = BigUint::zero();
    let mut b = BigUint::one();

    let mut mask = 1 << (usize::BITS - n.leading_zeros() - 1);
    while mask > 0 {
        let c = &a * ((&b << 1) - &a);
        let d = &a * &a + &b * &b;

        if n & mask == 0 {
            a = c;
            b = d;
        } else {
            // Performace is lost here:
            a = d.clone();
            b = &c + &d;
        }
        mask >>= 1;
    }
    (a, b)
}
