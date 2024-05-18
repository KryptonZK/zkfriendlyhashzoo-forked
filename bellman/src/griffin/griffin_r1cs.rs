use std::sync::Arc;

use bellman_ce::pairing::ff::{PrimeField, SqrtField};

use crate::r1cs::{Constraint, R1CS};

use super::griffin_params::GriffinParams;

#[derive(Clone, Debug)]
pub struct GriffinR1CS<S: PrimeField + SqrtField> {
    params: Arc<GriffinParams<S>>,
    r1cs: R1CS<S>,
    ready: bool,
}

impl<S: PrimeField + SqrtField> GriffinR1CS<S> {
    pub fn new(params: &Arc<GriffinParams<S>>) -> Self {
        let vars = Self::get_num_vars(params.t, params.d, params.rounds);
        let r1cs = R1CS::new(vars);

        GriffinR1CS {
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
        let constraints = rounds * (d_inc + d_inc + 2 * t - 4) + t;
        t + 1 + constraints
    }

    fn l(
        &self,
        y01_i: &mut Constraint<S>,
        y0: &Constraint<S>,
        x: &Constraint<S>,
        i: usize,
    ) -> Constraint<S> {
        if i == 0 {
            y01_i.to_owned()
        } else {
            *y01_i = self.r1cs.addition(y01_i, y0);
            self.r1cs.addition(y01_i, x)
        }
    }

    fn non_linear(&mut self, constraints: &[Constraint<S>]) -> Vec<Constraint<S>> {
        let mut result = constraints.to_owned();
        // x0
        result[0] = self.r1cs.new_variable();
        let mut sq = self.r1cs.multiplication_new(&result[0], &result[0]);
        if self.params.d == 5 {
            let qu = self.r1cs.multiplication_new(&sq, &sq);
            sq = qu;
        }
        self.r1cs.multiplication(&result[0], &sq, &constraints[0]);

        // x1
        let mut sq = self.r1cs.multiplication_new(&result[1], &result[1]);
        if self.params.d == 5 {
            let qu = self.r1cs.multiplication_new(&sq, &sq);
            sq = qu;
        }
        result[1] = self.r1cs.multiplication_new(&result[1], &sq);

        let y0 = result[0].to_owned(); // y0
        let mut y01_i = self.r1cs.addition(&y0, &result[1]); // y0 + y1

        // rest of the state
        for (i, ((out, inp), con)) in result
            .iter_mut()
            .skip(2)
            .zip(constraints.iter().skip(1))
            .zip(self.params.alpha_beta.iter())
            .enumerate()
        {
            let mut l = self.l(&mut y01_i, &y0, inp, i);
            let l_squ = self.r1cs.multiplication_new(&l, &l);
            l = self.r1cs.scale(&l, &con[0]);
            l = self.r1cs.addition(&l, &l_squ);
            l = self.r1cs.add_const(&l, &con[1]);
            *out = self.r1cs.multiplication_new(out, &l);
        }

        result
    }

    pub fn permutation(&mut self, constraints: &[Constraint<S>]) -> Vec<Constraint<S>> {
        let mut current_state = constraints.to_owned();
        current_state = self.r1cs.matrix_mul(&current_state, &self.params.mat);

        for r in 0..self.params.rounds {
            current_state = self.non_linear(&current_state);
            current_state = self.r1cs.matrix_mul(&current_state, &self.params.mat);
            if r < self.params.rounds - 1 {
                current_state = self
                    .r1cs
                    .add_rc(&current_state, &self.params.round_constants[r]);
            }
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
pub struct GriffinWitness<S: PrimeField + SqrtField> {
    params: Arc<GriffinParams<S>>,
    vars: usize,
}

impl<S: PrimeField + SqrtField> GriffinWitness<S> {
    pub fn new(params: &Arc<GriffinParams<S>>) -> Self {
        let vars = GriffinR1CS::<S>::get_num_vars(params.t, params.d, params.rounds);

        GriffinWitness {
            params: Arc::clone(params),
            vars,
        }
    }

    fn affine_3(&self, input: &mut [S], round: usize) {
        // multiplication by circ(2 1 1) is equal to state + sum(state)
        let mut sum = input[0];
        input.iter().skip(1).for_each(|el| sum.add_assign(el));

        if round < self.params.rounds - 1 {
            for (el, rc) in input
                .iter_mut()
                .zip(self.params.round_constants[round].iter())
            {
                el.add_assign(&sum);
                el.add_assign(rc); // add round constant
            }
        } else {
            // no round constant
            for el in input.iter_mut() {
                el.add_assign(&sum);
            }
        }
    }

    fn affine_4(&self, input: &mut [S], round: usize) {
        let mut t_0 = input[0];
        t_0.add_assign(&input[1]);
        let mut t_1 = input[2];
        t_1.add_assign(&input[3]);
        let mut t_2 = input[1];
        t_2.double();
        t_2.add_assign(&t_1);
        let mut t_3 = input[3];
        t_3.double();
        t_3.add_assign(&t_0);
        let mut t_4 = t_1;
        t_4.double();
        t_4.double();
        t_4.add_assign(&t_3);
        let mut t_5 = t_0;
        t_5.double();
        t_5.double();
        t_5.add_assign(&t_2);
        let mut t_6 = t_3;
        t_6.add_assign(&t_5);
        let mut t_7 = t_2;
        t_7.add_assign(&t_4);
        input[0] = t_6;
        input[1] = t_5;
        input[2] = t_7;
        input[3] = t_4;

        if round < self.params.rounds - 1 {
            for (i, rc) in input
                .iter_mut()
                .zip(self.params.round_constants[round].iter())
            {
                i.add_assign(rc);
            }
        }
    }

    fn affine(&self, input: &mut [S], round: usize) {
        if self.params.t == 3 {
            self.affine_3(input, round);
            return;
        }
        if self.params.t == 4 {
            self.affine_4(input, round);
            return;
        }

        // first matrix
        let t4 = self.params.t / 4;
        for i in 0..t4 {
            let start_index = i * 4;
            let mut t_0 = input[start_index];
            t_0.add_assign(&input[start_index + 1]);
            let mut t_1 = input[start_index + 2];
            t_1.add_assign(&input[start_index + 3]);
            let mut t_2 = input[start_index + 1];
            t_2.double();
            t_2.add_assign(&t_1);
            let mut t_3 = input[start_index + 3];
            t_3.double();
            t_3.add_assign(&t_0);
            let mut t_4 = t_1;
            t_4.double();
            t_4.double();
            t_4.add_assign(&t_3);
            let mut t_5 = t_0;
            t_5.double();
            t_5.double();
            t_5.add_assign(&t_2);
            let mut t_6 = t_3;
            t_6.add_assign(&t_5);
            let mut t_7 = t_2;
            t_7.add_assign(&t_4);
            input[start_index] = t_6;
            input[start_index + 1] = t_5;
            input[start_index + 2] = t_7;
            input[start_index + 3] = t_4;
        }

        // second matrix
        let mut stored = [S::zero(); 4];
        for l in 0..4 {
            stored[l] = input[l];
            for j in 1..t4 {
                stored[l].add_assign(&input[4 * j + l]);
            }
        }

        for i in 0..input.len() {
            input[i].add_assign(&stored[i % 4]);
            if round < self.params.rounds - 1 {
                input[i].add_assign(&self.params.round_constants[round][i]); // add round constant
            }
        }
    }

    fn l(y01_i: &mut S, y0: &S, x: &S, i: usize) -> S {
        if i == 0 {
            y01_i.to_owned()
        } else {
            y01_i.add_assign(y0);
            let mut out = y01_i.to_owned();
            out.add_assign(x);
            out
        }
    }

    fn non_linear(&self, input: &[S], w: &mut [Option<S>], index: usize) -> Vec<S> {
        let mut index = index;

        // first two state words
        let mut output = input.to_owned();
        output[0] = output[0].pow(self.params.d_inv);
        w[index] = Some(output[0]);
        index += 1;

        let mut power = output[0];
        power.square();
        w[index] = Some(power);
        index += 1;
        match self.params.d {
            3 => {}
            5 => {
                power.square();
                w[index] = Some(power);
                index += 1;
            }
            _ => panic!(),
        }

        output[1].square();
        w[index] = Some(output[1]);
        index += 1;
        match self.params.d {
            3 => {}
            5 => {
                output[1].square();
                w[index] = Some(output[1]);
                index += 1;
            }
            _ => panic!(),
        }
        output[1].mul_assign(&input[1]);
        w[index] = Some(output[1]);
        index += 1;

        let mut y01_i = output[0].to_owned(); // y0
        let y0 = y01_i.to_owned(); // y0
        y01_i.add_assign(&output[1]); // y0 + y1

        // rest of the state
        for (i, ((out, inp), con)) in output
            .iter_mut()
            .skip(2)
            .zip(input.iter().skip(1))
            .zip(self.params.alpha_beta.iter())
            .enumerate()
        {
            let mut l = Self::l(&mut y01_i, &y0, inp, i);
            let mut l_squ = l.to_owned();
            l_squ.square();
            w[index] = Some(l_squ);
            index += 1;
            l.mul_assign(&con[0]);
            l.add_assign(&l_squ);
            l.add_assign(&con[1]);
            out.mul_assign(&l);
            w[index] = Some(out.to_owned());
            index += 1;
        }

        output
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
        self.affine(&mut current_state, self.params.rounds); // no RC

        for r in 0..self.params.rounds {
            current_state = self.non_linear(&current_state, w, index);
            index += inc + inc + 2 * t - 4;
            self.affine(&mut current_state, r);
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
mod griffin_r1cs_test {
    use bellman_ce::pairing::{bls12_381, bn256, ff::SqrtField};

    use crate::{
        circuits::Permutation,
        griffin::{griffin::Griffin, griffin_instances::*},
        utils,
    };

    use super::*;

    static TESTRUNS: usize = 5;

    fn mat_vector_mul<S: PrimeField + SqrtField>(
        mat: &[Constraint<S>],
        vec: &[Option<S>],
    ) -> Vec<S> {
        let rows = mat.len();
        let cols = vec.len();

        let mut out = vec![S::zero(); rows];
        for row in 0..rows {
            for (col, v) in vec.iter().enumerate().take(cols) {
                let mut tmp = mat[row][col];
                let vecval = v.unwrap();
                tmp.mul_assign(&vecval);
                out[row].add_assign(&tmp);
            }
        }
        out
    }

    fn witness_perm<S: PrimeField + SqrtField>(params: &Arc<GriffinParams<S>>) {
        let griffin = Griffin::new(params);
        let griffin_wit = GriffinWitness::new(params);
        let t = griffin.params.t;
        for _ in 0..TESTRUNS {
            let input: Vec<S> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let perm = griffin.permutation(&input);
            let wit = griffin_wit.create(&input);
            let start = wit.len() - t;
            for i in 0..t {
                assert_eq!(perm[i], wit[start + i].unwrap());
            }
        }
    }

    fn witness_r1cs<S: PrimeField + SqrtField>(params: &Arc<GriffinParams<S>>) {
        let mut griffin_r1cs = GriffinR1CS::new(params);
        let griffin_wit = GriffinWitness::new(params);
        let t = params.t;
        griffin_r1cs.synthesize();

        for _ in 0..TESTRUNS {
            let input: Vec<S> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let wit = griffin_wit.create(&input);
            let lhs = mat_vector_mul(griffin_r1cs.r1cs.get_lhs(), &wit);
            let rhs = mat_vector_mul(griffin_r1cs.r1cs.get_rhs(), &wit);
            let res = mat_vector_mul(griffin_r1cs.r1cs.get_res(), &wit);

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
    fn griffin_bls12_t3_witness_test() {
        witness_perm::<bls12_381::Fr>(&GRIFFIN_BLS_3_PARAMS);
    }

    #[test]
    fn griffin_bls12_t4_witness_test() {
        witness_perm::<bls12_381::Fr>(&GRIFFIN_BLS_4_PARAMS);
    }

    #[test]
    fn griffin_bls12_t8_witness_test() {
        witness_perm::<bls12_381::Fr>(&GRIFFIN_BLS_8_PARAMS);
    }

    #[test]
    fn griffin_bls12_t12_witness_test() {
        witness_perm::<bls12_381::Fr>(&GRIFFIN_BLS_12_PARAMS);
    }

    #[test]
    fn griffin_bn256_t3_witness_test() {
        witness_perm::<bn256::Fr>(&GRIFFIN_BN_3_PARAMS);
    }

    #[test]
    fn griffin_bn256_t4_witness_test() {
        witness_perm::<bn256::Fr>(&GRIFFIN_BN_4_PARAMS);
    }

    #[test]
    fn griffin_bn256_t8_witness_test() {
        witness_perm::<bn256::Fr>(&GRIFFIN_BN_8_PARAMS);
    }

    #[test]
    fn griffin_bn256_t12_witness_test() {
        witness_perm::<bn256::Fr>(&GRIFFIN_BN_12_PARAMS);
    }

    #[test]
    fn griffin_bls12_t3_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&GRIFFIN_BLS_3_PARAMS);
    }

    #[test]
    fn griffin_bls12_t4_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&GRIFFIN_BLS_4_PARAMS);
    }

    #[test]
    fn griffin_bls12_t8_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&GRIFFIN_BLS_8_PARAMS);
    }

    #[test]
    fn griffin_bls12_t12_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&GRIFFIN_BLS_12_PARAMS);
    }

    #[test]
    fn griffin_bn256_t3_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&GRIFFIN_BN_3_PARAMS);
    }

    #[test]
    fn griffin_bn256_t4_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&GRIFFIN_BN_4_PARAMS);
    }

    #[test]
    fn griffin_bn256_t8_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&GRIFFIN_BN_8_PARAMS);
    }

    #[test]
    fn griffin_bn256_t12_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&GRIFFIN_BN_12_PARAMS);
    }
}
