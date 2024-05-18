use bellman_ce::{
    pairing::{ff::PrimeField, Engine},
    ConstraintSystem,
};

use crate::constraint_builder::ProofVar;

pub trait PermCircuit<E: Engine> {
    fn set_input(&mut self, preimage: &[E::Fr]);
    fn permutation<CS: ConstraintSystem<E>>(
        &self,
        cs: &mut CS,
        state: &[ProofVar<E>],
    ) -> Vec<ProofVar<E>>;
    fn get_t(&self) -> usize;
}

pub trait Permutation<S: PrimeField> {
    fn permutation(&self, input: &[S]) -> Vec<S>;
    fn get_t(&self) -> usize;
}
