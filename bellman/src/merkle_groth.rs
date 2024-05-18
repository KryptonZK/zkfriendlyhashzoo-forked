use bellman_ce::pairing::Engine;
use bellman_ce::{
    groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
        Parameters, PreparedVerifyingKey, Proof,
    },
    Circuit, SynthesisError,
};

use rand::Rng;

use crate::circuits::PermCircuit;
use crate::merkle_tree::ProofNode;
use crate::mt_circuit::MerkleTreeCircuit;

#[derive(Clone)]
pub struct MerkleGroth<E: Engine, T: Circuit<E> + PermCircuit<E> + Clone> {
    params: Option<Parameters<E>>,
    circuit: MerkleTreeCircuit<E, T>,
}

impl<E: Engine, T: Circuit<E> + PermCircuit<E> + Clone> MerkleGroth<E, T> {
    pub fn new(circuit: MerkleTreeCircuit<E, T>) -> Self {
        MerkleGroth {
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

    pub fn create_proof<R: Rng>(
        &mut self,
        input: &E::Fr,
        mt_witness: &[ProofNode<E::Fr>],
        rng: &mut R,
    ) -> Proof<E> {
        self.circuit.set_input(input, mt_witness);
        create_random_proof(self.circuit.clone(), self.params.as_ref().unwrap(), rng).unwrap()
    }

    pub fn verify_proof(
        pvk: &PreparedVerifyingKey<E>,
        proof: &Proof<E>,
        image: &E::Fr,
    ) -> Result<bool, SynthesisError> {
        verify_proof(pvk, proof, &[*image])
    }
}

#[cfg(test)]
mod rescue_proof_mt_tests {
    use std::sync::Arc;

    use bellman_ce::pairing::{bls12_381::Bls12, bn256::Bn256};
    use rand::{
        distributions::{IndependentSample, Range},
        thread_rng,
    };

    use crate::{
        merkle_tree::MerkleTree,
        rescue::{
            rescue::Rescue, rescue_circuit::RescueCircuit, rescue_instance_bls12::*,
            rescue_instance_bn256::*, rescue_params::RescueParams,
        },
        utils,
    };

    use super::*;

    static TESTRUNS: usize = 5;
    static LOG_SET_SIZE: usize = 10;

    fn proof_verify<E: Engine>(params: &Arc<RescueParams<E::Fr>>) {
        let perm = Rescue::new(params);
        let mut mt = MerkleTree::new(perm);
        let set_size = 1 << LOG_SET_SIZE;
        let arity = mt.get_arity();
        let levels = f64::ceil((set_size as f64).log(arity as f64)) as usize;
        let mut rng = thread_rng();
        let dist: Range<usize> = Range::new(0, set_size);

        let perm_circ = RescueCircuit::new(params);
        let mt_circ = MerkleTreeCircuit::new(perm_circ, levels, arity);
        let mut groth = MerkleGroth::<E, RescueCircuit<E>>::new(mt_circ);
        groth.create_crs(&mut rng);
        let pvk = groth.create_verify_key();

        let t = params.t;
        assert!(t > arity);
        assert_eq!(arity & (arity - 1), 0); // power of 2

        for _ in 0..TESTRUNS {
            let set: Vec<E::Fr> = (0..set_size)
                .map(|_| utils::random_scalar_rng(true, &mut rng))
                .collect();

            let index = dist.ind_sample(&mut rng);
            mt.accumulate(&set);
            let wit = mt.create_witness(&set[index]).unwrap();
            assert_eq!(levels, wit.len());
            let proof = groth.create_proof(&set[index], &wit, &mut rng);
            assert!(MerkleGroth::<E, RescueCircuit<E>>::verify_proof(
                &pvk,
                &proof,
                &mt.get_root().unwrap()
            )
            .unwrap());
            assert!(
                !MerkleGroth::<E, RescueCircuit<E>>::verify_proof(&pvk, &proof, &set[0]).unwrap()
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
mod poseidon_proof_mt_tests {
    use std::sync::Arc;

    use bellman_ce::pairing::{bls12_381::Bls12, bn256::Bn256};
    use rand::{
        distributions::{IndependentSample, Range},
        thread_rng,
    };

    use crate::{
        merkle_tree::MerkleTree,
        poseidon::{
            poseidon::Poseidon, poseidon_circuit::PoseidonCircuit, poseidon_instance_bls12::*,
            poseidon_instance_bn256::*, poseidon_params::PoseidonParams,
        },
        utils,
    };

    use super::*;

    static TESTRUNS: usize = 5;
    static LOG_SET_SIZE: usize = 10;

    fn proof_verify<E: Engine>(params: &Arc<PoseidonParams<E::Fr>>) {
        let perm = Poseidon::new(params);
        let mut mt = MerkleTree::new(perm);
        let set_size = 1 << LOG_SET_SIZE;
        let arity = mt.get_arity();
        let levels = f64::ceil((set_size as f64).log(arity as f64)) as usize;
        let mut rng = thread_rng();
        let dist: Range<usize> = Range::new(0, set_size);

        let perm_circ = PoseidonCircuit::new(params);
        let mt_circ = MerkleTreeCircuit::new(perm_circ, levels, arity);
        let mut groth = MerkleGroth::<E, PoseidonCircuit<E>>::new(mt_circ);
        groth.create_crs(&mut rng);
        let pvk = groth.create_verify_key();

        let t = params.t;
        assert!(t > arity);
        assert_eq!(arity & (arity - 1), 0); // power of 2

        for _ in 0..TESTRUNS {
            let set: Vec<E::Fr> = (0..set_size)
                .map(|_| utils::random_scalar_rng(true, &mut rng))
                .collect();

            let index = dist.ind_sample(&mut rng);
            mt.accumulate(&set);
            let wit = mt.create_witness(&set[index]).unwrap();
            assert_eq!(levels, wit.len());
            let proof = groth.create_proof(&set[index], &wit, &mut rng);
            assert!(MerkleGroth::<E, PoseidonCircuit<E>>::verify_proof(
                &pvk,
                &proof,
                &mt.get_root().unwrap()
            )
            .unwrap());
            assert!(
                !MerkleGroth::<E, PoseidonCircuit<E>>::verify_proof(&pvk, &proof, &set[0]).unwrap()
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
mod griffin_proof_mt_tests {
    use std::sync::Arc;

    use bellman_ce::pairing::{bls12_381::Bls12, bn256::Bn256};
    use rand::{
        distributions::{IndependentSample, Range},
        thread_rng,
    };

    use crate::{
        griffin::{
            griffin::Griffin, griffin_circuit::GriffinCircuit, griffin_instances::*,
            griffin_params::GriffinParams,
        },
        merkle_tree::MerkleTree,
        utils,
    };

    use super::*;

    static TESTRUNS: usize = 5;
    static LOG_SET_SIZE: usize = 10;

    fn proof_verify<E: Engine>(params: &Arc<GriffinParams<E::Fr>>) {
        let perm = Griffin::new(params);
        let mut mt = MerkleTree::new(perm);
        let set_size = 1 << LOG_SET_SIZE;
        let arity = mt.get_arity();
        let levels = f64::ceil((set_size as f64).log(arity as f64)) as usize;
        let mut rng = thread_rng();
        let dist: Range<usize> = Range::new(0, set_size);

        let perm_circ = GriffinCircuit::new(params);
        let mt_circ = MerkleTreeCircuit::new(perm_circ, levels, arity);
        let mut groth = MerkleGroth::<E, GriffinCircuit<E>>::new(mt_circ);
        groth.create_crs(&mut rng);
        let pvk = groth.create_verify_key();

        let t = params.t;
        assert!(t > arity);
        assert_eq!(arity & (arity - 1), 0); // power of 2

        for _ in 0..TESTRUNS {
            let set: Vec<E::Fr> = (0..set_size)
                .map(|_| utils::random_scalar_rng(true, &mut rng))
                .collect();

            let index = dist.ind_sample(&mut rng);
            mt.accumulate(&set);
            let wit = mt.create_witness(&set[index]).unwrap();
            assert_eq!(levels, wit.len());
            let proof = groth.create_proof(&set[index], &wit, &mut rng);
            assert!(MerkleGroth::<E, GriffinCircuit<E>>::verify_proof(
                &pvk,
                &proof,
                &mt.get_root().unwrap()
            )
            .unwrap());
            assert!(
                !MerkleGroth::<E, GriffinCircuit<E>>::verify_proof(&pvk, &proof, &set[0]).unwrap()
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
mod grendel_proof_mt_tests {
    use std::sync::Arc;

    use bellman_ce::pairing::{bls12_381::Bls12, bn256::Bn256};
    use rand::{
        distributions::{IndependentSample, Range},
        thread_rng,
    };

    use crate::{
        grendel::{
            grendel::Grendel, grendel_circuit::GrendelCircuit, grendel_instance_bls12::*,
            grendel_instance_bn256::*, grendel_params::GrendelParams,
        },
        merkle_tree::MerkleTree,
        utils,
    };

    use super::*;

    static TESTRUNS: usize = 5;
    static LOG_SET_SIZE: usize = 10;

    fn proof_verify<E: Engine>(params: &Arc<GrendelParams<E::Fr>>) {
        let perm = Grendel::new(params);
        let mut mt = MerkleTree::new(perm);
        let set_size = 1 << LOG_SET_SIZE;
        let arity = mt.get_arity();
        let levels = f64::ceil((set_size as f64).log(arity as f64)) as usize;
        let mut rng = thread_rng();
        let dist: Range<usize> = Range::new(0, set_size);

        let perm_circ = GrendelCircuit::new(params);
        let mt_circ = MerkleTreeCircuit::new(perm_circ, levels, arity);
        let mut groth = MerkleGroth::<E, GrendelCircuit<E>>::new(mt_circ);
        groth.create_crs(&mut rng);
        let pvk = groth.create_verify_key();

        let t = params.t;
        assert!(t > arity);
        assert_eq!(arity & (arity - 1), 0); // power of 2

        for _ in 0..TESTRUNS {
            let set: Vec<E::Fr> = (0..set_size)
                .map(|_| utils::random_scalar_rng(true, &mut rng))
                .collect();

            let index = dist.ind_sample(&mut rng);
            mt.accumulate(&set);
            let wit = mt.create_witness(&set[index]).unwrap();
            assert_eq!(levels, wit.len());
            let proof = groth.create_proof(&set[index], &wit, &mut rng);
            assert!(MerkleGroth::<E, GrendelCircuit<E>>::verify_proof(
                &pvk,
                &proof,
                &mt.get_root().unwrap()
            )
            .unwrap());
            assert!(
                !MerkleGroth::<E, GrendelCircuit<E>>::verify_proof(&pvk, &proof, &set[0]).unwrap()
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
mod gmimc_proof_mt_tests {
    use std::sync::Arc;

    use bellman_ce::pairing::{bls12_381::Bls12, bn256::Bn256};
    use rand::{
        distributions::{IndependentSample, Range},
        thread_rng,
    };

    use crate::{
        gmimc::{
            gmimc::Gmimc, gmimc_circuit::GmimcCircuit, gmimc_instance_bls12::*,
            gmimc_instance_bn256::*, gmimc_params::GmimcParams,
        },
        merkle_tree::MerkleTree,
        utils,
    };

    use super::*;

    static TESTRUNS: usize = 5;
    static LOG_SET_SIZE: usize = 10;

    fn proof_verify<E: Engine>(params: &Arc<GmimcParams<E::Fr>>) {
        let perm = Gmimc::new(params);
        let mut mt = MerkleTree::new(perm);
        let set_size = 1 << LOG_SET_SIZE;
        let arity = mt.get_arity();
        let levels = f64::ceil((set_size as f64).log(arity as f64)) as usize;
        let mut rng = thread_rng();
        let dist: Range<usize> = Range::new(0, set_size);

        let perm_circ = GmimcCircuit::new(params);
        let mt_circ = MerkleTreeCircuit::new(perm_circ, levels, arity);
        let mut groth = MerkleGroth::<E, GmimcCircuit<E>>::new(mt_circ);
        groth.create_crs(&mut rng);
        let pvk = groth.create_verify_key();

        let t = params.t;
        assert!(t > arity);
        assert_eq!(arity & (arity - 1), 0); // power of 2

        for _ in 0..TESTRUNS {
            let set: Vec<E::Fr> = (0..set_size)
                .map(|_| utils::random_scalar_rng(true, &mut rng))
                .collect();

            let index = dist.ind_sample(&mut rng);
            mt.accumulate(&set);
            let wit = mt.create_witness(&set[index]).unwrap();
            assert_eq!(levels, wit.len());
            let proof = groth.create_proof(&set[index], &wit, &mut rng);
            assert!(MerkleGroth::<E, GmimcCircuit<E>>::verify_proof(
                &pvk,
                &proof,
                &mt.get_root().unwrap()
            )
            .unwrap());
            assert!(
                !MerkleGroth::<E, GmimcCircuit<E>>::verify_proof(&pvk, &proof, &set[0]).unwrap()
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
mod neptune_proof_mt_tests {
    use std::sync::Arc;

    use bellman_ce::pairing::{bls12_381::Bls12, bn256::Bn256};
    use rand::{
        distributions::{IndependentSample, Range},
        thread_rng,
    };

    use crate::{
        merkle_tree::MerkleTree,
        neptune::{
            neptune::Neptune, neptune_circuit::NeptuneCircuit, neptune_instances::*,
            neptune_params::NeptuneParams,
        },
        utils,
    };

    use super::*;

    static TESTRUNS: usize = 5;
    static LOG_SET_SIZE: usize = 10;

    fn proof_verify<E: Engine>(params: &Arc<NeptuneParams<E::Fr>>) {
        let perm = Neptune::new(params);
        let mut mt = MerkleTree::new(perm);
        let set_size = 1 << LOG_SET_SIZE;
        let arity = mt.get_arity();
        let levels = f64::ceil((set_size as f64).log(arity as f64)) as usize;
        let mut rng = thread_rng();
        let dist: Range<usize> = Range::new(0, set_size);

        let perm_circ = NeptuneCircuit::new(params);
        let mt_circ = MerkleTreeCircuit::new(perm_circ, levels, arity);
        let mut groth = MerkleGroth::<E, NeptuneCircuit<E>>::new(mt_circ);
        groth.create_crs(&mut rng);
        let pvk = groth.create_verify_key();

        let t = params.t;
        assert!(t > arity);
        assert_eq!(arity & (arity - 1), 0); // power of 2

        for _ in 0..TESTRUNS {
            let set: Vec<E::Fr> = (0..set_size)
                .map(|_| utils::random_scalar_rng(true, &mut rng))
                .collect();

            let index = dist.ind_sample(&mut rng);
            mt.accumulate(&set);
            let wit = mt.create_witness(&set[index]).unwrap();
            assert_eq!(levels, wit.len());
            let proof = groth.create_proof(&set[index], &wit, &mut rng);
            assert!(MerkleGroth::<E, NeptuneCircuit<E>>::verify_proof(
                &pvk,
                &proof,
                &mt.get_root().unwrap()
            )
            .unwrap());
            assert!(
                !MerkleGroth::<E, NeptuneCircuit<E>>::verify_proof(&pvk, &proof, &set[0]).unwrap()
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
