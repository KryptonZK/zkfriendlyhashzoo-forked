use super::reinforced_concrete_st_params::ReinforcedConcreteStParams;
use crate::{fields::utils4, merkle_tree::merkle_tree_fp::MerkleTreeHash};
use ff::PrimeField;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ReinforcedConcreteSt<F: PrimeField> {
    pub(crate) params: Arc<ReinforcedConcreteStParams<F>>,
}

impl<F: PrimeField> ReinforcedConcreteSt<F> {
    const BIT: usize = 10;
    const LEN: usize = 25;
    const STAGE_SIZE: usize = 6;
    const MASK_0: u64 = (1u64 << Self::BIT) - 1;
    const MASK_1: u64 = Self::MASK_0 << Self::BIT;
    const MASK_2: u64 = Self::MASK_1 << Self::BIT;
    const MASK_3: u64 = Self::MASK_2 << Self::BIT;
    const MASK_4: u64 = Self::MASK_3 << Self::BIT;
    const MASK_5: u64 = Self::MASK_4 << Self::BIT;
    const S_24: u16 = 1023;

    // For the division with precomputed reciprocal (credit shamatar)
    const R: (u64, u64) = utils4::compute_normalized_divisor_and_reciproical(Self::S_24);
    const DIVISOR: u64 = Self::R.0;
    const RECIP: u64 = Self::R.1;
    const S: u32 = (Self::S_24 as u64).leading_zeros();

    #[allow(clippy::assertions_on_constants)]
    pub fn new(params: &Arc<ReinforcedConcreteStParams<F>>) -> Self {
        debug_assert!(ReinforcedConcreteStParams::<F>::T == 3);
        debug_assert!(params.betas[0].into_repr().as_ref().len() == 4);
        ReinforcedConcreteSt {
            params: Arc::clone(params),
        }
    }

    pub fn concrete(&self, state: &mut [F; 3], round: usize) {
        // multiplication by circ(2 1 1) is equal to state + sum(state)

        let mut sum = state[0];
        state.iter().skip(1).for_each(|el| sum.add_assign(el));

        for (el, rc) in state
            .iter_mut()
            .zip(self.params.round_constants[round].iter())
        {
            el.add_assign(&sum);
            el.add_assign(rc); // add round constant
        }
    }

    pub fn bricks(&self, state: &[F; 3]) -> [F; 3] {
        let mut new_state: [F; 3] = [F::zero(); 3];

        // squaring
        let mut x1_sq = state[0];
        x1_sq.square();
        let mut x2_sq = state[1];
        x2_sq.square();

        // x1
        let mut x1 = x1_sq;
        match self.params.d {
            3 => {}
            5 => x1.square(),
            _ => panic!("not implemented!"),
        }
        x1.mul_assign(&state[0]);
        new_state[0] = x1;

        // x2
        x1_sq.add_assign(&state[0]);
        x1_sq.add_assign(&self.params.betas[0]);
        x1_sq.mul_assign(&state[1]);
        new_state[1] = x1_sq;

        // x3
        x2_sq.add_assign(&state[1]);
        x2_sq.add_assign(&state[1]);
        x2_sq.add_assign(&self.params.betas[1]);
        x2_sq.mul_assign(&state[2]);
        new_state[2] = x2_sq;

        new_state
    }

