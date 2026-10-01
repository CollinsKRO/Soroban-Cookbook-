#![allow(deprecated)]

use super::*;
use soroban_sdk::{vec, Env};

#[test]
fn test_sum_formula() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ComputationOptimizationContract);
    let client = ComputationOptimizationContractClient::new(&env, &contract_id);

    // Sum of 1..=10 is 55
    let sum = client.sum_formula(&10);
    assert_eq!(sum, 55);
}

#[test]
fn test_pow_fast() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ComputationOptimizationContract);
    let client = ComputationOptimizationContractClient::new(&env, &contract_id);

    // 2^10 = 1024
    let val = client.pow_fast(&2, &10);
    assert_eq!(val, 1024);
}

#[test]
fn test_multiply_array_short_circuit() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ComputationOptimizationContract);
    let client = ComputationOptimizationContractClient::new(&env, &contract_id);

    let list = vec![&env, 2, 4, 0, 8, 10];
    let product = client.multiply_array(&list);
    assert_eq!(product, 0);
}

#[test]
fn test_get_or_compute_fibonacci() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ComputationOptimizationContract);
    let client = ComputationOptimizationContractClient::new(&env, &contract_id);

    // fib(7) = 13 (0, 1, 1, 2, 3, 5, 8, 13)
    let val1 = client.get_or_compute_fibonacci(&7);
    assert_eq!(val1, 13);

    // Cached retrieval
    let val2 = client.get_or_compute_fibonacci(&7);
    assert_eq!(val2, 13);
}
