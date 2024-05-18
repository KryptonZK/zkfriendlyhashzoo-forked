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