    // TODO use constant as soon as possible
    pub fn decompose(&self, val: &F) -> [u16; 25] {
        let mut res = [0; 25];
        let mut repr = utils4::into_limbs(val);

        // first a division
        let (r, m) = utils4::divide_long_using_recip(&repr, Self::DIVISOR, Self::RECIP, Self::S);

        res[Self::LEN - 1] = m;
        repr = r;

        {
            let lsw = repr[0];

            res[Self::LEN - 2] = (lsw & Self::MASK_0) as u16;
            res[Self::LEN - 2 - 1] = ((lsw & Self::MASK_1) >> Self::BIT) as u16;
            res[Self::LEN - 2 - 2] = ((lsw & Self::MASK_2) >> (2 * Self::BIT)) as u16;
            res[Self::LEN - 2 - 3] = ((lsw & Self::MASK_3) >> (3 * Self::BIT)) as u16;
            res[Self::LEN - 2 - 4] = ((lsw & Self::MASK_4) >> (4 * Self::BIT)) as u16;
            res[Self::LEN - 2 - 5] = ((lsw & Self::MASK_5) >> (5 * Self::BIT)) as u16;

            repr = utils4::full_shr(&repr, (Self::BIT * Self::STAGE_SIZE) as u32);
        }

        {
            let lsw = repr[0];

            res[Self::LEN - 2 - Self::STAGE_SIZE] = (lsw & Self::MASK_0) as u16;
            res[Self::LEN - 2 - Self::STAGE_SIZE - 1] = ((lsw & Self::MASK_1) >> Self::BIT) as u16;
            res[Self::LEN - 2 - Self::STAGE_SIZE - 2] =
                ((lsw & Self::MASK_2) >> (2 * Self::BIT)) as u16;
            res[Self::LEN - 2 - Self::STAGE_SIZE - 3] =
                ((lsw & Self::MASK_3) >> (3 * Self::BIT)) as u16;
            res[Self::LEN - 2 - Self::STAGE_SIZE - 4] =
                ((lsw & Self::MASK_4) >> (4 * Self::BIT)) as u16;
            res[Self::LEN - 2 - Self::STAGE_SIZE - 5] =
                ((lsw & Self::MASK_5) >> (5 * Self::BIT)) as u16;

            repr = utils4::full_shr(&repr, (Self::BIT * Self::STAGE_SIZE) as u32);
        }

        {
            let lsw = repr[0];

            res[Self::LEN - 2 - 2 * Self::STAGE_SIZE] = (lsw & Self::MASK_0) as u16;
            res[Self::LEN - 2 - 2 * Self::STAGE_SIZE - 1] =
                ((lsw & Self::MASK_1) >> Self::BIT) as u16;
            res[Self::LEN - 2 - 2 * Self::STAGE_SIZE - 2] =
                ((lsw & Self::MASK_2) >> (2 * Self::BIT)) as u16;
            res[Self::LEN - 2 - 2 * Self::STAGE_SIZE - 3] =
                ((lsw & Self::MASK_3) >> (3 * Self::BIT)) as u16;
            res[Self::LEN - 2 - 2 * Self::STAGE_SIZE - 4] =
                ((lsw & Self::MASK_4) >> (4 * Self::BIT)) as u16;
            res[Self::LEN - 2 - 2 * Self::STAGE_SIZE - 5] =
                ((lsw & Self::MASK_5) >> (5 * Self::BIT)) as u16;

            repr = utils4::full_shr(&repr, (Self::BIT * Self::STAGE_SIZE) as u32);
        }

        {
            let lsw = repr[0];

            res[Self::LEN - 2 - 3 * Self::STAGE_SIZE] = (lsw & Self::MASK_0) as u16;
            res[Self::LEN - 2 - 3 * Self::STAGE_SIZE - 1] =
                ((lsw & Self::MASK_1) >> Self::BIT) as u16;
            res[Self::LEN - 2 - 3 * Self::STAGE_SIZE - 2] =
                ((lsw & Self::MASK_2) >> (2 * Self::BIT)) as u16;
            res[Self::LEN - 2 - 3 * Self::STAGE_SIZE - 3] =
                ((lsw & Self::MASK_3) >> (3 * Self::BIT)) as u16;
            res[Self::LEN - 2 - 3 * Self::STAGE_SIZE - 4] =
                ((lsw & Self::MASK_4) >> (4 * Self::BIT)) as u16;
            res[Self::LEN - 2 - 3 * Self::STAGE_SIZE - 5] =
                ((lsw & Self::MASK_5) >> (5 * Self::BIT)) as u16;

            // repr = utils4::full_shr(&repr, (Self::BIT * Self::STAGE_SIZE) as u32);
        }

        res
    }

