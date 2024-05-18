use std::sync::Arc;

use bellman_ce::pairing::ff::PrimeField;

use crate::r1cs::{Constraint, R1CS};

use super::rescue_params::RescueParams;

#[derive(Clone, Debug)]
pub struct RescueR1CS<S: PrimeField> {
    params: Arc<RescueParams<S>>,
    r1cs: R1CS<S>,
    ready: bool,
}

impl<S: PrimeField> RescueR1CS<S> {
    pub fn new(params: &Arc<RescueParams<S>>) -> Self {
        let vars = Self::get_num_vars(params.t, params.d, params.rounds);
        let r1cs = R1CS::new(vars);

        RescueR1CS {
            params: Arc::clone(params),
            r1cs,
            ready: false,
        }
    }

    pub fn get_num_vars(t: usize, d: usize, rounds: usize) -> usize {
        let d_inc;
        if d == 3 {
            d_inc = 4;
        } else if d == 5 {
            d_inc = 6;
        } else {
            panic!();
        }

        // includes input, output constraints and var=1
        let constraints = d_inc * t * rounds + t;
        t + 1 + constraints
    }

    pub fn permutation(&mut self, constraints: &[Constraint<S>]) -> Vec<Constraint<S>> {
        let mut current_state = constraints.to_owned();
        for r in 0..self.params.rounds {
            current_state = self.sbox(&current_state);
            current_state = self.r1cs.matrix_mul(&current_state, &self.params.mds);
            current_state = self
                .r1cs
                .add_rc(&current_state, &self.params.round_constants[2 * r]);

            current_state = self.sbox_inverse(&current_state);
            current_state = self.r1cs.matrix_mul(&current_state, &self.params.mds);
            current_state = self
                .r1cs
                .add_rc(&current_state, &self.params.round_constants[2 * r + 1]);
        }
        current_state
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

    fn sbox(&mut self, constraints: &[Constraint<S>]) -> Vec<Constraint<S>> {
        let t = self.params.t;
        let mut result = constraints.to_owned();
        let mut sqs: Vec<Constraint<S>> = Vec::with_capacity(t);
        for r in result.iter() {
            let mut sq = self.r1cs.multiplication_new(r, r);
            if self.params.d == 5 {
                let qu = self.r1cs.multiplication_new(&sq, &sq);
                sq = qu;
            }
            sqs.push(sq);
        }

        for i in 0..t {
            result[i] = self.r1cs.multiplication_new(&result[i], &sqs[i]);
        }
        result
    }

    fn sbox_inverse(&mut self, constraints: &[Constraint<S>]) -> Vec<Constraint<S>> {
        let t = self.params.t;
        let result: Vec<Constraint<S>> = (0..t).map(|_| self.r1cs.new_variable()).collect();

        let mut sqs: Vec<Constraint<S>> = Vec::with_capacity(t);
        for r in result.iter() {
            let mut sq = self.r1cs.multiplication_new(r, r);
            if self.params.d == 5 {
                let qu = self.r1cs.multiplication_new(&sq, &sq);
                sq = qu;
            }
            sqs.push(sq);
        }

        for i in 0..t {
            self.r1cs
                .multiplication(&result[i], &sqs[i], &constraints[i])
        }
        result
    }
}

#[derive(Clone, Debug)]
pub struct RescueWitness<S: PrimeField> {
    params: Arc<RescueParams<S>>,
    vars: usize,
}

impl<S: PrimeField> RescueWitness<S> {
    pub fn new(params: &Arc<RescueParams<S>>) -> Self {
        let vars = RescueR1CS::<S>::get_num_vars(params.t, params.d, params.rounds);

        RescueWitness {
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

        for r in 0..self.params.rounds {
            current_state = self.sbox(&current_state, w, index);
            current_state = self.matmul(&current_state, &self.params.mds);
            current_state = self.add_rc(&current_state, &self.params.round_constants[2 * r]);
            index += t * inc;

            current_state = self.sbox_inverse(&current_state, w, index);
            current_state = self.matmul(&current_state, &self.params.mds);
            current_state = self.add_rc(&current_state, &self.params.round_constants[2 * r + 1]);
            index += t * inc;
        }

        // output state
        for i in 0..t {
            w[index + i] = Some(current_state[i]);
        }
        current_state
    }

    fn sbox(&self, input: &[S], w: &mut [Option<S>], index: usize) -> Vec<S> {
        let mut index = index;
        let result: Vec<S> = input
            .iter()
            .map(|inp| {
                let mut inp2 = *inp;
                inp2.square();
                w[index] = Some(inp2);
                index += 1;

                match self.params.d {
                    3 => {
                        let mut r = inp2;
                        r.mul_assign(inp);
                        r
                    }
                    5 => {
                        let mut inp4 = inp2;
                        inp4.square();
                        w[index] = Some(inp4);
                        index += 1;
                        inp4.mul_assign(inp);
                        inp4
                    }
                    _ => {
                        panic!();
                    }
                }
            })
            .collect();
        for el in &result {
            w[index] = Some(*el);
            index += 1;
        }
        result
    }

    fn sbox_inverse(&self, input: &[S], w: &mut [Option<S>], index: usize) -> Vec<S> {
        let power: Vec<S> = input.iter().map(|el| el.pow(self.params.d_inv)).collect();

        let mut index = index;
        for el in &power {
            w[index] = Some(*el);
            index += 1;
        }

        power.iter().for_each(|inp| {
            let mut inp2 = *inp;
            inp2.square();
            w[index] = Some(inp2);
            index += 1;
            match self.params.d {
                3 => {}
                5 => {
                    let mut inp4 = inp2;
                    inp4.square();
                    w[index] = Some(inp4);
                    index += 1;
                }
                _ => {
                    panic!();
                }
            };
        });

        power
    }

    fn matmul(&self, input: &[S], mat: &[Vec<S>]) -> Vec<S> {
        let t = mat.len();
        debug_assert!(t == input.len());
        let mut out = vec![S::zero(); t];
        for row in 0..t {
            for (col, inp) in input.iter().enumerate() {
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
mod rescue_r1cs_test {
    use bellman_ce::pairing::{bls12_381, bn256};

    use crate::{
        circuits::Permutation,
        rescue::{rescue::Rescue, rescue_instance_bls12::*, rescue_instance_bn256::*},
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

    fn witness_perm<S: PrimeField>(params: &Arc<RescueParams<S>>) {
        let rescue = Rescue::new(params);
        let rescue_wit = RescueWitness::new(params);
        let t = rescue.params.t;
        for _ in 0..TESTRUNS {
            let input: Vec<S> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let perm = rescue.permutation(&input);
            let wit = rescue_wit.create(&input);
            let start = wit.len() - t;
            for i in 0..t {
                assert_eq!(perm[i], wit[start + i].unwrap());
            }
        }
    }

    fn witness_r1cs<S: PrimeField>(params: &Arc<RescueParams<S>>) {
        let mut rescue_r1cs = RescueR1CS::new(params);
        let rescue_wit = RescueWitness::new(params);
        let t = params.t;
        rescue_r1cs.synthesize();

        for _ in 0..TESTRUNS {
            let input: Vec<S> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let wit = rescue_wit.create(&input);
            let lhs = mat_vector_mul(rescue_r1cs.r1cs.get_lhs(), &wit);
            let rhs = mat_vector_mul(rescue_r1cs.r1cs.get_rhs(), &wit);
            let res = mat_vector_mul(rescue_r1cs.r1cs.get_res(), &wit);

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
    fn rescue_bls12_t3_witness_test() {
        witness_perm::<bls12_381::Fr>(&RESCUE_BLS_3_PARAMS);
    }

    #[test]
    fn rescue_bls12_t4_witness_test() {
        witness_perm::<bls12_381::Fr>(&RESCUE_BLS_4_PARAMS);
    }

    #[test]
    fn rescue_bls12_t5_witness_test() {
        witness_perm::<bls12_381::Fr>(&RESCUE_BLS_5_PARAMS);
    }

    #[test]
    fn rescue_bls12_t8_witness_test() {
        witness_perm::<bls12_381::Fr>(&RESCUE_BLS_8_PARAMS);
    }

    #[test]
    fn rescue_bls12_t9_witness_test() {
        witness_perm::<bls12_381::Fr>(&RESCUE_BLS_9_PARAMS);
    }

    #[test]
    fn rescue_bls12_t12_witness_test() {
        witness_perm::<bls12_381::Fr>(&RESCUE_BLS_12_PARAMS);
    }

    #[test]
    fn rescue_bn256_t3_witness_test() {
        witness_perm::<bn256::Fr>(&RESCUE_BN_3_PARAMS);
    }

    #[test]
    fn rescue_bn256_t4_witness_test() {
        witness_perm::<bn256::Fr>(&RESCUE_BN_4_PARAMS);
    }

    #[test]
    fn rescue_bn256_t5_witness_test() {
        witness_perm::<bn256::Fr>(&RESCUE_BN_5_PARAMS);
    }

    #[test]
    fn rescue_bn256_t8_witness_test() {
        witness_perm::<bn256::Fr>(&RESCUE_BN_8_PARAMS);
    }

    #[test]
    fn rescue_bn256_t9_witness_test() {
        witness_perm::<bn256::Fr>(&RESCUE_BN_9_PARAMS);
    }

    #[test]
    fn rescue_bn256_t12_witness_test() {
        witness_perm::<bn256::Fr>(&RESCUE_BN_12_PARAMS);
    }

    #[test]
    fn rescue_bls12_t3_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&RESCUE_BLS_3_PARAMS);
    }

    #[test]
    fn rescue_bls12_t4_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&RESCUE_BLS_4_PARAMS);
    }

    #[test]
    fn rescue_bls12_t5_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&RESCUE_BLS_5_PARAMS);
    }

    #[test]
    fn rescue_bls12_t8_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&RESCUE_BLS_8_PARAMS);
    }

    #[test]
    fn rescue_bls12_t9_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&RESCUE_BLS_9_PARAMS);
    }

    #[test]
    fn rescue_bls12_t12_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&RESCUE_BLS_12_PARAMS);
    }

    #[test]
    fn rescue_bn256_t3_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&RESCUE_BN_3_PARAMS);
    }

    #[test]
    fn rescue_bn256_t4_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&RESCUE_BN_4_PARAMS);
    }

    #[test]
    fn rescue_bn256_t5_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&RESCUE_BN_5_PARAMS);
    }

    #[test]
    fn rescue_bn256_t8_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&RESCUE_BN_8_PARAMS);
    }

    #[test]
    fn rescue_bn256_t9_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&RESCUE_BN_9_PARAMS);
    }

    #[test]
    fn rescue_bn256_t12_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&RESCUE_BN_12_PARAMS);
    }
}
