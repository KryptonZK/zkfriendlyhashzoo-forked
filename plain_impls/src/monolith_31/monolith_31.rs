use super::{
    mds_16, mds_24,
    monolith_31_params::{BarsType, Monolith31Params},
};
use crate::{fields::f31::Field32, merkle_tree::merkle_tree_fp_t::MerkleTreeHash};
use ff::PrimeField;
use generic_array::GenericArray;
use std::{convert::TryInto, sync::Arc};

pub struct Monolith31<F: Field32 + PrimeField, const T: usize> {
    pub(crate) params: Arc<Monolith31Params<F, T>>,
}

impl<F: Field32 + PrimeField, const T: usize> Monolith31<F, T> {
    pub fn new(params: &Arc<Monolith31Params<F, T>>) -> Self {
        Monolith31 {
            params: Arc::clone(params),
        }
    }

    #[inline(always)]
    pub fn concrete_u64(&self, state_u64: &mut [u64; T], round_constants: &[F; T]) {
        if T == 16 {
            mds_16::mds_multiply_with_rc_u64::<F>(
                state_u64.as_mut().try_into().unwrap(),
                round_constants.as_ref().try_into().unwrap(),
            );
        } else if T == 24 {
            mds_24::mds_multiply_with_rc_u64::<F>(
                state_u64.as_mut().try_into().unwrap(),
                round_constants.as_ref().try_into().unwrap(),
            );
        } else {
            self.generic_affine_with_rc_u64(state_u64, round_constants);
        }
    }

    #[inline(always)]
    pub fn first_concrete_u64(&self, state_u64: &mut [u64; T]) {
        if T == 16 {
            mds_16::mds_multiply_u64::<F>(state_u64.as_mut().try_into().unwrap());
        } else if T == 24 {
            mds_24::mds_multiply_u64::<F>(state_u64.as_mut().try_into().unwrap());
        } else {
            self.generic_affine_u64(state_u64);
        }
    }

    fn generic_affine_u64(&self, state_u64: &mut [u64; T]) {
        let mut out = [0u64; T];
        for (row, o) in out.iter_mut().enumerate() {
            for (col, inp) in state_u64.iter().enumerate() {
                debug_assert!(*inp < (1u64 << 32)); // Input might not be reduced, but is still 32 bit
                let mut tmp = self.params.mds[row][col].to_u32() as u64 * inp;
                F::reduce64(&mut tmp);
                *o += tmp;
            }
        }
        state_u64.clone_from_slice(&out);
        state_u64.iter_mut().for_each(|s| F::reduce64(s));

        // Check if output is reduced since input might not be reduced
        #[cfg(debug_assertions)]
        {
            for s in state_u64 {
                debug_assert!(*s < F::char().as_ref()[0]);
            }
        }
    }

    fn generic_affine_with_rc_u64(&self, state_u64: &mut [u64; T], round_constants: &[F; T]) {
        let mut out = [0u64; T];
        for (row, (o, rc)) in out.iter_mut().zip(round_constants.iter()).enumerate() {
            *o = rc.to_u32() as u64;
            for (col, inp) in state_u64.iter().enumerate() {
                debug_assert!(*inp < (1u64 << 32)); // Input might not be reduced, but is still 32 bit
                let mut tmp = self.params.mds[row][col].to_u32() as u64 * inp;
                F::reduce64(&mut tmp);
                *o += tmp;
            }
        }
        state_u64.clone_from_slice(&out);
        state_u64.iter_mut().for_each(|s| F::reduce64(s));

        // Check if output is reduced since input might not be reduced
        #[cfg(debug_assertions)]
        {
            for s in state_u64 {
                debug_assert!(*s < F::char().as_ref()[0]);
            }
        }
    }

    // Result is not reduced!
    pub fn bricks_u64(state_u64: &mut [u64; T]) {
        // Feistel Type-3
        let tmp = state_u64.to_owned();
        for (x_, x) in tmp.iter().zip(state_u64.iter_mut().skip(1)) {
            // Every time at bricks the input is technically a u32, so we tell the compiler
            let mut tmp_square = (x_ & 0xFFFFFFFF_u64) * (x_ & 0xFFFFFFFF_u64);
            F::reduce64(&mut tmp_square);
            *x = (*x & 0xFFFFFFFF_u64) + (tmp_square & 0xFFFFFFFF_u64);
        }
    }

