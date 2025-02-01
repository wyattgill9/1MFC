use num_bigint::BigUint;
use num_traits::One;

#[inline(always)]
fn matrix_multiply(a: &[[u128; 2]; 2], b: &[[u128; 2]; 2]) -> [[u128; 2]; 2] {
    [
        [a[0][0] * b[0][0] + a[0][1] * b[1][0], a[0][0] * b[0][1] + a[0][1] * b[1][1]],
        [a[1][0] * b[0][0] + a[1][1] * b[1][0], a[1][0] * b[0][1] + a[1][1] * b[1][1]],
    ]
}

#[inline(always)]
fn matrix_power(matrix: [[u128; 2]; 2], mut n: u32) -> [[u128; 2]; 2] {
    let mut result = [[1, 0], [0, 1]];
    let mut base = matrix;

    while n > 0 {
        if n & 1 == 1 {
            result = matrix_multiply(&result, &base);
        }
        base = matrix_multiply(&base, &base);
        n >>= 1;
    }
    result
}

fn fib(n: usize) -> BigUint {
    if n == 0 {
        return 0;
    }
    let base_matrix = [[1, 1], [1, 0]];
    let result = matrix_power(base_matrix, n - 1);
    result[0][0]
}
