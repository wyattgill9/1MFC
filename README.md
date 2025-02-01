# The 1 Millionth Fibonacci Challenge

## Introduction
The 1 Millionth Fibonacci Challenge is a Rust-based competition where participants submit a pull request (PR) attempting to compute the 1 Millionth Fibonacci number F(1,000,000) as efficiently as possible. The goal is to achieve the fastest computation time using Rust.

## The Fibonacci Sequence
The Fibonacci sequence is a series of numbers where each term is the sum of the two preceding ones. It is defined recursively as follows:

\[
F(0) = 0, \quad F(1) = 1,
\]
\[
F(n) = F(n-1) + F(n-2), \quad \text{for } n \geq 2.
\]

The sequence begins as:

\[
0, 1, 1, 2, 3, 5, 8, 13, 21, 34, \dots
\]

## Rules and Limits

### General Rules
1. **Objective:** Submit a Rust PR that computes the 1 Millionth Fibonacci number as efficiently as possible.
2. **Allowed Methods:** Any computational method is allowed, including algorithmic optimizations and parallel computing, as long as it runs in Rust.
3. **Language Restriction:** Only Rust is allowed.
4. **Submission Format:** PRs must include code! the benchmark will be made when the workflow is approved!
5. **Time Limit:** Solutions must complete within a reasonable execution time.
6. **Verification:** Submissions will be tested for correctness and performance.
7. **Leaderboard:** The fastest verified solution will be displayed on a public leaderboard.

### Implementation Constraints
1. **External Libraries:** No external dependencies may be used (only Rust’s standard library is allowed).
2. **File Structure:** Implementations, add a file `src/github_user.rs` with your code, edit `benches/fib_bench.rs` with your solution, (import and use the fib_bench function) and submit the PR! Example can be found at `src/code.rs`
3. **Computation Time:** The Fibonacci computation must happen at runtime—precomputed values or build-time optimizations are not allowed.
4. **Output Requirements:**
   - The full number must be computed if feasible.
   - If full computation is impractical, the last 10 digits must be correctly computed.

### Performance Constraints
1. **Memory Usage:** Implementations must not require excessive memory.
2. **Parallelization:** Multithreading and SIMD optimizations are allowed.
3. **Benchmarking:** Performance will be measured on a modern CPU under identical conditions.
4. **Fair Play:** Implementations must work for arbitrary large Fibonacci numbers.

## Leaderboard

| Rank | Github      |    Time   |
|------|-------------|-----------|
| 1    | bend-n      | 14.874 ms |
| 2    | example     | 3.7556 s  |   

