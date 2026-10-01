#![cfg_attr(target_family = "wasm", no_std)]
#![allow(deprecated)]

//! # Computation Optimization Patterns for Soroban
//!
//! This contract demonstrates key computation and gas optimization techniques:
//! 1. **Algorithm optimization**: O(1) mathematical formulas vs O(N) loops, fast exponentiation by squaring.
//! 2. **Loop optimization**: early exit and short-circuiting on neutral/zero elements.
//! 3. **Memoization / Caching**: caching expensive calculation results in instance storage to avoid redundant execution.

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Vec};

#[contract]
pub struct ComputationOptimizationContract;

#[contractimpl]
impl ComputationOptimizationContract {
    /// O(1) arithmetic series sum: n * (n + 1) / 2
    /// Avoids an O(N) loop consuming CPU instructions.
    pub fn sum_formula(_env: Env, n: u64) -> u64 {
        n.checked_mul(n.saturating_add(1))
            .map(|val| val / 2)
            .unwrap_or(u64::MAX)
    }

    /// O(log exp) exponentiation by squaring
    /// Greatly reduces multiplication iterations compared to naive O(exp) loop.
    pub fn pow_fast(_env: Env, mut base: u64, mut exp: u32) -> u64 {
        let mut result: u64 = 1;
        while exp > 0 {
            if exp % 2 == 1 {
                result = result.saturating_mul(base);
            }
            base = base.saturating_mul(base);
            exp /= 2;
        }
        result
    }

    /// Loop optimization with early exit on zero values.
    /// Skips further iterations and computations once zero is encountered.
    pub fn multiply_array(_env: Env, values: Vec<u64>) -> u64 {
        let mut product: u64 = 1;
        for val in values.iter() {
            if val == 0 {
                return 0; // Short-circuit: product with 0 is always 0
            }
            product = product.saturating_mul(val);
        }
        product
    }

    /// Caching/Memoization: checks instance storage for previously computed result
    /// before performing computationally heavy work.
    pub fn get_or_compute_fibonacci(env: Env, n: u32) -> u64 {
        let key: Symbol = symbol_short!("fib");
        if let Some(cached_val) = env.storage().instance().get::<(Symbol, u32), u64>(&(key, n)) {
            return cached_val;
        }

        let mut a: u64 = 0;
        let mut b: u64 = 1;
        for _ in 0..n {
            let next = a.saturating_add(b);
            a = b;
            b = next;
        }

        env.storage().instance().set(&(key, n), &a);
        a
    }
}

#[cfg(test)]
mod test;
