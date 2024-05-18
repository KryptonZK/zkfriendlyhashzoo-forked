use std::sync::Arc;

use bellman_ce::pairing::{
    ff::{PrimeField, SqrtField},
    LegendreSymbol,
};

use crate::r1cs::{Constraint, R1CS};

use super::grendel_params::GrendelParams;

#[derive(Clone, Debug)]
pub struct GrendelR1CS<S: PrimeField + SqrtField> {
    params: Arc<GrendelParams<S>>,
    r1cs: R1CS<S>,
    ready: bool,
}

impl<S: PrimeField + SqrtField> GrendelR1CS<S> {
    pub fn new(params: &Arc<GrendelParams<S>>) -> Self {
        let vars = Self::get_num_vars(params.t, params.d, params.rounds);
        let r1cs = R1CS::new(vars);

        GrendelR1CS {
            params: Arc::clone(params),
            r1cs,
            ready: false,
        }
    }

    pub fn get_num_vars(t: usize, d: usize, rounds: usize) -> usize {
        let d_inc;
        if d == 2 {
            d_inc = 1;
        } else if d == 3 {
            d_inc = 2;
        } else if d == 5 {
            d_inc = 3;
        } else {
            panic!();
        }

        // includes input, output constraints and var=1
        let constraints = rounds * t * (7 + d_inc) + t;
        t + 1 + constraints
    }

    fn sbox(&mut self, constraints: &[Constraint<S>]) -> Vec<Constraint<S>> {
        let t = self.params.t;
        let zero = Constraint::zeros(self.r1cs.get_num_vars());
        let one = Constraint::one(self.r1cs.get_num_vars());
        let mut result = constraints.to_owned();
        let mut powers: Vec<Constraint<S>> = Vec::with_capacity(t);
        let mut ls: Vec<Constraint<S>> = Vec::with_capacity(t);
        for i in 0..t {
            // power
            let mut sq = self.r1cs.multiplication_new(&result[i], &result[i]);
            if self.params.d == 2 {
                powers.push(sq);
            } else {
                if self.params.d == 5 {
                    let qu = self.r1cs.multiplication_new(&sq, &sq);
                    sq = qu;
                }
                let power = self.r1cs.multiplication_new(&result[i], &sq);
                powers.push(power);
            }

            // legendre: (6 constraints)

            // l (l - 1)(l + 1) = 0
            let l = self.r1cs.new_variable();
            let lp1 = self.r1cs.add_const(&l, &S::one());
            let lm1 = self.r1cs.sub_const(&l, &S::one());
            let l_lm1 = self.r1cs.multiplication_new(&l, &lm1);
            self.r1cs.multiplication(&l_lm1, &lp1, &zero);
            ls.push(l);

            // l (l - 1)(b^2 - na) + (l + 1)(b^2 -a) = 0
            let b = self.r1cs.new_variable();
            let b_sq = self.r1cs.multiplication_new(&b, &b);
            let b_sq_a = self.r1cs.subtraction(&b_sq, &constraints[i]);
            let na = self.r1cs.scale(&constraints[i], &self.params.n);
            let b_sq_na = self.r1cs.subtraction(&b_sq, &na);
            let lhs = self.r1cs.multiplication_new(&l_lm1, &b_sq_na);
            let rhs = self.r1cs.multiplication_new(&lp1, &b_sq_a);
            let sum = self.r1cs.addition(&lhs, &rhs);

            self.r1cs.register_constraints(&sum, &one, &zero);
        }

        // combine powers and legendres
        for i in 0..t {
            result[i] = self.r1cs.multiplication_new(&powers[i], &ls[i]);
        }
        result
    }

