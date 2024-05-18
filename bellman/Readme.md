# Performance Comparison of different Hash Functions for ZKP

This repository contains Rust implementations of different hash functions for Zero-Knowledge applications, as well as implementations of proving knowledge of preimages and witnesses of Merkle-tree accumulators in the [bellman_ce](https://github.com/matter-labs/bellman) library. For benchmarks we refer to [1].

## Hash Functions

The following hash functions are already implemented:

- [Griffin](https://eprint.iacr.org/2022/403.pdf)
- [Rescue-Prime](https://www.esat.kuleuven.be/cosic/publications/article-3259.pdf)
- [Poseidon](https://eprint.iacr.org/2019/458.pdf)
- [Neptune](https://eprint.iacr.org/2021/1695.pdf)
- [Grendel](https://eprint.iacr.org/2021/984.pdf)
- [GMiMC](https://eprint.iacr.org/2019/397.pdf) (Updated round numbers as described [here](https://eprint.iacr.org/2021/267.pdf))

[1] [https://eprint.iacr.org/2022/403.pdf](https://eprint.iacr.org/2022/403.pdf)
