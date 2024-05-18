use blake2::{Blake2b512, Blake2s256};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use group::ff::Field;
use group_ped::ff::Field as PedField;
use pasta_curves::pallas::Base;
use random::{thread_rng, Rng};
use sha2::{digest::Output, Digest, Sha256};
use sha3::Sha3_256;
use zkhash::{
    merkle_tree::merkle_tree_f2::MerkleTree,
    pedersen_hash::PedersenHasher,
    sinsemilla::{constants::MERKLE_CRH_PERSONALIZATION, sinsemilla::HashDomain},
};

fn sha256(c: &mut Criterion, log_set_size: usize) {
    let mut mt = MerkleTree::<Sha256>::default();
    let mut rng = thread_rng();

    let set_size = 1 << log_set_size;
    let set: Vec<Output<Sha256>> = (0..set_size)
        .map(|_| {
            (0..Sha256::output_size())
                .map(|_| rng.gen::<u8>())
                .collect()
        })
        .collect();

    let id = format!("SHA256 MT (set_size = 2^{})", log_set_size);

    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            mt.accumulate(black_box(&set));
        });
    });
}

fn sha3_256(c: &mut Criterion, log_set_size: usize) {
    let mut mt = MerkleTree::<Sha3_256>::default();
    let mut rng = thread_rng();

    let set_size = 1 << log_set_size;
    let set: Vec<Output<Sha3_256>> = (0..set_size)
        .map(|_| {
            (0..Sha3_256::output_size())
                .map(|_| rng.gen::<u8>())
                .collect()
        })
        .collect();

    let id = format!("SHA3-256 MT (set_size = 2^{})", log_set_size);

    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            mt.accumulate(black_box(&set));
        });
    });
}

fn blake2s(c: &mut Criterion, log_set_size: usize) {
    let mut mt = MerkleTree::<Blake2s256>::default();
    let mut rng = thread_rng();

    let set_size = 1 << log_set_size;
    let set: Vec<Output<Blake2s256>> = (0..set_size)
        .map(|_| {
            (0..Blake2s256::output_size())
                .map(|_| rng.gen::<u8>())
                .collect()
        })
        .collect();

    let id = format!("Blake2s MT (set_size = 2^{})", log_set_size);

    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            mt.accumulate(black_box(&set));
        });
    });
}

fn blake2b(c: &mut Criterion, log_set_size: usize) {
    let mut mt = MerkleTree::<Blake2b512>::default();
    let mut rng = thread_rng();

    let set_size = 1 << log_set_size;
    let set: Vec<Output<Blake2b512>> = (0..set_size)
        .map(|_| {
            (0..Blake2b512::output_size())
                .map(|_| rng.gen::<u8>())
                .collect()
        })
        .collect();

    let id = format!("Blake2b MT (set_size = 2^{})", log_set_size);

    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            mt.accumulate(black_box(&set));
        });
    });
}

fn sinsemilla(c: &mut Criterion, log_set_size: usize) {
    let mut mt = zkhash::merkle_tree::merkle_tree_orchard::MerkleTree::new(HashDomain::new(
        MERKLE_CRH_PERSONALIZATION,
    ));
    let mut rng = thread_rng();

    let set_size = 1 << log_set_size;
    let set: Vec<Base> = (0..set_size).map(|_| Base::random(&mut rng)).collect();

    let id = format!("Sinsemilla MT (set_size = 2^{})", log_set_size);

    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            mt.accumulate(black_box(&set));
        });
    });
}

fn pedersen(c: &mut Criterion, log_set_size: usize) {
    let mut mt =
        zkhash::merkle_tree::merkle_tree_sapling::MerkleTree::new(PedersenHasher::default());
    let mut rng = thread_rng();

    let set_size = 1 << log_set_size;
    let set: Vec<jubjub::Base> = (0..set_size)
        .map(|_| jubjub::Base::random(&mut rng))
        .collect();

    let id = format!("Pedersen MT (set_size = 2^{})", log_set_size);

    c.bench_function(&id, move |bench| {
        bench.iter(|| {
            mt.accumulate(black_box(&set));
        });
    });
}

fn criterion_benchmark_mt_hashes(c: &mut Criterion) {
    let log_set_sizes = vec![20];

    for log_set_size in log_set_sizes {
        sha256(c, log_set_size);
        sha3_256(c, log_set_size);
        blake2s(c, log_set_size);
        blake2b(c, log_set_size);
        sinsemilla(c, log_set_size);
        pedersen(c, log_set_size);
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark_mt_hashes
);
criterion_main!(benches);
