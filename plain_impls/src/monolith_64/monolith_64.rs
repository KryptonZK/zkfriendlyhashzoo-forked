use super::{
    mds_12, mds_8,
    monolith_64_params::{BarsType, Monolith64Params},
};
use crate::{fields::f64::Field64, merkle_tree::merkle_tree_fp_t::MerkleTreeHash};
use ff::PrimeField;
use generic_array::GenericArray;
use std::{convert::TryInto, sync::Arc};

pub struct Monolith64<F: Field64 + PrimeField, const T: usize> {
    pub(crate) params: Arc<Monolith64Params<F, T>>,
}

impl<F: Field64 + PrimeField, const T: usize> Monolith64<F, T> {
    pub fn new(params: &Arc<Monolith64Params<F, T>>) -> Self {
        Monolith64 {
            params: Arc::clone(params),
        }
    }

    #[inline(always)]
    pub fn concrete(&self, state: &mut [F; T], round_constants: &[F; T]) {
        if T == 8 {
            mds_8::mds_multiply_with_rc(
                state.as_mut().try_into().unwrap(),
                round_constants.as_ref().try_into().unwrap(),
            );
        } else if T == 12 {
            mds_12::mds_multiply_with_rc(
                state.as_mut().try_into().unwrap(),
                round_constants.as_ref().try_into().unwrap(),
            );
        } else {
            self.generic_affine_with_rc(state, round_constants);
        }
    }

    #[inline(always)]
    pub fn first_concrete(&self, state: &mut [F; T]) {
        if T == 8 {
            mds_8::mds_multiply(state.as_mut().try_into().unwrap());
        } else if T == 12 {
            mds_12::mds_multiply(state.as_mut().try_into().unwrap());
        } else {
            self.generic_affine(state);
        }
    }

    fn generic_affine_with_rc(&self, state: &mut [F; T], round_constants: &[F; T]) {
        let mut out = [F::zero(); T];
        for (row, (o, rc)) in out.iter_mut().zip(round_constants.iter()).enumerate() {
            *o = rc.to_owned();
            for (col, inp) in state.iter().enumerate() {
                let mut tmp = self.params.mds[row][col];
                tmp.mul_assign(inp);
                o.add_assign(&tmp);
            }
        }
        state.clone_from_slice(&out);
    }

    fn generic_affine(&self, state: &mut [F; T]) {
        let mut out = [F::zero(); T];
        for (row, o) in out.iter_mut().enumerate() {
            for (col, inp) in state.iter().enumerate() {
                let mut tmp = self.params.mds[row][col];
                tmp.mul_assign(inp);
                o.add_assign(&tmp);
            }
        }
        state.clone_from_slice(&out);
    }

    pub fn bricks(state: &mut [F; T]) {
        // y_0 = x_0
        // y_i = x_i + (x_{i-1})^2 for i \in {1, 2, ..., T-1}
        // Feistel rotation
        let mut tmp = state.to_owned();
        for (x_, x) in tmp.iter_mut().zip(state.iter_mut().skip(1)) {
            x_.square();
            x.add_assign(x_);
        }
    }

    #[inline(always)]
    pub fn concrete_u128(&self, state_u128: &mut [u128; T], round_constants: &[F; T]) {
        if T == 8 {
            mds_8::mds_multiply_with_rc_u128(
                state_u128.as_mut().try_into().unwrap(),
                round_constants.as_ref().try_into().unwrap(),
            );
        } else if T == 12 {
            mds_12::mds_multiply_with_rc_u128(
                state_u128.as_mut().try_into().unwrap(),
                round_constants.as_ref().try_into().unwrap(),
            );
        } else {
            self.generic_affine_with_rc_u128(state_u128, round_constants);
        }
    }

    #[inline(always)]
    pub fn first_concrete_u128(&self, state_u128: &mut [u128; T]) {
        if T == 8 {
            mds_8::mds_multiply_u128::<F>(state_u128.as_mut().try_into().unwrap());
        } else if T == 12 {
            mds_12::mds_multiply_u128::<F>(state_u128.as_mut().try_into().unwrap());
        } else {
            self.generic_affine_u128(state_u128);
        }
    }

