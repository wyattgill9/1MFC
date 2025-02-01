use num_bigint::BigUint;
use num_traits::One;

pub fn fib(n: usize) -> BigUint {
    let mut a = BigUint::one();
    let mut b = BigUint::one();

    for _ in 2..n {
        let temp = b.clone();
        b += &a;
        a = temp;
    }

    b
}
