use rand::{thread_rng, Rng};
use twenty_first::shared_math::{
    b_field_element::BFieldElement, rescue_prime_optimized::RescuePrimeOptimized, tip5::Tip5,
};
use winter_utils::rand_array;
use winterfell::{
    crypto::{
        hashers::{Tip4_256, Tip4p_256, Tip5_320},
        ElementHasher,
    },
    math::fields::f64::BaseElement,
};
use zkhash::{
    fields::{
        const_f64::ConstF64,
        f64::{Field64, F64},
        utils,
    },
    gmimc::{
        gmimc::Gmimc,
        gmimc_instance_goldilocks::{GMIMC_GOLDILOCKS_12_PARAMS, GMIMC_GOLDILOCKS_8_PARAMS},
    },
    griffin::{
        griffin::Griffin,
        griffin_instances::{GRIFFIN_GOLDILOCKS_12_PARAMS, GRIFFIN_GOLDILOCKS_8_PARAMS},
    },
    monolith_64::{
        mds_12, mds_8,
        monolith_64::Monolith64,
        monolith_64_instances::{
            MONOLITH_64_12_PARAMS, MONOLITH_64_8_PARAMS, MONOLITH_CONST64_12_PARAMS,
            MONOLITH_CONST64_8_PARAMS,
        },
    },
    neptune::{
        neptune::Neptune,
        neptune_instances::{NEPTUNE_GOLDILOCKS_12_PARAMS, NEPTUNE_GOLDILOCKS_8_PARAMS},
    },
    poseidon::{
        poseidon::Poseidon,
        poseidon_instance_goldilocks::{
            POSEIDON_GOLDILOCKS_12_PARAMS, POSEIDON_GOLDILOCKS_8_PARAMS,
        },
        poseidon_instance_goldilocks_const::{
            POSEIDON_GOLDILOCKS_12_PARAMS_CONST, POSEIDON_GOLDILOCKS_8_PARAMS_CONST,
        },
    },
    poseidon2::{
        poseidon2::Poseidon2,
        poseidon2_instance_goldilocks::{
            POSEIDON2_GOLDILOCKS_12_PARAMS, POSEIDON2_GOLDILOCKS_8_PARAMS,
        },
        poseidon2_instance_goldilocks_const::{
            POSEIDON2_GOLDILOCKS_12_PARAMS_CONST, POSEIDON2_GOLDILOCKS_8_PARAMS_CONST,
        },
    },
    rescue_prime::{
        rescue_prime::RescuePrime,
        rescue_prime_instance_goldilocks::{
            RESCUE_PRIME_GOLDILOCKS_12_PARAMS, RESCUE_PRIME_GOLDILOCKS_8_PARAMS,
        },
    },
};
type Scalar = F64;
type ConstScalar = ConstF64;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn neptune_goldilocks_8(c: &mut Criterion) {
    let neptune = Neptune::new(&NEPTUNE_GOLDILOCKS_8_PARAMS);
    let t = neptune.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Neptune Goldilocks plain (t = 8)", move |bench| {
        bench.iter(|| {
            let perm = neptune.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

#[allow(unused)]
fn neptune_goldilocks_12(c: &mut Criterion) {
    let neptune = Neptune::new(&NEPTUNE_GOLDILOCKS_12_PARAMS);
    let t = neptune.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Neptune Goldilocks plain (t = 12)", move |bench| {
        bench.iter(|| {
            let perm = neptune.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn griffin_goldilocks_8(c: &mut Criterion) {
    let griffin = Griffin::new(&GRIFFIN_GOLDILOCKS_8_PARAMS);
    let t = griffin.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Griffin Goldilocks plain (t = 8)", move |bench| {
        bench.iter(|| {
            let perm = griffin.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

#[allow(unused)]
fn griffin_goldilocks_12(c: &mut Criterion) {
    let griffin = Griffin::new(&GRIFFIN_GOLDILOCKS_12_PARAMS);
    let t = griffin.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Griffin Goldilocks plain (t = 12)", move |bench| {
        bench.iter(|| {
            let perm = griffin.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn rescue_goldilocks_8(c: &mut Criterion) {
    let rescue = RescuePrime::new(&RESCUE_PRIME_GOLDILOCKS_8_PARAMS);
    let t = rescue.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Rescue Prime Goldilocks plain (t = 8)", move |bench| {
        bench.iter(|| {
            let perm = rescue.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

#[allow(unused)]
fn rescue_goldilocks_12(c: &mut Criterion) {
    let rescue = RescuePrime::new(&RESCUE_PRIME_GOLDILOCKS_12_PARAMS);
    let t = rescue.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Rescue Prime Goldilocks plain (t = 12)", move |bench| {
        bench.iter(|| {
            let perm = rescue.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn poseidon_goldilocks_8(c: &mut Criterion) {
    let poseidon = Poseidon::new(&POSEIDON_GOLDILOCKS_8_PARAMS);
    let t = poseidon.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Poseidon Goldilocks plain (t = 8)", move |bench| {
        bench.iter(|| {
            let perm = poseidon.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

#[allow(unused)]
fn poseidon_goldilocks_12(c: &mut Criterion) {
    let poseidon = Poseidon::new(&POSEIDON_GOLDILOCKS_12_PARAMS);
    let t = poseidon.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Poseidon Goldilocks plain (t = 12)", move |bench| {
        bench.iter(|| {
            let perm = poseidon.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn poseidon_goldilocks_8_const(c: &mut Criterion) {
    let poseidon = Poseidon::new(&POSEIDON_GOLDILOCKS_8_PARAMS_CONST);
    let t = poseidon.get_t();
    let input: Vec<ConstScalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function(
        "Poseidon Goldilocks plain (t = 8, constant time)",
        move |bench| {
            bench.iter(|| {
                let perm = poseidon.permutation(black_box(&input));
                black_box(perm)
            });
        },
    );
}

#[allow(unused)]
fn poseidon_goldilocks_12_const(c: &mut Criterion) {
    let poseidon = Poseidon::new(&POSEIDON_GOLDILOCKS_12_PARAMS_CONST);
    let t = poseidon.get_t();
    let input: Vec<ConstScalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function(
        "Poseidon Goldilocks plain (t = 12, constant time)",
        move |bench| {
            bench.iter(|| {
                let perm = poseidon.permutation(black_box(&input));
                black_box(perm)
            });
        },
    );
}

fn poseidon2_goldilocks_8(c: &mut Criterion) {
    let poseidon2 = Poseidon2::new(&POSEIDON2_GOLDILOCKS_8_PARAMS);
    let t = poseidon2.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Poseidon2 Goldilocks plain (t = 8)", move |bench| {
        bench.iter(|| {
            let perm = poseidon2.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn poseidon2_goldilocks_12(c: &mut Criterion) {
    let poseidon2 = Poseidon2::new(&POSEIDON2_GOLDILOCKS_12_PARAMS);
    let t = poseidon2.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Poseidon2 Goldilocks plain (t = 12)", move |bench| {
        bench.iter(|| {
            let perm = poseidon2.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn poseidon2_goldilocks_8_const(c: &mut Criterion) {
    let poseidon2 = Poseidon2::new(&POSEIDON2_GOLDILOCKS_8_PARAMS_CONST);
    let t = poseidon2.get_t();
    let input: Vec<ConstScalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function(
        "Poseidon2 Goldilocks plain (t = 8, constant time)",
        move |bench| {
            bench.iter(|| {
                let perm = poseidon2.permutation(black_box(&input));
                black_box(perm)
            });
        },
    );
}

fn poseidon2_goldilocks_12_const(c: &mut Criterion) {
    let poseidon2 = Poseidon2::new(&POSEIDON2_GOLDILOCKS_12_PARAMS_CONST);
    let t = poseidon2.get_t();
    let input: Vec<ConstScalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function(
        "Poseidon2 Goldilocks plain (t = 12, constant time)",
        move |bench| {
            bench.iter(|| {
                let perm = poseidon2.permutation(black_box(&input));
                black_box(perm)
            });
        },
    );
}

fn gmimc_goldilocks_8(c: &mut Criterion) {
    let gmimc = Gmimc::new(&GMIMC_GOLDILOCKS_8_PARAMS);
    let t = gmimc.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("GMiMC Goldilocks plain (t = 8)", move |bench| {
        bench.iter(|| {
            let perm = gmimc.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn gmimc_goldilocks_12(c: &mut Criterion) {
    let gmimc = Gmimc::new(&GMIMC_GOLDILOCKS_12_PARAMS);
    let t = gmimc.get_t();
    let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("GMiMC Goldilocks plain (t = 12)", move |bench| {
        bench.iter(|| {
            let perm = gmimc.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn tip5(c: &mut Criterion) {
    let mut rng = thread_rng();
    let input: [BFieldElement; 10] = [
        BFieldElement::new(rng.gen()),
        BFieldElement::new(rng.gen()),
        BFieldElement::new(rng.gen()),
        BFieldElement::new(rng.gen()),
        BFieldElement::new(rng.gen()),
        BFieldElement::new(rng.gen()),
        BFieldElement::new(rng.gen()),
        BFieldElement::new(rng.gen()),
        BFieldElement::new(rng.gen()),
        BFieldElement::new(rng.gen()),
    ];

    c.bench_function("Tip5 (t = 16)", move |bench| {
        bench.iter(|| {
            let perm = Tip5::hash_10(black_box(&input));
            black_box(perm)
        });
    });
}

fn tip5_winterfell(c: &mut Criterion) {
    let input: [BaseElement; 10] = rand_array();

    c.bench_function("Tip5 (Winterfell) (t = 16)", move |bench| {
        bench.iter(|| {
            let perm = Tip5_320::hash_elements(black_box(&input));
            black_box(perm)
        });
    });
}

fn tip4(c: &mut Criterion) {
    let input: [BaseElement; 10] = rand_array();

    c.bench_function("Tip4 (t = 16)", move |bench| {
        bench.iter(|| {
            let perm = Tip4_256::hash_elements(black_box(&input));
            black_box(perm)
        });
    });
}

fn tip4_prime(c: &mut Criterion) {
    let input: [BaseElement; 8] = rand_array();

    c.bench_function("Tip4' (t = 12)", move |bench| {
        bench.iter(|| {
            let perm = Tip4p_256::hash_elements(black_box(&input));
            black_box(perm)
        });
    });
}

fn rescue_prime_optimized(c: &mut Criterion) {
    let mut rng = thread_rng();
    let input: [BFieldElement; 10] = [
        BFieldElement::new(rng.gen()),
        BFieldElement::new(rng.gen()),
        BFieldElement::new(rng.gen()),
        BFieldElement::new(rng.gen()),
        BFieldElement::new(rng.gen()),
        BFieldElement::new(rng.gen()),
        BFieldElement::new(rng.gen()),
        BFieldElement::new(rng.gen()),
        BFieldElement::new(rng.gen()),
        BFieldElement::new(rng.gen()),
    ];

    c.bench_function("Rescue Prime optimized (t = 16)", move |bench| {
        bench.iter(|| {
            let perm = RescuePrimeOptimized::hash_10(black_box(&input));
            black_box(perm)
        });
    });
}

fn permutation_8_lookup(c: &mut Criterion) {
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

    c.bench_function("Monolith F64 Permutation Lookup t=8", move |bench| {
        bench.iter(|| {
            let perm = monolith.permutation_u128_lookup(black_box(&input));
            black_box(perm)
        });
    });
}

fn permutation_12_lookup(c: &mut Criterion) {
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

    c.bench_function("Monolith F64 Permutation Lookup t=12", move |bench| {
        bench.iter(|| {
            let perm = monolith.permutation_u128_lookup(black_box(&input));
            black_box(perm)
        });
    });
}

fn permutation_8(c: &mut Criterion) {
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
            let perm = monolith.permutation(black_box(&input));
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
            let perm = monolith.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn permutation_8_const(c: &mut Criterion) {
    let monolith = Monolith64::new(&MONOLITH_CONST64_8_PARAMS);
    let input: [ConstScalar; 8] = [
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
    ];

    c.bench_function("Monolith F64 Permutation t=8 constant time", move |bench| {
        bench.iter(|| {
            let perm = monolith.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn permutation_12_const(c: &mut Criterion) {
    let monolith = Monolith64::new(&MONOLITH_CONST64_12_PARAMS);
    let input: [ConstScalar; 12] = [
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
        "Monolith F64 Permutation t=12 constant time",
        move |bench| {
            bench.iter(|| {
                let perm = monolith.permutation(black_box(&input));
                black_box(perm)
            });
        },
    );
}

fn permutation_8_const_lookup(c: &mut Criterion) {
    let monolith = Monolith64::new(&MONOLITH_CONST64_8_PARAMS);
    let input: [ConstScalar; 8] = [
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
        "Monolith F64 Permutation Lookup t=8 constant time field",
        move |bench| {
            bench.iter(|| {
                let perm = monolith.permutation_u128_lookup(black_box(&input));
                black_box(perm)
            });
        },
    );
}

fn permutation_12_const_lookup(c: &mut Criterion) {
    let monolith = Monolith64::new(&MONOLITH_CONST64_12_PARAMS);
    let input: [ConstScalar; 12] = [
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
        "Monolith F64 Permutation Lookup t=12 constant time field",
        move |bench| {
            bench.iter(|| {
                let perm = monolith.permutation_u128_lookup(black_box(&input));
                black_box(perm)
            });
        },
    );
}

fn mds_circ_8(c: &mut Criterion) {
    let input: [Scalar; 8] = [
        utils::random_scalar::<Scalar>(true),
        utils::random_scalar::<Scalar>(true),
        utils::random_scalar::<Scalar>(true),
        utils::random_scalar::<Scalar>(true),
        utils::random_scalar::<Scalar>(true),
        utils::random_scalar::<Scalar>(true),
        utils::random_scalar::<Scalar>(true),
        utils::random_scalar::<Scalar>(true),
    ];
    let round_const = MONOLITH_64_8_PARAMS.get_rc(0);

    c.bench_function(
        "Monolith F64 MDS (circ) t=8 (no roundconst)",
        move |bench| {
            bench.iter(|| {
                let mut perm = input.to_owned();
                mds_8::mds_multiply_with_rc(black_box(&mut perm), black_box(&round_const));
                black_box(perm)
            });
        },
    );
}

fn concrete_8(c: &mut Criterion) {
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

fn mds_circ_12(c: &mut Criterion) {
    let input: [Scalar; 12] = [
        utils::random_scalar::<Scalar>(true),
        utils::random_scalar::<Scalar>(true),
        utils::random_scalar::<Scalar>(true),
        utils::random_scalar::<Scalar>(true),
        utils::random_scalar::<Scalar>(true),
        utils::random_scalar::<Scalar>(true),
        utils::random_scalar::<Scalar>(true),
        utils::random_scalar::<Scalar>(true),
        utils::random_scalar::<Scalar>(true),
        utils::random_scalar::<Scalar>(true),
        utils::random_scalar::<Scalar>(true),
        utils::random_scalar::<Scalar>(true),
    ];
    let round_const = MONOLITH_64_12_PARAMS.get_rc(0);

    c.bench_function(
        "Monolith F64 MDS (circ) t=12 (no roundconst)",
        move |bench| {
            bench.iter(|| {
                let mut perm = input.to_owned();
                mds_12::mds_multiply_with_rc(black_box(&mut perm), black_box(&round_const));
                black_box(perm)
            });
        },
    );
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

fn bricks_8(c: &mut Criterion) {
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

fn bars_8(c: &mut Criterion) {
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

fn bars_8_lookup(c: &mut Criterion) {
    let monolith = Monolith64::new(&MONOLITH_CONST64_8_PARAMS);
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

    c.bench_function("Monolith F64 Bars_Lookup t=8", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            monolith.bars_u128_lookup(black_box(&mut perm));
            black_box(perm)
        });
    });
}

fn bars_12_lookup(c: &mut Criterion) {
    let monolith = Monolith64::new(&MONOLITH_CONST64_12_PARAMS);
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

    c.bench_function("Monolith F64 Bars_Lookup t=12", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            monolith.bars_u128_lookup(black_box(&mut perm));
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

fn bar_lookup(c: &mut Criterion) {
    let monolith = Monolith64::new(&MONOLITH_64_8_PARAMS);
    let input = utils::random_scalar::<Scalar>(true).to_u64();

    c.bench_function("Monolith F64 Bar_Lookup", move |bench| {
        let mut output = input.to_owned();
        bench.iter(|| {
            monolith.bar_u64_lookup(black_box(&mut output));
            black_box(output)
        });
    });
}

fn mds_circ_8_const(c: &mut Criterion) {
    let input: [ConstScalar; 8] = [
        utils::random_scalar::<ConstScalar>(true),
        utils::random_scalar::<ConstScalar>(true),
        utils::random_scalar::<ConstScalar>(true),
        utils::random_scalar::<ConstScalar>(true),
        utils::random_scalar::<ConstScalar>(true),
        utils::random_scalar::<ConstScalar>(true),
        utils::random_scalar::<ConstScalar>(true),
        utils::random_scalar::<ConstScalar>(true),
    ];
    let round_const: [ConstF64; 8] = MONOLITH_CONST64_8_PARAMS.get_rc(0);

    c.bench_function(
        "Monolith F64 MDS (circ) t=8 constant time (no roundconst)",
        move |bench| {
            bench.iter(|| {
                let mut perm = input.to_owned();
                mds_8::mds_multiply_with_rc(black_box(&mut perm), black_box(&round_const));
                black_box(perm)
            });
        },
    );
}

fn concrete_8_const(c: &mut Criterion) {
    let monolith = Monolith64::new(&MONOLITH_CONST64_8_PARAMS);
    let input: [u128; 8] = [
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
    ];
    let round_const: [ConstF64; 8] = MONOLITH_CONST64_8_PARAMS.get_rc(0);

    c.bench_function("Monolith F64 Concrete t=8 constant time", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            monolith.concrete_u128(black_box(&mut perm), black_box(&round_const));
            black_box(perm)
        });
    });
}

fn mds_circ_12_const(c: &mut Criterion) {
    let input: [ConstScalar; 12] = [
        utils::random_scalar::<ConstScalar>(true),
        utils::random_scalar::<ConstScalar>(true),
        utils::random_scalar::<ConstScalar>(true),
        utils::random_scalar::<ConstScalar>(true),
        utils::random_scalar::<ConstScalar>(true),
        utils::random_scalar::<ConstScalar>(true),
        utils::random_scalar::<ConstScalar>(true),
        utils::random_scalar::<ConstScalar>(true),
        utils::random_scalar::<ConstScalar>(true),
        utils::random_scalar::<ConstScalar>(true),
        utils::random_scalar::<ConstScalar>(true),
        utils::random_scalar::<ConstScalar>(true),
    ];
    let round_const = MONOLITH_CONST64_12_PARAMS.get_rc(0);

    c.bench_function(
        "Monolith F64 MDS (circ) t=12 constant time (no roundconst)",
        move |bench| {
            bench.iter(|| {
                let mut perm = input.to_owned();
                mds_12::mds_multiply_with_rc(black_box(&mut perm), black_box(&round_const));
                black_box(perm)
            });
        },
    );
}

fn concrete_12_const(c: &mut Criterion) {
    let monolith = Monolith64::new(&MONOLITH_CONST64_12_PARAMS);
    let input: [u128; 12] = [
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
    ];
    let round_const = MONOLITH_CONST64_12_PARAMS.get_rc(0);

    c.bench_function("Monolith F64 Concrete t=12 constant time", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            monolith.concrete_u128(black_box(&mut perm), black_box(&round_const));
            black_box(perm)
        });
    });
}

fn bricks_8_const(c: &mut Criterion) {
    let input: [u128; 8] = [
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
    ];

    c.bench_function("Monolith F64 Bricks t=8 constant time", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            Monolith64::<ConstScalar, 8>::bricks_u128(black_box(&mut perm));
            black_box(perm)
        });
    });
}

fn bricks_12_const(c: &mut Criterion) {
    let input: [u128; 12] = [
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
    ];

    c.bench_function("Monolith F64 Bricks t=12 constant time", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            Monolith64::<ConstScalar, 12>::bricks_u128(black_box(&mut perm));
            black_box(perm)
        });
    });
}

fn bars_8_const(c: &mut Criterion) {
    let input: [u128; 8] = [
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
    ];

    c.bench_function("Monolith F64 Bars t=8 constant time", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            Monolith64::<ConstScalar, 8>::bars_u128(black_box(&mut perm));
            black_box(perm)
        });
    });
}

fn bars_12_const(c: &mut Criterion) {
    let input: [u128; 12] = [
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
        utils::random_scalar::<ConstScalar>(true).to_u64() as u128,
    ];

    c.bench_function("Monolith F64 Bars t=12 constant time", move |bench| {
        bench.iter(|| {
            let mut perm = input.to_owned();
            Monolith64::<ConstScalar, 12>::bars_u128(black_box(&mut perm));
            black_box(perm)
        });
    });
}

fn bar_const(c: &mut Criterion) {
    let input = utils::random_scalar::<ConstScalar>(true).to_u64();

    c.bench_function("Monolith F64 Bar constant time", move |bench| {
        bench.iter(|| {
            let output = Monolith64::<ConstScalar, 8>::bar_u64(black_box(input));
            black_box(output)
        });
    });
}

fn criterion_benchmark_plain_goldilocks(c: &mut Criterion) {
    griffin_goldilocks_8(c);
    griffin_goldilocks_12(c);

    poseidon_goldilocks_8(c);
    poseidon_goldilocks_12(c);

    poseidon_goldilocks_8_const(c);
    poseidon_goldilocks_12_const(c);

    poseidon2_goldilocks_8(c);
    poseidon2_goldilocks_12(c);

    poseidon2_goldilocks_8_const(c);
    poseidon2_goldilocks_12_const(c);

    neptune_goldilocks_8(c);
    neptune_goldilocks_12(c);

    gmimc_goldilocks_8(c);
    gmimc_goldilocks_12(c);

    rescue_goldilocks_8(c);
    rescue_goldilocks_12(c);

    tip5(c);
    rescue_prime_optimized(c);
    tip5_winterfell(c);
    tip4(c);
    tip4_prime(c);

    permutation_8(c);
    permutation_12(c);
    permutation_8_lookup(c);
    permutation_12_lookup(c);

    permutation_8_const(c);
    permutation_12_const(c);
    permutation_8_const_lookup(c);
    permutation_12_const_lookup(c);

    bricks_8(c);
    bricks_12(c);

    mds_circ_8(c);
    mds_circ_12(c);

    concrete_8(c);
    concrete_12(c);

    bars_8(c);
    bars_8_lookup(c);
    bars_12(c);
    bars_12_lookup(c);
    bar(c);
    bar_lookup(c);

    bricks_8_const(c);
    bricks_12_const(c);

    mds_circ_8_const(c);
    mds_circ_12_const(c);

    concrete_8_const(c);
    concrete_12_const(c);

    bars_8_const(c);
    bar_const(c);
    bars_12_const(c);
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark_plain_goldilocks
);
criterion_main!(benches);
