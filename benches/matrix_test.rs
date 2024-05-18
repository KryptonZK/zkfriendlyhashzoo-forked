use std::convert::TryInto;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ff::Field;
use zkhash::fields::{
    f64::{Field64, F64},
    utils,
};

type Scalar = F64;

fn griffin_affine_4(input: &[Scalar; 4]) -> [u128; 4] {
    // multiplication by circ(3 2 1 1) is equal to state + state + rot(state) + sum(state)
    let sum = input.iter().fold(0, |s, el| s + el.to_u64() as u128);

    let mut output = [0u128; 4];
    // no round constant
    for (index, out) in output.iter_mut().enumerate() {
        *out = 2 * input[index].to_u64() as u128 + input[(index + 1) % 4].to_u64() as u128 + sum;
    }
    output
}

fn griffin_opt_no_reduce(state: &[Scalar; 8], rc: &[Scalar]) -> [u128; 8] {
    let mut out = [0u128; 8];
    let (state1, state2) = state.split_at(4);

    // first matrix
    let out1 = griffin_affine_4(state1.try_into().unwrap());
    let out2 = griffin_affine_4(state2.try_into().unwrap());

    // second matrix
    for (((o, s1), s2), r) in out
        .iter_mut()
        .take(4)
        .zip(out1.iter())
        .zip(out2.iter())
        .zip(rc.iter().take(4))
    {
        *o = 2 * s1 + s2 + r.to_u64() as u128;
    }

    for (((o, s1), s2), r) in out
        .iter_mut()
        .skip(4)
        .zip(out1.iter())
        .zip(out2.iter())
        .zip(rc.iter().skip(4))
    {
        *o = s1 + 2 * s2 + r.to_u64() as u128;
    }

    out
}

fn griffin_opt(state: &[Scalar; 8], rc: &[Scalar]) -> [Scalar; 8] {
    let mut out = [Scalar::zero(); 8];
    let (state1, state2) = state.split_at(4);

    // first matrix
    let out1 = griffin_affine_4(state1.try_into().unwrap());
    let out2 = griffin_affine_4(state2.try_into().unwrap());

    // second matrix
    for (((o, s1), s2), r) in out
        .iter_mut()
        .take(4)
        .zip(out1.iter())
        .zip(out2.iter())
        .zip(rc.iter().take(4))
    {
        let tmp = 2 * s1 + s2 + r.to_u64() as u128;
        *o = Scalar::reduce(&tmp);
    }

    for (((o, s1), s2), r) in out
        .iter_mut()
        .skip(4)
        .zip(out1.iter())
        .zip(out2.iter())
        .zip(rc.iter().skip(4))
    {
        let tmp = s1 + 2 * s2 + r.to_u64() as u128;
        *o = Scalar::reduce(&tmp);
    }

    out
}

fn griffin2_affine_4(input: &[Scalar; 4]) -> [Scalar; 4] {
    // multiplication by circ(3 2 1 1) is equal to state + state + rot(state) + sum(state)
    let mut sum = input[0];
    input.iter().skip(1).for_each(|el| sum.add_assign(el));

    let mut output = input.to_owned();
    // no round constant
    for (index, out) in output.iter_mut().enumerate() {
        out.double();
        out.add_assign(&sum);
        out.add_assign(&input[(index + 1) % 4]);
    }
    output
}

fn griffin2(state: &[Scalar; 8], rc: &[Scalar]) -> [Scalar; 8] {
    let mut out = [Scalar::zero(); 8];
    let (state1, state2) = state.split_at(4);

    // first matrix
    let out1 = griffin2_affine_4(state1.try_into().unwrap());
    let out2 = griffin2_affine_4(state2.try_into().unwrap());

    // second matrix
    for (((o, s1), s2), r) in out
        .iter_mut()
        .take(4)
        .zip(out1.iter())
        .zip(out2.iter())
        .zip(rc.iter().take(4))
    {
        *o = s1.to_owned();
        o.double();
        o.add_assign(s2);
        o.add_assign(r);
    }

    for (((o, s1), s2), r) in out
        .iter_mut()
        .skip(4)
        .zip(out1.iter())
        .zip(out2.iter())
        .zip(rc.iter().skip(4))
    {
        *o = s2.to_owned();
        o.double();
        o.add_assign(s1);
        o.add_assign(r);
    }

    out
}

