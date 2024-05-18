use super::reinforced_concrete_params::ReinforcedConcreteParams;
use crate::{fields::utils, merkle_tree::merkle_tree_fp::MerkleTreeHash};
use ff::PrimeField;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ReinforcedConcrete<F: PrimeField> {
    pub(crate) params: Arc<ReinforcedConcreteParams<F>>,
}

impl<F: PrimeField> ReinforcedConcrete<F> {
    #[allow(clippy::assertions_on_constants)]
    pub fn new(params: &Arc<ReinforcedConcreteParams<F>>) -> Self {
        debug_assert!(ReinforcedConcreteParams::<F>::T == 3);
        ReinforcedConcrete {
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
        for _ in 0..self.params.alphas[0] {
            x1_sq.add_assign(&state[0]);
        }
        x1_sq.add_assign(&self.params.betas[0]);
        x1_sq.mul_assign(&state[1]);
        new_state[1] = x1_sq;

        // x3
        for _ in 0..self.params.alphas[1] {
            x2_sq.add_assign(&state[1]);
        }
        x2_sq.add_assign(&self.params.betas[1]);
        x2_sq.mul_assign(&state[2]);
        new_state[2] = x2_sq;

        new_state
    }

    pub fn decompose(&self, val: &F) -> Vec<u16> {
        let len = self.params.si.len();
        let mut res = vec![0; len];
        let mut repr = val.into_repr();

        for i in (1..self.params.si.len()).rev() {
            let (r, m) = utils::divide_long_using_recip::<F>(
                &repr,
                self.params.divisor_i[i],
                self.params.reciprokal_i[i],
                self.params.norm_shift_i[i],
            );
            repr = r;
            res[i] = m;
        }

        res[0] = repr.as_ref()[0] as u16;

        // just debugging
        if cfg!(debug_assertions) {
            let repr_ref = repr.as_ref();
            debug_assert!(repr_ref[0] < self.params.si[0] as u64);
            repr_ref
                .iter()
                .skip(1)
                .for_each(|el| debug_assert!(*el == 0));
        }

        res
    }

    pub fn compose(&self, vals: &[u16]) -> F {
        let mut repr = F::Repr::default();
        repr.as_mut()[0] = vals[0] as u64;

        for (val, s) in vals.iter().zip(self.params.si.iter()).skip(1) {
            repr = utils::mul_by_single_word::<F>(&repr, *s as u64);
            repr = utils::add_single_word::<F>(&repr, *val as u64);
        }
        F::from_repr(repr).unwrap()
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
        for i in 1..=ReinforcedConcreteParams::<F>::PRE_ROUNDS {
            current_state = self.bricks(&current_state);
            self.concrete(&mut current_state, i);
        }

        // bar round
        current_state = self.bars(&current_state);
        self.concrete(
            &mut current_state,
            ReinforcedConcreteParams::<F>::PRE_ROUNDS + 1,
        );

        // final rounds
        for i in ReinforcedConcreteParams::<F>::PRE_ROUNDS + 2
            ..=ReinforcedConcreteParams::<F>::TOTAL_ROUNDS
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

impl<F: PrimeField> MerkleTreeHash<F> for ReinforcedConcrete<F> {
    fn compress(&self, input: &[&F; 2]) -> F {
        self.hash(input[0], input[1])
    }
}

#[cfg(test)]
mod reinforced_concrete_tests_bn256 {
    use ff::{from_hex, Field};

    use crate::{
        fields::bn256::FpBN256, reinforced_concrete::reinforced_concrete_instances::RC_BN_PARAMS,
    };

    type Scalar = FpBN256;

    use super::*;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let rc = ReinforcedConcrete::new(&RC_BN_PARAMS);
        for _ in 0..TESTRUNS {
            let input1: [Scalar; 3] = [
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
            ];

            let mut input2: [Scalar; 3];
            loop {
                input2 = [
                    utils::random_scalar(true),
                    utils::random_scalar(true),
                    utils::random_scalar(true),
                ];
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = rc.permutation(&input1);
            let perm2 = rc.permutation(&input1);
            let perm3 = rc.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    #[test]
    fn compose() {
        let rc = ReinforcedConcrete::new(&RC_BN_PARAMS);

        for _ in 0..TESTRUNS {
            let input: Scalar = utils::random_scalar(true);
            let output = rc.compose(&rc.decompose(&input));

            assert_eq!(input, output);
        }
    }

    #[test]
    fn consistent_hash() {
        let rc = ReinforcedConcrete::new(&RC_BN_PARAMS);
        for _ in 0..TESTRUNS {
            let input1: Scalar = utils::random_scalar(true);
            let mut input2: Scalar;
            loop {
                input2 = utils::random_scalar(true);
                if input1 != input2 {
                    break;
                }
            }
            let input3: Scalar = utils::random_scalar(true);

            let h1 = rc.hash(&input1, &input3);
            let h2 = rc.hash(&input1, &input3);
            let h3 = rc.hash(&input2, &input3);
            assert_eq!(h1, h2);
            assert_ne!(h1, h3);
        }
    }

    #[test]
    fn kats() {
        let rc = ReinforcedConcrete::new(&RC_BN_PARAMS);
        let input: [Scalar; 3] = [Scalar::zero(), Scalar::one(), utils::from_u64(2)];
        let perm = rc.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x2510ddf9405eebaa4d9a4e0a821bffc80ed439355c500985797becf45403e42e").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x1e8fd5b981b3b2d1cff86e3d99a9dbed002afdd7a29726de8f4d645d7841eafd").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x2c37d92c6d2b6831006bf8b53614f4f5fcc3ee6c5dff9d36a8460625d7ee6907").unwrap(),
        );
    }
}

#[cfg(test)]
mod reinforced_concrete_tests_bls12 {
    use ff::{from_hex, Field};

    use crate::{
        fields::bls12::FpBLS12, reinforced_concrete::reinforced_concrete_instances::RC_BLS_PARAMS,
    };

    type Scalar = FpBLS12;

    use super::*;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let rc = ReinforcedConcrete::new(&RC_BLS_PARAMS);
        for _ in 0..TESTRUNS {
            let input1: [Scalar; 3] = [
                utils::random_scalar(true),
                utils::random_scalar(true),
                utils::random_scalar(true),
            ];

            let mut input2: [Scalar; 3];
            loop {
                input2 = [
                    utils::random_scalar(true),
                    utils::random_scalar(true),
                    utils::random_scalar(true),
                ];
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = rc.permutation(&input1);
            let perm2 = rc.permutation(&input1);
            let perm3 = rc.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    #[test]
    fn compose() {
        let rc = ReinforcedConcrete::new(&RC_BLS_PARAMS);

        for _ in 0..TESTRUNS {
            let input: Scalar = utils::random_scalar(true);
            let output = rc.compose(&rc.decompose(&input));

            assert_eq!(input, output);
        }
    }

    #[test]
    fn consistent_hash() {
        let rc = ReinforcedConcrete::new(&RC_BLS_PARAMS);
        for _ in 0..TESTRUNS {
            let input1: Scalar = utils::random_scalar(true);
            let mut input2: Scalar;
            loop {
                input2 = utils::random_scalar(true);
                if input1 != input2 {
                    break;
                }
            }
            let input3: Scalar = utils::random_scalar(true);

            let h1 = rc.hash(&input1, &input3);
            let h2 = rc.hash(&input1, &input3);
            let h3 = rc.hash(&input2, &input3);
            assert_eq!(h1, h2);
            assert_ne!(h1, h3);
        }
    }

    #[test]
    fn kats() {
        let rc = ReinforcedConcrete::new(&RC_BLS_PARAMS);
        let input: [Scalar; 3] = [Scalar::zero(), Scalar::one(), utils::from_u64(2)];
        let perm = rc.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x737df8e5a548189a0d77821a907def6736ea6512ba4633f1001f27d8f242913c").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x579c286d69635c6e3136f76e99775b478b29412a05516ac6201527abbb3ea098").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x5abe7c734229be9122f936d919f8babb74b36b1ca98f133b00256e29be115aa8").unwrap(),
        );
    }
}
