use zkhash::{
    fields::{
        const_f31::ConstF31,
        f31::{Field32, F31},
        utils,
    },
    monolith_31::{
        monolith_31::Monolith31,
        monolith_31_instances::{
            MONOLITH_31_16_PARAMS, MONOLITH_31_24_PARAMS, MONOLITH_CONST31_16_PARAMS,
            MONOLITH_CONST31_24_PARAMS,
        },
    },
    poseidon::{
        poseidon::Poseidon,
        poseidon_instance_mersenne::{POSEIDON_MERSENNE_16_PARAMS, POSEIDON_MERSENNE_24_PARAMS},
        poseidon_instance_mersenne_const::{
            POSEIDON_MERSENNE_16_PARAMS_CONST, POSEIDON_MERSENNE_24_PARAMS_CONST,
        },
    },
    poseidon2::{
        poseidon2::Poseidon2,
        poseidon2_instance_mersenne::{POSEIDON2_MERSENNE_16_PARAMS, POSEIDON2_MERSENNE_24_PARAMS},
        poseidon2_instance_mersenne_const::{
            POSEIDON2_MERSENNE_16_PARAMS_CONST, POSEIDON2_MERSENNE_24_PARAMS_CONST,
        },
    },
};
type Scalar = F31;
type ConstScalar = ConstF31;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn poseidon_mersenne_16(c: &mut Criterion) {
    let poseidon = Poseidon::new(&POSEIDON_MERSENNE_16_PARAMS);
    let t = poseidon.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Poseidon Mersenne plain (t = 16)", move |bench| {
        bench.iter(|| {
            let perm = poseidon.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

#[allow(unused)]
fn poseidon_mersenne_24(c: &mut Criterion) {
    let poseidon = Poseidon::new(&POSEIDON_MERSENNE_24_PARAMS);
    let t = poseidon.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Poseidon Mersenne plain (t = 24)", move |bench| {
        bench.iter(|| {
            let perm = poseidon.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn poseidon_mersenne_16_const(c: &mut Criterion) {
    let poseidon = Poseidon::new(&POSEIDON_MERSENNE_16_PARAMS_CONST);
    let t = poseidon.get_t();
    let input: Vec<ConstScalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function(
        "Poseidon Mersenne plain (t = 16, constant time)",
        move |bench| {
            bench.iter(|| {
                let perm = poseidon.permutation(black_box(&input));
                black_box(perm)
            });
        },
    );
}

#[allow(unused)]
fn poseidon_mersenne_24_const(c: &mut Criterion) {
    let poseidon = Poseidon::new(&POSEIDON_MERSENNE_24_PARAMS_CONST);
    let t = poseidon.get_t();
    let input: Vec<ConstScalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function(
        "Poseidon Mersenne plain (t = 24, constant time)",
        move |bench| {
            bench.iter(|| {
                let perm = poseidon.permutation(black_box(&input));
                black_box(perm)
            });
        },
    );
}

fn poseidon2_mersenne_16(c: &mut Criterion) {
    let poseidon2 = Poseidon2::new(&POSEIDON2_MERSENNE_16_PARAMS);
    let t = poseidon2.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Poseidon2 Mersenne plain (t = 16)", move |bench| {
        bench.iter(|| {
            let perm = poseidon2.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn poseidon2_mersenne_24(c: &mut Criterion) {
    let poseidon2 = Poseidon2::new(&POSEIDON2_MERSENNE_24_PARAMS);
    let t = poseidon2.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Poseidon2 Mersenne plain (t = 24)", move |bench| {
        bench.iter(|| {
            let perm = poseidon2.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn poseidon2_mersenne_16_const(c: &mut Criterion) {
    let poseidon2 = Poseidon2::new(&POSEIDON2_MERSENNE_16_PARAMS_CONST);
    let t = poseidon2.get_t();
    let input: Vec<ConstScalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function(
        "Poseidon2 Mersenne plain (t = 16, constant time)",
        move |bench| {
            bench.iter(|| {
                let perm = poseidon2.permutation(black_box(&input));
                black_box(perm)
            });
        },
    );
}

fn poseidon2_mersenne_24_const(c: &mut Criterion) {
    let poseidon2 = Poseidon2::new(&POSEIDON2_MERSENNE_24_PARAMS_CONST);
    let t = poseidon2.get_t();
    let input: Vec<ConstScalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function(
        "Poseidon2 Mersenne plain (t = 24, constant time)",
        move |bench| {
            bench.iter(|| {
                let perm = poseidon2.permutation(black_box(&input));
                black_box(perm)
            });
        },
    );
}

fn permutation_16_lookup(c: &mut Criterion) {
    let monolith = Monolith31::new(&MONOLITH_31_16_PARAMS);
    let input: [Scalar; 16] = [
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
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
    ];

    c.bench_function("Monolith F31 Permutation Lookup t=16", move |bench| {
        bench.iter(|| {
            let perm = monolith.permutation_u64_lookup(black_box(&input));
            black_box(perm)
        });
    });
}

fn permutation_24_lookup(c: &mut Criterion) {
    let monolith = Monolith31::new(&MONOLITH_31_24_PARAMS);
    let input: [Scalar; 24] = [
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

    c.bench_function("Monolith F31 Permutation Lookup t=24", move |bench| {
        bench.iter(|| {
            let perm = monolith.permutation_u64_lookup(black_box(&input));
            black_box(perm)
        });
    });
}

fn permutation_16(c: &mut Criterion) {
    let monolith = Monolith31::new(&MONOLITH_31_16_PARAMS);
    let input: [Scalar; 16] = [
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
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
    ];

    c.bench_function("Monolith F31 Permutation t=16", move |bench| {
        bench.iter(|| {
            let perm = monolith.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn permutation_24(c: &mut Criterion) {
    let monolith = Monolith31::new(&MONOLITH_31_24_PARAMS);
    let input: [Scalar; 24] = [
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

    c.bench_function("Monolith F31 Permutation t=24", move |bench| {
        bench.iter(|| {
            let perm = monolith.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn permutation_16_const(c: &mut Criterion) {
    let monolith = Monolith31::new(&MONOLITH_CONST31_16_PARAMS);
    let input: [ConstScalar; 16] = [
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
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
    ];

    c.bench_function(
        "Monolith F31 Permutation t=16 constant time",
        move |bench| {
            bench.iter(|| {
                let perm = monolith.permutation(black_box(&input));
                black_box(perm)
            });
        },
    );
}

fn permutation_24_const(c: &mut Criterion) {
    let monolith = Monolith31::new(&MONOLITH_CONST31_24_PARAMS);
    let input: [ConstScalar; 24] = [
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

    c.bench_function(
        "Monolith F31 Permutation t=24 constant time",
        move |bench| {
            bench.iter(|| {
                let perm = monolith.permutation(black_box(&input));
                black_box(perm)
            });
        },
    );
}

fn permutation_16_const_lookup(c: &mut Criterion) {
    let monolith = Monolith31::new(&MONOLITH_CONST31_16_PARAMS);
    let input: [ConstScalar; 16] = [
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
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
    ];

    c.bench_function(
        "Monolith F31 Permutation Lookup t=16 constant time field",
        move |bench| {
            bench.iter(|| {
                let perm = monolith.permutation_u64_lookup(black_box(&input));
                black_box(perm)
            });
        },
    );
}

fn permutation_24_const_lookup(c: &mut Criterion) {
    let monolith = Monolith31::new(&MONOLITH_CONST31_24_PARAMS);
    let input: [ConstScalar; 24] = [
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

    c.bench_function(
        "Monolith F31 Permutation Lookup t=24 constant time field",
        move |bench| {
            bench.iter(|| {
                let perm = monolith.permutation_u64_lookup(black_box(&input));
                black_box(perm)
            });
        },
    );
}

fn concrete_16(c: &mut Criterion) {
    let monolith = Monolith31::new(&MONOLITH_31_16_PARAMS);
    let input: [u64; 16] = [
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
    ];
    let round_const = MONOLITH_31_16_PARAMS.get_rc(0);

    c.bench_function("Monolith F31 Concrete t=16", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            monolith.concrete_u64(black_box(&mut perm), black_box(&round_const));
            black_box(perm)
        });
    });
}

fn concrete_24(c: &mut Criterion) {
    let monolith = Monolith31::new(&MONOLITH_31_24_PARAMS);
    let input: [u64; 24] = [
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
    ];
    let round_const = MONOLITH_31_24_PARAMS.get_rc(0);

    c.bench_function("Monolith F31 Concrete t=24", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            monolith.concrete_u64(black_box(&mut perm), black_box(&round_const));
            black_box(perm)
        });
    });
}

fn bricks_16(c: &mut Criterion) {
    let input: [u64; 16] = [
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
    ];

    c.bench_function("Monolith F31 Bricks t=16", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            Monolith31::<Scalar, 16>::bricks_u64(black_box(&mut perm));
            black_box(perm)
        });
    });
}

fn bricks_24(c: &mut Criterion) {
    let input: [u64; 24] = [
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
    ];

    c.bench_function("Monolith F31 Bricks t=24", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            Monolith31::<Scalar, 24>::bricks_u64(black_box(&mut perm));
            black_box(perm)
        });
    });
}

fn bars_16(c: &mut Criterion) {
    let input: [u64; 16] = [
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
    ];

    c.bench_function("Monolith F31 Bars t=16", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            Monolith31::<Scalar, 16>::bars_u64(black_box(&mut perm));
            black_box(perm)
        });
    });
}

fn bars_24(c: &mut Criterion) {
    let input: [u64; 24] = [
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
    ];

    c.bench_function("Monolith F31 Bars t=24", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            Monolith31::<Scalar, 24>::bars_u64(black_box(&mut perm));
            black_box(perm)
        });
    });
}

fn bars_16_lookup(c: &mut Criterion) {
    let monolith = Monolith31::new(&MONOLITH_31_16_PARAMS);
    let input: [u64; 16] = [
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
    ];

    c.bench_function("Monolith F31 Bars_Lookup t=16", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            monolith.bars_u64_lookup(black_box(&mut perm));
            black_box(perm)
        });
    });
}

fn bars_24_lookup(c: &mut Criterion) {
    let monolith = Monolith31::new(&MONOLITH_31_24_PARAMS);
    let input: [u64; 24] = [
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
        utils::random_scalar::<Scalar>(true).to_u32() as u64,
    ];

    c.bench_function("Monolith F31 Bars_Lookup t=24", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            monolith.bars_u64_lookup(black_box(&mut perm));
            black_box(perm)
        });
    });
}

fn bar(c: &mut Criterion) {
    let input = utils::random_scalar::<Scalar>(true).to_u32();

    c.bench_function("Monolith F31 Bar", move |bench| {
        let mut output = input.to_owned();
        bench.iter(|| {
            Monolith31::<Scalar, 16>::bar_u32(black_box(&mut output));
            black_box(output)
        });
    });
}

fn bar_lookup(c: &mut Criterion) {
    let monolith = Monolith31::new(&MONOLITH_31_16_PARAMS);
    let input = utils::random_scalar::<Scalar>(true).to_u32();

    c.bench_function("Monolith F31 Bar_Lookup", move |bench| {
        let mut output = input.to_owned();
        bench.iter(|| {
            monolith.bar_u32_lookup(black_box(&mut output));
            black_box(output)
        });
    });
}

fn concrete_16_const(c: &mut Criterion) {
    let monolith = Monolith31::new(&MONOLITH_CONST31_16_PARAMS);
    let input: [u64; 16] = [
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
    ];
    let round_const = MONOLITH_CONST31_16_PARAMS.get_rc(0);

    c.bench_function("Monolith F31 Concrete t=16 constant time", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            monolith.concrete_u64(black_box(&mut perm), black_box(&round_const));
            black_box(perm)
        });
    });
}

