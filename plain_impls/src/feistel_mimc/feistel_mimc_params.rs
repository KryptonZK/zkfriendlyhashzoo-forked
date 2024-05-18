use ff::PrimeField;

use crate::fields::utils;
use sha3::{digest::ExtendableOutput, digest::Update, Shake128};

#[derive(Clone, Debug)]
pub struct FeistelMimcParams<F: PrimeField> {
    pub(crate) d: usize,
    pub(crate) rounds: usize,
    pub(crate) round_constants: Vec<F>,
}

impl<F: PrimeField> FeistelMimcParams<F> {
    pub const INIT_SHAKE: &'static str = "FeistelMiMC";

    pub fn new(d: usize) -> Self {
        let rounds = Self::get_num_rounds(d);
        let round_constants = Self::instantiate_rc(rounds);

        FeistelMimcParams {
            d,
            rounds,
            round_constants,
        }
    }

    fn get_num_rounds(d: usize) -> usize {
        let n = F::NUM_BITS + 1; // ceil(log_2(p))
        let log = f64::log(2f64, d as f64);
        f64::ceil(2f64 * (n as f64) * log) as usize
    }

    fn instantiate_rc(rounds: usize) -> Vec<F> {
        let mut shake = Shake128::default();
        shake.update(Self::INIT_SHAKE.as_bytes());
        for i in F::char().as_ref() {
            shake.update(&u64::to_le_bytes(*i));
        }
        let mut reader = shake.finalize_xof();

        (0..rounds)
            .map(|_| utils::field_element_from_shake(&mut reader))
            .collect()
    }

    pub fn get_rounds(&self) -> usize {
        self.rounds
    }
}