    fn generic_affine_with_rc_u128(&self, state_u128: &mut [u128; T], round_constants: &[F; T]) {
        // Input might not be reduced
        state_u128.iter_mut().for_each(|s| F::reduce96(s));

        let mut out = [0u128; T];
        for (row, (o, rc)) in out.iter_mut().zip(round_constants.iter()).enumerate() {
            *o = rc.to_u64() as u128;
            for (col, inp) in state_u128.iter().enumerate() {
                let mut tmp = self.params.mds[row][col].to_u64() as u128 * inp;
                F::reduce128(&mut tmp);
                *o += tmp;
            }
        }
        state_u128.clone_from_slice(&out);
        state_u128.iter_mut().for_each(|s| F::reduce96(s));
    }

    fn generic_affine_u128(&self, state_u128: &mut [u128; T]) {
        // Input might not be reduced
        state_u128.iter_mut().for_each(|s| F::reduce96(s));

        let mut out = [0u128; T];
        for (row, o) in out.iter_mut().enumerate() {
            for (col, inp) in state_u128.iter().enumerate() {
                let mut tmp = self.params.mds[row][col].to_u64() as u128 * inp;
                F::reduce128(&mut tmp);
                *o += tmp;
            }
        }
        state_u128.clone_from_slice(&out);
        state_u128.iter_mut().for_each(|s| F::reduce96(s));
    }

    // Result is not reduced!
    pub fn bricks_u128(state_u128: &mut [u128; T]) {
        // Feistel Type-3
        let tmp = state_u128.to_owned();
        for (x_, x) in tmp.iter().zip(state_u128.iter_mut().skip(1)) {
            // Every time at bricks the input is technically a u64, so we tell the compiler
            let mut tmp_square = (x_ & 0xFFFFFFFFFFFFFFFF_u128) * (x_ & 0xFFFFFFFFFFFFFFFF_u128);
            F::reduce128(&mut tmp_square);
            *x = (*x & 0xFFFFFFFFFFFFFFFF_u128) + (tmp_square & 0xFFFFFFFFFFFFFFFF_u128);
        }
    }

    pub fn bar_u64(limb: u64) -> u64 {
        debug_assert!(limb < F::char().as_ref()[0]);
        let limbl1 = ((limb & 0x8080808080808080) >> 7) | ((limb & 0x7F7F7F7F7F7F7F7F) << 1); //left rot by 1
        let limbl2 = ((limb & 0xC0C0C0C0C0C0C0C0) >> 6) | ((limb & 0x3F3F3F3F3F3F3F3F) << 2); //left rot by 2
        let limbl3 = ((limb & 0xE0E0E0E0E0E0E0E0) >> 5) | ((limb & 0x1F1F1F1F1F1F1F1F) << 3); //left rot by 3

        //yi = xi +  (1 + x{i+1}) * x{i+2} * x{i+3}
        let tmp = limb ^ !limbl1 & limbl2 & limbl3;
        ((tmp & 0x8080808080808080) >> 7) | ((tmp & 0x7F7F7F7F7F7F7F7F) << 1) // Final rotate for less S(x) = x
    }

    pub fn bar(el: &mut F) {
        let out = Self::bar_u64(el.to_u64());
        *el = F::from_u64(out);
    }

    // We have a u16 lookup here, which combines two 8 bit lookups
    pub fn bar_u64_lookup(&self, el: &mut u64) {
        debug_assert!(*el < F::char().as_ref()[0]);
        // safe because sbox has correct size
        unsafe {
            let limb1 = *self.params.lookup.get_unchecked(*el as u16 as usize) as u64;
            let limb2 = *self
                .params
                .lookup
                .get_unchecked((*el >> 16) as u16 as usize) as u64;
            let limb3 = *self
                .params
                .lookup
                .get_unchecked((*el >> 32) as u16 as usize) as u64;
            let limb4 = *self
                .params
                .lookup
                .get_unchecked((*el >> 48) as u16 as usize) as u64;

            *el = limb1 | limb2 << 16 | limb3 << 32 | limb4 << 48;
        }
    }

    pub fn bars(state: &mut [F; T]) {
        state
            .iter_mut()
            .take(Monolith64Params::<F, T>::BARS)
            .for_each(|el| Self::bar(el));
    }

    pub fn bars_u128(state_u128: &mut [u128; T]) {
        let mut state = GenericArray::<u64, BarsType>::default();
        for (des, src) in state.iter_mut().zip(state_u128.iter()) {
            *des = *src as u64;
        }

        state.iter_mut().for_each(|el| *el = Self::bar_u64(*el));

        for (des, src) in state_u128.iter_mut().zip(state.iter()) {
            *des = *src as u128;
        }
    }

