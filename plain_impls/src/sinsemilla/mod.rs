// implementation taken from https://github.com/zcash/orchard
// licensed under the Bootstrap Open Source Licence, version 1.0

pub mod addition;
pub mod constants;
#[allow(clippy::module_inception)]
pub mod sinsemilla;
pub mod sinsemilla_s;
pub mod spec;
pub mod util;

use self::sinsemilla::{i2lebsp_k, HashDomain, L_ORCHARD_MERKLE};
use crate::merkle_tree::merkle_tree_orchard::MerkleTreeHash;
use group::ff::PrimeFieldBits;
use pasta_curves::pallas::Base;
use std::iter;

impl MerkleTreeHash for HashDomain {
    fn compress(&self, level: usize, input: &[&Base; 2]) -> Base {
        self.hash(
            iter::empty()
                .chain(i2lebsp_k(level).iter().copied())
                .chain(
                    input[0]
                        .to_le_bits()
                        .iter()
                        .by_vals()
                        .take(L_ORCHARD_MERKLE),
                )
                .chain(
                    input[1]
                        .to_le_bits()
                        .iter()
                        .by_vals()
                        .take(L_ORCHARD_MERKLE),
                ),
        )
        .unwrap_or(Base::zero())
    }
}
