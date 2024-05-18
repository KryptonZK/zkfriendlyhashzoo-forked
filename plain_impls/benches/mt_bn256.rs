use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zkhash::{
    feistel_mimc::{feistel_mimc::FeistelMimc, feistel_mimc_instances::FM_BN_PARAMS},
    fields::{bn256::FpBN256, utils},
    griffin::{griffin::Griffin, griffin_instances::GRIFFIN_BN_PARAMS},
    merkle_tree::merkle_tree_fp::MerkleTree,
    neptune::{neptune::Neptune, neptune_instances::NEPTUNE_BN_PARAMS},
    poseidon::{poseidon::Poseidon, poseidon_instance_bn256::POSEIDON_BN_PARAMS},
    reinforced_concrete::{
        reinforced_concrete::ReinforcedConcrete, reinforced_concrete_instances::RC_BN_PARAMS,
    },
    rescue::{rescue::Rescue, rescue_instance_bn256::RESCUE_BN_PARAMS},
    rescue_prime::{
        rescue_prime::RescuePrime, rescue_prime_instance_bn256::RESCUE_PRIME_BN_PARAMS,
    },
};
type Scalar = FpBN256;

fn sample_set(set_size: usize) -> Vec<Scalar> {
    // (0..set_size).map(|_| utils::random_scalar(true)).collect()
    (0..set_size).map(|i| utils::from_u64(i as u64)).collect()
}

fn rc(c: &mut Criterion, log_set_size: usize) {
    let perm = ReinforcedConcrete::new(&RC_BN_PARAMS);
    let mut mt = MerkleTree::new(perm);
    let set_size = 1 << log_set_size;
    let set: Vec<Scalar> = sample_set(set_size);

    let id = format!(
        "ReinforcedConcrete BN256 MT (set_size = 2^{})",
        log_set_size
    );

    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            mt.accumulate(black_box(&set));
        });
    });
}

fn poseidon(c: &mut Criterion, log_set_size: usize) {
    let perm = Poseidon::new(&POSEIDON_BN_PARAMS);
    let mut mt = MerkleTree::new(perm);
    let set_size = 1 << log_set_size;
    let set: Vec<Scalar> = sample_set(set_size);

    let id = format!("Poseidon BN256 MT (set_size = 2^{})", log_set_size);

    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            mt.accumulate(black_box(&set));
        });
    });
}

fn neptune(c: &mut Criterion, log_set_size: usize) {
    let perm = Neptune::new(&NEPTUNE_BN_PARAMS);
    let mut mt = MerkleTree::new(perm);
    let set_size = 1 << log_set_size;
    let set: Vec<Scalar> = sample_set(set_size);

    let id = format!("Neptune BN256 MT (set_size = 2^{})", log_set_size);

    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            mt.accumulate(black_box(&set));
        });
    });
}

fn griffin(c: &mut Criterion, log_set_size: usize) {
    let perm = Griffin::new(&GRIFFIN_BN_PARAMS);
    let mut mt = MerkleTree::new(perm);
    let set_size = 1 << log_set_size;
    let set: Vec<Scalar> = sample_set(set_size);

    let id = format!("Griffin BN256 MT (set_size = 2^{})", log_set_size);

    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            mt.accumulate(black_box(&set));
        });
    });
}

fn rescue(c: &mut Criterion, log_set_size: usize) {
    let perm = Rescue::new(&RESCUE_BN_PARAMS);
    let mut mt = MerkleTree::new(perm);
    let set_size = 1 << log_set_size;
    let set: Vec<Scalar> = sample_set(set_size);

    let id = format!("Rescue BN256 MT (set_size = 2^{})", log_set_size);

    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            mt.accumulate(black_box(&set));
        });
    });
}

fn rescue_prime(c: &mut Criterion, log_set_size: usize) {
    let perm = RescuePrime::new(&RESCUE_PRIME_BN_PARAMS);
    let mut mt = MerkleTree::new(perm);
    let set_size = 1 << log_set_size;
    let set: Vec<Scalar> = sample_set(set_size);

    let id = format!("Rescue-Prime BN256 MT (set_size = 2^{})", log_set_size);

    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            mt.accumulate(black_box(&set));
        });
    });
}

fn feistel_mimc(c: &mut Criterion, log_set_size: usize) {
    let perm = FeistelMimc::new(&FM_BN_PARAMS);
    let mut mt = MerkleTree::new(perm);
    let set_size = 1 << log_set_size;
    let set: Vec<Scalar> = sample_set(set_size);

    let id = format!("Feistel MiMC BN256 MT (set_size = 2^{})", log_set_size);

    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            mt.accumulate(black_box(&set));
        });
    });
}

fn criterion_benchmark_mt_bn(c: &mut Criterion) {
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
    targets = criterion_benchmark_mt_bn
);
criterion_main!(benches);
