use std::sync::Arc;

use bellman_ce::{pairing::Engine, Circuit, ConstraintSystem, SynthesisError};

use crate::{
    circuits::PermCircuit,
    constraint_builder::{ConstraintBuilder, ProofVar},
};

use super::gmimc_params::GmimcParams;

type CB = ConstraintBuilder;

#[derive(Clone, Debug)]
pub struct GmimcCircuit<E: Engine> {
    input: Vec<Option<E::Fr>>,
    pub(crate) params: Arc<GmimcParams<E::Fr>>,
}

impl<E: Engine> GmimcCircuit<E> {
    pub fn new(params: &Arc<GmimcParams<E::Fr>>) -> Self {
        GmimcCircuit {
            input: vec![None; params.t],
            params: Arc::clone(params),
        }
    }

    fn sbox<CS: ConstraintSystem<E>>(
        &self,
        cs: &mut CS,
        state_0: &ProofVar<E>,
        round: usize,
    ) -> ProofVar<E> {
        let input = CB::add_constant::<E, CS>(state_0, &self.params.round_constants[round]);
        let mut sq = CB::multiplication_new(&input, &input, cs);
        if self.params.d == 5 {
            sq = CB::multiplication_new(&sq, &sq, cs);
        }
        CB::multiplication_new(&input, &sq, cs)
    }

    fn round<CS: ConstraintSystem<E>>(
        &self,
        cs: &mut CS,
        state: &[ProofVar<E>],
        round: usize,
    ) -> Vec<ProofVar<E>> {
        let power = self.sbox(cs, &state[0], round);
        let mut result = state.to_owned();

        result
            .iter_mut()
            .skip(1)
            .for_each(|f| *f = CB::addition(f, &power));

        result
    }
}

impl<E: Engine> PermCircuit<E> for GmimcCircuit<E> {
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
        for r in 0..self.params.rounds - 1 {
            current_state = self.round(cs, &current_state, r);
            current_state.rotate_right(1);
        }

        // finally without rotation
        self.round(cs, &current_state, self.params.rounds - 1)
    }

    fn get_t(&self) -> usize {
        self.params.t
    }
}

impl<E: Engine> Circuit<E> for GmimcCircuit<E> {
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
