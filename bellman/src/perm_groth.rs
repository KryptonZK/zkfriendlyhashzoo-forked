use bellman_ce::{
    groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
        Parameters, PreparedVerifyingKey, Proof,
    },
    pairing::Engine,
    Circuit, SynthesisError,
};
use rand::Rng;

use crate::circuits::PermCircuit;

#[derive(Clone)]
pub struct PermGroth<E: Engine, T: Circuit<E> + PermCircuit<E> + Clone> {
    params: Option<Parameters<E>>,
    circuit: T,
}

impl<E: Engine, T: Circuit<E> + PermCircuit<E> + Clone> PermGroth<E, T> {
    pub fn new(circuit: T) -> Self {
        PermGroth {
            params: None,
            circuit,
        }
    }

    pub fn create_crs<R: Rng>(&mut self, rng: &mut R) {
        self.params =
            Some(generate_random_parameters::<E, _, _>(self.circuit.clone(), rng).unwrap());
    }

    pub fn create_verify_key(&self) -> PreparedVerifyingKey<E> {
        prepare_verifying_key(&self.params.as_ref().unwrap().vk)
    }

    pub fn create_proof<R: Rng>(&mut self, preimage: &[E::Fr], rng: &mut R) -> Proof<E> {
        self.circuit.set_input(preimage);
        create_random_proof(self.circuit.clone(), self.params.as_ref().unwrap(), rng).unwrap()
    }

    pub fn verify_proof(
        pvk: &PreparedVerifyingKey<E>,
        proof: &Proof<E>,
        image: &[E::Fr],
    ) -> Result<bool, SynthesisError> {
        verify_proof(pvk, proof, image)
    }
}

#[cfg(test)]
mod rescue_proof_perm_tests {
    use std::sync::Arc;

    use bellman_ce::pairing::{bls12_381::Bls12, bn256::Bn256, ff::Field};
    use rand::thread_rng;

    use crate::{
        circuits::Permutation,
        rescue::{
            rescue::Rescue, rescue_circuit::RescueCircuit, rescue_instance_bls12::*,
            rescue_instance_bn256::*, rescue_params::RescueParams,
        },
        utils,
    };

    use super::*;

    static TESTRUNS: usize = 5;

    fn proof_verify<E: Engine>(params: &Arc<RescueParams<E::Fr>>) {
        let perm = Rescue::new(params);
        let circuit = RescueCircuit::<E>::new(params);
        let mut rng = thread_rng();
        let mut groth = PermGroth::<E, RescueCircuit<E>>::new(circuit);
        groth.create_crs(&mut rng);
        let pvk = groth.create_verify_key();

        let t = params.t;
        for _ in 0..TESTRUNS {
            let input: Vec<E::Fr> = (0..t)
                .map(|_| utils::random_scalar_rng(true, &mut rng))
                .collect();

            let perm = perm.permutation(&input);
            let mut false_perm = perm.to_owned();
            false_perm[0].add_assign(&E::Fr::one());
            let proof = groth.create_proof(&input, &mut rng);

            assert!(PermGroth::<E, RescueCircuit<E>>::verify_proof(&pvk, &proof, &perm).unwrap());
            assert!(
                !PermGroth::<E, RescueCircuit<E>>::verify_proof(&pvk, &proof, &false_perm).unwrap()
            );
        }
    }

    #[test]
    fn rescue_bls12_t3_proof_verify_test() {
        proof_verify::<Bls12>(&RESCUE_BLS_3_PARAMS);
    }

    #[test]
    fn rescue_bls12_t4_proof_verify_test() {
        proof_verify::<Bls12>(&RESCUE_BLS_4_PARAMS);
    }

    #[test]
    fn rescue_bls12_t5_proof_verify_test() {
        proof_verify::<Bls12>(&RESCUE_BLS_5_PARAMS);
    }

    #[test]
    fn rescue_bls12_t8_proof_verify_test() {
        proof_verify::<Bls12>(&RESCUE_BLS_8_PARAMS);
    }

    #[test]
    fn rescue_bls12_t9_proof_verify_test() {
        proof_verify::<Bls12>(&RESCUE_BLS_9_PARAMS);
    }

    #[test]
    fn rescue_bls12_t12_proof_verify_test() {
        proof_verify::<Bls12>(&RESCUE_BLS_12_PARAMS);
    }

    #[test]
    fn rescue_bn256_t3_proof_verify_test() {
        proof_verify::<Bn256>(&RESCUE_BN_3_PARAMS);
    }

