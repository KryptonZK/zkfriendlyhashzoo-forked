use crate::fields::utils;
use ff::PrimeField;
use sha3::{
    digest::{core_api::XofReaderCoreWrapper, ExtendableOutput, Update, XofReader},
    Shake128, Shake128ReaderCore,
};

#[derive(Clone, Debug)]
pub struct ReinforcedConcreteParams<F: PrimeField> {
    pub(crate) round_constants: Vec<Vec<F>>,
    pub(crate) alphas: [u16; 2],
    pub(crate) betas: [F; 2],
    pub(crate) si: Vec<u16>,
    pub(crate) divisor_i: Vec<u64>,
    pub(crate) reciprokal_i: Vec<u64>,
    pub(crate) norm_shift_i: Vec<u32>,
    pub(crate) sbox: Vec<u16>,
    pub(crate) d: usize,
}

impl<F: PrimeField> ReinforcedConcreteParams<F> {
    pub const PRE_ROUNDS: usize = 3;
    pub const POST_ROUNDS: usize = 3;
    pub const TOTAL_ROUNDS: usize = Self::PRE_ROUNDS + Self::POST_ROUNDS + 1;
    pub const T: usize = 3;
    pub const INIT_SHAKE: &'static str = "ReinforcedConcrete";

    pub fn new(d: usize, si: &[u16], sbox: &[u16], ab: &[u16]) -> Self {
        assert!(sbox.len() <= u16::MAX as usize);
        assert!(ab.len() == 4);

        let mut shake = Self::init_shake();
        let alphas = [ab[0], ab[1]];
        let betas = [utils::from_u64(ab[2] as u64), utils::from_u64(ab[3] as u64)];
        let round_constants = Self::instantiate_rc(&mut shake);

        let len = si.len();
        let mut divisor_i = Vec::with_capacity(len);
        let mut reciprokal_i = Vec::with_capacity(len);
        let mut norm_shift_i = Vec::with_capacity(len);
        for s in si {
            let (div, rec) = utils::compute_normalized_divisor_and_reciproical(*s);
            divisor_i.push(div);
            reciprokal_i.push(rec);
            norm_shift_i.push((*s as u64).leading_zeros());
        }

        ReinforcedConcreteParams {
            round_constants,
            alphas,
            betas,
            si: si.to_owned(),
            divisor_i,
            reciprokal_i,
            norm_shift_i,
            sbox: Self::pad_sbox(sbox, si),
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

    fn pad_sbox(sbox: &[u16], si: &[u16]) -> Vec<u16> {
        let len = sbox.len();

        let max = si.iter().max().expect("si are empty...").to_owned();
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
