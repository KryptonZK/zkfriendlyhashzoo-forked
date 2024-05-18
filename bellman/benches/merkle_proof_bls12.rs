use std::sync::Arc;

use bellman_ce::pairing::{bls12_381::Bls12, Engine};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hash_r1cs::{
    circuits::Permutation,
    gmimc::{
        gmimc::Gmimc, gmimc_circuit::GmimcCircuit, gmimc_instance_bls12::*,
        gmimc_params::GmimcParams,
    },
    grendel::{
        grendel::Grendel, grendel_circuit::GrendelCircuit, grendel_instance_bls12::*,
        grendel_params::GrendelParams,
    },
    griffin::{
        griffin::Griffin, griffin_circuit::GriffinCircuit, griffin_instances::*,
        griffin_params::GriffinParams,
    },
    merkle_groth::MerkleGroth,
    merkle_tree::MerkleTree,
    mt_circuit::MerkleTreeCircuit,
    neptune::{
        neptune::Neptune, neptune_circuit::NeptuneCircuit, neptune_instances::*,
        neptune_params::NeptuneParams,
    },
    poseidon::{
        poseidon::Poseidon, poseidon_circuit::PoseidonCircuit, poseidon_instance_bls12::*,
        poseidon_params::PoseidonParams,
    },
    rescue::{
        rescue::Rescue, rescue_circuit::RescueCircuit, rescue_instance_bls12::*,
        rescue_params::RescueParams,
    },
    utils,
};
use rand::{
    distributions::{IndependentSample, Range},
    thread_rng,
};

fn sample_set<E: Engine>(set_size: usize) -> Vec<E::Fr> {
    // (0..set_size).map(|_| utils::random_scalar(true)).collect()
    (0..set_size).map(|i| utils::from_u64(i as u64)).collect()
}

fn rescue_proof_verify<E: Engine>(
    c: &mut Criterion,
    params: &Arc<RescueParams<E::Fr>>,
    log_set_size: usize,
) {
    let perm = Rescue::new(params);
    let t = perm.get_t();
    let mut mt = MerkleTree::new(perm);
    let set_size = 1 << log_set_size;
    let arity = mt.get_arity();
    let levels = f64::ceil((set_size as f64).log(arity as f64)) as usize;
    let mut rng = thread_rng();
    let dist: Range<usize> = Range::new(0, set_size);
    let perm_circ = RescueCircuit::new(params);
    let mt_circ = MerkleTreeCircuit::new(perm_circ, levels, arity);

    let mut groth = MerkleGroth::<E, RescueCircuit<E>>::new(mt_circ);
    groth.create_crs(&mut rng);
    let pvk = groth.create_verify_key();

    let set: Vec<E::Fr> = sample_set::<E>(set_size);

    let index = dist.ind_sample(&mut rng);
    mt.accumulate(&set);
    let wit = mt.create_witness(&set[index]).unwrap();
    let root = mt.get_root().unwrap();

    let proof = groth.create_proof(&set[index], &wit, &mut rng);

    let id = format!(
        "Rescue MT BLS proof (t = {}, set_size = 2^{})",
        t, log_set_size
    );
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let proof =
                groth.create_proof(black_box(&set[index]), black_box(&wit), black_box(&mut rng));
            black_box(proof)
        });
    });

    let id = format!(
        "Rescue MT BLS verify (t = {}, set_size = 2^{})",
        t, log_set_size
    );
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let res = MerkleGroth::<E, RescueCircuit<E>>::verify_proof(
                black_box(&pvk),
                black_box(&proof),
                black_box(&root),
            );
            black_box(res)
        });
    });
}

