use std::sync::Arc;

use bellman_ce::pairing::ff::PrimeField;

use crate::r1cs::{Constraint, R1CS};

use super::gmimc_params::GmimcParams;

#[derive(Clone, Debug)]
pub struct GmimcR1CS<S: PrimeField> {
    params: Arc<GmimcParams<S>>,
    r1cs: R1CS<S>,
    ready: bool,
}

impl<S: PrimeField> GmimcR1CS<S> {
    pub fn new(params: &Arc<GmimcParams<S>>) -> Self {
        let vars = Self::get_num_vars(params.t, params.d, params.rounds);
        let r1cs = R1CS::new(vars);

        GmimcR1CS {
            params: Arc::clone(params),
            r1cs,
            ready: false,
        }
    }

    pub fn get_num_vars(t: usize, d: usize, rounds: usize) -> usize {
        let d_inc;
        if d == 3 {
            d_inc = 2;
        } else if d == 5 {
            d_inc = 3;
        } else {
            panic!();
        }

        // includes input, output constraints and var=1
        let constraints = d_inc * rounds + t;
        t + 1 + constraints
    }

    pub fn round(&mut self, constraints: &[Constraint<S>], round: usize) -> Vec<Constraint<S>> {
        let power = self.sbox(&constraints[0], round);
        let mut result = constraints.to_owned();

        result
            .iter_mut()
            .skip(1)
            .for_each(|f| *f = self.r1cs.addition(f, &power));

        result
    }

    pub fn permutation(&mut self, constraints: &[Constraint<S>]) -> Vec<Constraint<S>> {
        let mut current_state = constraints.to_owned();
        for r in 0..self.params.rounds - 1 {
            current_state = self.round(&current_state, r);
            current_state.rotate_right(1);
        }

        // finally without rotation
        self.round(&current_state, self.params.rounds - 1)
    }

    pub fn synthesize(&mut self) {
        if self.ready {
            return;
        }
        let t = self.params.t;

        let mut current_state: Vec<Constraint<S>> =
            (0..t).map(|_| self.r1cs.new_variable()).collect();

        current_state = self.permutation(&current_state);

        // instantiate output
        self.r1cs.linear_constraints(&current_state);
    }

    fn sbox(&mut self, constraint: &Constraint<S>, round: usize) -> Constraint<S> {
        let input = self
            .r1cs
            .add_const(constraint, &self.params.round_constants[round]);
        let mut sq = self.r1cs.multiplication_new(&input, &input);
        if self.params.d == 5 {
            sq = self.r1cs.multiplication_new(&sq, &sq);
        }
        self.r1cs.multiplication_new(&sq, &input)
    }
}

#[derive(Clone, Debug)]
pub struct GmimcWitness<S: PrimeField> {
    params: Arc<GmimcParams<S>>,
    vars: usize,
}

impl<S: PrimeField> GmimcWitness<S> {
    pub fn new(params: &Arc<GmimcParams<S>>) -> Self {
        let vars = GmimcR1CS::<S>::get_num_vars(params.t, params.d, params.rounds);

        GmimcWitness {
            params: Arc::clone(params),
            vars,
        }
    }

    fn permutation(&self, input: &[S], w: &mut [Option<S>]) -> Vec<S> {
        let t = self.params.t;
        assert_eq!(input.len(), t);

        let mut inc = 2;
        if self.params.d == 5 {
            inc = 3;
        }

        w[0] = Some(S::one());
        for i in 0..t {
            w[i + 1] = Some(input[i]);
        }
        let mut index = t + 1;

        let mut current_state = input.to_owned();
        let mut acc = S::zero();
        let mut acc_queue = vec![S::zero(); t - 1];

        for r in 0..self.params.rounds - 1 {
            let power = self.sbox(&current_state[0], r, w, index);
            acc_queue.rotate_right(1);
            acc.sub_assign(&acc_queue[0]);
            acc_queue[0] = power;
            acc.add_assign(&power);
            current_state.rotate_right(1);
            current_state[0].add_assign(&acc);
            index += inc;
        }

        // finally without rotation
        let power = self.sbox(&current_state[0], self.params.rounds - 1, w, index);
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
        index += inc;

        // output state
        for i in 0..t {
            w[index + i] = Some(current_state[i]);
        }
        current_state
    }

    fn sbox(&self, state_0: &S, round: usize, w: &mut [Option<S>], index: usize) -> S {
        let mut index = index;
        let mut input = *state_0;
        input.add_assign(&self.params.round_constants[round]);

        let mut input2 = input.to_owned();
        input2.square();
        w[index] = Some(input2);
        index += 1;
        match self.params.d {
            3 => {}
            5 => {
                input2.square();
                w[index] = Some(input2);
                index += 1;
            }
            _ => {
                panic!();
            }
        };

        input2.mul_assign(&input);
        w[index] = Some(input2);
        input2
    }

    pub fn create(&self, preimage: &[S]) -> Vec<Option<S>> {
        let mut w = vec![None; self.vars];
        self.permutation(preimage, &mut w);
        w
    }

    pub fn get_num_vars(&self) -> usize {
        self.vars
    }
}

#[cfg(test)]
mod gmimc_r1cs_test {
    use bellman_ce::pairing::{bls12_381, bn256};

    use crate::{
        circuits::Permutation,
        gmimc::{gmimc::Gmimc, gmimc_instance_bls12::*, gmimc_instance_bn256::*},
        utils,
    };

    use super::*;

    static TESTRUNS: usize = 5;

