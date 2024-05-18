use std::sync::Arc;

use bellman_ce::{pairing::ff::Field, pairing::Engine, Circuit, ConstraintSystem, SynthesisError};

use crate::{
    circuits::PermCircuit,
    constraint_builder::{ConstraintBuilder, ProofVar},
};

use super::rescue_params::RescueParams;

type CB = ConstraintBuilder;

#[derive(Clone, Debug)]
pub struct RescueCircuit<E: Engine> {
    input: Vec<Option<E::Fr>>,
    pub(crate) params: Arc<RescueParams<E::Fr>>,
}

impl<E: Engine> RescueCircuit<E> {
    pub fn new(params: &Arc<RescueParams<E::Fr>>) -> Self {
        RescueCircuit {
            input: vec![None; params.t],
            params: Arc::clone(params),
        }
    }

    fn sbox<CS: ConstraintSystem<E>>(
        &self,
        cs: &mut CS,
        state: &[ProofVar<E>],
    ) -> Vec<ProofVar<E>> {
        let mut result = state.to_owned();
        for r in result.iter_mut() {
            let mut sq = CB::multiplication_new(r, r, cs);
            if self.params.d == 5 {
                let qu = CB::multiplication_new(&sq, &sq, cs);
                sq = qu;
            }
            *r = CB::multiplication_new(r, &sq, cs);
        }
        result
    }

    fn sbox_inverse<CS: ConstraintSystem<E>>(
        &self,
        cs: &mut CS,
        state: &[ProofVar<E>],
    ) -> Vec<ProofVar<E>> {
        let mut res = Vec::with_capacity(state.len());
        for var in state {
            let power = var.value.map(|a| a.pow(self.params.d_inv));
            let new_var = CB::new_variable(power, cs);

            let mut sq = CB::multiplication_new(&new_var, &new_var, cs);
            if self.params.d == 5 {
                let qu = CB::multiplication_new(&sq, &sq, cs);
                sq = qu;
            }
            CB::multiplication(&new_var, &sq, var, cs);

            res.push(new_var);
        }
        res
    }
}

impl<E: Engine> PermCircuit<E> for RescueCircuit<E> {
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
            current_state =
                CB::add_rc::<E, CS>(&current_state, &self.params.round_constants[2 * r]);

            current_state = self.sbox_inverse(cs, &current_state);
            current_state = CB::matrix_mul(&current_state, &self.params.mds);
            current_state =
                CB::add_rc::<E, CS>(&current_state, &self.params.round_constants[2 * r + 1]);
        }
        current_state
    }

    fn get_t(&self) -> usize {
        self.params.t
    }
}

impl<E: Engine> Circuit<E> for RescueCircuit<E> {
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