    // TODO use constant as soon as possible
    pub fn compose(&self, vals: &[u16; 25]) -> F {
        let mut repr = [0; 4];
        {
            let mut val = vals[0] as u64;
            val <<= Self::BIT;
            val += vals[1] as u64;
            val <<= Self::BIT;
            val += vals[2] as u64;
            val <<= Self::BIT;
            val += vals[3] as u64;
            val <<= Self::BIT;
            val += vals[4] as u64;
            val <<= Self::BIT;
            val += vals[5] as u64;

            repr[0] |= val;
            repr = utils4::partial_shl(&repr, (Self::BIT * Self::STAGE_SIZE) as u32);
        }

        {
            let mut val = vals[Self::STAGE_SIZE] as u64;
            val <<= Self::BIT;
            val += vals[Self::STAGE_SIZE + 1] as u64;
            val <<= Self::BIT;
            val += vals[Self::STAGE_SIZE + 2] as u64;
            val <<= Self::BIT;
            val += vals[Self::STAGE_SIZE + 3] as u64;
            val <<= Self::BIT;
            val += vals[Self::STAGE_SIZE + 4] as u64;
            val <<= Self::BIT;
            val += vals[Self::STAGE_SIZE + 5] as u64;

            repr[0] |= val;
            repr = utils4::partial_shl(&repr, (Self::BIT * Self::STAGE_SIZE) as u32);
        }

        {
            let mut val = vals[2 * Self::STAGE_SIZE] as u64;
            val <<= Self::BIT;
            val += vals[2 * Self::STAGE_SIZE + 1] as u64;
            val <<= Self::BIT;
            val += vals[2 * Self::STAGE_SIZE + 2] as u64;
            val <<= Self::BIT;
            val += vals[2 * Self::STAGE_SIZE + 3] as u64;
            val <<= Self::BIT;
            val += vals[2 * Self::STAGE_SIZE + 4] as u64;
            val <<= Self::BIT;
            val += vals[2 * Self::STAGE_SIZE + 5] as u64;

            repr[0] |= val;
            repr = utils4::partial_shl(&repr, (Self::BIT * Self::STAGE_SIZE) as u32);
        }

        {
            let mut val = vals[3 * Self::STAGE_SIZE] as u64;
            val <<= Self::BIT;
            val += vals[3 * Self::STAGE_SIZE + 1] as u64;
            val <<= Self::BIT;
            val += vals[3 * Self::STAGE_SIZE + 2] as u64;
            val <<= Self::BIT;
            val += vals[3 * Self::STAGE_SIZE + 3] as u64;
            val <<= Self::BIT;
            val += vals[3 * Self::STAGE_SIZE + 4] as u64;
            val <<= Self::BIT;
            val += vals[3 * Self::STAGE_SIZE + 5] as u64;

            repr[0] |= val;
            // repr = utils4::partial_shl(&repr, (Self::BIT * Self::STAGE_SIZE) as u32);
        }

        repr = utils4::mul_by_single_word(&repr, Self::S_24 as u64);
        repr = utils4::add_single_word(&repr, vals[Self::LEN - 1] as u64);

        utils4::from_limbs(&repr)
    }

    pub fn bars(&self, state: &[F; 3]) -> [F; 3] {
        let mut s = state.to_owned();
        for el in s.iter_mut() {
            let mut vals = self.decompose(el);
            for val in vals.iter_mut() {
                // *val = self.params.sbox[*val as usize];
                // safe because sbox is padded to the correct size in params
                unsafe {
                    *val = *self.params.sbox.get_unchecked(*val as usize);
                }
            }
            *el = self.compose(&vals);
        }
        s
    }

