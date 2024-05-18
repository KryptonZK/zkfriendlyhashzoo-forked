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
    neptune::{
        neptune::Neptune, neptune_circuit::NeptuneCircuit, neptune_instances::*,
        neptune_params::NeptuneParams,
    },
    perm_groth::PermGroth,
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
use rand::thread_rng;

fn rescue_proof_verify<E: Engine>(c: &mut Criterion, params: &Arc<RescueParams<E::Fr>>) {
    let perm = Rescue::new(params);
    let circuit = RescueCircuit::<E>::new(params);
    let mut rng = thread_rng();
    let mut groth = PermGroth::<E, RescueCircuit<E>>::new(circuit);
    groth.create_crs(&mut rng);
    let pvk = groth.create_verify_key();

    let t = perm.get_t();

    let input: Vec<E::Fr> = (0..t)
        .map(|_| utils::random_scalar_rng(true, &mut rng))
        .collect();

    let perm = perm.permutation(&input);
    let proof = groth.create_proof(&input, &mut rng);

    let id = format!("Rescue BLS proof (t = {})", t);
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let proof = groth.create_proof(black_box(&input), black_box(&mut rng));
            black_box(proof)
        });
    });

    let id = format!("Rescue BLS verify (t = {})", t);
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let res = PermGroth::<E, RescueCircuit<E>>::verify_proof(
                black_box(&pvk),
                black_box(&proof),
                black_box(&perm),
            );
            black_box(res)
        });
    });
}

fn poseidon_proof_verify<E: Engine>(c: &mut Criterion, params: &Arc<PoseidonParams<E::Fr>>) {
    let perm = Poseidon::new(params);
    let circuit = PoseidonCircuit::<E>::new(params);
    let mut rng = thread_rng();
    let mut groth = PermGroth::<E, PoseidonCircuit<E>>::new(circuit);
    groth.create_crs(&mut rng);
    let pvk = groth.create_verify_key();

    let t = perm.get_t();

    let input: Vec<E::Fr> = (0..t)
        .map(|_| utils::random_scalar_rng(true, &mut rng))
        .collect();

    let perm = perm.permutation(&input);
    let proof = groth.create_proof(&input, &mut rng);

    let id = format!("Poseidon BLS proof (t = {})", t);
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let proof = groth.create_proof(black_box(&input), black_box(&mut rng));
            black_box(proof)
        });
    });

    let id = format!("Poseidon BLS verify (t = {})", t);
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let res = PermGroth::<E, PoseidonCircuit<E>>::verify_proof(
                black_box(&pvk),
                black_box(&proof),
                black_box(&perm),
            );
            black_box(res)
        });
    });
}

fn griffin_proof_verify<E: Engine>(c: &mut Criterion, params: &Arc<GriffinParams<E::Fr>>) {
    let perm = Griffin::new(params);
    let circuit = GriffinCircuit::<E>::new(params);
    let mut rng = thread_rng();
    let mut groth = PermGroth::<E, GriffinCircuit<E>>::new(circuit);
    groth.create_crs(&mut rng);
    let pvk = groth.create_verify_key();

    let t = perm.get_t();

    let input: Vec<E::Fr> = (0..t)
        .map(|_| utils::random_scalar_rng(true, &mut rng))
        .collect();

    let perm = perm.permutation(&input);
    let proof = groth.create_proof(&input, &mut rng);

    let id = format!("Griffin BLS proof (t = {})", t);
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let proof = groth.create_proof(black_box(&input), black_box(&mut rng));
            black_box(proof)
        });
    });

    let id = format!("Griffin BLS verify (t = {})", t);
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let res = PermGroth::<E, GriffinCircuit<E>>::verify_proof(
                black_box(&pvk),
                black_box(&proof),
                black_box(&perm),
            );
            black_box(res)
        });
    });
}

fn grendel_proof_verify<E: Engine>(c: &mut Criterion, params: &Arc<GrendelParams<E::Fr>>) {
    let perm = Grendel::new(params);
    let circuit = GrendelCircuit::<E>::new(params);
    let mut rng = thread_rng();
    let mut groth = PermGroth::<E, GrendelCircuit<E>>::new(circuit);
    groth.create_crs(&mut rng);
    let pvk = groth.create_verify_key();

    let t = perm.get_t();

    let input: Vec<E::Fr> = (0..t)
        .map(|_| utils::random_scalar_rng(true, &mut rng))
        .collect();

    let perm = perm.permutation(&input);
    let proof = groth.create_proof(&input, &mut rng);

    let id = format!("Grendel BLS proof (t = {})", t);
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let proof = groth.create_proof(black_box(&input), black_box(&mut rng));
            black_box(proof)
        });
    });

    let id = format!("Grendel BLS verify (t = {})", t);
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let res = PermGroth::<E, GrendelCircuit<E>>::verify_proof(
                black_box(&pvk),
                black_box(&proof),
                black_box(&perm),
            );
            black_box(res)
        });
    });
}