    pub fn bars_u128_lookup(&self, state_u128: &mut [u128; T]) {
        let mut state = GenericArray::<u64, BarsType>::default();
        for (des, src) in state.iter_mut().zip(state_u128.iter()) {
            *des = *src as u64;
        }

        state.iter_mut().for_each(|el| self.bar_u64_lookup(el));

        for (des, src) in state_u128.iter_mut().zip(state.iter()) {
            *des = *src as u128;
        }
    }

    pub fn permutation_u128(&self, input: &[F; T]) -> [F; T] {
        let mut state_u128 = [0; T];
        for (out, inp) in state_u128.iter_mut().zip(input.iter()) {
            *out = inp.to_u64() as u128;
        }

        debug_assert_eq!(
            self.params.round_constants.len(),
            Monolith64Params::<F, T>::R - 1
        );
        self.first_concrete_u128(&mut state_u128);
        for rc in self.params.round_constants.iter() {
            Self::bars_u128(&mut state_u128);
            Self::bricks_u128(&mut state_u128);
            self.concrete_u128(&mut state_u128, rc);
        }
        // Final round with no round constants (can set to zero and use loop above instead)
        Self::bars_u128(&mut state_u128);
        Self::bricks_u128(&mut state_u128);
        self.first_concrete_u128(&mut state_u128);

        // Convert back
        let mut state_f = [F::zero(); T];
        for (out, inp) in state_f.iter_mut().zip(state_u128.iter()) {
            *out = F::from_u64(*inp as u64);
        }
        state_f
    }

    pub fn permutation_u128_lookup(&self, input: &[F; T]) -> [F; T] {
        let mut state_u128 = [0; T];
        for (out, inp) in state_u128.iter_mut().zip(input.iter()) {
            *out = inp.to_u64() as u128;
        }

        debug_assert_eq!(
            self.params.round_constants.len(),
            Monolith64Params::<F, T>::R - 1
        );
        self.first_concrete_u128(&mut state_u128);
        for rc in self.params.round_constants.iter() {
            self.bars_u128_lookup(&mut state_u128);
            Self::bricks_u128(&mut state_u128);
            self.concrete_u128(&mut state_u128, rc);
        }
        // Final round with no round constants (can set to zero and use loop above instead)
        self.bars_u128_lookup(&mut state_u128);
        Self::bricks_u128(&mut state_u128);
        self.first_concrete_u128(&mut state_u128);

        // Convert back
        let mut state_f = [F::zero(); T];
        for (out, inp) in state_f.iter_mut().zip(state_u128.iter()) {
            *out = F::from_u64(*inp as u64);
        }
        state_f
    }

    pub fn permutation_(&self, input: &[F; T]) -> [F; T] {
        let mut current_state = input.to_owned();

        debug_assert_eq!(
            self.params.round_constants.len(),
            Monolith64Params::<F, T>::R - 1
        );

        self.first_concrete(&mut current_state);
        for rc in self.params.round_constants.iter() {
            Self::bars(&mut current_state);
            Self::bricks(&mut current_state);
            self.concrete(&mut current_state, rc);
        }
        // Final round with no round constants (can set to zero and use loop above instead)
        Self::bars(&mut current_state);
        Self::bricks(&mut current_state);
        self.first_concrete(&mut current_state);
        current_state
    }

    #[inline(always)]
    pub fn permutation(&self, input: &[F; T]) -> [F; T] {
        self.permutation_u128(input)
    }
}

impl<F: Field64 + PrimeField> Monolith64<F, 8> {
    pub fn hash(&self, el1: &[F; 4], el2: &[F; 4]) -> [F; 4] {
        let input: [F; 8] = {
            let mut whole: [F; 8] = [F::zero(); 8];
            let (one, two) = whole.split_at_mut(el1.len());
            one.copy_from_slice(el1);
            two.copy_from_slice(el2);
            whole
        };
        let perm = self.permutation(&input);
        let mut result: [F; 4] = el1.to_owned();
        for (r, el) in result.iter_mut().zip(perm.iter()) {
            r.add_assign(el);
        }
        result
    }
}

impl<F: Field64 + PrimeField> MerkleTreeHash<F, 4> for Monolith64<F, 8> {
    fn compress(&self, input1: &[F; 4], input2: &[F; 4]) -> [F; 4] {
        self.hash(input1, input2)
    }
}