    fn mat_vector_mul<S: PrimeField>(mat: &[Constraint<S>], vec: &[Option<S>]) -> Vec<S> {
        let rows = mat.len();

        let mut out = vec![S::zero(); rows];
        for row in 0..rows {
            for (col, v) in vec.iter().enumerate() {
                let mut tmp = mat[row][col];
                let vecval = v.unwrap();
                tmp.mul_assign(&vecval);
                out[row].add_assign(&tmp);
            }
        }
        out
    }

    fn witness_perm<S: PrimeField>(params: &Arc<GmimcParams<S>>) {
        let gmimc = Gmimc::new(params);
        let gmimc_wit = GmimcWitness::new(params);
        let t = gmimc.params.t;
        for _ in 0..TESTRUNS {
            let input: Vec<S> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let perm = gmimc.permutation(&input);
            let wit = gmimc_wit.create(&input);
            let start = wit.len() - t;
            for i in 0..t {
                assert_eq!(perm[i], wit[start + i].unwrap());
            }
        }
    }

    fn witness_r1cs<S: PrimeField>(params: &Arc<GmimcParams<S>>) {
        let mut gmimc_r1cs = GmimcR1CS::new(params);
        let gmimc_wit = GmimcWitness::new(params);
        let t = params.t;
        gmimc_r1cs.synthesize();

        for _ in 0..TESTRUNS {
            let input: Vec<S> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let wit = gmimc_wit.create(&input);
            let lhs = mat_vector_mul(gmimc_r1cs.r1cs.get_lhs(), &wit);
            let rhs = mat_vector_mul(gmimc_r1cs.r1cs.get_rhs(), &wit);
            let res = mat_vector_mul(gmimc_r1cs.r1cs.get_res(), &wit);

            let zero: Vec<S> = lhs
                .iter()
                .zip(rhs.iter())
                .zip(res.iter())
                .map(|((l, r), o)| {
                    let mut tmp = *l;
                    tmp.mul_assign(r);
                    tmp.sub_assign(o);
                    tmp
                })
                .collect();

            assert_eq!(zero.len(), wit.len() - t - 1);
            for (i, el) in zero.iter().enumerate() {
                assert_eq!(*el, S::zero(), "i = {}", i)
            }
        }
    }

    #[test]
    fn gmimc_bls12_t3_witness_test() {
        witness_perm::<bls12_381::Fr>(&GMIMC_BLS_3_PARAMS);
    }

    #[test]
    fn gmimc_bls12_t4_witness_test() {
        witness_perm::<bls12_381::Fr>(&GMIMC_BLS_4_PARAMS);
    }

    #[test]
    fn gmimc_bls12_t5_witness_test() {
        witness_perm::<bls12_381::Fr>(&GMIMC_BLS_5_PARAMS);
    }

    #[test]
    fn gmimc_bls12_t8_witness_test() {
        witness_perm::<bls12_381::Fr>(&GMIMC_BLS_8_PARAMS);
    }

    #[test]
    fn gmimc_bls12_t9_witness_test() {
        witness_perm::<bls12_381::Fr>(&GMIMC_BLS_9_PARAMS);
    }

    #[test]
    fn gmimc_bls12_t12_witness_test() {
        witness_perm::<bls12_381::Fr>(&GMIMC_BLS_12_PARAMS);
    }

    #[test]
    fn gmimc_bn256_t3_witness_test() {
        witness_perm::<bn256::Fr>(&GMIMC_BN_3_PARAMS);
    }

    #[test]
    fn gmimc_bn256_t4_witness_test() {
        witness_perm::<bn256::Fr>(&GMIMC_BN_4_PARAMS);
    }

    #[test]
    fn gmimc_bn256_t5_witness_test() {
        witness_perm::<bn256::Fr>(&GMIMC_BN_5_PARAMS);
    }

    #[test]
    fn gmimc_bn256_t8_witness_test() {
        witness_perm::<bn256::Fr>(&GMIMC_BN_8_PARAMS);
    }

    #[test]
    fn gmimc_bn256_t9_witness_test() {
        witness_perm::<bn256::Fr>(&GMIMC_BN_9_PARAMS);
    }

    #[test]
    fn gmimc_bn256_t12_witness_test() {
        witness_perm::<bn256::Fr>(&GMIMC_BN_12_PARAMS);
    }

    #[test]
    fn gmimc_bls12_t3_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&GMIMC_BLS_3_PARAMS);
    }

    #[test]
    fn gmimc_bls12_t4_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&GMIMC_BLS_4_PARAMS);
    }

    #[test]
    fn gmimc_bls12_t5_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&GMIMC_BLS_5_PARAMS);
    }

    #[test]
    fn gmimc_bls12_t8_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&GMIMC_BLS_8_PARAMS);
    }

    #[test]
    fn gmimc_bls12_t9_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&GMIMC_BLS_9_PARAMS);
    }

    #[test]
    fn gmimc_bls12_t12_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&GMIMC_BLS_12_PARAMS);
    }

    #[test]
    fn gmimc_bn256_t3_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&GMIMC_BN_3_PARAMS);
    }

    #[test]
    fn gmimc_bn256_t4_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&GMIMC_BN_4_PARAMS);
    }

    #[test]
    fn gmimc_bn256_t5_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&GMIMC_BN_5_PARAMS);
    }

    #[test]
    fn gmimc_bn256_t8_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&GMIMC_BN_8_PARAMS);
    }

    #[test]
    fn gmimc_bn256_t9_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&GMIMC_BN_9_PARAMS);
    }

    #[test]
    fn gmimc_bn256_t12_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&GMIMC_BN_12_PARAMS);
    }
}
