use ff::PrimeField;
use std::sync::Arc;

use super::poseidon_params::PoseidonParams;

#[derive(Clone, Debug)]
pub struct Poseidon<S: PrimeField> {
    pub(crate) params: Arc<PoseidonParams<S>>,
}

impl<S: PrimeField> Poseidon<S> {
    pub fn new(params: &Arc<PoseidonParams<S>>) -> Self {
        Poseidon {
            params: Arc::clone(params),
        }
    }

    pub fn get_t(&self) -> usize {
        self.params.t
    }

    pub fn permutation(&self, input: &[S]) -> Vec<S> {
        let t = self.params.t;
        assert_eq!(input.len(), t);

        let mut current_state = input.to_owned();

        for r in 0..self.params.rounds_f_beginning {
            current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
            current_state = self.sbox(&current_state);
            current_state = self.matmul(&current_state, &self.params.mds);
        }
        let p_end = self.params.rounds_f_beginning + self.params.rounds_p;
        for r in self.params.rounds_f_beginning..p_end {
            current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
            current_state[0] = self.sbox_p(&current_state[0]);
            current_state = self.matmul(&current_state, &self.params.mds);
        }
        for r in p_end..self.params.rounds {
            current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
            current_state = self.sbox(&current_state);
            current_state = self.matmul(&current_state, &self.params.mds);
        }
        current_state
    }

    fn sbox(&self, input: &[S]) -> Vec<S> {
        input.iter().map(|el| self.sbox_p(el)).collect()
    }

    fn sbox_p(&self, input: &S) -> S {
        let mut input2 = *input;
        input2.square();

        match self.params.d {
            3 => {
                let mut out = input2;
                out.mul_assign(input);
                out
            }
            5 => {
                let mut out = input2;
                out.square();
                out.mul_assign(input);
                out
            }
            _ => {
                panic!();
            }
        }
    }

    fn matmul(&self, input: &[S], mat: &[Vec<S>]) -> Vec<S> {
        let t = mat.len();
        debug_assert!(t == input.len());
        let mut out = vec![S::zero(); t];
        for row in 0..t {
            for (col, inp) in input.iter().enumerate().take(t) {
                let mut tmp = mat[row][col];
                tmp.mul_assign(inp);
                out[row].add_assign(&tmp);
            }
        }
        out
    }

    fn add_rc(&self, input: &[S], rc: &[S]) -> Vec<S> {
        input
            .iter()
            .zip(rc.iter())
            .map(|(a, b)| {
                let mut r = *a;
                r.add_assign(b);
                r
            })
            .collect()
    }
}

// #############################################################################

#[cfg(test)]
mod poseidon_kats {
    use super::*;

    use crate::fields::{field64::Fp64, utils};
    use crate::poseidon::poseidon_instances::*;

    use ff::{from_hex, Field};

    type Scalar = Fp64;

    #[test]
    fn easy1_kats() {
        let poseidon = Poseidon::new(&POSEIDON_PARAMS_EASY1);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = poseidon.permutation(&input);
        assert_eq!(perm[0], from_hex("0x0d87dd06afba8e44").unwrap());
        assert_eq!(perm[1], from_hex("0x9f6d7d50cb950fb3").unwrap(),);
        assert_eq!(perm[2], from_hex("0xb58e0fe2773a4a65").unwrap(),);
    }

    #[test]
    fn easy2_kats() {
        let poseidon = Poseidon::new(&POSEIDON_PARAMS_EASY2);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = poseidon.permutation(&input);
        assert_eq!(perm[0], from_hex("0x37e4a6ac9833eba7").unwrap());
        assert_eq!(perm[1], from_hex("0x671d18304757c182").unwrap(),);
        assert_eq!(perm[2], from_hex("0xc848f18666ffe27d").unwrap(),);
    }

    #[test]
    fn medium_kats() {
        let poseidon = Poseidon::new(&POSEIDON_PARAMS_MEDIUM);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = poseidon.permutation(&input);
        assert_eq!(perm[0], from_hex("0x354ebe61f1f15f2f").unwrap());
        assert_eq!(perm[1], from_hex("0xed8f5b65944632ea").unwrap(),);
        assert_eq!(perm[2], from_hex("0x30b527e6ee6b13ff").unwrap(),);
    }

    #[test]
    fn hard1_kats() {
        let poseidon = Poseidon::new(&POSEIDON_PARAMS_HARD1);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = poseidon.permutation(&input);
        assert_eq!(perm[0], from_hex("0x318aa75bd963c759").unwrap());
        assert_eq!(perm[1], from_hex("0x779e6a6e4bc42460").unwrap(),);
        assert_eq!(perm[2], from_hex("0x8b88496cdd156ad0").unwrap(),);
    }

    #[test]
    fn hard2_kats() {
        let poseidon = Poseidon::new(&POSEIDON_PARAMS_HARD2);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = poseidon.permutation(&input);
        assert_eq!(perm[0], from_hex("0x333abc4e07a6b63e").unwrap());
        assert_eq!(perm[1], from_hex("0xb76df3cc040e13a4").unwrap(),);
        assert_eq!(perm[2], from_hex("0xd26faa26239f0005").unwrap(),);
    }
}