    pub fn permutation(&self, input: &[F; 3]) -> [F; 3] {
        let mut current_state = input.to_owned();
        // first concrete
        self.concrete(&mut current_state, 0);

        // first rounds
        for i in 1..=ReinforcedConcreteStParams::<F>::PRE_ROUNDS {
            current_state = self.bricks(&current_state);
            self.concrete(&mut current_state, i);
        }

        // bar round
        current_state = self.bars(&current_state);
        self.concrete(
            &mut current_state,
            ReinforcedConcreteStParams::<F>::PRE_ROUNDS + 1,
        );

        // final rounds
        for i in ReinforcedConcreteStParams::<F>::PRE_ROUNDS + 2
            ..=ReinforcedConcreteStParams::<F>::TOTAL_ROUNDS
        {
            current_state = self.bricks(&current_state);
            self.concrete(&mut current_state, i);
        }
        current_state
    }

    pub fn hash(&self, el1: &F, el2: &F) -> F {
        let input: [F; 3] = [el1.to_owned(), el2.to_owned(), F::zero()];
        self.permutation(&input)[0]
    }
}

impl<F: PrimeField> MerkleTreeHash<F> for ReinforcedConcreteSt<F> {
    fn compress(&self, input: &[&F; 2]) -> F {
        self.hash(input[0], input[1])
    }
}

#[cfg(test)]
mod reinforced_concrete_st_tests {
    use ff::{from_hex, Field};
    use lazy_static::lazy_static;

    use crate::{
        fields::st::FpST,
        reinforced_concrete::{
            reinforced_concrete::ReinforcedConcrete,
            reinforced_concrete_params::ReinforcedConcreteParams,
        },
        reinforced_concrete_st::reinforced_concrete_st_instances::RC_ST_PARAMS,
    };

    type Scalar = FpST;

    use super::*;

    static TESTRUNS: usize = 5;
    static SI: [u16; 25] = [
        1024, 1024, 1024, 1024, 1024, 1024, 1024, 1024, 1024, 1024, 1024, 1024, 1024, 1024, 1024,
        1024, 1024, 1024, 1024, 1024, 1024, 1024, 1024, 1024, 1023,
    ];

    lazy_static! {
        static ref RC_ST_P: Arc<ReinforcedConcreteParams<Scalar>> = Arc::new(
            ReinforcedConcreteParams::new(RC_ST_PARAMS.d, &SI, &RC_ST_PARAMS.sbox, &[1, 2, 3, 4])
        );
    }

    #[test]
    fn consistent_perm() {
        let rc = ReinforcedConcreteSt::new(&RC_ST_PARAMS);
        let rc2 = ReinforcedConcrete::new(&RC_ST_P);

        for _ in 0..TESTRUNS {
            let input1: [Scalar; 3] = [
                utils4::random_scalar(true),
                utils4::random_scalar(true),
                utils4::random_scalar(true),
            ];

            let mut input2: [Scalar; 3];
            loop {
                input2 = [
                    utils4::random_scalar(true),
                    utils4::random_scalar(true),
                    utils4::random_scalar(true),
                ];
                if input1 != input2 {
                    break;
                }
            }

            let perm1_1 = rc.permutation(&input1);
            let perm1_2 = rc.permutation(&input1);
            let perm1_3 = rc.permutation(&input2);
            let perm2_1 = rc2.permutation(&input1);
            let perm2_2 = rc2.permutation(&input1);
            let perm2_3 = rc2.permutation(&input2);
            assert_eq!(perm1_1, perm2_1);
            assert_eq!(perm1_2, perm2_2);
            assert_eq!(perm1_3, perm2_3);
            assert_eq!(perm1_1, perm1_2);
            assert_ne!(perm1_1, perm1_3);
        }
    }

    #[test]
    fn consistent_hash() {
        let rc = ReinforcedConcreteSt::new(&RC_ST_PARAMS);
        let rc2 = ReinforcedConcrete::new(&RC_ST_P);

        for _ in 0..TESTRUNS {
            let input1: Scalar = utils4::random_scalar(true);
            let mut input2: Scalar;
            loop {
                input2 = utils4::random_scalar(true);
                if input1 != input2 {
                    break;
                }
            }
            let input3: Scalar = utils4::random_scalar(true);

            let h1_1 = rc.hash(&input1, &input3);
            let h1_2 = rc.hash(&input1, &input3);
            let h1_3 = rc.hash(&input2, &input3);
            let h2_1 = rc2.hash(&input1, &input3);
            let h2_2 = rc2.hash(&input1, &input3);
            let h2_3 = rc2.hash(&input2, &input3);
            assert_eq!(h1_1, h2_1);
            assert_eq!(h1_2, h2_2);
            assert_eq!(h1_3, h2_3);
            assert_eq!(h1_1, h1_2);
            assert_ne!(h1_1, h1_3);
        }
    }

