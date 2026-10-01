# Computation Optimization

Demonstrates computation and gas optimization patterns in Soroban smart contracts:

- **Algorithmic Optimization**: Using closed-form formulas (e.g. arithmetic progression $O(1)$) and fast exponentiation by squaring $O(\log n)$ instead of naive loops.
- **Loop Optimization**: Short-circuiting and early termination on zero or absorbing elements to avoid wasteful iterations.
- **Memoization & Storage Caching**: Storing computed results in contract instance storage to prevent re-executing heavy computation on future calls.

## Building and Checking

Build with:
```bash
cargo build -p computation-optimization
```