fn griffin(state: &mut [Scalar; 8], rc: &[Scalar]) {
    // first matrix
    const T4: usize = 2;
    for i in 0..T4 {
        let startindex = i * 4;
        let mut sum = state[startindex];
        let start_el = sum;
        state
            .iter()
            .skip(startindex + 1)
            .take(3)
            .for_each(|el| sum.add_assign(el));
        for j in startindex..startindex + 3 {
            state[j].double();
            let tmp = state[j + 1];
            state[j].add_assign(&tmp);
            state[j].add_assign(&sum);
        }
        state[startindex + 3].double();
        state[startindex + 3].add_assign(&start_el);
        state[startindex + 3].add_assign(&sum);
    }

    // second matrix
    let mut stored = [Scalar::zero(); 4];
    for l in 0..4 {
        stored[l] = state[l];
        for j in 1..T4 {
            stored[l].add_assign(&state[4 * j + l]);
        }
    }

    for i in 0..state.len() {
        state[i].add_assign(&stored[i % 4]);
        state[i].add_assign(&rc[i]); // add round constant
    }
}

fn concrete_opt_no_reduce(state: &[Scalar; 8], mat: &[Vec<u64>], rc: &[Scalar]) -> [u128; 8] {
    let mut out_128 = [0u128; 8];
    for (row, o) in out_128.iter_mut().enumerate() {
        for (col, inp) in state.iter().enumerate().take(8) {
            let tmp = mat[row][col] as u128 * inp.to_u64() as u128;
            *o += tmp;
        }
        *o += rc[row].to_u64() as u128;
    }

    out_128
}

fn concrete_opt(state: &[Scalar; 8], mat: &[Vec<u64>], rc: &[Scalar]) -> [Scalar; 8] {
    let mut out = [Scalar::zero(); 8];

    for (row, o) in out.iter_mut().enumerate() {
        let mut o128 = 0;
        for (col, inp) in state.iter().enumerate().take(8) {
            let tmp = mat[row][col] as u128 * inp.to_u64() as u128;
            o128 += tmp;
        }
        o128 += rc[row].to_u64() as u128;
        *o = Scalar::reduce(&o128);
    }

    out
}

fn concrete(state: &[Scalar; 8], mat: &[Vec<u64>], rc: &[Scalar]) -> [Scalar; 8] {
    let mut out = [Scalar::zero(); 8];

    for (row, o) in out.iter_mut().enumerate() {
        for (col, inp) in state.iter().enumerate().take(8) {
            let mut tmp = inp.to_owned();
            let m = utils::from_u64(mat[row][col]);
            tmp.mul_assign(&m);
            o.add_assign(&tmp);
        }
        o.add_assign(&rc[row]);
    }

    out
}

fn instantiate_griffin_matrix() -> Vec<Vec<u64>> {
    let row = vec![3, 2, 1, 1];
    let c_mat = circ_mat(&row);

    let mut mat: Vec<Vec<u64>> = vec![vec![0; 8]; 8];
    for (row, matrow) in mat.iter_mut().enumerate() {
        for (col, matitem) in matrow.iter_mut().enumerate() {
            let row_mod = row % 4;
            let col_mod = col % 4;
            *matitem = c_mat[row_mod][col_mod];
            if row / 4 == col / 4 {
                *matitem += c_mat[row_mod][col_mod];
            }
        }
    }
    mat
}

fn circ_mat(row: &[u64]) -> Vec<Vec<u64>> {
    let t = row.len();
    let mut mat: Vec<Vec<u64>> = Vec::with_capacity(t);
    let mut rot = row.to_owned();
    mat.push(rot.clone());
    for _ in 1..t {
        rot.rotate_right(1);
        mat.push(rot.clone());
    }
    mat
}