    pub(super) fn bar0_24(limb: u32) -> u32 {
        debug_assert_eq!(limb >> 24, 0); // limb must be reduced
        let limbl1 = ((limb & 0x808080) >> 7) | ((limb & 0x7F7F7F) << 1); //left rot by 1
        let limbl2 = ((limb & 0xC0C0C0) >> 6) | ((limb & 0x3F3F3F) << 2); //left rot by 2
        let limbl3 = ((limb & 0xE0E0E0) >> 5) | ((limb & 0x1F1F1F) << 3); //left rot by 3

        //yi = xi +  (1 + x{i+1}) * x{i+2} * x{i+3}
        let tmp = limb ^ !limbl1 & limbl2 & limbl3;
        ((tmp & 0x808080) >> 7) | ((tmp & 0x7F7F7F) << 1) // Final rotate for less S(x) = x
    }

    // Works over 7 bits!
    pub(super) fn bar1_7(limb: u8) -> u8 {
        debug_assert_eq!(limb >> 7, 0); // limb must be reduced

        let limbl1 = (limb >> 6) | (limb << 1); // 7 bit left rot by 1
        let limbl2 = (limb >> 5) | (limb << 2); // 7 bit left rot by 2

        //yi = xi +  (1 + x{i+1}) * x{i+2}
        let tmp = (limb ^ !limbl1 & limbl2) & 0x7F;
        ((tmp >> 6) | (tmp << 1)) & 0x7F // Final rotate for less S(x) = x
    }

    pub fn bar_u32(el: &mut u32) {
        debug_assert!(*el < F::char().as_ref()[0].to_owned() as u32);
        let low = Self::bar0_24(*el & 0xFFFFFF);
        let high = Self::bar1_7((*el >> 24) as u8) as u32;
        *el = low | (high << 24);
    }

    // We have a u16 lookup here, which combines two 8 bit lookups each
    pub fn bar_u32_lookup(&self, el: &mut u32) {
        debug_assert!(*el < F::char().as_ref()[0].to_owned() as u32);
        debug_assert_eq!(*el >> 31, 0); // limb must be reduced

        // safe because sbox has correct size
        unsafe {
            let low = *self.params.lookup1.get_unchecked(*el as u16 as usize) as u32;
            let high = *self
                .params
                .lookup2
                .get_unchecked((*el >> 16) as u16 as usize) as u32;
            *el = low | (high << 16);
        }
    }

    pub fn bars_u64(state_u64: &mut [u64; T]) {
        let mut state = GenericArray::<u32, BarsType>::default();
        for (des, src) in state.iter_mut().zip(state_u64.iter()) {
            *des = *src as u32;
        }

        state.iter_mut().for_each(|el| Self::bar_u32(el));

        for (des, src) in state_u64.iter_mut().zip(state.iter()) {
            *des = *src as u64;
        }
    }

    pub fn bars_u64_lookup(&self, state_u64: &mut [u64; T]) {
        state_u64
            .iter_mut()
            .take(Monolith31Params::<F, T>::BARS)
            .for_each(|el| {
                let mut tmp = *el as u32;
                self.bar_u32_lookup(&mut tmp);
                *el = tmp as u64
            });
    }

    pub fn permutation_u64(&self, input: &[F; T]) -> [F; T] {
        let mut state_u64 = [0; T];
        for (out, inp) in state_u64.iter_mut().zip(input.iter()) {
            *out = inp.to_u32() as u64;
        }

        debug_assert_eq!(
            self.params.round_constants.len(),
            Monolith31Params::<F, T>::R - 1
        );
        self.first_concrete_u64(&mut state_u64);
        for rc in self.params.round_constants.iter() {
            Self::bars_u64(&mut state_u64);
            Self::bricks_u64(&mut state_u64);
            self.concrete_u64(&mut state_u64, rc);
        }
        // Final round with no round constants (can set to zero and use loop above instead)
        Self::bars_u64(&mut state_u64);
        Self::bricks_u64(&mut state_u64);
        self.first_concrete_u64(&mut state_u64);

        // Convert back
        let mut state_f = [F::zero(); T];
        for (out, inp) in state_f.iter_mut().zip(state_u64.iter()) {
            *out = F::from_u32(*inp as u32);
        }
        state_f
    }

