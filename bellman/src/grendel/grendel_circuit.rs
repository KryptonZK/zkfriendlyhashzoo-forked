use std::sync::Arc;

use bellman_ce::{
    pairing::Engine,
    pairing::{
        ff::{Field, SqrtField},
        LegendreSymbol,
    },
    Circuit, ConstraintSystem, SynthesisError,
};

use crate::{
    circuits::PermCircuit,
    constraint_builder::{ConstraintBuilder, ProofVar},
};

use super::grendel_params::GrendelParams;

type CB = ConstraintBuilder;

#[derive(Clone, Debug)]
pub struct GrendelCircuit<E: Engine> {
    input: Vec<Option<E::Fr>>,
    pub(crate) params: Arc<GrendelParams<E::Fr>>,
}

impl<E: Engine> GrendelCircuit<E> {
    pub fn new(params: &Arc<GrendelParams<E::Fr>>) -> Self {
        GrendelCircuit {
            input: vec![None; params.t],
            params: Arc::clone(params),
        }
    }

    fn sbox<CS: ConstraintSystem<E>>(
        &self,
        cs: &mut CS,
        state: &[ProofVar<E>],
    ) -> Vec<ProofVar<E>> {
        let mut m1 = E::Fr::zero();
        m1.sub_assign(&E::Fr::one());

        let t = state.len();
        let mut result: Vec<ProofVar<E>> = Vec::with_capacity(t);
        let mut powers: Vec<ProofVar<E>> = Vec::with_capacity(t);
        let mut ls: Vec<ProofVar<E>> = Vec::with_capacity(t);

        for s in state.iter() {
            let mut sq = CB::multiplication_new(s, s, cs);
            if self.params.d == 2 {
                powers.push(sq);
            } else {
                if self.params.d == 5 {
                    let qu = CB::multiplication_new(&sq, &sq, cs);
                    sq = qu;
                }
                let power = CB::multiplication_new(s, &sq, cs);
                powers.push(power);
            }

            // legendre: (6 constraints)

            let val = s.get_value();
            let mut var_l = None;
            let mut var_b = None;

            if let Some(v) = val {
                // legendre symbol
                let l = match v.legendre() {
                    LegendreSymbol::QuadraticNonResidue => m1,
                    LegendreSymbol::QuadraticResidue => E::Fr::one(),
                    _ => E::Fr::zero(),
                };
                var_l = Some(l);
                // b
                let mut an = v.to_owned();
                an.mul_assign(&self.params.n);
                let b = match v.sqrt() {
                    None => an.sqrt().expect("Something wrong?"),
                    Some(a) => a,
                };
                var_b = Some(b);
            }

            // l (l - 1)(l + 1) = 0
            let l = CB::new_variable(var_l, cs);
            let lp1 = CB::add_constant::<E, CS>(&l, &E::Fr::one());
            let lm1 = CB::sub_constant::<E, CS>(&l, &E::Fr::one());
            let l_lm1 = CB::multiplication_new(&l, &lm1, cs);
            CB::enforce_zero(&l_lm1, &lp1, cs);
            ls.push(l);

            // l (l - 1)(b^2 - na) + (l + 1)(b^2 -a) = 0
            let b = CB::new_variable(var_b, cs);
            let b_sq = CB::multiplication_new(&b, &b, cs);
            let b_sq_a = CB::subtraction(&b_sq, s);
            let na = CB::scale(s, &self.params.n);
            let b_sq_na = CB::subtraction(&b_sq, &na);
            let lhs = CB::multiplication_new(&l_lm1, &b_sq_na, cs);
            let rhs = CB::multiplication_new(&lp1, &b_sq_a, cs);
            let sum = CB::addition(&lhs, &rhs);

            CB::enforce_is_zero(&sum, cs);
        }
        // combine powers and legendres
        for i in 0..t {
            let res = CB::multiplication_new(&powers[i], &ls[i], cs);
            result.push(res);
        }
        result
    }
}

impl<E: Engine> PermCircuit<E> for GrendelCircuit<E> {
    fn set_input(&mut self, preimage: &[E::Fr]) {
        debug_assert!(preimage.len() == self.params.t);
        self.input = preimage.iter().map(|el| Some(*el)).collect();
    }

    fn permutation<CS: ConstraintSystem<E>>(
        &self,
        cs: &mut CS,
        state: &[ProofVar<E>],
    ) -> Vec<ProofVar<E>> {
        let mut current_state = state.to_owned();
        for r in 0..self.params.rounds {
            current_state = self.sbox(cs, &current_state);
            current_state = CB::matrix_mul(&current_state, &self.params.mds);
            current_state = CB::add_rc::<E, CS>(&current_state, &self.params.round_constants[r]);
        }
        current_state
    }

    fn get_t(&self) -> usize {
        self.params.t
    }
}

impl<E: Engine> Circuit<E> for GrendelCircuit<E> {
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let t = self.params.t;
        debug_assert!(self.input.len() == t);
        let current_state: Vec<ProofVar<E>> = self
            .input
            .iter()
            .map(|el| CB::new_variable(*el, cs))
            .collect();

        let current_state = self.permutation(cs, &current_state);

        for var in current_state {
            CB::enforce_final_linear(&var, cs);
        }

        Ok(())
    }
}