    #[test]
    fn rescue_bn256_t4_proof_verify_test() {
        proof_verify::<Bn256>(&RESCUE_BN_4_PARAMS);
    }

    #[test]
    fn rescue_bn256_t5_proof_verify_test() {
        proof_verify::<Bn256>(&RESCUE_BN_5_PARAMS);
    }

    #[test]
    fn rescue_bn256_t8_proof_verify_test() {
        proof_verify::<Bn256>(&RESCUE_BN_8_PARAMS);
    }

    #[test]
    fn rescue_bn256_t9_proof_verify_test() {
        proof_verify::<Bn256>(&RESCUE_BN_9_PARAMS);
    }

    #[test]
    fn rescue_bn256_t12_proof_verify_test() {
        proof_verify::<Bn256>(&RESCUE_BN_12_PARAMS);
    }
}

#[cfg(test)]
mod poseidon_proof_perm_tests {
    use std::sync::Arc;

    use bellman_ce::pairing::{bls12_381::Bls12, bn256::Bn256, ff::Field};
    use rand::thread_rng;

    use crate::{
        circuits::Permutation,
        poseidon::{
            poseidon::Poseidon, poseidon_circuit::PoseidonCircuit, poseidon_instance_bls12::*,
            poseidon_instance_bn256::*, poseidon_params::PoseidonParams,
        },
        utils,
    };

    use super::*;

    static TESTRUNS: usize = 5;

    fn proof_verify<E: Engine>(params: &Arc<PoseidonParams<E::Fr>>) {
        let perm = Poseidon::new(params);
        let circuit = PoseidonCircuit::<E>::new(params);
        let mut rng = thread_rng();
        let mut groth = PermGroth::<E, PoseidonCircuit<E>>::new(circuit);
        groth.create_crs(&mut rng);
        let pvk = groth.create_verify_key();

        let t = params.t;
        for _ in 0..TESTRUNS {
            let input: Vec<E::Fr> = (0..t)
                .map(|_| utils::random_scalar_rng(true, &mut rng))
                .collect();

            let perm = perm.permutation(&input);
            let mut false_perm = perm.to_owned();
            false_perm[0].add_assign(&E::Fr::one());
            let proof = groth.create_proof(&input, &mut rng);

            assert!(PermGroth::<E, PoseidonCircuit<E>>::verify_proof(&pvk, &proof, &perm).unwrap());
            assert!(
                !PermGroth::<E, PoseidonCircuit<E>>::verify_proof(&pvk, &proof, &false_perm)
                    .unwrap()
            );
        }
    }

    #[test]
    fn poseidon_bls12_t3_proof_verify_test() {
        proof_verify::<Bls12>(&POSEIDON_BLS_3_PARAMS);
    }

    #[test]
    fn poseidon_bls12_t4_proof_verify_test() {
        proof_verify::<Bls12>(&POSEIDON_BLS_4_PARAMS);
    }

    #[test]
    fn poseidon_bls12_t5_proof_verify_test() {
        proof_verify::<Bls12>(&POSEIDON_BLS_5_PARAMS);
    }

    #[test]
    fn poseidon_bls12_t8_proof_verify_test() {
        proof_verify::<Bls12>(&POSEIDON_BLS_8_PARAMS);
    }

    #[test]
    fn poseidon_bls12_t9_proof_verify_test() {
        proof_verify::<Bls12>(&POSEIDON_BLS_9_PARAMS);
    }

    #[test]
    fn poseidon_bls12_t12_proof_verify_test() {
        proof_verify::<Bls12>(&POSEIDON_BLS_12_PARAMS);
    }

    #[test]
    fn poseidon_bn256_t3_proof_verify_test() {
        proof_verify::<Bn256>(&POSEIDON_BN_3_PARAMS);
    }

    #[test]
    fn poseidon_bn256_t4_proof_verify_test() {
        proof_verify::<Bn256>(&POSEIDON_BN_4_PARAMS);
    }

    #[test]
    fn poseidon_bn256_t5_proof_verify_test() {
        proof_verify::<Bn256>(&POSEIDON_BN_5_PARAMS);
    }

    #[test]
    fn poseidon_bn256_t8_proof_verify_test() {
        proof_verify::<Bn256>(&POSEIDON_BN_8_PARAMS);
    }

    #[test]
    fn poseidon_bn256_t9_proof_verify_test() {
        proof_verify::<Bn256>(&POSEIDON_BN_9_PARAMS);
    }

