use super::feistel_mimc_params::FeistelMimcParams;
use crate::merkle_tree::merkle_tree_fp::MerkleTreeHash;
use ff::PrimeField;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct FeistelMimc<F: PrimeField> {
    pub(crate) params: Arc<FeistelMimcParams<F>>,
}

impl<F: PrimeField> FeistelMimc<F> {
    pub fn new(params: &Arc<FeistelMimcParams<F>>) -> Self {
        FeistelMimc {
            params: Arc::clone(params),
        }
    }

    fn sbox(&self, state_0: &F, round: usize) -> F {
        let mut input = *state_0;
        input.add_assign(&self.params.round_constants[round]);

        let mut input2 = input.to_owned();
        input2.square();
        match self.params.d {
            3 => {}
            5 => input2.square(),
            _ => panic!(),
        }
        input2.mul_assign(&input);
        input2
    }

    fn round(&self, state: &mut [F; 2], round: usize) {
        let power = self.sbox(&state[0], round);
        state[1].add_assign(&power);
    }

    pub fn permutation(&self, input: &[F; 2]) -> [F; 2] {
        let mut current_state = input.to_owned();
        for r in 0..self.params.rounds - 1 {
            self.round(&mut current_state, r);
            current_state.swap(0, 1);
        }

        // finally without rotation
        self.round(&mut current_state, self.params.rounds - 1);

        current_state
    }

    pub fn hash_two(&self, input1: &F, input2: &F) -> F {
        let perm_in = [input1.to_owned(), F::zero()];
        let mut perm_out = self.permutation(&perm_in);
        perm_out[0].add_assign(input2);
        self.permutation(&perm_out)[0]
    }
}

impl<F: PrimeField> MerkleTreeHash<F> for FeistelMimc<F> {
    fn compress(&self, input: &[&F; 2]) -> F {
        self.hash_two(input[0], input[1])
    }
}

#[cfg(test)]
mod feistel_mimc_tests_bls12 {
    use super::*;
    use crate::feistel_mimc::feistel_mimc_instances::FM_BLS_PARAMS;
    use crate::fields::{bls12::FpBLS12, utils};

    type Scalar = FpBLS12;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let feistel_mimc = FeistelMimc::new(&FM_BLS_PARAMS);
        for _ in 0..TESTRUNS {
            let input1: [Scalar; 2] = [utils::random_scalar(true), utils::random_scalar(true)];

            let mut input2: [Scalar; 2];
            loop {
                input2 = [utils::random_scalar(true), utils::random_scalar(true)];
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = feistel_mimc.permutation(&input1);
            let perm2 = feistel_mimc.permutation(&input1);
            let perm3 = feistel_mimc.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }
}

#[cfg(test)]
mod feistel_mimc_tests_bn256 {
    use super::*;
    use crate::feistel_mimc::feistel_mimc_instances::FM_BN_PARAMS;
    use crate::fields::{bn256::FpBN256, utils};

    type Scalar = FpBN256;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let feistel_mimc = FeistelMimc::new(&FM_BN_PARAMS);
        for _ in 0..TESTRUNS {
            let input1: [Scalar; 2] = [utils::random_scalar(true), utils::random_scalar(true)];

            let mut input2: [Scalar; 2];
            loop {
                input2 = [utils::random_scalar(true), utils::random_scalar(true)];
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = feistel_mimc.permutation(&input1);
            let perm2 = feistel_mimc.permutation(&input1);
            let perm3 = feistel_mimc.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }
}

#[cfg(test)]
mod feistel_mimc_tests_st {
    use super::*;
    use crate::feistel_mimc::feistel_mimc_instances::FM_ST_PARAMS;
    use crate::fields::{st::FpST, utils};

    type Scalar = FpST;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let feistel_mimc = FeistelMimc::new(&FM_ST_PARAMS);
        for _ in 0..TESTRUNS {
            let input1: [Scalar; 2] = [utils::random_scalar(true), utils::random_scalar(true)];

            let mut input2: [Scalar; 2];
            loop {
                input2 = [utils::random_scalar(true), utils::random_scalar(true)];
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = feistel_mimc.permutation(&input1);
            let perm2 = feistel_mimc.permutation(&input1);
            let perm3 = feistel_mimc.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }
}
