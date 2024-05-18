# Plain Performance Comparison of different Hash Functions for ZKP

This repository contains Rust implementations of different hash functions for Zero-Knowledge applications. For benchmarks we refer to [1] and [2].

## Hash Functions

The following hash functions are already implemented:

- [ReinforcedConcrete](https://eprint.iacr.org/2021/1038.pdf)
- [Monolith](https://eprint.iacr.org/2023/1025.pdf)
- [Poseidon](https://eprint.iacr.org/2019/458.pdf)
- [Rescue](https://eprint.iacr.org/2019/426.pdf)
- [Rescue-Prime](https://www.esat.kuleuven.be/cosic/publications/article-3259.pdf)
- [Griffin](https://eprint.iacr.org/2022/403.pdf)
- [Neptune](https://eprint.iacr.org/2021/1695.pdf)
- [Feistel-MiMC](https://eprint.iacr.org/2016/492.pdf)
- [Pedersen-Hash](https://zips.z.cash/protocol/protocol.pdf#concretepedersenhash), code extracted from [Zcash](https://github.com/zcash/librustzcash)
- [Sinsemilla](https://zips.z.cash/protocol/protocol.pdf#concretesinsemillahash), code extracted from [Orchard](https://github.com/zcash/orchard)

We also benchmark against various classical hash algorithms implemented in [RustCrypto](https://github.com/RustCrypto/hashes).

[1] [https://eprint.iacr.org/2021/1038.pdf](https://eprint.iacr.org/2021/1038.pdf)
[2] [https://eprint.iacr.org/2023/1025.pdf](https://eprint.iacr.org/2023/1025.pdf)