fn griffin_bench(c: &mut Criterion) {
    let mut input: [Scalar; 8] = [
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
        utils::random_scalar(true),
    ];

    let rc: Vec<Scalar> = (0..8).map(|_| utils::random_scalar(true)).collect();

    // test correctness
    let mat = instantiate_griffin_matrix();
    let output = concrete(&input, &mat, &rc);
    let mut output_ = input.to_owned();
    griffin(&mut output_, &rc);
    assert_eq!(output, output_);

    c.bench_function("Griffin Matrix Bench", move |bench| {
        bench.iter(|| {
            griffin(black_box(&mut input), black_box(&rc));
            black_box(input)
        });
    });
}

fn griffin2_bench(c: &mut Criterion) {
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

    let rc: Vec<Scalar> = (0..8).map(|_| utils::random_scalar(true)).collect();

    // test correctness
    let mat = instantiate_griffin_matrix();
    let output = concrete(&input, &mat, &rc);
    let output_ = griffin2(&input, &rc);
    assert_eq!(output, output_);

    c.bench_function("Griffin Matrix2 Bench", move |bench| {
        bench.iter(|| {
            let perm = griffin2(black_box(&input), black_box(&rc));
            black_box(perm)
        });
    });
}

fn griffin_opt_bench(c: &mut Criterion) {
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

    let rc: Vec<Scalar> = (0..8).map(|_| utils::random_scalar(true)).collect();

    // test correctness
    let mat = instantiate_griffin_matrix();
    let output = concrete_opt(&input, &mat, &rc);
    let output_ = griffin_opt(&input, &rc);
    assert_eq!(output, output_);

    c.bench_function("Griffin Opt Matrix Bench", move |bench| {
        bench.iter(|| {
            let perm = griffin_opt(black_box(&input), black_box(&rc));
            black_box(perm)
        });
    });
}

fn griffin_opt_no_reduce_bench(c: &mut Criterion) {
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

    let rc: Vec<Scalar> = (0..8).map(|_| utils::random_scalar(true)).collect();

    // test correctness
    let mat = instantiate_griffin_matrix();
    let output = concrete_opt_no_reduce(&input, &mat, &rc);
    let output_ = griffin_opt_no_reduce(&input, &rc);
    assert_eq!(output, output_);

    c.bench_function("Griffin Opt w/o Reduction Matrix Bench", move |bench| {
        bench.iter(|| {
            let perm = griffin_opt_no_reduce(black_box(&input), black_box(&rc));
            black_box(perm)
        });
    });
}

fn concrete_opt_no_reduce_bench(c: &mut Criterion) {
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

    let row = vec![5, 3, 4, 3, 6, 2, 1, 1];
    let mat = circ_mat(&row);
    let rc: Vec<Scalar> = (0..8).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Concrete Opt w/o Reduction Matrix Bench", move |bench| {
        bench.iter(|| {
            let perm = concrete_opt_no_reduce(black_box(&input), black_box(&mat), black_box(&rc));
            black_box(perm)
        });
    });
}

fn concrete_opt_bench(c: &mut Criterion) {
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

    let row = vec![5, 3, 4, 3, 6, 2, 1, 1];
    let mat = circ_mat(&row);
    let rc: Vec<Scalar> = (0..8).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Concrete Opt Matrix Bench", move |bench| {
        bench.iter(|| {
            let perm = concrete_opt(black_box(&input), black_box(&mat), black_box(&rc));
            black_box(perm)
        });
    });
}

fn concrete_bench(c: &mut Criterion) {
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

    let row = vec![5, 3, 4, 3, 6, 2, 1, 1];
    let mat = circ_mat(&row);
    let rc: Vec<Scalar> = (0..8).map(|_| utils::random_scalar(true)).collect();

    c.bench_function("Concrete Matrix Bench", move |bench| {
        bench.iter(|| {
            let perm = concrete(black_box(&input), black_box(&mat), black_box(&rc));
            black_box(perm)
        });
    });
}

fn criterion_benchmark_matrices(c: &mut Criterion) {
    concrete_bench(c);
    concrete_opt_bench(c);
    concrete_opt_no_reduce_bench(c);
    griffin_bench(c);
    griffin2_bench(c);
    griffin_opt_bench(c);
    griffin_opt_no_reduce_bench(c);
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark_matrices
);
criterion_main!(benches);