fn poseidon_proof_verify<E: Engine>(
    c: &mut Criterion,
    params: &Arc<PoseidonParams<E::Fr>>,
    log_set_size: usize,
) {
    let perm = Poseidon::new(params);
    let t = perm.get_t();
    let mut mt = MerkleTree::new(perm);
    let set_size = 1 << log_set_size;
    let arity = mt.get_arity();
    let levels = f64::ceil((set_size as f64).log(arity as f64)) as usize;
    let mut rng = thread_rng();
    let dist: Range<usize> = Range::new(0, set_size);
    let perm_circ = PoseidonCircuit::new(params);
    let mt_circ = MerkleTreeCircuit::new(perm_circ, levels, arity);

    let mut groth = MerkleGroth::<E, PoseidonCircuit<E>>::new(mt_circ);
    groth.create_crs(&mut rng);
    let pvk = groth.create_verify_key();

    let set: Vec<E::Fr> = sample_set::<E>(set_size);

    let index = dist.ind_sample(&mut rng);
    mt.accumulate(&set);
    let wit = mt.create_witness(&set[index]).unwrap();
    let root = mt.get_root().unwrap();

    let proof = groth.create_proof(&set[index], &wit, &mut rng);

    let id = format!(
        "Poseidon MT BLS proof (t = {}, set_size = 2^{})",
        t, log_set_size
    );
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let proof =
                groth.create_proof(black_box(&set[index]), black_box(&wit), black_box(&mut rng));
            black_box(proof)
        });
    });

    let id = format!(
        "Poseidon MT BLS verify (t = {}, set_size = 2^{})",
        t, log_set_size
    );
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let res = MerkleGroth::<E, PoseidonCircuit<E>>::verify_proof(
                black_box(&pvk),
                black_box(&proof),
                black_box(&root),
            );
            black_box(res)
        });
    });
}

fn griffin_proof_verify<E: Engine>(
    c: &mut Criterion,
    params: &Arc<GriffinParams<E::Fr>>,
    log_set_size: usize,
) {
    let perm = Griffin::new(params);
    let t = perm.get_t();
    let mut mt = MerkleTree::new(perm);
    let set_size = 1 << log_set_size;
    let arity = mt.get_arity();
    let levels = f64::ceil((set_size as f64).log(arity as f64)) as usize;
    let mut rng = thread_rng();
    let dist: Range<usize> = Range::new(0, set_size);
    let perm_circ = GriffinCircuit::new(params);
    let mt_circ = MerkleTreeCircuit::new(perm_circ, levels, arity);

    let mut groth = MerkleGroth::<E, GriffinCircuit<E>>::new(mt_circ);
    groth.create_crs(&mut rng);
    let pvk = groth.create_verify_key();

    let set: Vec<E::Fr> = sample_set::<E>(set_size);

    let index = dist.ind_sample(&mut rng);
    mt.accumulate(&set);
    let wit = mt.create_witness(&set[index]).unwrap();
    let root = mt.get_root().unwrap();

    let proof = groth.create_proof(&set[index], &wit, &mut rng);

    let id = format!(
        "Griffin MT BLS proof (t = {}, set_size = 2^{})",
        t, log_set_size
    );
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let proof =
                groth.create_proof(black_box(&set[index]), black_box(&wit), black_box(&mut rng));
            black_box(proof)
        });
    });

    let id = format!(
        "Griffin MT BLS verify (t = {}, set_size = 2^{})",
        t, log_set_size
    );
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let res = MerkleGroth::<E, GriffinCircuit<E>>::verify_proof(
                black_box(&pvk),
                black_box(&proof),
                black_box(&root),
            );
            black_box(res)
        });
    });
}
fn grendel_proof_verify<E: Engine>(
    c: &mut Criterion,
    params: &Arc<GrendelParams<E::Fr>>,
    log_set_size: usize,
) {
    let perm = Grendel::new(params);
    let t = perm.get_t();
    let mut mt = MerkleTree::new(perm);
    let set_size = 1 << log_set_size;
    let arity = mt.get_arity();
    let levels = f64::ceil((set_size as f64).log(arity as f64)) as usize;
    let mut rng = thread_rng();
    let dist: Range<usize> = Range::new(0, set_size);
    let perm_circ = GrendelCircuit::new(params);
    let mt_circ = MerkleTreeCircuit::new(perm_circ, levels, arity);

    let mut groth = MerkleGroth::<E, GrendelCircuit<E>>::new(mt_circ);
    groth.create_crs(&mut rng);
    let pvk = groth.create_verify_key();

    let set: Vec<E::Fr> = sample_set::<E>(set_size);

    let index = dist.ind_sample(&mut rng);
    mt.accumulate(&set);
    let wit = mt.create_witness(&set[index]).unwrap();
    let root = mt.get_root().unwrap();

    let proof = groth.create_proof(&set[index], &wit, &mut rng);

    let id = format!(
        "Grendel MT BLS proof (t = {}, set_size = 2^{})",
        t, log_set_size
    );
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let proof =
                groth.create_proof(black_box(&set[index]), black_box(&wit), black_box(&mut rng));
            black_box(proof)
        });
    });

    let id = format!(
        "Grendel MT BLS verify (t = {}, set_size = 2^{})",
        t, log_set_size
    );
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let res = MerkleGroth::<E, GrendelCircuit<E>>::verify_proof(
                black_box(&pvk),
                black_box(&proof),
                black_box(&root),
            );
            black_box(res)
        });
    });
}