#[cfg(test)]
mod monolith_64_tests {
    use super::*;
    use crate::{
        fields::{f64::F64, utils},
        monolith_64::monolith_64_instances::{MONOLITH_64_12_PARAMS, MONOLITH_64_8_PARAMS},
    };
    use ff::Field;

    static TESTRUNS: usize = 5;
    type Scalar = F64;

    #[test]
    fn consistent_perm() {
        let monolith = Monolith64::new(&MONOLITH_64_8_PARAMS);

        for _ in 0..TESTRUNS {
            let input1: [Scalar; 8] = [
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
            ];

            let mut input2: [Scalar; 8];
            loop {
                input2 = [
                    utils::random_scalar(true),
                    utils::random_scalar(true),
                    utils::random_scalar(true),
                    utils::random_scalar(true),
                    utils::random_scalar(true),
                    utils::random_scalar(true),
                    utils::random_scalar(true),
                    utils::random_scalar(true),
                ];
                if input1 != input2 {
                    break;
                }
            }

            let perm_1 = monolith.permutation_u128(&input1);
            let perm_2 = monolith.permutation_u128(&input1);
            let perm_3 = monolith.permutation_u128(&input2);
            assert_eq!(perm_1, perm_2);
            assert_ne!(perm_1, perm_3);
        }
    }

    #[test]
    fn equal_perms() {
        let monolith = Monolith64::new(&MONOLITH_64_8_PARAMS);

        for _ in 0..TESTRUNS {
            let input: [Scalar; 8] = [
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
            ];

            let perm1 = monolith.permutation_(&input);
            let perm2 = monolith.permutation_u128(&input);
            let perm3 = monolith.permutation_u128_lookup(&input);

            assert_eq!(perm1, perm2);
            assert_eq!(perm1, perm3);
        }
    }

    #[test]
    fn kats_8() {
        let monolith = Monolith64::new(&MONOLITH_64_8_PARAMS);

        let mut input: [Scalar; 8] = [Scalar::zero(); 8];
        for (i, inp) in input.iter_mut().enumerate() {
            *inp = utils::from_u64(i as u64);
        }
        let perm1 = monolith.permutation(&input);
        assert_eq!(perm1[0], utils::from_u64::<Scalar>(3656442354255169651));
        assert_eq!(perm1[1], utils::from_u64::<Scalar>(1088199316401146975));
        assert_eq!(perm1[2], utils::from_u64::<Scalar>(22941152274975507));
        assert_eq!(perm1[3], utils::from_u64::<Scalar>(14434181924633355796));
        assert_eq!(perm1[4], utils::from_u64::<Scalar>(6981961052218049719));
        assert_eq!(perm1[5], utils::from_u64::<Scalar>(16492720827407246378));
        assert_eq!(perm1[6], utils::from_u64::<Scalar>(17986182688944525029));
        assert_eq!(perm1[7], utils::from_u64::<Scalar>(9161400698613172623));
    }

    #[test]
    fn kats_12() {
        let monolith = Monolith64::new(&MONOLITH_64_12_PARAMS);

        let mut input: [Scalar; 12] = [Scalar::zero(); 12];
        for (i, inp) in input.iter_mut().enumerate() {
            *inp = utils::from_u64(i as u64);
        }
        let perm1 = monolith.permutation(&input);
        assert_eq!(perm1[0], utils::from_u64::<Scalar>(5867581605548782913));
        assert_eq!(perm1[1], utils::from_u64::<Scalar>(588867029099903233));
        assert_eq!(perm1[2], utils::from_u64::<Scalar>(6043817495575026667));
        assert_eq!(perm1[3], utils::from_u64::<Scalar>(805786589926590032));
        assert_eq!(perm1[4], utils::from_u64::<Scalar>(9919982299747097782));
        assert_eq!(perm1[5], utils::from_u64::<Scalar>(6718641691835914685));
        assert_eq!(perm1[6], utils::from_u64::<Scalar>(7951881005429661950));
        assert_eq!(perm1[7], utils::from_u64::<Scalar>(15453177927755089358));
        assert_eq!(perm1[8], utils::from_u64::<Scalar>(974633365445157727));
        assert_eq!(perm1[9], utils::from_u64::<Scalar>(9654662171963364206));
        assert_eq!(perm1[10], utils::from_u64::<Scalar>(6281307445101925412));
        assert_eq!(perm1[11], utils::from_u64::<Scalar>(13745376999934453119));
    }

