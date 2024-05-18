use std::sync::Arc;

use bellman_ce::{pairing::Engine, Circuit, ConstraintSystem, SynthesisError};

use crate::{
    circuits::PermCircuit,
    constraint_builder::{ConstraintBuilder, ProofVar},
};

use super::poseidon_params::PoseidonParams;

type CB = ConstraintBuilder;

#[derive(Clone, Debug)]
pub struct PoseidonCircuit<E: Engine> {
    input: Vec<Option<E::Fr>>,
    pub(crate) params: Arc<PoseidonParams<E::Fr>>,
}

impl<E: Engine> PoseidonCircuit<E> {
    pub fn new(params: &Arc<PoseidonParams<E::Fr>>) -> Self {
        PoseidonCircuit {
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
            *r = self.sbox_p(cs, r);
        }
        result
    }

    fn sbox_p<CS: ConstraintSystem<E>>(&self, cs: &mut CS, state: &ProofVar<E>) -> ProofVar<E> {
        let mut sq = CB::multiplication_new(state, state, cs);
        if self.params.d == 5 {
            let qu = CB::multiplication_new(&sq, &sq, cs);
            sq = qu;
        }
        CB::multiplication_new(state, &sq, cs)
    }

    fn cheap_matmul(&self, state: &[ProofVar<E>], r: usize) -> Vec<ProofVar<E>> {
        let v = &self.params.v[r];
        let w_hat = &self.params.w_hat[r];
        let t = self.params.t;

        let mut new_state = state.to_owned();
        new_state[0] = CB::scale(&new_state[0], &self.params.mds[0][0]);
        for i in 1..t {
            let tmp = CB::scale(&state[i], &w_hat[i - 1]);
            new_state[0] = CB::addition(&new_state[0], &tmp);
        }
        for i in 1..t {
            new_state[i] = state[0].clone();
            new_state[i] = CB::scale(&new_state[i], &v[i - 1]);
            new_state[i] = CB::addition(&new_state[i], &state[i]);
        }

        new_state
    }
}

impl<E: Engine> PermCircuit<E> for PoseidonCircuit<E> {
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
        for r in 0..self.params.rounds_f_beginning {
            current_state = CB::add_rc::<E, CS>(&current_state, &self.params.round_constants[r]);
            current_state = self.sbox(cs, &current_state);
            current_state = CB::matrix_mul(&current_state, &self.params.mds);
        }

        let p_end = self.params.rounds_f_beginning + self.params.rounds_p;
        current_state = CB::add_rc::<E, CS>(&current_state, &self.params.opt_round_constants[0]);
        current_state = CB::matrix_mul(&current_state, &self.params.m_i);

        for r in self.params.rounds_f_beginning..p_end {
            current_state[0] = self.sbox_p(cs, &current_state[0]);
            if r < p_end - 1 {
                current_state[0] = CB::add_constant::<E, CS>(
                    &current_state[0],
                    &self.params.opt_round_constants[r + 1 - self.params.rounds_f_beginning][0],
                )
            }
            current_state = self.cheap_matmul(&current_state, p_end - r - 1);
        }

        for r in p_end..self.params.rounds {
            current_state = CB::add_rc::<E, CS>(&current_state, &self.params.round_constants[r]);
            current_state = self.sbox(cs, &current_state);
            current_state = CB::matrix_mul(&current_state, &self.params.mds);
        }

        current_state
    }

    fn get_t(&self) -> usize {
        self.params.t
    }
}

impl<E: Engine> Circuit<E> for PoseidonCircuit<E> {
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
