use ff::PrimeField;

use crate::fields::utils;
use sha3::{
    digest::{core_api::XofReaderCoreWrapper, ExtendableOutput, Update, XofReader},
    Shake128, Shake128ReaderCore,
};

#[derive(Clone, Debug)]
pub struct ReinforcedConcreteStParams<F: PrimeField> {
    pub(crate) round_constants: Vec<Vec<F>>,
    pub(crate) betas: [F; 2],
    pub(crate) sbox: Vec<u16>,
    pub(crate) d: usize,
}

impl<F: PrimeField> ReinforcedConcreteStParams<F> {
    pub const PRE_ROUNDS: usize = 3;
    pub const POST_ROUNDS: usize = 3;
    pub const TOTAL_ROUNDS: usize = Self::PRE_ROUNDS + Self::POST_ROUNDS + 1;
    pub const T: usize = 3;
    const SI_MAX: u16 = 1024;
    pub const INIT_SHAKE: &'static str = "ReinforcedConcrete";

    pub fn new(d: usize, sbox: &[u16]) -> Self {
        assert!(sbox.len() <= u16::MAX as usize);

        let mut shake = Self::init_shake();
        let betas = [utils::from_u64(3), utils::from_u64(4)];
        let round_constants = Self::instantiate_rc(&mut shake);

        ReinforcedConcreteStParams {
            round_constants,
            betas,
            sbox: Self::pad_sbox(sbox, Self::SI_MAX),
            d,
        }
    }

    fn init_shake() -> XofReaderCoreWrapper<Shake128ReaderCore> {
        let mut shake = Shake128::default();
        shake.update(Self::INIT_SHAKE.as_bytes());
        for i in F::char().as_ref() {
            shake.update(&u64::to_le_bytes(*i));
        }
        shake.finalize_xof()
    }

    fn pad_sbox(sbox: &[u16], max: u16) -> Vec<u16> {
        let len = sbox.len();
        let mut out = sbox.to_owned();

        out.reserve((max as usize) - len);
        for i in (len as u16)..max {
            out.push(i);
        }

        out
    }

    fn instantiate_rc(shake: &mut dyn XofReader) -> Vec<Vec<F>> {
        (0..=Self::TOTAL_ROUNDS)
            .map(|_| {
                (0..Self::T)
                    .map(|_| utils::field_element_from_shake(shake))
                    .collect()
            })
            .collect()
    }

    pub fn get_t(&self) -> usize {
        Self::T
    }

    pub fn get_rounds(&self) -> usize {
        Self::TOTAL_ROUNDS
    }
}