fn gmimc_proof_verify<E: Engine>(c: &mut Criterion, params: &Arc<GmimcParams<E::Fr>>) {
    let perm = Gmimc::new(params);
    let circuit = GmimcCircuit::<E>::new(params);
    let mut rng = thread_rng();
    let mut groth = PermGroth::<E, GmimcCircuit<E>>::new(circuit);
    groth.create_crs(&mut rng);
    let pvk = groth.create_verify_key();

    let t = perm.get_t();

    let input: Vec<E::Fr> = (0..t)
        .map(|_| utils::random_scalar_rng(true, &mut rng))
        .collect();

    let perm = perm.permutation(&input);
    let proof = groth.create_proof(&input, &mut rng);

    let id = format!("Gmimc BLS proof (t = {})", t);
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let proof = groth.create_proof(black_box(&input), black_box(&mut rng));
            black_box(proof)
        });
    });

    let id = format!("Gmimc BLS verify (t = {})", t);
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let res = PermGroth::<E, GmimcCircuit<E>>::verify_proof(
                black_box(&pvk),
                black_box(&proof),
                black_box(&perm),
            );
            black_box(res)
        });
    });
}

fn neptune_proof_verify<E: Engine>(c: &mut Criterion, params: &Arc<NeptuneParams<E::Fr>>) {
    let perm = Neptune::new(params);
    let circuit = NeptuneCircuit::<E>::new(params);
    let mut rng = thread_rng();
    let mut groth = PermGroth::<E, NeptuneCircuit<E>>::new(circuit);
    groth.create_crs(&mut rng);
    let pvk = groth.create_verify_key();

    let t = perm.get_t();

    let input: Vec<E::Fr> = (0..t)
        .map(|_| utils::random_scalar_rng(true, &mut rng))
        .collect();

    let perm = perm.permutation(&input);
    let proof = groth.create_proof(&input, &mut rng);

    let id = format!("Neptune BLS proof (t = {})", t);
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let proof = groth.create_proof(black_box(&input), black_box(&mut rng));
            black_box(proof)
        });
    });

    let id = format!("Neptune BLS verify (t = {})", t);
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let res = PermGroth::<E, NeptuneCircuit<E>>::verify_proof(
                black_box(&pvk),
                black_box(&proof),
                black_box(&perm),
            );
            black_box(res)
        });
    });
}

fn criterion_benchmark_perm_proof_bls12(c: &mut Criterion) {
    rescue_proof_verify::<Bls12>(c, &RESCUE_BLS_3_PARAMS);
    rescue_proof_verify::<Bls12>(c, &RESCUE_BLS_4_PARAMS);
    rescue_proof_verify::<Bls12>(c, &RESCUE_BLS_5_PARAMS);
    rescue_proof_verify::<Bls12>(c, &RESCUE_BLS_8_PARAMS);
    rescue_proof_verify::<Bls12>(c, &RESCUE_BLS_9_PARAMS);
    rescue_proof_verify::<Bls12>(c, &RESCUE_BLS_12_PARAMS);

    poseidon_proof_verify::<Bls12>(c, &POSEIDON_BLS_3_PARAMS);
    poseidon_proof_verify::<Bls12>(c, &POSEIDON_BLS_4_PARAMS);
    poseidon_proof_verify::<Bls12>(c, &POSEIDON_BLS_5_PARAMS);
    poseidon_proof_verify::<Bls12>(c, &POSEIDON_BLS_8_PARAMS);
    poseidon_proof_verify::<Bls12>(c, &POSEIDON_BLS_9_PARAMS);
    poseidon_proof_verify::<Bls12>(c, &POSEIDON_BLS_12_PARAMS);

    griffin_proof_verify::<Bls12>(c, &GRIFFIN_BLS_3_PARAMS);
    griffin_proof_verify::<Bls12>(c, &GRIFFIN_BLS_4_PARAMS);
    griffin_proof_verify::<Bls12>(c, &GRIFFIN_BLS_8_PARAMS);
    griffin_proof_verify::<Bls12>(c, &GRIFFIN_BLS_12_PARAMS);

    grendel_proof_verify::<Bls12>(c, &GRENDEL_BLS_3_PARAMS);
    grendel_proof_verify::<Bls12>(c, &GRENDEL_BLS_4_PARAMS);
    grendel_proof_verify::<Bls12>(c, &GRENDEL_BLS_5_PARAMS);
    grendel_proof_verify::<Bls12>(c, &GRENDEL_BLS_8_PARAMS);
    grendel_proof_verify::<Bls12>(c, &GRENDEL_BLS_9_PARAMS);
    grendel_proof_verify::<Bls12>(c, &GRENDEL_BLS_12_PARAMS);

    gmimc_proof_verify::<Bls12>(c, &GMIMC_BLS_3_PARAMS);
    gmimc_proof_verify::<Bls12>(c, &GMIMC_BLS_4_PARAMS);
    gmimc_proof_verify::<Bls12>(c, &GMIMC_BLS_5_PARAMS);
    gmimc_proof_verify::<Bls12>(c, &GMIMC_BLS_8_PARAMS);
    gmimc_proof_verify::<Bls12>(c, &GMIMC_BLS_9_PARAMS);
    gmimc_proof_verify::<Bls12>(c, &GMIMC_BLS_12_PARAMS);

    neptune_proof_verify::<Bls12>(c, &NEPTUNE_BLS_4_PARAMS);
    neptune_proof_verify::<Bls12>(c, &NEPTUNE_BLS_8_PARAMS);
    neptune_proof_verify::<Bls12>(c, &NEPTUNE_BLS_12_PARAMS);
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark_perm_proof_bls12
);
criterion_main!(benches);