    pub fn permutation_u64_lookup(&self, input: &[F; T]) -> [F; T] {
        let mut state_u64 = [0; T];
        for (out, inp) in state_u64.iter_mut().zip(input.iter()) {
            *out = inp.to_u32() as u64;
        }

        debug_assert_eq!(
            self.params.round_constants.len(),
            Monolith31Params::<F, T>::R - 1
        );
        self.first_concrete_u64(&mut state_u64);
        for rc in self.params.round_constants.iter() {
            self.bars_u64_lookup(&mut state_u64);
            Self::bricks_u64(&mut state_u64);
            self.concrete_u64(&mut state_u64, rc);
        }
        // Final round with no round constants (can set to zero and use loop above instead)
        self.bars_u64_lookup(&mut state_u64);
        Self::bricks_u64(&mut state_u64);
        self.first_concrete_u64(&mut state_u64);

        // Convert back
        let mut state_f = [F::zero(); T];
        for (out, inp) in state_f.iter_mut().zip(state_u64.iter()) {
            *out = F::from_u32(*inp as u32);
        }
        state_f
    }

    #[inline(always)]
    pub fn permutation(&self, input: &[F; T]) -> [F; T] {
        self.permutation_u64(input)
    }
}

impl<F: Field32 + PrimeField> Monolith31<F, 16> {
    pub fn hash(&self, el1: &[F; 8], el2: &[F; 8]) -> [F; 8] {
        let input: [F; 16] = {
            let mut whole: [F; 16] = [F::zero(); 16];
            let (one, two) = whole.split_at_mut(el1.len());
            one.copy_from_slice(el1);
            two.copy_from_slice(el2);
            whole
        };
        let perm = self.permutation(&input);
        let mut result: [F; 8] = el1.to_owned();
        for (r, el) in result.iter_mut().zip(perm.iter()) {
            r.add_assign(el);
        }
        result
    }
}

impl<F: Field32 + PrimeField> MerkleTreeHash<F, 8> for Monolith31<F, 16> {
    fn compress(&self, input1: &[F; 8], input2: &[F; 8]) -> [F; 8] {
        self.hash(input1, input2)
    }
}

#[cfg(test)]
mod monolith_31_tests {
    use super::*;
    use crate::{
        fields::{f31::F31, utils},
        monolith_31::monolith_31_instances::{MONOLITH_31_16_PARAMS, MONOLITH_31_24_PARAMS},
    };
    use ff::Field;

    static TESTRUNS: usize = 5;
    type Scalar = F31;

    #[test]
    fn consistent_perm() {
        let monolith = Monolith31::new(&MONOLITH_31_16_PARAMS);

        for _ in 0..TESTRUNS {
            let input1: [Scalar; 16] = [
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
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
            ];

            let mut input2: [Scalar; 16];
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

            let perm_1 = monolith.permutation_u64(&input1);
            let perm_2 = monolith.permutation_u64(&input1);
            let perm_3 = monolith.permutation_u64(&input2);
            assert_eq!(perm_1, perm_2);
            assert_ne!(perm_1, perm_3);
        }
    }

    #[test]
    fn equal_perms() {
        let monolith = Monolith31::new(&MONOLITH_31_16_PARAMS);

        for _ in 0..TESTRUNS {
            let input: [Scalar; 16] = [
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
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
            ];

            let perm1 = monolith.permutation_u64(&input);
            let perm2 = monolith.permutation_u64_lookup(&input);

            assert_eq!(perm1, perm2);
        }
    }