    #[test]
    fn kats() {
        let rc = ReinforcedConcreteSt::new(&RC_ST_PARAMS);
        let rc2 = ReinforcedConcrete::new(&RC_ST_P);

        let input: [Scalar; 3] = [Scalar::zero(), Scalar::one(), utils4::from_u64(2)];
        let perm1 = rc.permutation(&input);
        let perm2 = rc2.permutation(&input);
        assert_eq!(perm1, perm2);
        assert_eq!(
            perm1[0],
            from_hex("0x0026b02ce8c46a43773c7b8e2335642224aec1f72d060697faa4c3e99c7b524e").unwrap()
        );
        assert_eq!(
            perm1[1],
            from_hex("0x0314c340fa9da579d2b3466947836d130616e1ca35f1884ab36ed5a8d2e9212e").unwrap(),
        );
        assert_eq!(
            perm1[2],
            from_hex("0x02ebb8984a6b0d773bf79e7b24bb3b722313e9c4e5be7cd36e279ed0d22a918a").unwrap(),
        );
    }

    #[test]
    fn compose() {
        let rc = ReinforcedConcreteSt::new(&RC_ST_PARAMS);

        for _ in 0..TESTRUNS {
            let input: Scalar = utils4::random_scalar(true);
            let output = rc.compose(&rc.decompose(&input));

            assert_eq!(input, output);
        }
    }
}

#[cfg(test)]
mod reinforced_concrete_st_compose_tests {
    use super::*;
    use crate::{
        fields::st::FpST, reinforced_concrete_st::reinforced_concrete_st_instances::RC_ST_PARAMS,
    };

    static TESTRUNS: usize = 5;
    static SI: [u16; 25] = [
        1024, 1024, 1024, 1024, 1024, 1024, 1024, 1024, 1024, 1024, 1024, 1024, 1024, 1024, 1024,
        1024, 1024, 1024, 1024, 1024, 1024, 1024, 1024, 1024, 1023,
    ];

    fn compose<F: PrimeField>(vals: &[u16; 25]) -> F {
        let mut repr = [0u64; 4];
        repr[0] = vals[0] as u64;

        for (val, s) in vals.iter().zip(SI.iter()).skip(1) {
            repr = utils4::mul_by_single_word(&repr, *s as u64);
            repr = utils4::add_single_word(&repr, *val as u64);
        }
        utils4::from_limbs(&repr)
    }

    fn decompose<F: PrimeField>(val: &F) -> [u16; 25] {
        let mut res = [0; 25];
        let mut repr = utils4::into_limbs(val);

        let mut offset = 0;

        for (val, s) in res.iter_mut().zip(SI.iter()).skip(1).rev() {
            let (r, m) = utils4::divide_long_decomp(&repr, *s, &mut offset);
            repr = r;
            *val = m;
        }

        res[0] = repr[0] as u16;
        res
    }

    #[test]
    fn consistent_compose() {
        type Scalar = FpST;

        let rc = ReinforcedConcreteSt::new(&RC_ST_PARAMS);

        for _ in 0..TESTRUNS {
            let input: Scalar = utils4::random_scalar(true);
            let output = rc.compose(&decompose(&input));

            assert_eq!(input, output);
        }
    }

    #[test]
    fn consistent_decompose() {
        type Scalar = FpST;

        let rc = ReinforcedConcreteSt::new(&RC_ST_PARAMS);

        for _ in 0..TESTRUNS {
            let input: Scalar = utils4::random_scalar(true);
            let output = compose(&rc.decompose(&input));

            assert_eq!(input, output);
        }
    }
}
