use std::sync::Arc;

use bellman_ce::{pairing::ff::Field, pairing::Engine, Circuit, ConstraintSystem, SynthesisError};

use crate::{
    circuits::PermCircuit,
    constraint_builder::{ConstraintBuilder, ProofVar},
};

use super::griffin_params::GriffinParams;

type CB = ConstraintBuilder;

#[derive(Clone, Debug)]
pub struct GriffinCircuit<E: Engine> {
    input: Vec<Option<E::Fr>>,
    pub(crate) params: Arc<GriffinParams<E::Fr>>,
}

impl<E: Engine> GriffinCircuit<E> {
    pub fn new(params: &Arc<GriffinParams<E::Fr>>) -> Self {
        GriffinCircuit {
            input: vec![None; params.t],
            params: Arc::clone(params),
        }
    }

    fn l(
        &self,
        y01_i: &mut ProofVar<E>,
        y0: &ProofVar<E>,
        x: &ProofVar<E>,
        i: usize,
    ) -> ProofVar<E> {
        if i == 0 {
            y01_i.to_owned()
        } else {
            *y01_i = CB::addition(y01_i, y0);
            CB::addition(y01_i, x)
        }
    }

    fn non_linear<CS: ConstraintSystem<E>>(
        &self,
        cs: &mut CS,
        state: &[ProofVar<E>],
    ) -> Vec<ProofVar<E>> {
        let mut result = state.to_owned();
        // x0
        let power = result[0].value.map(|a| a.pow(self.params.d_inv));

        result[0] = CB::new_variable(power, cs);
        let mut sq = CB::multiplication_new(&result[0], &result[0], cs);
        if self.params.d == 5 {
            sq = CB::multiplication_new(&sq, &sq, cs);
        }
        CB::multiplication(&result[0], &sq, &state[0], cs);

        // x1
        let mut sq = CB::multiplication_new(&result[1], &result[1], cs);
        if self.params.d == 5 {
            sq = CB::multiplication_new(&sq, &sq, cs);
        }
        result[1] = CB::multiplication_new(&result[1], &sq, cs);

        let y0 = result[0].to_owned(); // y0
        let mut y01_i = CB::addition(&y0, &result[1]); // y0 + y1

        // rest of the state
        for (i, ((out, inp), con)) in result
            .iter_mut()
            .skip(2)
            .zip(state.iter().skip(1))
            .zip(self.params.alpha_beta.iter())
            .enumerate()
        {
            let mut l = self.l(&mut y01_i, &y0, inp, i);
            let l_squ = CB::multiplication_new(&l, &l, cs);
            l = CB::scale(&l, &con[0]);
            l = CB::addition(&l, &l_squ);
            l = CB::add_constant::<E, CS>(&l, &con[1]);
            *out = CB::multiplication_new(out, &l, cs);
        }

        result
    }
}

impl<E: Engine> PermCircuit<E> for GriffinCircuit<E> {
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
        current_state = CB::matrix_mul(&current_state, &self.params.mat);

        for r in 0..self.params.rounds {
            current_state = self.non_linear(cs, &current_state);
            current_state = CB::matrix_mul(&current_state, &self.params.mat);
            if r < self.params.rounds - 1 {
                current_state =
                    CB::add_rc::<E, CS>(&current_state, &self.params.round_constants[r]);
            }
        }
        current_state
    }

    fn get_t(&self) -> usize {
        self.params.t
    }
}

impl<E: Engine> Circuit<E> for GriffinCircuit<E> {
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
