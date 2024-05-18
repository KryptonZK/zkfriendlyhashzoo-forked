use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zkhash::{
    fields::{
        f64::{Field64, F64},
        utils,
    },
    monolith_64::{
        monolith_64::Monolith64,
        monolith_64_instances::{MONOLITH_64_12_PARAMS, MONOLITH_64_8_PARAMS},
    },
};

type Scalar = F64;

fn permutation(c: &mut Criterion) {
    let monolith = Monolith64::new(&MONOLITH_64_8_PARAMS);
    let input: [Scalar; 8] = [
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
    ];

    c.bench_function("Monolith F64 Permutation t=8", move |bench| {
        bench.iter(|| {
            let perm = monolith.permutation_(black_box(&input));
            black_box(perm)
        });
    });
}

fn permutation_u128(c: &mut Criterion) {
    let monolith = Monolith64::new(&MONOLITH_64_8_PARAMS);
    let input: [Scalar; 8] = [
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
    ];

    c.bench_function("Monolith F64 Permutation t=8 (u128)", move |bench| {
        bench.iter(|| {
            let perm = monolith.permutation_u128(black_box(&input));
            black_box(perm)
        });
    });
}

fn permutation_12(c: &mut Criterion) {
    let monolith = Monolith64::new(&MONOLITH_64_12_PARAMS);
    let input: [Scalar; 12] = [
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
    ];

    c.bench_function("Monolith F64 Permutation t=12", move |bench| {
        bench.iter(|| {
            let perm = monolith.permutation_(black_box(&input));
            black_box(perm)
        });
    });
}

fn permutation_u128_12(c: &mut Criterion) {
    let monolith = Monolith64::new(&MONOLITH_64_12_PARAMS);
    let input: [Scalar; 12] = [
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
    ];

    c.bench_function("Monolith F64 Permutation t=12 (u128)", move |bench| {
        bench.iter(|| {
            let perm = monolith.permutation_u128(black_box(&input));
            black_box(perm)
        });
    });
}

fn hash(c: &mut Criterion) {
    let monolith = Monolith64::new(&MONOLITH_64_8_PARAMS);
    let input1: [Scalar; 4] = [
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
    ];
    let input2: [Scalar; 4] = [
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
    ];

    c.bench_function("Monolith F64 Hash", move |bench| {
        bench.iter(|| {
            let hash = monolith.hash(black_box(&input1), black_box(&input2));
            black_box(hash)
        });
    });
}

fn concrete(c: &mut Criterion) {
    let monolith = Monolith64::new(&MONOLITH_64_8_PARAMS);
    let input: [u128; 8] = [
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
    ];
    let round_const = MONOLITH_64_8_PARAMS.get_rc(0);

    c.bench_function("Monolith F64 Concrete t=8", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            monolith.concrete_u128(black_box(&mut perm), black_box(&round_const));
            black_box(perm)
        });
    });
}

fn concrete_12(c: &mut Criterion) {
    let monolith = Monolith64::new(&MONOLITH_64_12_PARAMS);
    let input: [u128; 12] = [
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
    ];
    let round_const = MONOLITH_64_12_PARAMS.get_rc(0);

    c.bench_function("Monolith F64 Concrete t=12", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            monolith.concrete_u128(black_box(&mut perm), black_box(&round_const));
            black_box(perm)
        });
    });
}

fn bricks(c: &mut Criterion) {
    let input: [u128; 8] = [
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
    ];

    c.bench_function("Monolith F64 Bricks t=8", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            Monolith64::<Scalar, 8>::bricks_u128(black_box(&mut perm));
            black_box(perm)
        });
    });
}

fn bricks_12(c: &mut Criterion) {
    let input: [u128; 12] = [
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
    ];

    c.bench_function("Monolith F64 Bricks t=12", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            Monolith64::<Scalar, 12>::bricks_u128(black_box(&mut perm));
            black_box(perm)
        });
    });
}

fn bars(c: &mut Criterion) {
    let input: [u128; 8] = [
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
    ];

    c.bench_function("Monolith F64 Bars t=8", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            Monolith64::<Scalar, 8>::bars_u128(black_box(&mut perm));
            black_box(perm)
        });
    });
}

fn bars_12(c: &mut Criterion) {
    let input: [u128; 12] = [
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
        utils::random_scalar::<Scalar>(true).to_u64() as u128,
    ];

    c.bench_function("Monolith F64 Bars t=12", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            Monolith64::<Scalar, 12>::bars_u128(black_box(&mut perm));
            black_box(perm)
        });
    });
}

fn bar(c: &mut Criterion) {
    let input = utils::random_scalar::<Scalar>(true).to_u64();

    c.bench_function("Monolith F64 Bar", move |bench| {
        bench.iter(|| {
            let output = Monolith64::<Scalar, 8>::bar_u64(black_box(input));
            black_box(output)
        });
    });
}

fn criterion_benchmark_plain_f64(c: &mut Criterion) {
    permutation(c);
    permutation_12(c);
    permutation_u128(c);
    permutation_u128_12(c);
    hash(c);
    bricks(c);
    bricks_12(c);
    concrete(c);
    concrete_12(c);
    bars(c);
    bar(c);
    bars_12(c);
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark_plain_f64
);
criterion_main!(benches);
