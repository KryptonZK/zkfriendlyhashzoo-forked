use std::iter;

use bitvec::order::Lsb0;
use bitvec::view::AsBits;
use blake2::{Blake2b512, Blake2s256};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use group::ff::Field;
use group::ff::PrimeFieldBits;
use group_ped::ff::Field as PedField;
use group_ped::ff::PrimeField;
use group_ped::Curve;
use jubjub::ExtendedPoint;
use pasta_curves::pallas::Base;
use random::thread_rng;
use sha2::{Digest, Sha256};
use sha3::Sha3_256;
use tiny_keccak::Hasher;
use zkhash::pedersen_hash::pedersen_hash::{pedersen_hash, Personalization};
use zkhash::sinsemilla::constants::L_ORCHARD_MERKLE;
use zkhash::sinsemilla::sinsemilla::{i2lebsp_k, HashDomain, MERKLE_CRH_PERSONALIZATION};

fn sha256(c: &mut Criterion) {
    let input = b"hello_world";

    c.bench_function("SHA256 Hash", move |bench| {
        bench.iter(|| {
            let hash = Sha256::digest(black_box(input));
            black_box(hash)
        });
    });
}

fn sha3_256(c: &mut Criterion) {
    let input = b"hello_world";

    c.bench_function("SHA3-256 Hash", move |bench| {
        bench.iter(|| {
            let hash = Sha3_256::digest(black_box(input));
            black_box(hash)
        });
    });
}

fn sha3_256_tiny(c: &mut Criterion) {
    let input = b"hello_world";
    let mut output = [0u8; 32];

    c.bench_function("SHA3-256 Hash (Tiny Keccak)", move |bench| {
        bench.iter(|| {
            let mut sha3 = tiny_keccak::Sha3::v256();
            sha3.update(black_box(input));
            sha3.finalize(black_box(&mut output));
        });
    });
}

fn blake2s(c: &mut Criterion) {
    let input = b"hello_world";

    c.bench_function("Blake2s Hash", move |bench| {
        bench.iter(|| {
            let hash = Blake2s256::digest(black_box(input));
            black_box(hash)
        });
    });
}

fn blake2b(c: &mut Criterion) {
    let input = b"hello_world";

    c.bench_function("Blake2b Hash", move |bench| {
        bench.iter(|| {
            let hash = Blake2b512::digest(black_box(input));
            black_box(hash)
        });
    });
}

fn sinsemilla(c: &mut Criterion) {
    let domain = HashDomain::new(MERKLE_CRH_PERSONALIZATION);

    let left = Base::random(thread_rng()).to_le_bits();
    let right = Base::random(thread_rng()).to_le_bits();

    let first = i2lebsp_k(0);

    let input: Vec<bool> = iter::empty()
        .chain(first.iter().copied())
        .chain(left.iter().by_vals().take(L_ORCHARD_MERKLE))
        .chain(right.iter().by_vals().take(L_ORCHARD_MERKLE))
        .collect();

    c.bench_function("Sinsemilla Hash", move |bench| {
        bench.iter(|| {
            let hash = domain.hash(black_box(input.iter().copied()));
            black_box(hash)
        });
    });
}

fn pedersen(c: &mut Criterion) {
    let mut rng = thread_rng();
    let personalization = Personalization::MerkleTree(0);

    let input = [
        jubjub::Base::random(&mut rng),
        jubjub::Base::random(&mut rng),
    ];

    let lhs = {
        let mut tmp = [false; 256];
        for (a, b) in tmp.iter_mut().zip(input[0].to_repr().as_bits::<Lsb0>()) {
            *a = *b;
        }
        tmp
    };

    let rhs = {
        let mut tmp = [false; 256];
        for (a, b) in tmp.iter_mut().zip(input[1].to_repr().as_bits::<Lsb0>()) {
            *a = *b;
        }
        tmp
    };
    let input = lhs
        .iter()
        .copied()
        .take(bls12_381::Scalar::NUM_BITS as usize)
        .chain(
            rhs.iter()
                .copied()
                .take(bls12_381::Scalar::NUM_BITS as usize),
        );

    c.bench_function("Pedersen Hash", move |bench| {
        bench.iter(|| {
            let hash = pedersen_hash(black_box(personalization), black_box(input.clone()));
            let out = ExtendedPoint::from(hash).to_affine().get_u();
            black_box(out)
        });
    });
}

fn criterion_benchmark_hashes(c: &mut Criterion) {
    sha256(c);
    sha3_256(c);
    sha3_256_tiny(c);
    blake2s(c);
    blake2b(c);
    sinsemilla(c);
    pedersen(c);
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark_hashes
);
criterion_main!(benches);