    #[test]
    fn poseidon_bn256_t12_proof_verify_test() {
        proof_verify::<Bn256>(&POSEIDON_BN_12_PARAMS);
    }
}

#[cfg(test)]
mod griffin_proof_perm_tests {
    use std::sync::Arc;

    use bellman_ce::pairing::{bls12_381::Bls12, bn256::Bn256, ff::Field};
    use rand::thread_rng;

    use crate::{
        circuits::Permutation,
        griffin::{
            griffin::Griffin, griffin_circuit::GriffinCircuit, griffin_instances::*,
            griffin_params::GriffinParams,
        },
        utils,
    };

    use super::*;

    static TESTRUNS: usize = 5;

    fn proof_verify<E: Engine>(params: &Arc<GriffinParams<E::Fr>>) {
        let perm = Griffin::new(params);
        let circuit = GriffinCircuit::<E>::new(params);
        let mut rng = thread_rng();
        let mut groth = PermGroth::<E, GriffinCircuit<E>>::new(circuit);
        groth.create_crs(&mut rng);
        let pvk = groth.create_verify_key();

        let t = params.t;
        for _ in 0..TESTRUNS {
            let input: Vec<E::Fr> = (0..t)
                .map(|_| utils::random_scalar_rng(true, &mut rng))
                .collect();

            let perm = perm.permutation(&input);
            let mut false_perm = perm.to_owned();
            false_perm[0].add_assign(&E::Fr::one());
            let proof = groth.create_proof(&input, &mut rng);

            assert!(PermGroth::<E, GriffinCircuit<E>>::verify_proof(&pvk, &proof, &perm).unwrap());
            assert!(
                !PermGroth::<E, GriffinCircuit<E>>::verify_proof(&pvk, &proof, &false_perm)
                    .unwrap()
            );
        }
    }

    #[test]
    fn griffin_bls12_t3_proof_verify_test() {
        proof_verify::<Bls12>(&GRIFFIN_BLS_3_PARAMS);
    }

    #[test]
    fn griffin_bls12_t4_proof_verify_test() {
        proof_verify::<Bls12>(&GRIFFIN_BLS_4_PARAMS);
    }

    #[test]
    fn griffin_bls12_t8_proof_verify_test() {
        proof_verify::<Bls12>(&GRIFFIN_BLS_8_PARAMS);
    }

    #[test]
    fn griffin_bls12_t12_proof_verify_test() {
        proof_verify::<Bls12>(&GRIFFIN_BLS_12_PARAMS);
    }

    #[test]
    fn griffin_bn256_t3_proof_verify_test() {
        proof_verify::<Bn256>(&GRIFFIN_BN_3_PARAMS);
    }

    #[test]
    fn griffin_bn256_t4_proof_verify_test() {
        proof_verify::<Bn256>(&GRIFFIN_BN_4_PARAMS);
    }

    #[test]
    fn griffin_bn256_t8_proof_verify_test() {
        proof_verify::<Bn256>(&GRIFFIN_BN_8_PARAMS);
    }

    #[test]
    fn griffin_bn256_t12_proof_verify_test() {
        proof_verify::<Bn256>(&GRIFFIN_BN_12_PARAMS);
    }
}

#[cfg(test)]
mod grendel_proof_perm_tests {
    use std::sync::Arc;

    use bellman_ce::pairing::{bls12_381::Bls12, bn256::Bn256, ff::Field};
    use rand::thread_rng;

    use crate::{
        circuits::Permutation,
        grendel::{
            grendel::Grendel, grendel_circuit::GrendelCircuit, grendel_instance_bls12::*,
            grendel_instance_bn256::*, grendel_params::GrendelParams,
        },
        utils,
    };

    use super::*;

    static TESTRUNS: usize = 5;

    fn proof_verify<E: Engine>(params: &Arc<GrendelParams<E::Fr>>) {
        let perm = Grendel::new(params);
        let circuit = GrendelCircuit::<E>::new(params);
        let mut rng = thread_rng();
        let mut groth = PermGroth::<E, GrendelCircuit<E>>::new(circuit);
        groth.create_crs(&mut rng);
        let pvk = groth.create_verify_key();

        let t = params.t;
        for _ in 0..TESTRUNS {
            let input: Vec<E::Fr> = (0..t)
                .map(|_| utils::random_scalar_rng(true, &mut rng))
                .collect();

            let perm = perm.permutation(&input);
            let mut false_perm = perm.to_owned();
            false_perm[0].add_assign(&E::Fr::one());
            let proof = groth.create_proof(&input, &mut rng);

            assert!(PermGroth::<E, GrendelCircuit<E>>::verify_proof(&pvk, &proof, &perm).unwrap());
            assert!(
                !PermGroth::<E, GrendelCircuit<E>>::verify_proof(&pvk, &proof, &false_perm)
                    .unwrap()
            );
        }
    }

