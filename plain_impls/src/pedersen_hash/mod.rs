// implementation taken from https://github.com/zcash/librustzcash/blob/master/zcash_primitives/src/sapling/pedersen_hash.rs

pub mod constants;
#[allow(clippy::module_inception)]
pub mod pedersen_hash;
#[cfg(test)]
pub mod test_vectors;

use self::pedersen_hash::{pedersen_hash, Personalization};
use crate::merkle_tree::merkle_tree_sapling::MerkleTreeHash;
use bitvec::{order::Lsb0, view::AsBits};
use group_ped::{ff::PrimeField, Curve};
use jubjub::Base;

#[derive(Default)]
pub struct PedersenHasher {}

impl MerkleTreeHash for PedersenHasher {
    fn compress(&self, level: usize, input: &[&Base; 2]) -> Base {
        let lhs = {
            let mut tmp = [false; 256];
            for (a, b) in tmp.iter_mut().zip(input[0].to_repr().as_bits::<Lsb0>()) {
                *a = *b;
            }
            tmp
        };

        let rhs = {
            let mut tmp = [false; 256];
            for (a, b) in tmp.iter_mut().zip(input[1].to_repr().as_bits::<Lsb0>()) {
                *a = *b;
            }
            tmp
        };

        jubjub::ExtendedPoint::from(pedersen_hash(
            Personalization::MerkleTree(level),
            lhs.iter()
                .copied()
                .take(bls12_381::Scalar::NUM_BITS as usize)
                .chain(
                    rhs.iter()
                        .copied()
                        .take(bls12_381::Scalar::NUM_BITS as usize),
                ),
        ))
        .to_affine()
        .get_u()
    }
}
