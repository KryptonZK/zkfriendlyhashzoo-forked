use std::sync::Arc;

use bellman_ce::pairing::ff::PrimeField;

use super::gmimc_params::GmimcParams;
use crate::circuits::Permutation;

#[derive(Clone, Debug)]
pub struct Gmimc<S: PrimeField> {
    pub(crate) params: Arc<GmimcParams<S>>,
}

impl<S: PrimeField> Gmimc<S> {
    pub fn new(params: &Arc<GmimcParams<S>>) -> Self {
        Gmimc {
            params: Arc::clone(params),
        }
    }

    fn sbox(&self, state_0: &S, round: usize) -> S {
        let mut input = *state_0;
        input.add_assign(&self.params.round_constants[round]);

        let mut input2 = input.to_owned();
        input2.square();
        match self.params.d {
            3 => {}
            5 => {
                input2.square();
            }
            _ => {
                panic!();
            }
        };

        input2.mul_assign(&input);
        input2
    }

    fn round(&self, state: &mut [S], round: usize) {
        let power = self.sbox(&state[0], round);
        state.iter_mut().skip(1).for_each(|f| f.add_assign(&power));
    }

    pub fn permutation_not_opt(&self, input: &[S]) -> Vec<S> {
        assert_eq!(self.params.t, input.len());
        let mut current_state = input.to_owned();
        for r in 0..self.params.rounds - 1 {
            self.round(&mut current_state, r);
            current_state.rotate_right(1);
        }

        // finally without rotation
        self.round(&mut current_state, self.params.rounds - 1);

        current_state
    }
}

impl<S: PrimeField> Permutation<S> for Gmimc<S> {
    fn permutation(&self, input: &[S]) -> Vec<S> {
        let t = self.params.t;
        // not opt is faster for small t
        if t < 8 {
            return self.permutation_not_opt(input);
        }

        assert_eq!(t, input.len());
        let mut current_state = input.to_owned();
        let mut acc = S::zero();
        let mut acc_queue = vec![S::zero(); t - 1];
        for r in 0..self.params.rounds - 1 {
            let power = self.sbox(&current_state[0], r);
            acc_queue.rotate_right(1);
            acc.sub_assign(&acc_queue[0]);
            acc_queue[0] = power;
            acc.add_assign(&power);

            current_state.rotate_right(1);
            current_state[0].add_assign(&acc);
        }

        // finally without rotation
        let power = self.sbox(&current_state[0], self.params.rounds - 1);
        acc_queue.rotate_right(1);
        acc.sub_assign(&acc_queue[0]);
        acc_queue[0] = power;
        acc.add_assign(&power);
        current_state[t - 1].add_assign(&acc);

        // final adds
        for el in current_state.iter_mut().skip(1).take(t - 2).rev() {
            acc_queue.rotate_right(1);
            acc.sub_assign(&acc_queue[0]);
            el.add_assign(&acc);
        }

        current_state
    }

    fn get_t(&self) -> usize {
        self.params.t
    }
}

#[cfg(test)]
mod gmimc_tests_bls12 {
    use super::*;
    use crate::{gmimc::gmimc_instance_bls12::GMIMC_BLS_3_PARAMS, utils};
    use bellman_ce::pairing::bls12_381;

    type Scalar = bls12_381::Fr;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let gmimc = Gmimc::new(&GMIMC_BLS_3_PARAMS);
        let t = gmimc.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = gmimc.permutation(&input1);
            let perm2 = gmimc.permutation(&input1);
            let perm3 = gmimc.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    #[test]
    fn opt_equals_not_opt() {
        let gmimc = Gmimc::new(&GMIMC_BLS_3_PARAMS);
        let t = gmimc.params.t;
        for _ in 0..TESTRUNS {
            let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let perm1 = gmimc.permutation(&input);
            let perm2 = gmimc.permutation_not_opt(&input);
            assert_eq!(perm1, perm2);
        }
    }
}

#[cfg(test)]
mod gmimc_tests_bn256 {
    use super::*;
    use crate::{gmimc::gmimc_instance_bn256::GMIMC_BN_3_PARAMS, utils};
    use bellman_ce::pairing::bn256;

    type Scalar = bn256::Fr;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let gmimc = Gmimc::new(&GMIMC_BN_3_PARAMS);
        let t = gmimc.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = gmimc.permutation(&input1);
            let perm2 = gmimc.permutation(&input1);
            let perm3 = gmimc.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    #[test]
    fn opt_equals_not_opt() {
        let gmimc = Gmimc::new(&GMIMC_BN_3_PARAMS);
        let t = gmimc.params.t;
        for _ in 0..TESTRUNS {
            let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let perm1 = gmimc.permutation(&input);
            let perm2 = gmimc.permutation_not_opt(&input);
            assert_eq!(perm1, perm2);
        }
    }
}
