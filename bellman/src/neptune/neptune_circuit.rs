use std::sync::Arc;

use bellman_ce::{pairing::ff::Field, pairing::Engine, Circuit, ConstraintSystem, SynthesisError};

use crate::{
    circuits::PermCircuit,
    constraint_builder::{ConstraintBuilder, ProofVar},
};

use super::neptune_params::NeptuneParams;

type CB = ConstraintBuilder;

#[derive(Clone, Debug)]
pub struct NeptuneCircuit<E: Engine> {
    input: Vec<Option<E::Fr>>,
    pub(crate) params: Arc<NeptuneParams<E::Fr>>,
}

impl<E: Engine> NeptuneCircuit<E> {
    pub fn new(params: &Arc<NeptuneParams<E::Fr>>) -> Self {
        NeptuneCircuit {
            input: vec![None; params.t],
            params: Arc::clone(params),
        }
    }

    fn external_round<CS: ConstraintSystem<E>>(
        &self,
        cs: &mut CS,
        input: &[ProofVar<E>],
        r: usize,
    ) -> Vec<ProofVar<E>> {
        let mut output = self.external_sbox(cs, input);
        output = self.external_matmul(&output);
        CB::add_rc::<E, CS>(&output, &self.params.round_constants[r])
    }

    fn internal_round<CS: ConstraintSystem<E>>(
        &self,
        cs: &mut CS,
        input: &[ProofVar<E>],
        r: usize,
    ) -> Vec<ProofVar<E>> {
        let mut output = self.internal_sbox(cs, input);
        output = self.internal_matmul(&output);
        CB::add_rc::<E, CS>(&output, &self.params.round_constants[r])
    }

    fn sbox_d<CS: ConstraintSystem<E>>(
        &self,
        cs: &mut CS,
        constraints: &ProofVar<E>,
    ) -> ProofVar<E> {
        let mut sq = CB::multiplication_new(constraints, constraints, cs);
        if self.params.d == 5 {
            let qu = CB::multiplication_new(&sq, &sq, cs);
            sq = qu;
        }
        CB::multiplication_new(&sq, constraints, cs)
    }

    fn external_sbox_prime<CS: ConstraintSystem<E>>(
        &self,
        cs: &mut CS,
        x1: &ProofVar<E>,
        x2: &ProofVar<E>,
    ) -> (ProofVar<E>, ProofVar<E>) {
        let zi = CB::subtraction(x1, x2);
        let zib = CB::multiplication_new(&zi, &zi, cs);
        // zib = CB::scale(&zib, &self.params.abc[1]); // beta = 1

        // first terms
        let mut y1 = CB::addition(x1, x2);
        let mut y2 = y1.to_owned();
        y1 = CB::addition(&y1, x1);
        y2 = CB::addition(&y2, x2);
        y2 = CB::addition(&y2, x2);
        // y1 = CB::scale(&y1, &self.params.a_[0]); // alpha = 1
        // y2 = CB::scale(&y2, &self.params.a_[0]); // alpha = 1

        // middle terms
        let tmp1 = CB::addition(&zib, &zib);
        let tmp1 = CB::addition(&tmp1, &zib);
        let tmp2 = CB::addition(&tmp1, &zib);
        // let tmp1 = CB::scale(&zib, &self.params.a_[1]); // done with additions, since alpha = beta = 1
        // let tmp2 = CB::scale(&zib, &self.params.a_[2]); // done with additions, since alpha = beta = 1
        y1 = CB::addition(&y1, &tmp1);
        y2 = CB::addition(&y2, &tmp2);

        // third terms
        let mut tmp = CB::subtraction(&zi, x2);
        // tmp = CB::scale(&tmp, &self.params.abc[0]); // alpha = 1
        tmp = CB::subtraction(&tmp, &zib);
        tmp = CB::add_constant::<E, CS>(&tmp, &self.params.abc[2]);
        tmp = CB::multiplication_new(&tmp, &tmp, cs);
        // tmp = CB::scale(&tmp, &self.params.abc[1]); // beta = 1
        y1 = CB::addition(&y1, &tmp);
        y2 = CB::addition(&y2, &tmp);

        (y1, y2)
    }

