use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zkhash::{
    feistel_mimc::{feistel_mimc::FeistelMimc, feistel_mimc_instances::FM_BLS_PARAMS},
    fields::{bls12::FpBLS12, utils},
    griffin::{griffin::Griffin, griffin_instances::GRIFFIN_BLS_PARAMS},
    merkle_tree::merkle_tree_fp::MerkleTree,
    neptune::{neptune::Neptune, neptune_instances::NEPTUNE_BLS_PARAMS},
    poseidon::{poseidon::Poseidon, poseidon_instance_bls12::POSEIDON_BLS_PARAMS},
    reinforced_concrete::{
        reinforced_concrete::ReinforcedConcrete, reinforced_concrete_instances::RC_BLS_PARAMS,
    },
    rescue::{rescue::Rescue, rescue_instance_bls12::RESCUE_BLS_PARAMS},
    rescue_prime::{
        rescue_prime::RescuePrime, rescue_prime_instance_bls12::RESCUE_PRIME_BLS_PARAMS,
    },
};
type Scalar = FpBLS12;

fn sample_set(set_size: usize) -> Vec<Scalar> {
    // (0..set_size).map(|_| utils::random_scalar(true)).collect()
    (0..set_size).map(|i| utils::from_u64(i as u64)).collect()
}

fn rc(c: &mut Criterion, log_set_size: usize) {
    let perm = ReinforcedConcrete::new(&RC_BLS_PARAMS);
    let mut mt = MerkleTree::new(perm);
    let set_size = 1 << log_set_size;
    let set: Vec<Scalar> = sample_set(set_size);

    let id = format!(
        "ReinforcedConcrete BLS12 MT (set_size = 2^{})",
        log_set_size
    );

    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            mt.accumulate(black_box(&set));
        });
    });
}

fn poseidon(c: &mut Criterion, log_set_size: usize) {
    let perm = Poseidon::new(&POSEIDON_BLS_PARAMS);
    let mut mt = MerkleTree::new(perm);
    let set_size = 1 << log_set_size;
    let set: Vec<Scalar> = sample_set(set_size);

    let id = format!("Poseidon BLS12 MT (set_size = 2^{})", log_set_size);

    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            mt.accumulate(black_box(&set));
        });
    });
}

fn neptune(c: &mut Criterion, log_set_size: usize) {
    let perm = Neptune::new(&NEPTUNE_BLS_PARAMS);
    let mut mt = MerkleTree::new(perm);
    let set_size = 1 << log_set_size;
    let set: Vec<Scalar> = sample_set(set_size);

    let id = format!("Neptune BLS12 MT (set_size = 2^{})", log_set_size);

    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            mt.accumulate(black_box(&set));
        });
    });
}

fn griffin(c: &mut Criterion, log_set_size: usize) {
    let perm = Griffin::new(&GRIFFIN_BLS_PARAMS);
    let mut mt = MerkleTree::new(perm);
    let set_size = 1 << log_set_size;
    let set: Vec<Scalar> = sample_set(set_size);

    let id = format!("Griffin BLS12 MT (set_size = 2^{})", log_set_size);

    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            mt.accumulate(black_box(&set));
        });
    });
}

fn rescue(c: &mut Criterion, log_set_size: usize) {
    let perm = Rescue::new(&RESCUE_BLS_PARAMS);
    let mut mt = MerkleTree::new(perm);
    let set_size = 1 << log_set_size;
    let set: Vec<Scalar> = sample_set(set_size);

    let id = format!("Rescue BLS12 MT (set_size = 2^{})", log_set_size);

    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            mt.accumulate(black_box(&set));
        });
    });
}

fn rescue_prime(c: &mut Criterion, log_set_size: usize) {
    let perm = RescuePrime::new(&RESCUE_PRIME_BLS_PARAMS);
    let mut mt = MerkleTree::new(perm);
    let set_size = 1 << log_set_size;
    let set: Vec<Scalar> = sample_set(set_size);

    let id = format!("Rescue-Prime BLS12 MT (set_size = 2^{})", log_set_size);

    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            mt.accumulate(black_box(&set));
        });
    });
}

fn feistel_mimc(c: &mut Criterion, log_set_size: usize) {
    let perm = FeistelMimc::new(&FM_BLS_PARAMS);
    let mut mt = MerkleTree::new(perm);
    let set_size = 1 << log_set_size;
    let set: Vec<Scalar> = sample_set(set_size);

    let id = format!("Feistel MiMC BLS12 MT (set_size = 2^{})", log_set_size);

    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            mt.accumulate(black_box(&set));
        });
    });
}

fn criterion_benchmark_mt_bls(c: &mut Criterion) {
    let log_set_sizes = vec![20];

    for log_set_size in log_set_sizes {
        rc(c, log_set_size);
        poseidon(c, log_set_size);
        rescue(c, log_set_size);
        rescue_prime(c, log_set_size);
        feistel_mimc(c, log_set_size);
        neptune(c, log_set_size);
        griffin(c, log_set_size);
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark_mt_bls
);
criterion_main!(benches);
