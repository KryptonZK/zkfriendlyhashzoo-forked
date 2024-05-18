use bellman_ce::{
    pairing::{ff::Field, Engine},
    Circuit, ConstraintSystem, SynthesisError,
};

use crate::{
    circuits::PermCircuit,
    constraint_builder::{ConstraintBuilder, ProofVar},
    merkle_tree::ProofNode,
};

type CB = ConstraintBuilder;

#[derive(Clone, Debug)]
pub struct MerkleTreeCircuit<E: Engine, P: PermCircuit<E>> {
    perm_circuit: P,
    input: Option<E::Fr>,
    mt_witness: Vec<Option<ProofNode<E::Fr>>>,
    mt_levels: usize,
    mt_arity: usize,
}

impl<E: Engine, P: PermCircuit<E>> MerkleTreeCircuit<E, P> {
    pub fn new(perm_circuit: P, mt_levels: usize, mt_arity: usize) -> Self {
        let mt_witness = vec![None; mt_levels];

        MerkleTreeCircuit {
            perm_circuit,
            input: None,
            mt_witness,
            mt_levels,
            mt_arity,
        }
    }

    pub fn set_input(&mut self, input: &E::Fr, mt_witness: &[ProofNode<E::Fr>]) {
        self.input = Some(*input);
        self.mt_witness = mt_witness.iter().map(|el| Some(el.to_owned())).collect();
    }
}

impl<E: Engine, P: PermCircuit<E>> Circuit<E> for MerkleTreeCircuit<E, P> {
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let t = self.perm_circuit.get_t();
        debug_assert!(self.mt_witness.len() == self.mt_levels);

        let digest = CB::new_variable(self.input, cs);
        let mut current_state: Vec<ProofVar<E>> = vec![digest; t];

        for wit in self.mt_witness {
            // keep previous digest at index 0
            // add witnesses
            let mut position: Option<usize> = None;
            match wit {
                Some(node) => {
                    debug_assert!(node.digests.len() == self.mt_arity - 1);
                    position = Some(node.position);
                    for (i, val) in node.digests.iter().enumerate() {
                        current_state[i + 1] = CB::new_variable(Some(*val), cs);
                    }
                }
                None => {
                    for s in current_state.iter_mut().take(self.mt_arity).skip(1) {
                        *s = CB::new_variable(None, cs);
                    }
                }
            }

            // permute input
            match self.mt_arity {
                2 => CB::mt_perm2(&mut current_state, &position, cs)?,
                4 => CB::mt_perm4(&mut current_state, &position, cs)?,
                8 => CB::mt_perm8(&mut current_state, &position, cs)?,
                _ => panic!("Not implemented!"),
            }

            // Add capacity
            for s in current_state.iter_mut().take(t).skip(self.mt_arity) {
                *s = CB::new_variable(Some(E::Fr::zero()), cs);
            }

            current_state = self.perm_circuit.permutation(cs, &current_state);
        }
        CB::enforce_final_linear(&current_state[0], cs);

        Ok(())
    }
}
