use ff::PrimeField;

use crate::fields::utils;
use sha3::{digest::ExtendableOutput, digest::Update, Sha3XofReader, Shake128};

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

    pub fn new(d: usize, si: &[u16], v: usize, ab: &[u16]) -> Self {
        assert!(ab.len() == 4);
        assert!(v <= u16::MAX as usize);

        let mut shake = Self::init_shake();
        let alphas = [ab[0], ab[1]];
        let betas = [utils::from_u64(ab[2] as u64), utils::from_u64(ab[3] as u64)];
        let round_constants = Self::instantiate_rc(&mut shake);

        let sbox = Self::instantiate_sbox(v);

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
            sbox: Self::pad_sbox(&sbox, si),
            d,
        }
    }

    fn init_shake() -> Sha3XofReader {
        let mut shake = Shake128::default();
        shake.update(Self::INIT_SHAKE);
        for i in F::char().as_ref() {
            shake.update(u64::to_le_bytes(*i));
        }
        shake.finalize_xof()
    }

    fn mod_inverse(val: u64, m: u64) -> u64 {
        if val == 0 {
            panic!("0 has no inverse");
        }

        let mut v = val;
        let mut prev_a = 1i64;
        let mut a = 0i64;
        let mut modulus = m;

        while modulus != 0 {
            let q = (v / modulus) as i64;
            let mut temp = (v % modulus) as i64;
            v = modulus;
            modulus = temp as u64;

            temp = a;
            a = prev_a - q * a;
            prev_a = temp;
        }

        if v != 1 {
            panic!("{} has no inverse", val);
        }

        let mut res = prev_a as u64;
        if prev_a < 0 {
            res = (prev_a + m as i64) as u64;
        }
        debug_assert!((res as u128 * val as u128) % m as u128 == 1);
        res
    }

    fn instantiate_sbox(v: usize) -> Vec<u16> {
        let mut sbox = Vec::with_capacity(v);
        sbox.push(0);
        (1..v).for_each(|i| sbox.push(Self::mod_inverse(i as u64, v as u64) as u16));
        sbox
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

    fn instantiate_rc(shake: &mut Sha3XofReader) -> Vec<Vec<F>> {
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
