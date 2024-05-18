//! # zkhash
//!
//! A pure Rust implementation of the ReinforcedConcrete Permutation
#![cfg_attr(feature = "asm", feature(asm))]

pub extern crate ff;

pub mod feistel_mimc;
pub mod fields;
pub mod gmimc;
pub mod griffin;
pub mod merkle_tree;
pub mod monolith_31;
pub mod monolith_64;
pub mod neptune;
pub mod pedersen_hash;
pub mod poseidon;
pub mod poseidon2;
pub mod reinforced_concrete;
pub mod reinforced_concrete_st;
pub mod rescue;
pub mod rescue_prime;
pub mod sinsemilla;
pub mod utils;