fn concrete_24_const(c: &mut Criterion) {
    let monolith = Monolith31::new(&MONOLITH_CONST31_24_PARAMS);
    let input: [u64; 24] = [
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
    ];
    let round_const = MONOLITH_CONST31_24_PARAMS.get_rc(0);

    c.bench_function("Monolith F31 Concrete t=24 constant time", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            monolith.concrete_u64(black_box(&mut perm), black_box(&round_const));
            black_box(perm)
        });
    });
}

fn bricks_16_const(c: &mut Criterion) {
    let input: [u64; 16] = [
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
    ];
    c.bench_function("Monolith F31 Bricks t=16 constant time", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            Monolith31::<ConstScalar, 16>::bricks_u64(black_box(&mut perm));
            black_box(perm)
        });
    });
}

fn bricks_24_const(c: &mut Criterion) {
    let input: [u64; 24] = [
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
    ];

    c.bench_function("Monolith F31 Bricks t=24 constant time", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            Monolith31::<ConstScalar, 24>::bricks_u64(black_box(&mut perm));
            black_box(perm)
        });
    });
}

fn bars_16_const(c: &mut Criterion) {
    let input: [u64; 16] = [
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
    ];

    c.bench_function("Monolith F31 Bars t=16 constant time", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            Monolith31::<ConstScalar, 16>::bars_u64(black_box(&mut perm));
            black_box(perm)
        });
    });
}

