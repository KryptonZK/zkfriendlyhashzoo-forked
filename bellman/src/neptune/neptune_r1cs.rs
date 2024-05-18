use std::sync::Arc;

use bellman_ce::pairing::ff::{PrimeField, SqrtField};

use crate::r1cs::{Constraint, R1CS};

use super::neptune_params::NeptuneParams;

#[derive(Clone, Debug)]
pub struct NeptuneR1CS<S: PrimeField + SqrtField> {
    params: Arc<NeptuneParams<S>>,
    r1cs: R1CS<S>,
    ready: bool,
}

impl<S: PrimeField + SqrtField> NeptuneR1CS<S> {
    pub fn new(params: &Arc<NeptuneParams<S>>) -> Self {
        let vars = Self::get_num_vars(
            params.t,
            params.d,
            params.rounds_f_beginning + params.rounds_f_end,
            params.rounds_p,
        );
        let r1cs = R1CS::new(vars);

        NeptuneR1CS {
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
        let constraints = rounds_f * t + d_inc * rounds_p + t;
        t + 1 + constraints
    }

    fn external_round(&mut self, input: &[Constraint<S>], r: usize) -> Vec<Constraint<S>> {
        let output = self.external_sbox(input);
        let output = self.external_matmul(&output);
        self.r1cs.add_rc(&output, &self.params.round_constants[r])
    }

    fn internal_round(&mut self, input: &[Constraint<S>], r: usize) -> Vec<Constraint<S>> {
        let output = self.internal_sbox(input);
        let output = self.internal_matmul(&output);
        self.r1cs.add_rc(&output, &self.params.round_constants[r])
    }

    fn sbox_d(&mut self, constraints: &Constraint<S>) -> Constraint<S> {
        let mut sq = self.r1cs.multiplication_new(constraints, constraints);
        if self.params.d == 5 {
            let qu = self.r1cs.multiplication_new(&sq, &sq);
            sq = qu;
        }
        self.r1cs.multiplication_new(&sq, constraints)
    }

    fn external_sbox_prime(
        &mut self,
        x1: &Constraint<S>,
        x2: &Constraint<S>,
    ) -> (Constraint<S>, Constraint<S>) {
        let zi = self.r1cs.subtraction(x1, x2);
        let zib = self.r1cs.multiplication_new(&zi, &zi);
        // zib = self.r1cs.scale(&zib, &self.params.abc[1]); // beta = 1

        // first terms
        let mut y1 = self.r1cs.addition(x1, x2);
        let mut y2 = y1.to_owned();
        y1 = self.r1cs.addition(&y1, x1);
        y2 = self.r1cs.addition(&y2, x2);
        y2 = self.r1cs.addition(&y2, x2);
        // y1 = self.r1cs.scale(&y1, &self.params.a_[0]); // alpha = 1
        // y2 = self.r1cs.scale(&y2, &self.params.a_[0]); // alpha = 1

        // middle terms
        let tmp1 = self.r1cs.addition(&zib, &zib);
        let tmp1 = self.r1cs.addition(&tmp1, &zib);
        let tmp2 = self.r1cs.addition(&tmp1, &zib);
        // let tmp1 = self.r1cs.scale(&zib, &self.params.a_[1]); // done with additions, since alpha = beta = 1
        // let tmp2 = self.r1cs.scale(&zib, &self.params.a_[2]); // done with additions, since alpha = beta = 1
        y1 = self.r1cs.addition(&y1, &tmp1);
        y2 = self.r1cs.addition(&y2, &tmp2);

        // third terms
        let mut tmp = self.r1cs.subtraction(&zi, x2);
        // tmp = self.r1cs.scale(&tmp, &self.params.abc[0]); // alpha = 1
        tmp = self.r1cs.subtraction(&tmp, &zib);
        tmp = self.r1cs.add_const(&tmp, &self.params.abc[2]);
        tmp = self.r1cs.multiplication_new(&tmp, &tmp);
        // tmp = self.r1cs.scale(&tmp, &self.params.abc[1]); // beta = 1
        y1 = self.r1cs.addition(&y1, &tmp);
        y2 = self.r1cs.addition(&y2, &tmp);

        (y1, y2)
    }

    fn external_sbox(&mut self, input: &[Constraint<S>]) -> Vec<Constraint<S>> {
        let t = self.params.t;
        let t_ = t >> 1;
        let mut output = Vec::with_capacity(t);
        for i in 0..t_ {
            let out = self.external_sbox_prime(&input[2 * i], &input[2 * i + 1]);
            output.push(out.0);
            output.push(out.1);
        }
        output
    }

    fn internal_sbox(&mut self, input: &[Constraint<S>]) -> Vec<Constraint<S>> {
        let mut output = input.to_owned();
        output[0] = self.sbox_d(&input[0]);
        output
    }

    fn external_matmul(&self, input: &[Constraint<S>]) -> Vec<Constraint<S>> {
        let t = self.params.t;
        let mut result: Vec<Constraint<S>> = Vec::with_capacity(t);
        for row in self.params.m_e.iter() {
            let mut new_constraint = Constraint::zeros(self.r1cs.get_num_vars());
            for (con, vec) in input.iter().zip(row.iter()) {
                if *vec == S::zero() {
                    continue;
                }
                let tmp = self.r1cs.scale(con, vec);
                new_constraint = self.r1cs.addition(&new_constraint, &tmp);
            }
            result.push(new_constraint);
        }
        result
    }

    fn internal_matmul(&self, input: &[Constraint<S>]) -> Vec<Constraint<S>> {
        let mut result = input.to_owned();

        let mut sum = input[0].to_owned();
        input
            .iter()
            .skip(1)
            .for_each(|el| sum = self.r1cs.addition(&sum, el));

        for (r, mu) in result.iter_mut().zip(self.params.mu.iter()) {
            *r = self.r1cs.scale(r, mu);
            // *r = self.r1cs.sub_const(r, &input[row]); // Already done in parameter creation
            *r = self.r1cs.addition(r, &sum);
        }

        result
    }

    pub fn permutation(&mut self, constraints: &[Constraint<S>]) -> Vec<Constraint<S>> {
        // inital matmul
        let mut current_state = self.external_matmul(constraints);

        for r in 0..self.params.rounds_f_beginning {
            current_state = self.external_round(&current_state, r);
        }
        let p_end = self.params.rounds_f_beginning + self.params.rounds_p;
        for r in self.params.rounds_f_beginning..p_end {
            current_state = self.internal_round(&current_state, r);
        }
        for r in p_end..self.params.rounds {
            current_state = self.external_round(&current_state, r);
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
}

#[derive(Clone, Debug)]
pub struct NeptuneWitness<S: PrimeField + SqrtField> {
    params: Arc<NeptuneParams<S>>,
    vars: usize,
}

impl<S: PrimeField + SqrtField> NeptuneWitness<S> {
    pub fn new(params: &Arc<NeptuneParams<S>>) -> Self {
        let vars = NeptuneR1CS::<S>::get_num_vars(
            params.t,
            params.d,
            params.rounds_f_beginning + params.rounds_f_end,
            params.rounds_p,
        );

        NeptuneWitness {
            params: Arc::clone(params),
            vars,
        }
    }

    fn external_round(&self, input: &[S], r: usize, w: &mut [Option<S>], index: usize) -> Vec<S> {
        let output = self.external_sbox(input, w, index);
        let output = self.external_matmul(&output);
        self.add_rc(&output, &self.params.round_constants[r])
    }

    fn internal_round(&self, input: &[S], r: usize, w: &mut [Option<S>], index: usize) -> Vec<S> {
        let output = self.internal_sbox(input, w, index);
        let output = self.internal_matmul(&output);
        self.add_rc(&output, &self.params.round_constants[r])
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

    fn sbox_d(&self, input: &S, w: &mut [Option<S>], index: usize) -> S {
        let mut input2 = *input;
        input2.square();
        w[index] = Some(input2);
        let mut index = index + 1;

        match self.params.d {
            3 => {
                let mut out = input2;
                out.mul_assign(input);
                w[index] = Some(out);
                out
            }
            5 => {
                let mut out = input2;
                out.square();
                w[index] = Some(out);
                index += 1;
                out.mul_assign(input);
                w[index] = Some(out);
                out
            }
            _ => {
                panic!();
            }
        }
    }

    fn external_sbox_prime(&self, x1: &S, x2: &S, w: &mut [Option<S>], index: usize) -> (S, S) {
        let mut zi = x1.to_owned();
        zi.sub_assign(x2);
        let mut zib = zi;
        zib.square();
        w[index] = Some(zib);
        // zib.mul_assign(&self.params.abc[1]); // beta = 1

        // first terms
        let mut sum = x1.to_owned();
        sum.add_assign(x2);
        let mut y1 = sum.to_owned();
        let mut y2 = sum.to_owned();
        y1.add_assign(x1);
        y2.add_assign(x2);
        y2.add_assign(x2);
        // y1.mul_assign(&self.params.a_[0]); // alpha = 1
        // y2.mul_assign(&self.params.a_[0]); // alpha = 1

        // middle terms
        let mut tmp1 = zib.to_owned();
        tmp1.double();
        let mut tmp2 = tmp1.to_owned();
        tmp1.add_assign(&zib);
        tmp2.double();
        // tmp1.mul_assign(&self.params.a_[1]); // done with additions, since alpha = beta = 1
        // tmp2.mul_assign(&self.params.a_[2]); // done with additions, since alpha = beta = 1
        y1.add_assign(&tmp1);
        y2.add_assign(&tmp2);

        // third terms
        let mut tmp = zi.to_owned();
        tmp.sub_assign(x2);
        // tmp.mul_assign(&self.params.abc[0]); // alpha = 1
        tmp.sub_assign(&zib);
        tmp.add_assign(&self.params.abc[2]);
        tmp.square();
        w[index + 1] = Some(tmp);
        // tmp.mul_assign(&self.params.abc[1]); // beta = 1
        y1.add_assign(&tmp);
        y2.add_assign(&tmp);

        (y1, y2)
    }

    fn external_sbox(&self, input: &[S], w: &mut [Option<S>], mut index: usize) -> Vec<S> {
        let t = input.len();
        let t_ = t >> 1;
        let mut output = vec![S::zero(); t];
        for i in 0..t_ {
            let out = self.external_sbox_prime(&input[2 * i], &input[2 * i + 1], w, index);
            index += 2;
            output[2 * i] = out.0;
            output[2 * i + 1] = out.1;
        }
        output
    }

    fn internal_sbox(&self, input: &[S], w: &mut [Option<S>], index: usize) -> Vec<S> {
        let mut output = input.to_owned();
        output[0] = self.sbox_d(&input[0], w, index);
        output
    }

    fn external_matmul_4(input: &[S]) -> Vec<S> {
        let mut output = input.to_owned();
        output.swap(1, 3);

        let mut sum1 = input[0].to_owned();
        sum1.add_assign(&input[2]);
        let mut sum2 = input[1].to_owned();
        sum2.add_assign(&input[3]);

        output[0].add_assign(&sum1);
        output[1].add_assign(&sum2);
        output[2].add_assign(&sum1);
        output[3].add_assign(&sum2);

        output
    }

    fn external_matmul_8(input: &[S]) -> Vec<S> {
        // multiplication by circ(3 2 1 1) is equal to state + state + rot(state) + sum(state)
        let mut output = input.to_owned();
        output.swap(1, 7);
        output.swap(3, 5);

        let mut sum1 = input[0].to_owned();
        let mut sum2 = input[1].to_owned();

        input
            .iter()
            .step_by(2)
            .skip(1)
            .for_each(|el| sum1.add_assign(el));
        input
            .iter()
            .skip(1)
            .step_by(2)
            .skip(1)
            .for_each(|el| sum2.add_assign(el));

        let mut output_rot = output.to_owned();
        output_rot.rotate_left(2);

        for ((i, el), rot) in output.iter_mut().enumerate().zip(output_rot.iter()) {
            el.double();
            el.add_assign(rot);
            if i & 1 == 0 {
                el.add_assign(&sum1);
            } else {
                el.add_assign(&sum2);
            }
        }

        output.swap(3, 7);
        output
    }

    fn external_matmul(&self, input: &[S]) -> Vec<S> {
        let t = self.params.t;

        if t == 4 {
            return Self::external_matmul_4(input);
        } else if t == 8 {
            return Self::external_matmul_8(input);
        }

        let mut out = vec![S::zero(); t];
        let t_ = t >> 1;
        for row in 0..t_ {
            for col in 0..t_ {
                // even rows
                let mut tmp_e = self.params.m_e[2 * row][2 * col];
                tmp_e.mul_assign(&input[2 * col]);
                out[2 * row].add_assign(&tmp_e);

                // odd rows
                let mut tmp_o = self.params.m_e[2 * row + 1][2 * col + 1];
                tmp_o.mul_assign(&input[2 * col + 1]);
                out[2 * row + 1].add_assign(&tmp_o);
            }
        }
        out
    }

    fn internal_matmul(&self, input: &[S]) -> Vec<S> {
        let mut out = input.to_owned();

        let mut sum = input[0];
        input.iter().skip(1).for_each(|el| sum.add_assign(el));

        for (o, mu) in out.iter_mut().zip(self.params.mu.iter()) {
            o.mul_assign(mu);
            // o.sub_assign(input[row]); // Already done in parameter creation
            o.add_assign(&sum);
        }
        out
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

        let mut current_state = self.external_matmul(input);

        for r in 0..self.params.rounds_f_beginning {
            current_state = self.external_round(&current_state, r, w, index);
            index += t
        }
        let p_end = self.params.rounds_f_beginning + self.params.rounds_p;
        for r in self.params.rounds_f_beginning..p_end {
            current_state = self.internal_round(&current_state, r, w, index);
            index += inc
        }

        for r in p_end..self.params.rounds {
            current_state = self.external_round(&current_state, r, w, index);
            index += t
        }

        // output state
        for i in 0..t {
            w[index + i] = Some(current_state[i]);
        }
        current_state
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
mod neptune_r1cs_test {
    use bellman_ce::pairing::{bls12_381, bn256, ff::SqrtField};

    use crate::{
        circuits::Permutation,
        neptune::{neptune::Neptune, neptune_instances::*},
        utils,
    };

    use super::*;

    static TESTRUNS: usize = 5;

    fn mat_vector_mul<S: PrimeField + SqrtField>(
        mat: &[Constraint<S>],
        vec: &[Option<S>],
    ) -> Vec<S> {
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

    fn witness_perm<S: PrimeField + SqrtField>(params: &Arc<NeptuneParams<S>>) {
        let neptune = Neptune::new(params);
        let neptune_wit = NeptuneWitness::new(params);
        let t = neptune.params.t;
        for _ in 0..TESTRUNS {
            let input: Vec<S> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let perm = neptune.permutation(&input);
            let wit = neptune_wit.create(&input);
            let start = wit.len() - t;
            for i in 0..t {
                assert_eq!(perm[i], wit[start + i].unwrap());
            }
        }
    }

    fn witness_r1cs<S: PrimeField + SqrtField>(params: &Arc<NeptuneParams<S>>) {
        let mut neptune_r1cs = NeptuneR1CS::new(params);
        let neptune_wit = NeptuneWitness::new(params);
        let t = params.t;
        neptune_r1cs.synthesize();

        for _ in 0..TESTRUNS {
            let input: Vec<S> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let wit = neptune_wit.create(&input);
            let lhs = mat_vector_mul(neptune_r1cs.r1cs.get_lhs(), &wit);
            let rhs = mat_vector_mul(neptune_r1cs.r1cs.get_rhs(), &wit);
            let res = mat_vector_mul(neptune_r1cs.r1cs.get_res(), &wit);

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
    fn neptune_bls12_t4_witness_test() {
        witness_perm::<bls12_381::Fr>(&NEPTUNE_BLS_4_PARAMS);
    }

    #[test]
    fn neptune_bls12_t8_witness_test() {
        witness_perm::<bls12_381::Fr>(&NEPTUNE_BLS_8_PARAMS);
    }

    #[test]
    fn neptune_bls12_t12_witness_test() {
        witness_perm::<bls12_381::Fr>(&NEPTUNE_BLS_12_PARAMS);
    }

    #[test]
    fn neptune_bn256_t4_witness_test() {
        witness_perm::<bn256::Fr>(&NEPTUNE_BN_4_PARAMS);
    }

    #[test]
    fn neptune_bn256_t8_witness_test() {
        witness_perm::<bn256::Fr>(&NEPTUNE_BN_8_PARAMS);
    }

    #[test]
    fn neptune_bn256_t12_witness_test() {
        witness_perm::<bn256::Fr>(&NEPTUNE_BN_12_PARAMS);
    }

    #[test]
    fn neptune_bls12_t4_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&NEPTUNE_BLS_4_PARAMS);
    }

    #[test]
    fn neptune_bls12_t8_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&NEPTUNE_BLS_8_PARAMS);
    }

    #[test]
    fn neptune_bls12_t12_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&NEPTUNE_BLS_12_PARAMS);
    }

    #[test]
    fn neptune_bn256_t4_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&NEPTUNE_BN_4_PARAMS);
    }

    #[test]
    fn neptune_bn256_t8_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&NEPTUNE_BN_8_PARAMS);
    }

    #[test]
    fn neptune_bn256_t12_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&NEPTUNE_BN_12_PARAMS);
    }
}
