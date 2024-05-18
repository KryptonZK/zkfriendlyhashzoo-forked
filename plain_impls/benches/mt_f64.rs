use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zkhash::{
    fields::{f64::F64, utils},
    merkle_tree::merkle_tree_fp_t::MerkleTree,
    monolith_64::{monolith_64::Monolith64, monolith_64_instances::MONOLITH_64_8_PARAMS},
};

type Scalar = F64;

fn sample_set(set_size: usize) -> Vec<[Scalar; 4]> {
    // (0..set_size).map(|_| utils::random_scalar(true)).collect()
    (0..set_size)
        .map(|i| {
            [
                utils::from_u64(i as u64 * 4),
                utils::from_u64(i as u64 * 4 + 1),
                utils::from_u64(i as u64 * 4 + 2),
                utils::from_u64(i as u64 * 4 + 3),
            ]
        })
        .collect()
}

fn monolith(c: &mut Criterion, log_set_size: usize) {
    let perm = Monolith64::new(&MONOLITH_64_8_PARAMS);
    let mut mt = MerkleTree::new(perm);
    let set_size = 1 << log_set_size;
    let set: Vec<[Scalar; 4]> = sample_set(set_size);

    let id = format!("Monolith F64 MT (set_size = 2^{})", log_set_size);

    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            mt.accumulate(black_box(&set));
        });
    });
}

fn criterion_benchmark_mt_f64(c: &mut Criterion) {
    let log_set_sizes = vec![20];

    for log_set_size in log_set_sizes {
        monolith(c, log_set_size);
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark_mt_f64
);
criterion_main!(benches);