    fn matmul<const T: usize>(input: &[Scalar; T], mat: &[[Scalar; T]; T]) -> Vec<Scalar> {
        let mut out = vec![Scalar::zero(); T];
        for row in 0..T {
            for (col, inp) in input.iter().enumerate() {
                let mut tmp = mat[row][col];
                tmp.mul_assign(inp);
                out[row].add_assign(&tmp);
            }
        }
        out
    }

    #[test]
    fn affine_test_8() {
        let monolith = Monolith64::new(&MONOLITH_64_8_PARAMS);
        let mat = &monolith.params.mds;
        let round_const = [Scalar::zero(); 8];

        for _ in 0..TESTRUNS {
            let input: [Scalar; 8] = [
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
            ];

            let output1 = matmul(&input, mat);
            let mut output2 = input.to_owned();
            monolith.concrete(&mut output2, &round_const);
            assert_eq!(output1, output2);

            let mut output_u128 = [0u128; 8];
            output_u128
                .iter_mut()
                .zip(input.iter())
                .for_each(|(des, src)| *des = src.to_u64() as u128);
            monolith.concrete_u128(&mut output_u128, &round_const);
            for (res1, res2) in output1.iter().zip(output_u128.iter()) {
                assert_eq!(res1.to_u64(), *res2 as u64);
            }
        }
    }

    #[test]
    fn affine_test_12() {
        let monolith = Monolith64::new(&MONOLITH_64_12_PARAMS);
        let mat = &monolith.params.mds;
        let round_const = [Scalar::zero(); 12];

        for _ in 0..TESTRUNS {
            let input: [Scalar; 12] = [
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
            ];

            let output1 = matmul(&input, mat);
            let mut output2 = input.to_owned();
            monolith.concrete(&mut output2, &round_const);
            assert_eq!(output1, output2);

            let mut output_u128 = [0u128; 12];
            output_u128
                .iter_mut()
                .zip(input.iter())
                .for_each(|(des, src)| *des = src.to_u64() as u128);
            monolith.concrete_u128(&mut output_u128, &round_const);
            for (res1, res2) in output1.iter().zip(output_u128.iter()) {
                assert_eq!(res1.to_u64(), *res2 as u64);
            }
        }
    }
}

#[cfg(test)]
mod monolith_64_constant_time_tests {
    use super::*;
    use crate::{
        fields::{const_f64::ConstF64, utils},
        monolith_64::monolith_64_instances::{
            MONOLITH_CONST64_12_PARAMS, MONOLITH_CONST64_8_PARAMS,
        },
    };
    use ff::Field;

    static TESTRUNS: usize = 5;
    type Scalar = ConstF64;

    #[test]
    fn consistent_perm() {
        let monolith = Monolith64::new(&MONOLITH_CONST64_8_PARAMS);

        for _ in 0..TESTRUNS {
            let input1: [Scalar; 8] = [
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
            ];

            let mut input2: [Scalar; 8];
            loop {
                input2 = [
                    utils::random_scalar(true),
                    utils::random_scalar(true),
                    utils::random_scalar(true),
                    utils::random_scalar(true),
                    utils::random_scalar(true),
                    utils::random_scalar(true),
                    utils::random_scalar(true),
                    utils::random_scalar(true),
                ];
                if input1 != input2 {
                    break;
                }
            }

            let perm_1 = monolith.permutation_u128(&input1);
            let perm_2 = monolith.permutation_u128(&input1);
            let perm_3 = monolith.permutation_u128(&input2);
            assert_eq!(perm_1, perm_2);
            assert_ne!(perm_1, perm_3);
        }
    }

    #[test]
    fn equal_perms() {
        let monolith = Monolith64::new(&MONOLITH_CONST64_8_PARAMS);

        for _ in 0..TESTRUNS {
            let input: [Scalar; 8] = [
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
            ];

            let perm1 = monolith.permutation(&input);
            let perm2 = monolith.permutation_u128(&input);
            let perm3 = monolith.permutation_u128_lookup(&input);

            assert_eq!(perm1, perm2);
            assert_eq!(perm1, perm3);
        }
    }

