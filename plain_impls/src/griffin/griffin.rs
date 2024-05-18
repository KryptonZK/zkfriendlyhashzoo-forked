use crate::merkle_tree::merkle_tree_fp::MerkleTreeHash;

use super::griffin_params::GriffinParams;
use ff::{PrimeField, SqrtField};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Griffin<S: PrimeField + SqrtField> {
    pub(crate) params: Arc<GriffinParams<S>>,
}

impl<S: PrimeField + SqrtField> Griffin<S> {
    pub fn new(params: &Arc<GriffinParams<S>>) -> Self {
        Griffin {
            params: Arc::clone(params),
        }
    }

    pub fn get_t(&self) -> usize {
        self.params.t
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

        for el in input.chunks_exact_mut(4) {
            let mut t_0 = el[0];
            t_0.add_assign(&el[1]);
            let mut t_1 = el[2];
            t_1.add_assign(&el[3]);
            let mut t_2 = el[1];
            t_2.double();
            t_2.add_assign(&t_1);
            let mut t_3 = el[3];
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
            el[0] = t_6;
            el[1] = t_5;
            el[2] = t_7;
            el[3] = t_4;
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

    fn non_linear(&self, input: &[S]) -> Vec<S> {
        // first two state words
        let mut output = input.to_owned();
        output[0] = output[0].pow(self.params.d_inv);

        output[1].square();
        match self.params.d {
            3 => {}
            5 => output[1].square(),
            7 => {
                let tmp = output[1];
                output[1].square();
                output[1].mul_assign(&tmp);
            }
            _ => panic!(),
        }
        output[1].mul_assign(&input[1]);

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
            l.mul_assign(&con[0]);
            l.add_assign(&l_squ);
            l.add_assign(&con[1]);
            out.mul_assign(&l);
        }

        output
    }

    pub fn permutation(&self, input: &[S]) -> Vec<S> {
        let mut current_state = input.to_owned();
        self.affine(&mut current_state, self.params.rounds); // no RC

        for r in 0..self.params.rounds {
            current_state = self.non_linear(&current_state);
            self.affine(&mut current_state, r);
        }
        current_state
    }
}

impl<S: PrimeField + SqrtField> MerkleTreeHash<S> for Griffin<S> {
    fn compress(&self, input: &[&S; 2]) -> S {
        self.permutation(&[input[0].to_owned(), input[1].to_owned(), S::zero()])[0]
    }
}

#[cfg(test)]
mod griffin_tests_bls12 {
    use ff::Field;

    use super::*;
    use crate::{
        fields::{bls12::FpBLS12, utils},
        griffin::griffin_instances::GRIFFIN_BLS_PARAMS,
    };

    type Scalar = FpBLS12;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let griffin = Griffin::new(&GRIFFIN_BLS_PARAMS);
        let t = griffin.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = griffin.permutation(&input1);
            let perm2 = griffin.permutation(&input1);
            let perm3 = griffin.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    fn matmul(input: &[Scalar], mat: &[Vec<Scalar>]) -> Vec<Scalar> {
        let t = mat.len();
        debug_assert!(t == input.len());
        let mut out = vec![Scalar::zero(); t];
        for row in 0..t {
            for (col, inp) in input.iter().enumerate().take(t) {
                let mut tmp = mat[row][col];
                tmp.mul_assign(inp);
                out[row].add_assign(&tmp);
            }
        }
        out
    }

    fn affine_test(t: usize) {
        let griffin_param = Arc::new(GriffinParams::<Scalar>::new(t, 5, 1));
        let griffin = Griffin::<Scalar>::new(&griffin_param);

        let mat = &griffin_param.mat;

        for _ in 0..TESTRUNS {
            let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            // affine 1
            let output1 = matmul(&input, mat);
            let mut output2 = input.to_owned();
            griffin.affine(&mut output2, 1);
            assert_eq!(output1, output2);
        }
    }

    #[test]
    fn affine_3() {
        affine_test(3);
    }

    #[test]
    fn affine_4() {
        affine_test(4);
    }

    #[test]
    fn affine_8() {
        affine_test(8);
    }

    #[test]
    fn affine_60() {
        affine_test(60);
    }
}

#[cfg(test)]
mod griffin_tests_bn256 {
    use ff::Field;

    use super::*;
    use crate::{
        fields::{bn256::FpBN256, utils},
        griffin::griffin_instances::GRIFFIN_BN_PARAMS,
    };

    type Scalar = FpBN256;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let griffin = Griffin::new(&GRIFFIN_BN_PARAMS);
        let t = griffin.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = griffin.permutation(&input1);
            let perm2 = griffin.permutation(&input1);
            let perm3 = griffin.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    fn matmul(input: &[Scalar], mat: &[Vec<Scalar>]) -> Vec<Scalar> {
        let t = mat.len();
        debug_assert!(t == input.len());
        let mut out = vec![Scalar::zero(); t];
        for row in 0..t {
            for (col, inp) in input.iter().enumerate().take(t) {
                let mut tmp = mat[row][col];
                tmp.mul_assign(inp);
                out[row].add_assign(&tmp);
            }
        }
        out
    }

    fn affine_test(t: usize) {
        let griffin_param = Arc::new(GriffinParams::<Scalar>::new(t, 5, 1));
        let griffin = Griffin::<Scalar>::new(&griffin_param);

        let mat = &griffin_param.mat;

        for _ in 0..TESTRUNS {
            let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            // affine 1
            let output1 = matmul(&input, mat);
            let mut output2 = input.to_owned();
            griffin.affine(&mut output2, 1);
            assert_eq!(output1, output2);
        }
    }

    #[test]
    fn affine_3() {
        affine_test(3);
    }

    #[test]
    fn affine_4() {
        affine_test(4);
    }

    #[test]
    fn affine_8() {
        affine_test(8);
    }

    #[test]
    fn affine_60() {
        affine_test(60);
    }
}

#[cfg(test)]
mod griffin_tests_st {
    use ff::Field;

    use super::*;
    use crate::{
        fields::{st::FpST, utils},
        griffin::griffin_instances::GRIFFIN_ST_PARAMS,
    };

    type Scalar = FpST;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let griffin = Griffin::new(&GRIFFIN_ST_PARAMS);
        let t = griffin.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = griffin.permutation(&input1);
            let perm2 = griffin.permutation(&input1);
            let perm3 = griffin.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    fn matmul(input: &[Scalar], mat: &[Vec<Scalar>]) -> Vec<Scalar> {
        let t = mat.len();
        debug_assert!(t == input.len());
        let mut out = vec![Scalar::zero(); t];
        for row in 0..t {
            for (col, inp) in input.iter().enumerate().take(t) {
                let mut tmp = mat[row][col];
                tmp.mul_assign(inp);
                out[row].add_assign(&tmp);
            }
        }
        out
    }

    fn affine_test(t: usize) {
        let griffin_param = Arc::new(GriffinParams::<Scalar>::new(t, 5, 1));
        let griffin = Griffin::<Scalar>::new(&griffin_param);

        let mat = &griffin_param.mat;

        for _ in 0..TESTRUNS {
            let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            // affine 1
            let output1 = matmul(&input, mat);
            let mut output2 = input.to_owned();
            griffin.affine(&mut output2, 1);
            assert_eq!(output1, output2);
        }
    }

    #[test]
    fn affine_3() {
        affine_test(3);
    }

    #[test]
    fn affine_4() {
        affine_test(4);
    }

    #[test]
    fn affine_8() {
        affine_test(8);
    }

    #[test]
    fn affine_60() {
        affine_test(60);
    }
}