    #[test]
    fn kats_16() {
        let monolith = Monolith31::new(&MONOLITH_31_16_PARAMS);

        let mut input: [Scalar; 16] = [Scalar::zero(); 16];
        for (i, inp) in input.iter_mut().enumerate() {
            *inp = utils::from_u64(i as u64);
        }
        let perm1 = monolith.permutation(&input);
        assert_eq!(perm1[0], utils::from_u64::<Scalar>(609156607));
        assert_eq!(perm1[1], utils::from_u64::<Scalar>(290107110));
        assert_eq!(perm1[2], utils::from_u64::<Scalar>(1900746598));
        assert_eq!(perm1[3], utils::from_u64::<Scalar>(1734707571));
        assert_eq!(perm1[4], utils::from_u64::<Scalar>(2050994835));
        assert_eq!(perm1[5], utils::from_u64::<Scalar>(1648553244));
        assert_eq!(perm1[6], utils::from_u64::<Scalar>(1307647296));
        assert_eq!(perm1[7], utils::from_u64::<Scalar>(1941164548));
        assert_eq!(perm1[8], utils::from_u64::<Scalar>(1707113065));
        assert_eq!(perm1[9], utils::from_u64::<Scalar>(1477714255));
        assert_eq!(perm1[10], utils::from_u64::<Scalar>(1170160793));
        assert_eq!(perm1[11], utils::from_u64::<Scalar>(93800695));
        assert_eq!(perm1[12], utils::from_u64::<Scalar>(769879348));
        assert_eq!(perm1[13], utils::from_u64::<Scalar>(375548503));
        assert_eq!(perm1[14], utils::from_u64::<Scalar>(1989726444));
        assert_eq!(perm1[15], utils::from_u64::<Scalar>(1349325635));
    }