    #[test]
    fn kats_8() {
        let monolith = Monolith64::new(&MONOLITH_CONST64_8_PARAMS);

        let mut input: [Scalar; 8] = [Scalar::zero(); 8];
        for (i, inp) in input.iter_mut().enumerate() {
            *inp = utils::from_u64(i as u64);
        }
        let perm1 = monolith.permutation(&input);
        assert_eq!(perm1[0], utils::from_u64::<Scalar>(3656442354255169651));
        assert_eq!(perm1[1], utils::from_u64::<Scalar>(1088199316401146975));
        assert_eq!(perm1[2], utils::from_u64::<Scalar>(22941152274975507));
        assert_eq!(perm1[3], utils::from_u64::<Scalar>(14434181924633355796));
        assert_eq!(perm1[4], utils::from_u64::<Scalar>(6981961052218049719));
        assert_eq!(perm1[5], utils::from_u64::<Scalar>(16492720827407246378));
        assert_eq!(perm1[6], utils::from_u64::<Scalar>(17986182688944525029));
        assert_eq!(perm1[7], utils::from_u64::<Scalar>(9161400698613172623));
    }

    #[test]
    fn kats_12() {
        let monolith = Monolith64::new(&MONOLITH_CONST64_12_PARAMS);

        let mut input: [Scalar; 12] = [Scalar::zero(); 12];
        for (i, inp) in input.iter_mut().enumerate() {
            *inp = utils::from_u64(i as u64);
        }
        let perm1 = monolith.permutation(&input);
        assert_eq!(perm1[0], utils::from_u64::<Scalar>(5867581605548782913));
        assert_eq!(perm1[1], utils::from_u64::<Scalar>(588867029099903233));
        assert_eq!(perm1[2], utils::from_u64::<Scalar>(6043817495575026667));
        assert_eq!(perm1[3], utils::from_u64::<Scalar>(805786589926590032));
        assert_eq!(perm1[4], utils::from_u64::<Scalar>(9919982299747097782));
        assert_eq!(perm1[5], utils::from_u64::<Scalar>(6718641691835914685));
        assert_eq!(perm1[6], utils::from_u64::<Scalar>(7951881005429661950));
        assert_eq!(perm1[7], utils::from_u64::<Scalar>(15453177927755089358));
        assert_eq!(perm1[8], utils::from_u64::<Scalar>(974633365445157727));
        assert_eq!(perm1[9], utils::from_u64::<Scalar>(9654662171963364206));
        assert_eq!(perm1[10], utils::from_u64::<Scalar>(6281307445101925412));
        assert_eq!(perm1[11], utils::from_u64::<Scalar>(13745376999934453119));
    }

    fn matmul<const T: usize>(input: &[Scalar; T], mat: &[[Scalar; T]; T]) -> Vec<Scalar> {
        let mut out = vec![Scalar::zero(); T];
        for row in 0..T {
            for (col, inp) in input.iter().enumerate() {
                let mut tmp = mat[row][col];
                tmp.mul_assign(inp);
                out[row].add_assign(&tmp);
            }
        }
        out
    }

    #[test]
    fn affine_test_8() {
        let monolith = Monolith64::new(&MONOLITH_CONST64_8_PARAMS);
        let mat = &monolith.params.mds;
        let round_const = [Scalar::zero(); 8];

        for _ in 0..TESTRUNS {
            let input: [Scalar; 8] = [
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
            ];

            let output1 = matmul(&input, mat);
            let mut output2 = input.to_owned();
            monolith.concrete(&mut output2, &round_const);
            assert_eq!(output1, output2);

            let mut output_u128 = [0u128; 8];
            output_u128
                .iter_mut()
                .zip(input.iter())
                .for_each(|(des, src)| *des = src.to_u64() as u128);
            monolith.concrete_u128(&mut output_u128, &round_const);
            for (res1, res2) in output1.iter().zip(output_u128.iter()) {
                assert_eq!(res1.to_u64(), *res2 as u64);
            }
        }
    }

    #[test]
    fn affine_test_12() {
        let monolith = Monolith64::new(&MONOLITH_CONST64_12_PARAMS);
        let mat = &monolith.params.mds;
        let round_const = [Scalar::zero(); 12];

        for _ in 0..TESTRUNS {
            let input: [Scalar; 12] = [
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
            ];

            let output1 = matmul(&input, mat);
            let mut output2 = input.to_owned();
            monolith.concrete(&mut output2, &round_const);
            assert_eq!(output1, output2);

            let mut output_u128 = [0u128; 12];
            output_u128
                .iter_mut()
                .zip(input.iter())
                .for_each(|(des, src)| *des = src.to_u64() as u128);
            monolith.concrete_u128(&mut output_u128, &round_const);
            for (res1, res2) in output1.iter().zip(output_u128.iter()) {
                assert_eq!(res1.to_u64(), *res2 as u64);
            }
        }
    }
}