fn gmimc_proof_verify<E: Engine>(
    c: &mut Criterion,
    params: &Arc<GmimcParams<E::Fr>>,
    log_set_size: usize,
) {
    let perm = Gmimc::new(params);
    let t = perm.get_t();
    let mut mt = MerkleTree::new(perm);
    let set_size = 1 << log_set_size;
    let arity = mt.get_arity();
    let levels = f64::ceil((set_size as f64).log(arity as f64)) as usize;
    let mut rng = thread_rng();
    let dist: Range<usize> = Range::new(0, set_size);
    let perm_circ = GmimcCircuit::new(params);
    let mt_circ = MerkleTreeCircuit::new(perm_circ, levels, arity);

    let mut groth = MerkleGroth::<E, GmimcCircuit<E>>::new(mt_circ);
    groth.create_crs(&mut rng);
    let pvk = groth.create_verify_key();

    let set: Vec<E::Fr> = sample_set::<E>(set_size);

    let index = dist.ind_sample(&mut rng);
    mt.accumulate(&set);
    let wit = mt.create_witness(&set[index]).unwrap();
    let root = mt.get_root().unwrap();

    let proof = groth.create_proof(&set[index], &wit, &mut rng);

    let id = format!(
        "Gmimc MT BLS proof (t = {}, set_size = 2^{})",
        t, log_set_size
    );
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let proof =
                groth.create_proof(black_box(&set[index]), black_box(&wit), black_box(&mut rng));
            black_box(proof)
        });
    });

    let id = format!(
        "Gmimc MT BLS verify (t = {}, set_size = 2^{})",
        t, log_set_size
    );
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let res = MerkleGroth::<E, GmimcCircuit<E>>::verify_proof(
                black_box(&pvk),
                black_box(&proof),
                black_box(&root),
            );
            black_box(res)
        });
    });
}

fn neptune_proof_verify<E: Engine>(
    c: &mut Criterion,
    params: &Arc<NeptuneParams<E::Fr>>,
    log_set_size: usize,
) {
    let perm = Neptune::new(params);
    let t = perm.get_t();
    let mut mt = MerkleTree::new(perm);
    let set_size = 1 << log_set_size;
    let arity = mt.get_arity();
    let levels = f64::ceil((set_size as f64).log(arity as f64)) as usize;
    let mut rng = thread_rng();
    let dist: Range<usize> = Range::new(0, set_size);
    let perm_circ = NeptuneCircuit::new(params);
    let mt_circ = MerkleTreeCircuit::new(perm_circ, levels, arity);

    let mut groth = MerkleGroth::<E, NeptuneCircuit<E>>::new(mt_circ);
    groth.create_crs(&mut rng);
    let pvk = groth.create_verify_key();

    let set: Vec<E::Fr> = sample_set::<E>(set_size);

    let index = dist.ind_sample(&mut rng);
    mt.accumulate(&set);
    let wit = mt.create_witness(&set[index]).unwrap();
    let root = mt.get_root().unwrap();

    let proof = groth.create_proof(&set[index], &wit, &mut rng);

    let id = format!(
        "Neptune MT BLS proof (t = {}, set_size = 2^{})",
        t, log_set_size
    );
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let proof =
                groth.create_proof(black_box(&set[index]), black_box(&wit), black_box(&mut rng));
            black_box(proof)
        });
    });

    let id = format!(
        "Neptune MT BLS verify (t = {}, set_size = 2^{})",
        t, log_set_size
    );
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let res = MerkleGroth::<E, NeptuneCircuit<E>>::verify_proof(
                black_box(&pvk),
                black_box(&proof),
                black_box(&root),
            );
            black_box(res)
        });
    });
}