    #[test]
    fn grendel_bls12_t3_proof_verify_test() {
        proof_verify::<Bls12>(&GRENDEL_BLS_3_PARAMS);
    }

    #[test]
    fn grendel_bls12_t4_proof_verify_test() {
        proof_verify::<Bls12>(&GRENDEL_BLS_4_PARAMS);
    }

    #[test]
    fn grendel_bls12_t5_proof_verify_test() {
        proof_verify::<Bls12>(&GRENDEL_BLS_5_PARAMS);
    }

    #[test]
    fn grendel_bls12_t8_proof_verify_test() {
        proof_verify::<Bls12>(&GRENDEL_BLS_8_PARAMS);
    }

    #[test]
    fn grendel_bls12_t9_proof_verify_test() {
        proof_verify::<Bls12>(&GRENDEL_BLS_9_PARAMS);
    }

    #[test]
    fn grendel_bls12_t12_proof_verify_test() {
        proof_verify::<Bls12>(&GRENDEL_BLS_12_PARAMS);
    }

    #[test]
    fn grendel_bn256_t3_proof_verify_test() {
        proof_verify::<Bn256>(&GRENDEL_BN_3_PARAMS);
    }

    #[test]
    fn grendel_bn256_t4_proof_verify_test() {
        proof_verify::<Bn256>(&GRENDEL_BN_4_PARAMS);
    }

    #[test]
    fn grendel_bn256_t5_proof_verify_test() {
        proof_verify::<Bn256>(&GRENDEL_BN_5_PARAMS);
    }

    #[test]
    fn grendel_bn256_t8_proof_verify_test() {
        proof_verify::<Bn256>(&GRENDEL_BN_8_PARAMS);
    }

    #[test]
    fn grendel_bn256_t9_proof_verify_test() {
        proof_verify::<Bn256>(&GRENDEL_BN_9_PARAMS);
    }

    #[test]
    fn grendel_bn256_t12_proof_verify_test() {
        proof_verify::<Bn256>(&GRENDEL_BN_12_PARAMS);
    }
}

#[cfg(test)]
mod gmimc_proof_perm_tests {
    use std::sync::Arc;

    use bellman_ce::pairing::{bls12_381::Bls12, bn256::Bn256, ff::Field};
    use rand::thread_rng;

    use crate::{
        circuits::Permutation,
        gmimc::{
            gmimc::Gmimc, gmimc_circuit::GmimcCircuit, gmimc_instance_bls12::*,
            gmimc_instance_bn256::*, gmimc_params::GmimcParams,
        },
        utils,
    };

    use super::*;

    static TESTRUNS: usize = 5;

    fn proof_verify<E: Engine>(params: &Arc<GmimcParams<E::Fr>>) {
        let perm = Gmimc::new(params);
        let circuit = GmimcCircuit::<E>::new(params);
        let mut rng = thread_rng();
        let mut groth = PermGroth::<E, GmimcCircuit<E>>::new(circuit);
        groth.create_crs(&mut rng);
        let pvk = groth.create_verify_key();

        let t = params.t;
        for _ in 0..TESTRUNS {
            let input: Vec<E::Fr> = (0..t)
                .map(|_| utils::random_scalar_rng(true, &mut rng))
                .collect();

            let perm = perm.permutation(&input);
            let mut false_perm = perm.to_owned();
            false_perm[0].add_assign(&E::Fr::one());
            let proof = groth.create_proof(&input, &mut rng);

            assert!(PermGroth::<E, GmimcCircuit<E>>::verify_proof(&pvk, &proof, &perm).unwrap());
            assert!(
                !PermGroth::<E, GmimcCircuit<E>>::verify_proof(&pvk, &proof, &false_perm).unwrap()
            );
        }
    }

    #[test]
    fn gmimc_bls12_t3_proof_verify_test() {
        proof_verify::<Bls12>(&GMIMC_BLS_3_PARAMS);
    }

    #[test]
    fn gmimc_bls12_t4_proof_verify_test() {
        proof_verify::<Bls12>(&GMIMC_BLS_4_PARAMS);
    }