    fn external_sbox<CS: ConstraintSystem<E>>(
        &self,
        cs: &mut CS,
        input: &[ProofVar<E>],
    ) -> Vec<ProofVar<E>> {
        let t = self.params.t;
        let t_ = t >> 1;
        let mut output = Vec::with_capacity(t);
        for i in 0..t_ {
            let out = self.external_sbox_prime(cs, &input[2 * i], &input[2 * i + 1]);
            output.push(out.0);
            output.push(out.1);
        }
        output
    }

    fn internal_sbox<CS: ConstraintSystem<E>>(
        &self,
        cs: &mut CS,
        input: &[ProofVar<E>],
    ) -> Vec<ProofVar<E>> {
        let mut output = input.to_owned();
        output[0] = self.sbox_d(cs, &input[0]);
        output
    }

    fn external_matmul(&self, input: &[ProofVar<E>]) -> Vec<ProofVar<E>> {
        let t = self.params.t;
        let mut result: Vec<ProofVar<E>> = Vec::with_capacity(t);
        for row in self.params.m_e.iter() {
            let mut new_constraint = ProofVar::zero();
            for (con, vec) in input.iter().zip(row.iter()) {
                if *vec == E::Fr::zero() {
                    continue;
                }
                let tmp = CB::scale(con, vec);
                new_constraint = CB::addition(&new_constraint, &tmp);
            }
            result.push(new_constraint);
        }
        result
    }

    fn internal_matmul(&self, input: &[ProofVar<E>]) -> Vec<ProofVar<E>> {
        let mut result = input.to_owned();

        let mut sum = input[0].to_owned();
        input
            .iter()
            .skip(1)
            .for_each(|el| sum = CB::addition(&sum, el));

        for (r, mu) in result.iter_mut().zip(self.params.mu.iter()) {
            *r = CB::scale(r, mu);
            // *r = CB::sub_constant(r, &input[row]); // Already done in parameter creation
            *r = CB::addition(r, &sum);
        }

        result
    }

    fn permutation_no_assert<CS: ConstraintSystem<E>>(
        &self,
        cs: &mut CS,
        state: &[ProofVar<E>],
    ) -> Vec<ProofVar<E>> {
        let mut current_state = self.external_matmul(state);

        for r in 0..self.params.rounds_f_beginning {
            current_state = self.external_round(cs, &current_state, r);
        }
        let p_end = self.params.rounds_f_beginning + self.params.rounds_p;
        for r in self.params.rounds_f_beginning..p_end {
            current_state = self.internal_round(cs, &current_state, r);
        }
        for r in p_end..self.params.rounds {
            current_state = self.external_round(cs, &current_state, r);
        }

        current_state
    }
}

impl<E: Engine> PermCircuit<E> for NeptuneCircuit<E> {
    fn set_input(&mut self, preimage: &[E::Fr]) {
        debug_assert!(preimage.len() == self.params.t);
        self.input = preimage.iter().map(|el| Some(*el)).collect();
    }

    fn permutation<CS: ConstraintSystem<E>>(
        &self,
        cs: &mut CS,
        state: &[ProofVar<E>],
    ) -> Vec<ProofVar<E>> {
        let perm_out = self.permutation_no_assert(cs, state);
        // Assert variables to make constraints more sparse
        perm_out.iter().map(|p| CB::enforce_linear(p, cs)).collect()
    }

    fn get_t(&self) -> usize {
        self.params.t
    }
}

impl<E: Engine> Circuit<E> for NeptuneCircuit<E> {
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let t = self.params.t;
        debug_assert!(self.input.len() == t);
        let current_state: Vec<ProofVar<E>> = self
            .input
            .iter()
            .map(|el| CB::new_variable(*el, cs))
            .collect();

        let current_state = self.permutation_no_assert(cs, &current_state);

        for var in current_state {
            CB::enforce_final_linear(&var, cs);
        }

        Ok(())
    }
}