fn criterion_benchmark_mt_proof_bls12(c: &mut Criterion) {
    let log_set_sizes = vec![24];

    for log_set_size in log_set_sizes {
        rescue_proof_verify::<Bls12>(c, &RESCUE_BLS_3_PARAMS, log_set_size);
        rescue_proof_verify::<Bls12>(c, &RESCUE_BLS_4_PARAMS, log_set_size);
        rescue_proof_verify::<Bls12>(c, &RESCUE_BLS_5_PARAMS, log_set_size);
        rescue_proof_verify::<Bls12>(c, &RESCUE_BLS_8_PARAMS, log_set_size);
        rescue_proof_verify::<Bls12>(c, &RESCUE_BLS_9_PARAMS, log_set_size);
        rescue_proof_verify::<Bls12>(c, &RESCUE_BLS_12_PARAMS, log_set_size);

        poseidon_proof_verify::<Bls12>(c, &POSEIDON_BLS_3_PARAMS, log_set_size);
        poseidon_proof_verify::<Bls12>(c, &POSEIDON_BLS_4_PARAMS, log_set_size);
        poseidon_proof_verify::<Bls12>(c, &POSEIDON_BLS_5_PARAMS, log_set_size);
        poseidon_proof_verify::<Bls12>(c, &POSEIDON_BLS_8_PARAMS, log_set_size);
        poseidon_proof_verify::<Bls12>(c, &POSEIDON_BLS_9_PARAMS, log_set_size);
        poseidon_proof_verify::<Bls12>(c, &POSEIDON_BLS_12_PARAMS, log_set_size);

        griffin_proof_verify::<Bls12>(c, &GRIFFIN_BLS_3_PARAMS, log_set_size);
        griffin_proof_verify::<Bls12>(c, &GRIFFIN_BLS_4_PARAMS, log_set_size);
        griffin_proof_verify::<Bls12>(c, &GRIFFIN_BLS_8_PARAMS, log_set_size);
        griffin_proof_verify::<Bls12>(c, &GRIFFIN_BLS_12_PARAMS, log_set_size);

        grendel_proof_verify::<Bls12>(c, &GRENDEL_BLS_3_PARAMS, log_set_size);
        grendel_proof_verify::<Bls12>(c, &GRENDEL_BLS_4_PARAMS, log_set_size);
        grendel_proof_verify::<Bls12>(c, &GRENDEL_BLS_5_PARAMS, log_set_size);
        grendel_proof_verify::<Bls12>(c, &GRENDEL_BLS_8_PARAMS, log_set_size);
        grendel_proof_verify::<Bls12>(c, &GRENDEL_BLS_9_PARAMS, log_set_size);
        grendel_proof_verify::<Bls12>(c, &GRENDEL_BLS_12_PARAMS, log_set_size);

        gmimc_proof_verify::<Bls12>(c, &GMIMC_BLS_3_PARAMS, log_set_size);
        gmimc_proof_verify::<Bls12>(c, &GMIMC_BLS_4_PARAMS, log_set_size);
        gmimc_proof_verify::<Bls12>(c, &GMIMC_BLS_5_PARAMS, log_set_size);
        gmimc_proof_verify::<Bls12>(c, &GMIMC_BLS_8_PARAMS, log_set_size);
        gmimc_proof_verify::<Bls12>(c, &GMIMC_BLS_9_PARAMS, log_set_size);
        gmimc_proof_verify::<Bls12>(c, &GMIMC_BLS_12_PARAMS, log_set_size);

        neptune_proof_verify::<Bls12>(c, &NEPTUNE_BLS_4_PARAMS, log_set_size);
        neptune_proof_verify::<Bls12>(c, &NEPTUNE_BLS_8_PARAMS, log_set_size);
        neptune_proof_verify::<Bls12>(c, &NEPTUNE_BLS_12_PARAMS, log_set_size);
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark_mt_proof_bls12
);
criterion_main!(benches);
