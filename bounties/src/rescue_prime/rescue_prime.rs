use ff::PrimeField;
use std::sync::Arc;

use super::rescue_prime_params::RescuePrimeParams;

#[derive(Clone, Debug)]
pub struct RescuePrime<S: PrimeField> {
    pub(crate) params: Arc<RescuePrimeParams<S>>,
}

impl<S: PrimeField> RescuePrime<S> {
    pub fn new(params: &Arc<RescuePrimeParams<S>>) -> Self {
        RescuePrime {
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

        for r in 0..self.params.rounds {
            current_state = self.sbox(&current_state);
            current_state = self.affine(&current_state, 2 * r);
            current_state = self.sbox_inverse(&current_state);
            current_state = self.affine(&current_state, 2 * r + 1);
        }

        current_state
    }

    fn sbox(&self, input: &[S]) -> Vec<S> {
        input
            .iter()
            .map(|el| {
                let mut el2 = *el;
                el2.square();

                match self.params.d {
                    3 => {
                        let mut out = el2;
                        out.mul_assign(el);
                        out
                    }
                    5 => {
                        let mut out = el2;
                        out.square();
                        out.mul_assign(el);
                        out
                    }
                    _ => {
                        panic!();
                    }
                }
            })
            .collect()
    }

    fn sbox_inverse(&self, input: &[S]) -> Vec<S> {
        input.iter().map(|el| el.pow(self.params.d_inv)).collect()
    }

    fn affine(&self, input: &[S], round: usize) -> Vec<S> {
        let mat_result = self.matmul(input, &self.params.mds);
        Self::add_rc(&mat_result, &self.params.round_constants[round])
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

    fn add_rc(input: &[S], round_constants: &[S]) -> Vec<S> {
        debug_assert!(input.len() == round_constants.len());
        input
            .iter()
            .zip(round_constants.iter())
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
mod rescue_prime_kats {
    use super::*;

    use crate::fields::{field64::Fp64, utils};
    use crate::rescue_prime::rescue_prime_instances::*;

    use ff::{from_hex, Field};

    type Scalar = Fp64;

    #[test]
    fn easy1_kats() {
        let rescue = RescuePrime::new(&RESCUE_PRIME_PARAMS_EASY1);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = rescue.permutation(&input);
        assert_eq!(perm[0], from_hex("0xeb3c888655db7d7d").unwrap());
        assert_eq!(perm[1], from_hex("0xc76fcb221ca5a1f5").unwrap(),);
        assert_eq!(perm[2], from_hex("0xfb46c8a3f0d58087").unwrap(),);
    }

    #[test]
    fn easy2_kats() {
        let rescue = RescuePrime::new(&RESCUE_PRIME_PARAMS_EASY2);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one()];
        let perm = rescue.permutation(&input);
        assert_eq!(perm[0], from_hex("0x8bc03c0e0c1e9eb3").unwrap());
        assert_eq!(perm[1], from_hex("0x8056ff8150590045").unwrap(),);
    }

    #[test]
    fn medium_kats() {
        let rescue = RescuePrime::new(&RESCUE_PRIME_PARAMS_MEDIUM);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one()];
        let perm = rescue.permutation(&input);
        assert_eq!(perm[0], from_hex("0x14ba297bfe1ca063").unwrap());
        assert_eq!(perm[1], from_hex("0xdaa72951b79bc857").unwrap(),);
    }

    #[test]
    fn hard1_kats() {
        let rescue = RescuePrime::new(&RESCUE_PRIME_PARAMS_HARD1);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = rescue.permutation(&input);
        assert_eq!(perm[0], from_hex("0x6cb9703b238c9c2b").unwrap());
        assert_eq!(perm[1], from_hex("0x36078bffce602399").unwrap(),);
        assert_eq!(perm[2], from_hex("0x809e9567e03037ed").unwrap(),);
    }

    #[test]
    fn hard2_kats() {
        let rescue = RescuePrime::new(&RESCUE_PRIME_PARAMS_HARD2);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one()];
        let perm = rescue.permutation(&input);
        assert_eq!(perm[0], from_hex("0x3a4fc00d47d18c87").unwrap());
        assert_eq!(perm[1], from_hex("0x53b831117922eb0b").unwrap(),);
    }
}
