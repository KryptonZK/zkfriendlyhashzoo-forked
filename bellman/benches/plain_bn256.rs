use bellman_ce::pairing::bn256;
use hash_r1cs::{
    circuits::Permutation,
    gmimc::{gmimc::Gmimc, gmimc_instance_bn256::*},
    grendel::{grendel::Grendel, grendel_instance_bn256::*},
    griffin::{griffin::Griffin, griffin_instances::*},
    neptune::{neptune::Neptune, neptune_instances::*},
    poseidon::{poseidon::Poseidon, poseidon_instance_bn256::*},
    rescue::{rescue::Rescue, rescue_instance_bn256::*},
    utils,
};
type Scalar = bn256::Fr;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn griffin_3(c: &mut Criterion) {
    let griffin = Griffin::new(&GRIFFIN_BN_3_PARAMS);
    let t = griffin.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Griffin BN plain (t = 3)", move |bench| {
        bench.iter(|| {
            let perm = griffin.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn griffin_4(c: &mut Criterion) {
    let griffin = Griffin::new(&GRIFFIN_BN_4_PARAMS);
    let t = griffin.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Griffin BN plain (t = 4)", move |bench| {
        bench.iter(|| {
            let perm = griffin.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn griffin_8(c: &mut Criterion) {
    let griffin = Griffin::new(&GRIFFIN_BN_8_PARAMS);
    let t = griffin.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Griffin BN plain (t = 8)", move |bench| {
        bench.iter(|| {
            let perm = griffin.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn griffin_12(c: &mut Criterion) {
    let griffin = Griffin::new(&GRIFFIN_BN_12_PARAMS);
    let t = griffin.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Griffin BN plain (t = 12)", move |bench| {
        bench.iter(|| {
            let perm = griffin.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn griffin_16(c: &mut Criterion) {
    let griffin = Griffin::new(&GRIFFIN_BN_16_PARAMS);
    let t = griffin.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Griffin BN plain (t = 16)", move |bench| {
        bench.iter(|| {
            let perm = griffin.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn griffin_20(c: &mut Criterion) {
    let griffin = Griffin::new(&GRIFFIN_BN_20_PARAMS);
    let t = griffin.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Griffin BN plain (t = 20)", move |bench| {
        bench.iter(|| {
            let perm = griffin.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn griffin_24(c: &mut Criterion) {
    let griffin = Griffin::new(&GRIFFIN_BN_24_PARAMS);
    let t = griffin.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Griffin BN plain (t = 24)", move |bench| {
        bench.iter(|| {
            let perm = griffin.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn rescue_3(c: &mut Criterion) {
    let rescue = Rescue::new(&RESCUE_BN_3_PARAMS);
    let t = rescue.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Rescue BN plain (t = 3)", move |bench| {
        bench.iter(|| {
            let perm = rescue.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn rescue_4(c: &mut Criterion) {
    let rescue = Rescue::new(&RESCUE_BN_4_PARAMS);
    let t = rescue.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Rescue BN plain (t = 4)", move |bench| {
        bench.iter(|| {
            let perm = rescue.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn rescue_5(c: &mut Criterion) {
    let rescue = Rescue::new(&RESCUE_BN_5_PARAMS);
    let t = rescue.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Rescue BN plain (t = 5)", move |bench| {
        bench.iter(|| {
            let perm = rescue.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn rescue_8(c: &mut Criterion) {
    let rescue = Rescue::new(&RESCUE_BN_8_PARAMS);
    let t = rescue.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Rescue BN plain (t = 8)", move |bench| {
        bench.iter(|| {
            let perm = rescue.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn rescue_9(c: &mut Criterion) {
    let rescue = Rescue::new(&RESCUE_BN_9_PARAMS);
    let t = rescue.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Rescue BN plain (t = 9)", move |bench| {
        bench.iter(|| {
            let perm = rescue.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn rescue_12(c: &mut Criterion) {
    let rescue = Rescue::new(&RESCUE_BN_12_PARAMS);
    let t = rescue.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Rescue BN plain (t = 12)", move |bench| {
        bench.iter(|| {
            let perm = rescue.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn rescue_16(c: &mut Criterion) {
    let rescue = Rescue::new(&RESCUE_BN_16_PARAMS);
    let t = rescue.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Rescue BN plain (t = 16)", move |bench| {
        bench.iter(|| {
            let perm = rescue.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn rescue_20(c: &mut Criterion) {
    let rescue = Rescue::new(&RESCUE_BN_20_PARAMS);
    let t = rescue.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Rescue BN plain (t = 20)", move |bench| {
        bench.iter(|| {
            let perm = rescue.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn rescue_24(c: &mut Criterion) {
    let rescue = Rescue::new(&RESCUE_BN_24_PARAMS);
    let t = rescue.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Rescue BN plain (t = 24)", move |bench| {
        bench.iter(|| {
            let perm = rescue.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn poseidon_3(c: &mut Criterion) {
    let poseidon = Poseidon::new(&POSEIDON_BN_3_PARAMS);
    let t = poseidon.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Poseidon BN plain (t = 3)", move |bench| {
        bench.iter(|| {
            let perm = poseidon.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn poseidon_4(c: &mut Criterion) {
    let poseidon = Poseidon::new(&POSEIDON_BN_4_PARAMS);
    let t = poseidon.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Poseidon BN plain (t = 4)", move |bench| {
        bench.iter(|| {
            let perm = poseidon.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn poseidon_5(c: &mut Criterion) {
    let poseidon = Poseidon::new(&POSEIDON_BN_5_PARAMS);
    let t = poseidon.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Poseidon BN plain (t = 5)", move |bench| {
        bench.iter(|| {
            let perm = poseidon.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn poseidon_8(c: &mut Criterion) {
    let poseidon = Poseidon::new(&POSEIDON_BN_8_PARAMS);
    let t = poseidon.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Poseidon BN plain (t = 8)", move |bench| {
        bench.iter(|| {
            let perm = poseidon.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn poseidon_9(c: &mut Criterion) {
    let poseidon = Poseidon::new(&POSEIDON_BN_9_PARAMS);
    let t = poseidon.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Poseidon BN plain (t = 9)", move |bench| {
        bench.iter(|| {
            let perm = poseidon.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn poseidon_12(c: &mut Criterion) {
    let poseidon = Poseidon::new(&POSEIDON_BN_12_PARAMS);
    let t = poseidon.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Poseidon BN plain (t = 12)", move |bench| {
        bench.iter(|| {
            let perm = poseidon.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn poseidon_16(c: &mut Criterion) {
    let poseidon = Poseidon::new(&POSEIDON_BN_16_PARAMS);
    let t = poseidon.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Poseidon BN plain (t = 16)", move |bench| {
        bench.iter(|| {
            let perm = poseidon.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn poseidon_20(c: &mut Criterion) {
    let poseidon = Poseidon::new(&POSEIDON_BN_20_PARAMS);
    let t = poseidon.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Poseidon BN plain (t = 20)", move |bench| {
        bench.iter(|| {
            let perm = poseidon.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn poseidon_24(c: &mut Criterion) {
    let poseidon = Poseidon::new(&POSEIDON_BN_24_PARAMS);
    let t = poseidon.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Poseidon BN plain (t = 24)", move |bench| {
        bench.iter(|| {
            let perm = poseidon.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn grendel_3(c: &mut Criterion) {
    let grendel = Grendel::new(&GRENDEL_BN_3_PARAMS);
    let t = grendel.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Grendel BN plain (t = 3)", move |bench| {
        bench.iter(|| {
            let perm = grendel.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn grendel_4(c: &mut Criterion) {
    let grendel = Grendel::new(&GRENDEL_BN_4_PARAMS);
    let t = grendel.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Grendel BN plain (t = 4)", move |bench| {
        bench.iter(|| {
            let perm = grendel.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn grendel_5(c: &mut Criterion) {
    let grendel = Grendel::new(&GRENDEL_BN_5_PARAMS);
    let t = grendel.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Grendel BN plain (t = 5)", move |bench| {
        bench.iter(|| {
            let perm = grendel.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn grendel_8(c: &mut Criterion) {
    let grendel = Grendel::new(&GRENDEL_BN_8_PARAMS);
    let t = grendel.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Grendel BN plain (t = 8)", move |bench| {
        bench.iter(|| {
            let perm = grendel.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn grendel_9(c: &mut Criterion) {
    let grendel = Grendel::new(&GRENDEL_BN_9_PARAMS);
    let t = grendel.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Grendel BN plain (t = 9)", move |bench| {
        bench.iter(|| {
            let perm = grendel.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn grendel_12(c: &mut Criterion) {
    let grendel = Grendel::new(&GRENDEL_BN_12_PARAMS);
    let t = grendel.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Grendel BN plain (t = 12)", move |bench| {
        bench.iter(|| {
            let perm = grendel.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn grendel_16(c: &mut Criterion) {
    let grendel = Grendel::new(&GRENDEL_BN_16_PARAMS);
    let t = grendel.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Grendel BN plain (t = 16)", move |bench| {
        bench.iter(|| {
            let perm = grendel.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn grendel_20(c: &mut Criterion) {
    let grendel = Grendel::new(&GRENDEL_BN_20_PARAMS);
    let t = grendel.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Grendel BN plain (t = 20)", move |bench| {
        bench.iter(|| {
            let perm = grendel.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn grendel_24(c: &mut Criterion) {
    let grendel = Grendel::new(&GRENDEL_BN_24_PARAMS);
    let t = grendel.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Grendel BN plain (t = 24)", move |bench| {
        bench.iter(|| {
            let perm = grendel.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn gmimc_3(c: &mut Criterion) {
    let gmimc = Gmimc::new(&GMIMC_BN_3_PARAMS);
    let t = gmimc.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Gmimc BN plain (t = 3)", move |bench| {
        bench.iter(|| {
            let perm = gmimc.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn gmimc_4(c: &mut Criterion) {
    let gmimc = Gmimc::new(&GMIMC_BN_4_PARAMS);
    let t = gmimc.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Gmimc BN plain (t = 4)", move |bench| {
        bench.iter(|| {
            let perm = gmimc.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn gmimc_5(c: &mut Criterion) {
    let gmimc = Gmimc::new(&GMIMC_BN_5_PARAMS);
    let t = gmimc.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Gmimc BN plain (t = 5)", move |bench| {
        bench.iter(|| {
            let perm = gmimc.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn gmimc_8(c: &mut Criterion) {
    let gmimc = Gmimc::new(&GMIMC_BN_8_PARAMS);
    let t = gmimc.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Gmimc BN plain (t = 8)", move |bench| {
        bench.iter(|| {
            let perm = gmimc.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn gmimc_9(c: &mut Criterion) {
    let gmimc = Gmimc::new(&GMIMC_BN_9_PARAMS);
    let t = gmimc.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Gmimc BN plain (t = 9)", move |bench| {
        bench.iter(|| {
            let perm = gmimc.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn gmimc_12(c: &mut Criterion) {
    let gmimc = Gmimc::new(&GMIMC_BN_12_PARAMS);
    let t = gmimc.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Gmimc BN plain (t = 12)", move |bench| {
        bench.iter(|| {
            let perm = gmimc.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn gmimc_16(c: &mut Criterion) {
    let gmimc = Gmimc::new(&GMIMC_BN_16_PARAMS);
    let t = gmimc.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Gmimc BN plain (t = 16)", move |bench| {
        bench.iter(|| {
            let perm = gmimc.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn gmimc_20(c: &mut Criterion) {
    let gmimc = Gmimc::new(&GMIMC_BN_20_PARAMS);
    let t = gmimc.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Gmimc BN plain (t = 20)", move |bench| {
        bench.iter(|| {
            let perm = gmimc.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn gmimc_24(c: &mut Criterion) {
    let gmimc = Gmimc::new(&GMIMC_BN_24_PARAMS);
    let t = gmimc.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Gmimc BN plain (t = 24)", move |bench| {
        bench.iter(|| {
            let perm = gmimc.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn neptune_4(c: &mut Criterion) {
    let neptune = Neptune::new(&NEPTUNE_BN_4_PARAMS);
    let t = neptune.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Neptune BN plain (t = 4)", move |bench| {
        bench.iter(|| {
            let perm = neptune.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn neptune_8(c: &mut Criterion) {
    let neptune = Neptune::new(&NEPTUNE_BN_8_PARAMS);
    let t = neptune.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Neptune BN plain (t = 8)", move |bench| {
        bench.iter(|| {
            let perm = neptune.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn neptune_12(c: &mut Criterion) {
    let neptune = Neptune::new(&NEPTUNE_BN_12_PARAMS);
    let t = neptune.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Neptune BN plain (t = 12)", move |bench| {
        bench.iter(|| {
            let perm = neptune.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn neptune_16(c: &mut Criterion) {
    let neptune = Neptune::new(&NEPTUNE_BN_16_PARAMS);
    let t = neptune.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Neptune BN plain (t = 16)", move |bench| {
        bench.iter(|| {
            let perm = neptune.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn neptune_20(c: &mut Criterion) {
    let neptune = Neptune::new(&NEPTUNE_BN_20_PARAMS);
    let t = neptune.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Neptune BN plain (t = 20)", move |bench| {
        bench.iter(|| {
            let perm = neptune.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn neptune_24(c: &mut Criterion) {
    let neptune = Neptune::new(&NEPTUNE_BN_24_PARAMS);
    let t = neptune.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Neptune BN plain (t = 24)", move |bench| {
        bench.iter(|| {
            let perm = neptune.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn criterion_benchmark_plain_bn256(c: &mut Criterion) {
    griffin_3(c);
    griffin_4(c);
    griffin_8(c);
    griffin_12(c);
    griffin_16(c);
    griffin_20(c);
    griffin_24(c);

    rescue_3(c);
    rescue_4(c);
    rescue_5(c);
    rescue_8(c);
    rescue_9(c);
    rescue_12(c);
    rescue_16(c);
    rescue_20(c);
    rescue_24(c);

    poseidon_3(c);
    poseidon_4(c);
    poseidon_5(c);
    poseidon_8(c);
    poseidon_9(c);
    poseidon_12(c);
    poseidon_16(c);
    poseidon_20(c);
    poseidon_24(c);

    grendel_3(c);
    grendel_4(c);
    grendel_5(c);
    grendel_8(c);
    grendel_9(c);
    grendel_12(c);
    grendel_16(c);
    grendel_20(c);
    grendel_24(c);

    gmimc_3(c);
    gmimc_4(c);
    gmimc_5(c);
    gmimc_8(c);
    gmimc_9(c);
    gmimc_12(c);
    gmimc_16(c);
    gmimc_20(c);
    gmimc_24(c);

    neptune_4(c);
    neptune_8(c);
    neptune_12(c);
    neptune_16(c);
    neptune_20(c);
    neptune_24(c);
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark_plain_bn256
);
criterion_main!(benches);
