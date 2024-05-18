use std::sync::Arc;

use bellman_ce::pairing::ff::PrimeField;

use crate::r1cs::{Constraint, R1CS};

use super::poseidon_params::PoseidonParams;

#[derive(Clone, Debug)]
pub struct PoseidonR1CS<S: PrimeField> {
    params: Arc<PoseidonParams<S>>,
    r1cs: R1CS<S>,
    ready: bool,
}

impl<S: PrimeField> PoseidonR1CS<S> {
    pub fn new(params: &Arc<PoseidonParams<S>>) -> Self {
        let vars = Self::get_num_vars(
            params.t,
            params.d,
            params.rounds_f_beginning + params.rounds_f_end,
            params.rounds_p,
        );
        let r1cs = R1CS::new(vars);

        PoseidonR1CS {
            params: Arc::clone(params),
            r1cs,
            ready: false,
        }
    }

    pub fn get_num_vars(t: usize, d: usize, rounds_f: usize, rounds_p: usize) -> usize {
        let d_inc;
        if d == 3 {
            d_inc = 2;
        } else if d == 5 {
            d_inc = 3;
        } else {
            panic!();
        }

        // includes input, output constraints and var=1
        let constraints = (rounds_p + t * rounds_f) * d_inc + t;
        t + 1 + constraints
    }