fn bars_24_const(c: &mut Criterion) {
    let input: [u64; 24] = [
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
        utils::random_scalar::<ConstScalar>(true).to_u32() as u64,
    ];

    c.bench_function("Monolith F31 Bars t=24 constant time", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            Monolith31::<ConstScalar, 24>::bars_u64(black_box(&mut perm));
            black_box(perm)
        });
    });
}

fn bar_const(c: &mut Criterion) {
    let input = utils::random_scalar::<ConstScalar>(true).to_u32();

    c.bench_function("Monolith F31 Bar constant time", move |bench| {
        let mut output = input.to_owned();
        bench.iter(|| {
            Monolith31::<ConstScalar, 8>::bar_u32(black_box(&mut output));
            black_box(output)
        });
    });
}

fn criterion_benchmark_plain_mersenne(c: &mut Criterion) {
    poseidon_mersenne_16(c);
    poseidon_mersenne_24(c);

    poseidon_mersenne_16_const(c);
    poseidon_mersenne_24_const(c);

    poseidon2_mersenne_16(c);
    poseidon2_mersenne_24(c);

    poseidon2_mersenne_16_const(c);
    poseidon2_mersenne_24_const(c);

    permutation_16(c);
    permutation_24(c);
    permutation_16_lookup(c);
    permutation_24_lookup(c);

    permutation_16_const(c);
    permutation_24_const(c);
    permutation_16_const_lookup(c);
    permutation_24_const_lookup(c);

    bricks_16(c);
    bricks_24(c);

    concrete_16(c);
    concrete_24(c);

    bars_16(c);
    bars_16_lookup(c);
    bars_24(c);
    bars_24_lookup(c);
    bar(c);
    bar_lookup(c);

    bricks_16_const(c);
    bricks_24_const(c);

    concrete_16_const(c);
    concrete_24_const(c);

    bars_16_const(c);
    bar_const(c);
    bars_24_const(c);
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark_plain_mersenne
);
criterion_main!(benches);
