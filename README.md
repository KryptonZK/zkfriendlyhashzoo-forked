# Hash Functions for Zero-Knowledge Applications Zoo

This repository contains several Rust implementations of different hash functions for zero-knowledge applications and will be extended frequently.

The repository already contains the following implementations:

- `plain_impls` contains a comparison of the plain performance of various hash functions.
- `bounties` contains toy instances and implementations of some hash functions used for the [Cryptanalysis Bounties](https://www.zkhashbounties.info/).
- `bellman` contains a comparison of different hash functions in zero-knowledge applications using the [bellman_ce](https://github.com/matter-labs/bellman) library.

## Citing our work

Please use the following BibTeX entry to cite our work in academic papers.

```tex
@misc{HashZKP,
  title = {Hash functions for Zero-Knowledge applications Zoo},
  howpublished = {\url{https://extgit.iaik.tugraz.at/krypto/zkfriendlyhashzoo}},
  month = aug,
  year = 2021,
  note = {{IAIK}, Graz University of Technology},
}
```

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
