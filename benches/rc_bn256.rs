use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zkhash::{
    fields::{bn256::FpBN256, utils},
    reinforced_concrete::{
        reinforced_concrete::ReinforcedConcrete, reinforced_concrete_instances::RC_BN_PARAMS,
    },
};
type Scalar = FpBN256;

fn permutation(c: &mut Criterion) {
    let rc = ReinforcedConcrete::new(&RC_BN_PARAMS);
    let input: [Scalar; 3] = [
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
    ];

    c.bench_function("ReinforcedConcrete BN256 Permutation", move |bench| {
        bench.iter(|| {
            let perm = rc.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn hash(c: &mut Criterion) {
    let rc = ReinforcedConcrete::new(&RC_BN_PARAMS);
    let input1 = utils::random_scalar(true);
    let input2 = utils::random_scalar(true);

    c.bench_function("ReinforcedConcrete BN256 Hash", move |bench| {
        bench.iter(|| {
            let hash = rc.hash(black_box(&input1), black_box(&input2));
            black_box(hash)
        });
    });
}

fn concrete(c: &mut Criterion) {
    let rc = ReinforcedConcrete::new(&RC_BN_PARAMS);
    let input: [Scalar; 3] = [
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
    ];

    c.bench_function("ReinforcedConcrete BN256 Concrete", move |bench| {
        let mut output = input.to_owned();
        bench.iter(|| {
            rc.concrete(black_box(&mut output), black_box(0));
            black_box(&output);
        });
    });
}

fn bricks(c: &mut Criterion) {
    let rc = ReinforcedConcrete::new(&RC_BN_PARAMS);
    let input: [Scalar; 3] = [
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
    ];

    c.bench_function("ReinforcedConcrete BN256 Bricks", move |bench| {
        bench.iter(|| {
            let perm = rc.bricks(black_box(&input));
            black_box(perm)
        });
    });
}

fn bars(c: &mut Criterion) {
    let rc = ReinforcedConcrete::new(&RC_BN_PARAMS);
    let input: [Scalar; 3] = [
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
    ];

    c.bench_function("ReinforcedConcrete BN256 Bars", move |bench| {
        bench.iter(|| {
            let perm = rc.bars(black_box(&input));
            black_box(perm)
        });
    });
}

fn decompose(c: &mut Criterion) {
    let input = utils::random_scalar::<Scalar>(true);
    let rc = ReinforcedConcrete::new(&RC_BN_PARAMS);

    c.bench_function("BN256 decompose", move |bench| {
        bench.iter(|| {
            let res = rc.decompose(black_box(&input));
            black_box(res);
        });
    });
}

fn compose(c: &mut Criterion) {
    let input = utils::random_scalar::<Scalar>(true);
    let rc = ReinforcedConcrete::new(&RC_BN_PARAMS);

    let vals = rc.decompose(&input);

    c.bench_function("BN256 compose", move |bench| {
        bench.iter(|| {
            let res = rc.compose(black_box(&vals));
            black_box(res);
        });
    });
}

fn criterion_benchmark_plain_bn(c: &mut Criterion) {
    permutation(c);
    hash(c);
    concrete(c);
    bricks(c);
    bars(c);
    decompose(c);
    compose(c);
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark_plain_bn
);
criterion_main!(benches);