    pub fn permutation(&mut self, constraints: &[Constraint<S>]) -> Vec<Constraint<S>> {
        let mut current_state = constraints.to_owned();
        for r in 0..self.params.rounds_f_beginning {
            current_state = self
                .r1cs
                .add_rc(&current_state, &self.params.round_constants[r]);
            current_state = self.sbox(&current_state);
            current_state = self.r1cs.matrix_mul(&current_state, &self.params.mds);
        }

        let p_end = self.params.rounds_f_beginning + self.params.rounds_p;
        for r in self.params.rounds_f_beginning..p_end {
            current_state = self
                .r1cs
                .add_rc(&current_state, &self.params.round_constants[r]);
            current_state[0] = self.sbox_p(&current_state, 0);
            current_state = self.r1cs.matrix_mul(&current_state, &self.params.mds);
        }
        for r in p_end..self.params.rounds {
            current_state = self
                .r1cs
                .add_rc(&current_state, &self.params.round_constants[r]);
            current_state = self.sbox(&current_state);
            current_state = self.r1cs.matrix_mul(&current_state, &self.params.mds);
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

    fn sbox_p(&mut self, constraints: &[Constraint<S>], index: usize) -> Constraint<S> {
        let mut result = constraints[index].to_owned();
        let mut sq = self.r1cs.multiplication_new(&result, &result);
        if self.params.d == 5 {
            let qu = self.r1cs.multiplication_new(&sq, &sq);
            sq = qu;
        }
        result = self.r1cs.multiplication_new(&result, &sq);
        result
    }
}

#[derive(Clone, Debug)]
pub struct PoseidonWitness<S: PrimeField> {
    params: Arc<PoseidonParams<S>>,
    vars: usize,
}

impl<S: PrimeField> PoseidonWitness<S> {
    pub fn new(params: &Arc<PoseidonParams<S>>) -> Self {
        let vars = PoseidonR1CS::<S>::get_num_vars(
            params.t,
            params.d,
            params.rounds_f_beginning + params.rounds_f_end,
            params.rounds_p,
        );

        PoseidonWitness {
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

        for r in 0..self.params.rounds_f_beginning {
            current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
            current_state = self.sbox(&current_state, w, index);
            current_state = self.matmul(&current_state, &self.params.mds);
            index += inc * t;
        }
        let p_end = self.params.rounds_f_beginning + self.params.rounds_p;
        current_state = self.add_rc(&current_state, &self.params.opt_round_constants[0]);
        current_state = self.matmul(&current_state, &self.params.m_i);

        for r in self.params.rounds_f_beginning..p_end {
            current_state[0] = self.sbox_p(&current_state[0], w, index);
            if r < p_end - 1 {
                current_state[0].add_assign(
                    &self.params.opt_round_constants[r + 1 - self.params.rounds_f_beginning][0],
                );
            }
            current_state = self.cheap_matmul(&current_state, p_end - r - 1);
            index += inc;
        }
        for r in p_end..self.params.rounds {
            current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
            current_state = self.sbox(&current_state, w, index);
            current_state = self.matmul(&current_state, &self.params.mds);
            index += inc * t;
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

    fn sbox_p(&self, input: &S, w: &mut [Option<S>], index: usize) -> S {
        let mut input2 = *input;
        input2.square();
        w[index] = Some(input2);
        let mut index = index + 1;
        let res = match self.params.d {
            3 => {
                let mut r = input2;
                r.mul_assign(input);
                r
            }
            5 => {
                let mut input4 = input2;
                input4.square();
                w[index] = Some(input4);
                index += 1;
                input4.mul_assign(input);
                input4
            }
            _ => {
                panic!();
            }
        };
        w[index] = Some(res);
        res
    }

    fn cheap_matmul(&self, input: &[S], r: usize) -> Vec<S> {
        let v = &self.params.v[r];
        let w_hat = &self.params.w_hat[r];
        let t = self.params.t;

        let mut new_state = vec![S::zero(); t];
        new_state[0] = self.params.mds[0][0];
        new_state[0].mul_assign(&input[0]);
        for i in 1..t {
            let mut tmp = w_hat[i - 1];
            tmp.mul_assign(&input[i]);
            new_state[0].add_assign(&tmp);
        }
        for i in 1..t {
            new_state[i] = input[0];
            new_state[i].mul_assign(&v[i - 1]);
            new_state[i].add_assign(&input[i]);
        }

        new_state
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
mod poseidon_r1cs_test {
    use bellman_ce::pairing::{bls12_381, bn256};

    use crate::{
        circuits::Permutation,
        poseidon::{poseidon::Poseidon, poseidon_instance_bls12::*, poseidon_instance_bn256::*},
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

    fn witness_perm<S: PrimeField>(params: &Arc<PoseidonParams<S>>) {
        let poseidon = Poseidon::new(params);
        let poseidon_wit = PoseidonWitness::new(params);
        let t = poseidon.params.t;
        for _ in 0..TESTRUNS {
            let input: Vec<S> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let perm = poseidon.permutation(&input);
            let wit = poseidon_wit.create(&input);
            let start = wit.len() - t;
            for i in 0..t {
                assert_eq!(perm[i], wit[start + i].unwrap());
            }
        }
    }

    fn witness_r1cs<S: PrimeField>(params: &Arc<PoseidonParams<S>>) {
        let mut poseidon_r1cs = PoseidonR1CS::new(params);
        let poseidon_wit = PoseidonWitness::new(params);
        let t = params.t;
        poseidon_r1cs.synthesize();

        for _ in 0..TESTRUNS {
            let input: Vec<S> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let wit = poseidon_wit.create(&input);
            let lhs = mat_vector_mul(poseidon_r1cs.r1cs.get_lhs(), &wit);
            let rhs = mat_vector_mul(poseidon_r1cs.r1cs.get_rhs(), &wit);
            let res = mat_vector_mul(poseidon_r1cs.r1cs.get_res(), &wit);

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
    fn poseidon_bls12_t3_witness_test() {
        witness_perm::<bls12_381::Fr>(&POSEIDON_BLS_3_PARAMS);
    }

    #[test]
    fn poseidon_bls12_t4_witness_test() {
        witness_perm::<bls12_381::Fr>(&POSEIDON_BLS_4_PARAMS);
    }

    #[test]
    fn poseidon_bls12_t5_witness_test() {
        witness_perm::<bls12_381::Fr>(&POSEIDON_BLS_5_PARAMS);
    }

    #[test]
    fn poseidon_bls12_t8_witness_test() {
        witness_perm::<bls12_381::Fr>(&POSEIDON_BLS_8_PARAMS);
    }

    #[test]
    fn poseidon_bls12_t9_witness_test() {
        witness_perm::<bls12_381::Fr>(&POSEIDON_BLS_9_PARAMS);
    }

    #[test]
    fn poseidon_bls12_t12_witness_test() {
        witness_perm::<bls12_381::Fr>(&POSEIDON_BLS_12_PARAMS);
    }

    #[test]
    fn poseidon_bn256_t3_witness_test() {
        witness_perm::<bn256::Fr>(&POSEIDON_BN_3_PARAMS);
    }

    #[test]
    fn poseidon_bn256_t4_witness_test() {
        witness_perm::<bn256::Fr>(&POSEIDON_BN_4_PARAMS);
    }

    #[test]
    fn poseidon_bn256_t5_witness_test() {
        witness_perm::<bn256::Fr>(&POSEIDON_BN_5_PARAMS);
    }

    #[test]
    fn poseidon_bn256_t8_witness_test() {
        witness_perm::<bn256::Fr>(&POSEIDON_BN_8_PARAMS);
    }

    #[test]
    fn poseidon_bn256_t9_witness_test() {
        witness_perm::<bn256::Fr>(&POSEIDON_BN_9_PARAMS);
    }

    #[test]
    fn poseidon_bn256_t12_witness_test() {
        witness_perm::<bn256::Fr>(&POSEIDON_BN_12_PARAMS);
    }

    #[test]
    fn poseidon_bls12_t3_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&POSEIDON_BLS_3_PARAMS);
    }

    #[test]
    fn poseidon_bls12_t4_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&POSEIDON_BLS_4_PARAMS);
    }

    #[test]
    fn poseidon_bls12_t5_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&POSEIDON_BLS_5_PARAMS);
    }

    #[test]
    fn poseidon_bls12_t8_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&POSEIDON_BLS_8_PARAMS);
    }

    #[test]
    fn poseidon_bls12_t9_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&POSEIDON_BLS_9_PARAMS);
    }

    #[test]
    fn poseidon_bls12_t12_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&POSEIDON_BLS_12_PARAMS);
    }

    #[test]
    fn poseidon_bn256_t3_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&POSEIDON_BN_3_PARAMS);
    }

    #[test]
    fn poseidon_bn256_t4_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&POSEIDON_BN_4_PARAMS);
    }

    #[test]
    fn poseidon_bn256_t5_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&POSEIDON_BN_5_PARAMS);
    }

    #[test]
    fn poseidon_bn256_t8_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&POSEIDON_BN_8_PARAMS);
    }

    #[test]
    fn poseidon_bn256_t9_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&POSEIDON_BN_9_PARAMS);
    }

    #[test]
    fn poseidon_bn256_t12_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&POSEIDON_BN_12_PARAMS);
    }
}