    #[test]
    fn gmimc_bls12_t5_proof_verify_test() {
        proof_verify::<Bls12>(&GMIMC_BLS_5_PARAMS);
    }

    #[test]
    fn gmimc_bls12_t8_proof_verify_test() {
        proof_verify::<Bls12>(&GMIMC_BLS_8_PARAMS);
    }

    #[test]
    fn gmimc_bls12_t9_proof_verify_test() {
        proof_verify::<Bls12>(&GMIMC_BLS_9_PARAMS);
    }

    #[test]
    fn gmimc_bls12_t12_proof_verify_test() {
        proof_verify::<Bls12>(&GMIMC_BLS_12_PARAMS);
    }

    #[test]
    fn gmimc_bn256_t3_proof_verify_test() {
        proof_verify::<Bn256>(&GMIMC_BN_3_PARAMS);
    }

    #[test]
    fn gmimc_bn256_t4_proof_verify_test() {
        proof_verify::<Bn256>(&GMIMC_BN_4_PARAMS);
    }

    #[test]
    fn gmimc_bn256_t5_proof_verify_test() {
        proof_verify::<Bn256>(&GMIMC_BN_5_PARAMS);
    }

    #[test]
    fn gmimc_bn256_t8_proof_verify_test() {
        proof_verify::<Bn256>(&GMIMC_BN_8_PARAMS);
    }

    #[test]
    fn gmimc_bn256_t9_proof_verify_test() {
        proof_verify::<Bn256>(&GMIMC_BN_9_PARAMS);
    }

    #[test]
    fn gmimc_bn256_t12_proof_verify_test() {
        proof_verify::<Bn256>(&GMIMC_BN_12_PARAMS);
    }
}

#[cfg(test)]
mod neptune_proof_perm_tests {
    use std::sync::Arc;

    use bellman_ce::pairing::{bls12_381::Bls12, bn256::Bn256, ff::Field};
    use rand::thread_rng;

    use crate::{
        circuits::Permutation,
        neptune::{
            neptune::Neptune, neptune_circuit::NeptuneCircuit, neptune_instances::*,
            neptune_params::NeptuneParams,
        },
        utils,
    };

    use super::*;

    static TESTRUNS: usize = 5;

    fn proof_verify<E: Engine>(params: &Arc<NeptuneParams<E::Fr>>) {
        let perm = Neptune::new(params);
        let circuit = NeptuneCircuit::<E>::new(params);
        let mut rng = thread_rng();
        let mut groth = PermGroth::<E, NeptuneCircuit<E>>::new(circuit);
        groth.create_crs(&mut rng);
        let pvk = groth.create_verify_key();

        let t = params.t;
        for _ in 0..TESTRUNS {
            let input: Vec<E::Fr> = (0..t)
                .map(|_| utils::random_scalar_rng(true, &mut rng))
                .collect();

            let perm = perm.permutation(&input);
            let mut false_perm = perm.to_owned();
            false_perm[0].add_assign(&E::Fr::one());
            let proof = groth.create_proof(&input, &mut rng);

            assert!(PermGroth::<E, NeptuneCircuit<E>>::verify_proof(&pvk, &proof, &perm).unwrap());
            assert!(
                !PermGroth::<E, NeptuneCircuit<E>>::verify_proof(&pvk, &proof, &false_perm)
                    .unwrap()
            );
        }
    }

    #[test]
    fn neptune_bls12_t4_proof_verify_test() {
        proof_verify::<Bls12>(&NEPTUNE_BLS_4_PARAMS);
    }

    #[test]
    fn neptune_bls12_t8_proof_verify_test() {
        proof_verify::<Bls12>(&NEPTUNE_BLS_8_PARAMS);
    }

    #[test]
    fn neptune_bls12_t12_proof_verify_test() {
        proof_verify::<Bls12>(&NEPTUNE_BLS_12_PARAMS);
    }

    #[test]
    fn neptune_bn256_t4_proof_verify_test() {
        proof_verify::<Bn256>(&NEPTUNE_BN_4_PARAMS);
    }

    #[test]
    fn neptune_bn256_t8_proof_verify_test() {
        proof_verify::<Bn256>(&NEPTUNE_BN_8_PARAMS);
    }

    #[test]
    fn neptune_bn256_t12_proof_verify_test() {
        proof_verify::<Bn256>(&NEPTUNE_BN_12_PARAMS);
    }
}