    #[test]
    fn kats_24() {
        let monolith = Monolith31::new(&MONOLITH_31_24_PARAMS);

        let mut input: [Scalar; 24] = [Scalar::zero(); 24];
        for (i, inp) in input.iter_mut().enumerate() {
            *inp = utils::from_u64(i as u64);
        }
        let perm1 = monolith.permutation(&input);
        assert_eq!(perm1[0], utils::from_u64::<Scalar>(2067773075));
        assert_eq!(perm1[1], utils::from_u64::<Scalar>(1832201932));
        assert_eq!(perm1[2], utils::from_u64::<Scalar>(1944824478));
        assert_eq!(perm1[3], utils::from_u64::<Scalar>(1823377759));
        assert_eq!(perm1[4], utils::from_u64::<Scalar>(1441396277));
        assert_eq!(perm1[5], utils::from_u64::<Scalar>(2131077448));
        assert_eq!(perm1[6], utils::from_u64::<Scalar>(2132180368));
        assert_eq!(perm1[7], utils::from_u64::<Scalar>(1432941899));
        assert_eq!(perm1[8], utils::from_u64::<Scalar>(1347592327));
        assert_eq!(perm1[9], utils::from_u64::<Scalar>(1652902071));
        assert_eq!(perm1[10], utils::from_u64::<Scalar>(1809291778));
        assert_eq!(perm1[11], utils::from_u64::<Scalar>(1684517779));
        assert_eq!(perm1[12], utils::from_u64::<Scalar>(785982444));
        assert_eq!(perm1[13], utils::from_u64::<Scalar>(1037200378));
        assert_eq!(perm1[14], utils::from_u64::<Scalar>(1316286130));
        assert_eq!(perm1[15], utils::from_u64::<Scalar>(1391154514));
        assert_eq!(perm1[16], utils::from_u64::<Scalar>(1760346031));
        assert_eq!(perm1[17], utils::from_u64::<Scalar>(1412575993));
        assert_eq!(perm1[18], utils::from_u64::<Scalar>(2108791223));
        assert_eq!(perm1[19], utils::from_u64::<Scalar>(1657735769));
        assert_eq!(perm1[20], utils::from_u64::<Scalar>(219740691));
        assert_eq!(perm1[21], utils::from_u64::<Scalar>(1165267731));
        assert_eq!(perm1[22], utils::from_u64::<Scalar>(505815021));
        assert_eq!(perm1[23], utils::from_u64::<Scalar>(2080295871));
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
    fn affine_test_16() {
        let monolith = Monolith31::new(&MONOLITH_31_16_PARAMS);
        let mat = &monolith.params.mds;
        let round_const = [Scalar::zero(); 16];

        for _ in 0..TESTRUNS {
            let input: [Scalar; 16] = [
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
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
            ];

            let output1 = matmul(&input, mat);

            let mut output_u64 = [0u64; 16];
            output_u64
                .iter_mut()
                .zip(input.iter())
                .for_each(|(des, src)| *des = src.to_u32() as u64);
            monolith.concrete_u64(&mut output_u64, &round_const);
            for (res1, res2) in output1.iter().zip(output_u64.iter()) {
                assert_eq!(res1.to_u32(), *res2 as u32);
            }
        }
    }

    #[test]
    fn affine_test_24() {
        let monolith = Monolith31::new(&MONOLITH_31_24_PARAMS);
        let mat = &monolith.params.mds;
        let round_const = [Scalar::zero(); 24];

        for _ in 0..TESTRUNS {
            let input: [Scalar; 24] = [
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

            let mut output_u64 = [0u64; 24];
            output_u64
                .iter_mut()
                .zip(input.iter())
                .for_each(|(des, src)| *des = src.to_u32() as u64);
            monolith.concrete_u64(&mut output_u64, &round_const);
            for (res1, res2) in output1.iter().zip(output_u64.iter()) {
                assert_eq!(res1.to_u32(), *res2 as u32);
            }
        }
    }
}

#[cfg(test)]
mod monolith_31_constant_time_tests {
    use super::*;
    use crate::{
        fields::{const_f31::ConstF31, utils},
        monolith_31::monolith_31_instances::{
            MONOLITH_CONST31_16_PARAMS, MONOLITH_CONST31_24_PARAMS,
        },
    };
    use ff::Field;

    static TESTRUNS: usize = 5;
    type Scalar = ConstF31;

    #[test]
    fn consistent_perm() {
        let monolith = Monolith31::new(&MONOLITH_CONST31_16_PARAMS);

        for _ in 0..TESTRUNS {
            let input1: [Scalar; 16] = [
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
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
            ];

            let mut input2: [Scalar; 16];
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

            let perm_1 = monolith.permutation_u64(&input1);
            let perm_2 = monolith.permutation_u64(&input1);
            let perm_3 = monolith.permutation_u64(&input2);
            assert_eq!(perm_1, perm_2);
            assert_ne!(perm_1, perm_3);
        }
    }

    #[test]
    fn equal_perms() {
        let monolith = Monolith31::new(&MONOLITH_CONST31_16_PARAMS);

        for _ in 0..TESTRUNS {
            let input: [Scalar; 16] = [
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
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
            ];

            let perm1 = monolith.permutation_u64(&input);
            let perm2 = monolith.permutation_u64_lookup(&input);

            assert_eq!(perm1, perm2);
        }
    }

    #[test]
    fn kats_16() {
        let monolith = Monolith31::new(&MONOLITH_CONST31_16_PARAMS);

        let mut input: [Scalar; 16] = [Scalar::zero(); 16];
        for (i, inp) in input.iter_mut().enumerate() {
            *inp = utils::from_u64(i as u64);
        }
        let perm1 = monolith.permutation(&input);
        assert_eq!(perm1[0], utils::from_u64::<Scalar>(609156607));
        assert_eq!(perm1[1], utils::from_u64::<Scalar>(290107110));
        assert_eq!(perm1[2], utils::from_u64::<Scalar>(1900746598));
        assert_eq!(perm1[3], utils::from_u64::<Scalar>(1734707571));
        assert_eq!(perm1[4], utils::from_u64::<Scalar>(2050994835));
        assert_eq!(perm1[5], utils::from_u64::<Scalar>(1648553244));
        assert_eq!(perm1[6], utils::from_u64::<Scalar>(1307647296));
        assert_eq!(perm1[7], utils::from_u64::<Scalar>(1941164548));
        assert_eq!(perm1[8], utils::from_u64::<Scalar>(1707113065));
        assert_eq!(perm1[9], utils::from_u64::<Scalar>(1477714255));
        assert_eq!(perm1[10], utils::from_u64::<Scalar>(1170160793));
        assert_eq!(perm1[11], utils::from_u64::<Scalar>(93800695));
        assert_eq!(perm1[12], utils::from_u64::<Scalar>(769879348));
        assert_eq!(perm1[13], utils::from_u64::<Scalar>(375548503));
        assert_eq!(perm1[14], utils::from_u64::<Scalar>(1989726444));
        assert_eq!(perm1[15], utils::from_u64::<Scalar>(1349325635));
    }

    #[test]
    fn kats_24() {
        let monolith = Monolith31::new(&MONOLITH_CONST31_24_PARAMS);

        let mut input: [Scalar; 24] = [Scalar::zero(); 24];
        for (i, inp) in input.iter_mut().enumerate() {
            *inp = utils::from_u64(i as u64);
        }
        let perm1 = monolith.permutation(&input);
        assert_eq!(perm1[0], utils::from_u64::<Scalar>(2067773075));
        assert_eq!(perm1[1], utils::from_u64::<Scalar>(1832201932));
        assert_eq!(perm1[2], utils::from_u64::<Scalar>(1944824478));
        assert_eq!(perm1[3], utils::from_u64::<Scalar>(1823377759));
        assert_eq!(perm1[4], utils::from_u64::<Scalar>(1441396277));
        assert_eq!(perm1[5], utils::from_u64::<Scalar>(2131077448));
        assert_eq!(perm1[6], utils::from_u64::<Scalar>(2132180368));
        assert_eq!(perm1[7], utils::from_u64::<Scalar>(1432941899));
        assert_eq!(perm1[8], utils::from_u64::<Scalar>(1347592327));
        assert_eq!(perm1[9], utils::from_u64::<Scalar>(1652902071));
        assert_eq!(perm1[10], utils::from_u64::<Scalar>(1809291778));
        assert_eq!(perm1[11], utils::from_u64::<Scalar>(1684517779));
        assert_eq!(perm1[12], utils::from_u64::<Scalar>(785982444));
        assert_eq!(perm1[13], utils::from_u64::<Scalar>(1037200378));
        assert_eq!(perm1[14], utils::from_u64::<Scalar>(1316286130));
        assert_eq!(perm1[15], utils::from_u64::<Scalar>(1391154514));
        assert_eq!(perm1[16], utils::from_u64::<Scalar>(1760346031));
        assert_eq!(perm1[17], utils::from_u64::<Scalar>(1412575993));
        assert_eq!(perm1[18], utils::from_u64::<Scalar>(2108791223));
        assert_eq!(perm1[19], utils::from_u64::<Scalar>(1657735769));
        assert_eq!(perm1[20], utils::from_u64::<Scalar>(219740691));
        assert_eq!(perm1[21], utils::from_u64::<Scalar>(1165267731));
        assert_eq!(perm1[22], utils::from_u64::<Scalar>(505815021));
        assert_eq!(perm1[23], utils::from_u64::<Scalar>(2080295871));
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
    fn affine_test_16() {
        let monolith = Monolith31::new(&MONOLITH_CONST31_16_PARAMS);
        let mat = &monolith.params.mds;
        let round_const = [Scalar::zero(); 16];

        for _ in 0..TESTRUNS {
            let input: [Scalar; 16] = [
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
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
            ];

            let output1 = matmul(&input, mat);

            let mut output_u64 = [0u64; 16];
            output_u64
                .iter_mut()
                .zip(input.iter())
                .for_each(|(des, src)| *des = src.to_u32() as u64);
            monolith.concrete_u64(&mut output_u64, &round_const);
            for (res1, res2) in output1.iter().zip(output_u64.iter()) {
                assert_eq!(res1.to_u32(), *res2 as u32);
            }
        }
    }

    #[test]
    fn affine_test_24() {
        let monolith = Monolith31::new(&MONOLITH_CONST31_24_PARAMS);
        let mat = &monolith.params.mds;
        let round_const = [Scalar::zero(); 24];

        for _ in 0..TESTRUNS {
            let input: [Scalar; 24] = [
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

            let mut output_u64 = [0u64; 24];
            output_u64
                .iter_mut()
                .zip(input.iter())
                .for_each(|(des, src)| *des = src.to_u32() as u64);
            monolith.concrete_u64(&mut output_u64, &round_const);
            for (res1, res2) in output1.iter().zip(output_u64.iter()) {
                assert_eq!(res1.to_u32(), *res2 as u32);
            }
        }
    }
}
