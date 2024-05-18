use std::sync::Arc;

use ff::PrimeField;

use crate::fields::utils;

use super::reinforced_concrete_params::ReinforcedConcreteParams;

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
            let tmp = utils::mul_by_single_word::<F>(&repr, *s as u64);
            repr = utils::add_single_word::<F>(&tmp.0, *val as u64);
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
        assert_eq!(ReinforcedConcreteParams::<F>::T, input.len());
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

#[cfg(test)]
mod reinforced_concrete_kats {
    use super::*;

    use crate::{
        fields::{field48::Fp48, field56::Fp56, field64::Fp64},
        reinforced_concrete::reinforced_concrete_instances::*,
    };

    use ff::{from_hex, Field};

    static TESTRUNS: usize = 5;

    #[test]
    fn compose48() {
        type Scalar = Fp48;
        let rc = ReinforcedConcrete::new(&RC_PARAMS_EASY);

        for _ in 0..TESTRUNS {
            let input: Scalar = utils::random_scalar(true);
            let output = rc.compose(&rc.decompose(&input));

            assert_eq!(input, output);
        }
    }

    #[test]
    fn compose56() {
        type Scalar = Fp56;
        let rc = ReinforcedConcrete::new(&RC_PARAMS_MEDIUM);

        for _ in 0..TESTRUNS {
            let input: Scalar = utils::random_scalar(true);
            let output = rc.compose(&rc.decompose(&input));

            assert_eq!(input, output);
        }
    }

    #[test]
    fn compose64() {
        type Scalar = Fp64;
        let rc = ReinforcedConcrete::new(&RC_PARAMS_HARD);

        for _ in 0..TESTRUNS {
            let input: Scalar = utils::random_scalar(true);
            let output = rc.compose(&rc.decompose(&input));

            assert_eq!(input, output);
        }
    }

    #[test]
    fn easy_kats() {
        type Scalar = Fp48;
        let rc = ReinforcedConcrete::new(&RC_PARAMS_EASY);
        let input: [Scalar; 3] = [Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = rc.permutation(&input);
        assert_eq!(perm[0], from_hex("0x9dd81be4a029").unwrap());
        assert_eq!(perm[1], from_hex("0x2d73419eeae9").unwrap(),);
        assert_eq!(perm[2], from_hex("0x7de0e45ef4be").unwrap(),);
    }

    #[test]
    fn medium_kats() {
        type Scalar = Fp56;
        let rc = ReinforcedConcrete::new(&RC_PARAMS_MEDIUM);
        let input: [Scalar; 3] = [Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = rc.permutation(&input);
        assert_eq!(perm[0], from_hex("0x7ac8fe441eface").unwrap());
        assert_eq!(perm[1], from_hex("0x93b569b1b8f58a").unwrap(),);
        assert_eq!(perm[2], from_hex("0xaa6255d2aa3450").unwrap(),);
    }

    #[test]
    fn hard_kats() {
        type Scalar = Fp64;
        let rc = ReinforcedConcrete::new(&RC_PARAMS_HARD);
        let input: [Scalar; 3] = [Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = rc.permutation(&input);
        assert_eq!(perm[0], from_hex("0x0c3eb8259a579e18").unwrap());
        assert_eq!(perm[1], from_hex("0x92de8f70ea44896c").unwrap(),);
        assert_eq!(perm[2], from_hex("0xf4fa8563f3aec0ae").unwrap(),);
    }
}