    pub fn permutation(&mut self, constraints: &[Constraint<S>]) -> Vec<Constraint<S>> {
        let mut current_state = constraints.to_owned();

        for r in 0..self.params.rounds {
            current_state = self.sbox(&current_state);
            current_state = self.r1cs.matrix_mul(&current_state, &self.params.mds);
            current_state = self
                .r1cs
                .add_rc(&current_state, &self.params.round_constants[r]);
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
pub struct GrendelWitness<S: PrimeField + SqrtField> {
    params: Arc<GrendelParams<S>>,
    vars: usize,
}

impl<S: PrimeField + SqrtField> GrendelWitness<S> {
    pub fn new(params: &Arc<GrendelParams<S>>) -> Self {
        let vars = GrendelR1CS::<S>::get_num_vars(params.t, params.d, params.rounds);

        GrendelWitness {
            params: Arc::clone(params),
            vars,
        }
    }

    fn permutation(&self, input: &[S], w: &mut [Option<S>]) -> Vec<S> {
        let t = self.params.t;
        assert_eq!(input.len(), t);

        let mut inc = 1;
        if self.params.d == 3 {
            inc = 2;
        } else if self.params.d == 5 {
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
            index += (inc + 7) * t;
            current_state = self.matmul(&current_state, &self.params.mds);
            current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
        }

        // output state
        for i in 0..t {
            w[index + i] = Some(current_state[i]);
        }
        current_state
    }

    fn sbox(&self, input: &[S], w: &mut [Option<S>], index: usize) -> Vec<S> {
        let mut m1 = S::zero();
        m1.sub_assign(&S::one());

        let mut index = index;
        let result: Vec<S> = input
            .iter()
            .map(|inp| {
                // power
                let mut inp2 = *inp;
                inp2.square();
                w[index] = Some(inp2);
                index += 1;
                let mut res = match self.params.d {
                    2 => inp2,
                    3 => {
                        let mut r = inp2;
                        r.mul_assign(inp);
                        w[index] = Some(r);
                        index += 1;
                        r
                    }
                    5 => {
                        let mut inp4 = inp2;
                        inp4.square();
                        w[index] = Some(inp4);
                        index += 1;
                        inp4.mul_assign(inp);
                        w[index] = Some(inp4);
                        index += 1;
                        inp4
                    }
                    _ => {
                        panic!();
                    }
                };

                // legendre
                let mut l = match inp.legendre() {
                    LegendreSymbol::QuadraticNonResidue => {
                        res.negate();
                        m1
                    }
                    LegendreSymbol::QuadraticResidue => S::one(),
                    _ => {
                        res = S::zero();
                        S::zero()
                    }
                };

                w[index] = Some(l);
                index += 1;
                let mut lm1 = l.to_owned();
                lm1.sub_assign(&S::one());
                lm1.mul_assign(&l);
                w[index] = Some(lm1);
                index += 1;

                let mut an = inp.to_owned();
                an.mul_assign(&self.params.n);

                let mut b = match inp.sqrt() {
                    None => an.sqrt().expect("Something wrong?"),
                    Some(a) => a,
                };
                w[index] = Some(b);
                index += 1;

                b.square();
                w[index] = Some(b);
                index += 1;

                let mut b_sq_na = b.to_owned();
                b_sq_na.sub_assign(&an);
                b_sq_na.mul_assign(&lm1);
                w[index] = Some(b_sq_na);
                index += 1;

                b.sub_assign(inp);
                l.add_assign(&S::one());
                b.mul_assign(&l);
                w[index] = Some(b);
                index += 1;

                res
            })
            .collect();
        for el in &result {
            w[index] = Some(*el);
            index += 1;
        }
        result
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
mod grendel_r1cs_test {
    use bellman_ce::pairing::{bls12_381, bn256, ff::SqrtField};

    use crate::{
        circuits::Permutation,
        grendel::{grendel::Grendel, grendel_instance_bls12::*, grendel_instance_bn256::*},
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

    fn witness_perm<S: PrimeField + SqrtField>(params: &Arc<GrendelParams<S>>) {
        let grendel = Grendel::new(params);
        let grendel_wit = GrendelWitness::new(params);
        let t = grendel.params.t;
        for _ in 0..TESTRUNS {
            let input: Vec<S> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let perm = grendel.permutation(&input);
            let wit = grendel_wit.create(&input);
            let start = wit.len() - t;
            for i in 0..t {
                assert_eq!(perm[i], wit[start + i].unwrap());
            }
        }
    }

    fn witness_r1cs<S: PrimeField + SqrtField>(params: &Arc<GrendelParams<S>>) {
        let mut grendel_r1cs = GrendelR1CS::new(params);
        let grendel_wit = GrendelWitness::new(params);
        let t = params.t;
        grendel_r1cs.synthesize();

        for _ in 0..TESTRUNS {
            let input: Vec<S> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let wit = grendel_wit.create(&input);
            let lhs = mat_vector_mul(grendel_r1cs.r1cs.get_lhs(), &wit);
            let rhs = mat_vector_mul(grendel_r1cs.r1cs.get_rhs(), &wit);
            let res = mat_vector_mul(grendel_r1cs.r1cs.get_res(), &wit);

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
    fn grendel_bls12_t3_witness_test() {
        witness_perm::<bls12_381::Fr>(&GRENDEL_BLS_3_PARAMS);
    }

    #[test]
    fn grendel_bls12_t4_witness_test() {
        witness_perm::<bls12_381::Fr>(&GRENDEL_BLS_4_PARAMS);
    }

    #[test]
    fn grendel_bls12_t5_witness_test() {
        witness_perm::<bls12_381::Fr>(&GRENDEL_BLS_5_PARAMS);
    }

    #[test]
    fn grendel_bls12_t8_witness_test() {
        witness_perm::<bls12_381::Fr>(&GRENDEL_BLS_8_PARAMS);
    }

    #[test]
    fn grendel_bls12_t9_witness_test() {
        witness_perm::<bls12_381::Fr>(&GRENDEL_BLS_9_PARAMS);
    }

    #[test]
    fn grendel_bls12_t12_witness_test() {
        witness_perm::<bls12_381::Fr>(&GRENDEL_BLS_12_PARAMS);
    }

    #[test]
    fn grendel_bn256_t3_witness_test() {
        witness_perm::<bn256::Fr>(&GRENDEL_BN_3_PARAMS);
    }

    #[test]
    fn grendel_bn256_t4_witness_test() {
        witness_perm::<bn256::Fr>(&GRENDEL_BN_4_PARAMS);
    }

    #[test]
    fn grendel_bn256_t5_witness_test() {
        witness_perm::<bn256::Fr>(&GRENDEL_BN_5_PARAMS);
    }

    #[test]
    fn grendel_bn256_t8_witness_test() {
        witness_perm::<bn256::Fr>(&GRENDEL_BN_8_PARAMS);
    }

    #[test]
    fn grendel_bn256_t9_witness_test() {
        witness_perm::<bn256::Fr>(&GRENDEL_BN_9_PARAMS);
    }

    #[test]
    fn grendel_bn256_t12_witness_test() {
        witness_perm::<bn256::Fr>(&GRENDEL_BN_12_PARAMS);
    }

    #[test]
    fn grendel_bls12_t3_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&GRENDEL_BLS_3_PARAMS);
    }

    #[test]
    fn grendel_bls12_t4_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&GRENDEL_BLS_4_PARAMS);
    }

    #[test]
    fn grendel_bls12_t5_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&GRENDEL_BLS_5_PARAMS);
    }

    #[test]
    fn grendel_bls12_t8_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&GRENDEL_BLS_8_PARAMS);
    }

    #[test]
    fn grendel_bls12_t9_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&GRENDEL_BLS_9_PARAMS);
    }

    #[test]
    fn grendel_bls12_t12_r1cs_test() {
        witness_r1cs::<bls12_381::Fr>(&GRENDEL_BLS_12_PARAMS);
    }

    #[test]
    fn grendel_bn256_t3_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&GRENDEL_BN_3_PARAMS);
    }

    #[test]
    fn grendel_bn256_t4_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&GRENDEL_BN_4_PARAMS);
    }

    #[test]
    fn grendel_bn256_t5_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&GRENDEL_BN_5_PARAMS);
    }

    #[test]
    fn grendel_bn256_t8_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&GRENDEL_BN_8_PARAMS);
    }

    #[test]
    fn grendel_bn256_t9_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&GRENDEL_BN_9_PARAMS);
    }

    #[test]
    fn grendel_bn256_t12_r1cs_test() {
        witness_r1cs::<bn256::Fr>(&GRENDEL_BN_12_PARAMS);
    }
}
