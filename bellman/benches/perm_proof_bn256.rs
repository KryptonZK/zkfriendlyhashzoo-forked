use std::sync::Arc;

use bellman_ce::pairing::{bn256::Bn256, Engine};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hash_r1cs::{
    circuits::Permutation,
    gmimc::{
        gmimc::Gmimc, gmimc_circuit::GmimcCircuit, gmimc_instance_bn256::*,
        gmimc_params::GmimcParams,
    },
    grendel::{
        grendel::Grendel, grendel_circuit::GrendelCircuit, grendel_instance_bn256::*,
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
        poseidon::Poseidon, poseidon_circuit::PoseidonCircuit, poseidon_instance_bn256::*,
        poseidon_params::PoseidonParams,
    },
    rescue::{
        rescue::Rescue, rescue_circuit::RescueCircuit, rescue_instance_bn256::*,
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

    let id = format!("Rescue BN proof (t = {})", t);
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let proof = groth.create_proof(black_box(&input), black_box(&mut rng));
            black_box(proof)
        });
    });

    let id = format!("Rescue BN verify (t = {})", t);
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

    let id = format!("Poseidon BN proof (t = {})", t);
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let proof = groth.create_proof(black_box(&input), black_box(&mut rng));
            black_box(proof)
        });
    });

    let id = format!("Poseidon BN verify (t = {})", t);
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

    let id = format!("Griffin BN proof (t = {})", t);
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let proof = groth.create_proof(black_box(&input), black_box(&mut rng));
            black_box(proof)
        });
    });

    let id = format!("Griffin BN verify (t = {})", t);
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

    let id = format!("Grendel BN proof (t = {})", t);
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let proof = groth.create_proof(black_box(&input), black_box(&mut rng));
            black_box(proof)
        });
    });

    let id = format!("Grendel BN verify (t = {})", t);
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

    let id = format!("Gmimc BN proof (t = {})", t);
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let proof = groth.create_proof(black_box(&input), black_box(&mut rng));
            black_box(proof)
        });
    });

    let id = format!("Gmimc BN verify (t = {})", t);
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

    let id = format!("Neptune BN proof (t = {})", t);
    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            let proof = groth.create_proof(black_box(&input), black_box(&mut rng));
            black_box(proof)
        });
    });

    let id = format!("Neptune BN verify (t = {})", t);
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

fn criterion_benchmark_perm_proof_bn256(c: &mut Criterion) {
    rescue_proof_verify::<Bn256>(c, &RESCUE_BN_3_PARAMS);
    rescue_proof_verify::<Bn256>(c, &RESCUE_BN_4_PARAMS);
    rescue_proof_verify::<Bn256>(c, &RESCUE_BN_5_PARAMS);
    rescue_proof_verify::<Bn256>(c, &RESCUE_BN_8_PARAMS);
    rescue_proof_verify::<Bn256>(c, &RESCUE_BN_9_PARAMS);
    rescue_proof_verify::<Bn256>(c, &RESCUE_BN_12_PARAMS);

    poseidon_proof_verify::<Bn256>(c, &POSEIDON_BN_3_PARAMS);
    poseidon_proof_verify::<Bn256>(c, &POSEIDON_BN_4_PARAMS);
    poseidon_proof_verify::<Bn256>(c, &POSEIDON_BN_5_PARAMS);
    poseidon_proof_verify::<Bn256>(c, &POSEIDON_BN_8_PARAMS);
    poseidon_proof_verify::<Bn256>(c, &POSEIDON_BN_9_PARAMS);
    poseidon_proof_verify::<Bn256>(c, &POSEIDON_BN_12_PARAMS);

    griffin_proof_verify::<Bn256>(c, &GRIFFIN_BN_3_PARAMS);
    griffin_proof_verify::<Bn256>(c, &GRIFFIN_BN_4_PARAMS);
    griffin_proof_verify::<Bn256>(c, &GRIFFIN_BN_8_PARAMS);
    griffin_proof_verify::<Bn256>(c, &GRIFFIN_BN_12_PARAMS);

    grendel_proof_verify::<Bn256>(c, &GRENDEL_BN_3_PARAMS);
    grendel_proof_verify::<Bn256>(c, &GRENDEL_BN_4_PARAMS);
    grendel_proof_verify::<Bn256>(c, &GRENDEL_BN_5_PARAMS);
    grendel_proof_verify::<Bn256>(c, &GRENDEL_BN_8_PARAMS);
    grendel_proof_verify::<Bn256>(c, &GRENDEL_BN_9_PARAMS);
    grendel_proof_verify::<Bn256>(c, &GRENDEL_BN_12_PARAMS);

    gmimc_proof_verify::<Bn256>(c, &GMIMC_BN_3_PARAMS);
    gmimc_proof_verify::<Bn256>(c, &GMIMC_BN_4_PARAMS);
    gmimc_proof_verify::<Bn256>(c, &GMIMC_BN_5_PARAMS);
    gmimc_proof_verify::<Bn256>(c, &GMIMC_BN_8_PARAMS);
    gmimc_proof_verify::<Bn256>(c, &GMIMC_BN_9_PARAMS);
    gmimc_proof_verify::<Bn256>(c, &GMIMC_BN_12_PARAMS);

    neptune_proof_verify::<Bn256>(c, &NEPTUNE_BN_4_PARAMS);
    neptune_proof_verify::<Bn256>(c, &NEPTUNE_BN_8_PARAMS);
    neptune_proof_verify::<Bn256>(c, &NEPTUNE_BN_12_PARAMS);
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark_perm_proof_bn256
);
criterion_main!(benches);
