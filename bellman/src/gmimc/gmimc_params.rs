use bellman_ce::pairing::ff::PrimeField;
use sha3::{digest::ExtendableOutput, digest::Update, Shake128};

use crate::utils;

#[derive(Clone, Debug)]
pub struct GmimcParams<S: PrimeField> {
    pub(crate) t: usize, // statesize
    pub(crate) d: usize, // sbox degree
    pub(crate) rounds: usize,
    pub(crate) round_constants: Vec<S>,
}

impl<S: PrimeField> GmimcParams<S> {
    pub const INIT_SHAKE: &'static str = "GMiMC";

    pub fn new(t: usize, d: usize, rounds: usize) -> Self {
        let round_constants = Self::instantiate_rc(rounds);

        GmimcParams {
            t,
            d,
            rounds,
            round_constants,
        }
    }

    fn instantiate_rc(rounds: usize) -> Vec<S> {
        let mut shake = Shake128::default();
        shake.update(Self::INIT_SHAKE);
        for i in S::char().as_ref() {
            shake.update(u64::to_le_bytes(*i));
        }
        let mut reader = shake.finalize_xof();

        (0..rounds)
            .map(|_| utils::field_element_from_shake(&mut reader))
            .collect()
    }

    pub fn get_t(&self) -> usize {
        self.t
    }

    pub fn get_rounds(&self) -> usize {
        self.rounds
    }
}
