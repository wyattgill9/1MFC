use num_bigint::BigUint;
use num_traits::One;

pub fn fib(n: usize) -> BigUint {
    _fib(n).0
}

// ([[1, 1], [1, 0]]¹⁰⁰⁰⁰⁰⁰)[0][0] is slower.
fn _fib(n: usize) -> (BigUint, BigUint) {
    if n == 0 {
        return (BigUint::ZERO, BigUint::one());
    };
    let (α, β) = _fib(n >> 1);
    let γ = &α * ((&β << 1) - &α);
    let δ = &α * &α + &β * &β;
    if n & 1 == 0 {
        return (γ, δ);
    }
    let t = γ + &δ;
    (δ, t)
}
