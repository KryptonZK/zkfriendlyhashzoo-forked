use crate::fields::f31::Field32;
use ff::PrimeField;
use std::convert::TryInto;

/// The defining, first column of the (circulant) MDS matrix.
// pub const MDS_MATRIX_FIRST_COLUMN: [i64; 32] = [
//     87474966, 643163553, 230334003, 1739211637, 1490713897, 1941039814, 1467869360, 2071410473,
//     1358088042, 136918578, 1593997632, 1302354504, 1950203872, 60752863, 1160738787, 2024301343,
//     1740948995, 1137173171, 853820823, 1717383379, 669885656, 927918294, 1575767662, 1284124534,
//     593719941, 1520034124, 806711693, 1410252806, 937082352, 1387408269, 1138910529, 500304516,
// ];

#[allow(unused)]
pub fn mds_multiply<F: Field32 + PrimeField>(state: &mut [F; 24]) {
    let mut lo: [u64; 32] = [0; 32];
    let mut hi: [u64; 32] = [0; 32];

    for (i, b) in state.iter().enumerate() {
        let s = b.to_u32() as u64;
        lo[i] = s as u16 as u64;
        hi[i] = s >> 16;
    }

    lo = fast_cyclomul32(&lo);
    hi = fast_cyclomul32(&hi);

    for r in 0..24 {
        let s = ((hi[r] >> 16) << 1) + ((hi[r] as u16 as u64) << 16) + lo[r];
        state[r] = F::from_u64(s);
    }
}

#[allow(unused)]
pub fn mds_multiply_with_rc<F: Field32 + PrimeField>(
    state: &mut [F; 24],
    round_constants: &[F; 24],
) {
    let mut lo: [u64; 32] = [0; 32];
    let mut hi: [u64; 32] = [0; 32];

    for (i, b) in state.iter().enumerate() {
        let s = b.to_u32() as u64;
        lo[i] = s as u16 as u64;
        hi[i] = s >> 16;
    }

    lo = fast_cyclomul32(&lo);
    hi = fast_cyclomul32(&hi);

    for r in 0..24 {
        let s = ((hi[r] >> 16) << 1)
            + ((hi[r] as u16 as u64) << 16)
            + lo[r]
            + round_constants[r].to_u32() as u64;
        state[r] = F::from_u64(s);
    }
}

#[allow(unused)]
pub fn mds_multiply_u64<F: Field32 + PrimeField>(state: &mut [u64; 24]) {
    let mut lo: [u64; 32] = [0; 32];
    let mut hi: [u64; 32] = [0; 32];

    for (i, b) in state.iter().enumerate() {
        lo[i] = *b as u16 as u64;
        hi[i] = *b >> 16;
    }

    lo = fast_cyclomul32(&lo);
    hi = fast_cyclomul32(&hi);

    for r in 0..24 {
        state[r] = ((hi[r] >> 16) << 1) + ((hi[r] as u16 as u64) << 16) + lo[r];
        F::reduce64(&mut state[r]);
    }
}

#[allow(unused)]
pub fn mds_multiply_with_rc_u64<F: Field32 + PrimeField>(
    state: &mut [u64; 24],
    round_constants: &[F; 24],
) {
    let mut lo: [u64; 32] = [0; 32];
    let mut hi: [u64; 32] = [0; 32];

    for (i, b) in state.iter().enumerate() {
        lo[i] = *b as u16 as u64;
        hi[i] = *b >> 16;
    }

    lo = fast_cyclomul32(&lo);
    hi = fast_cyclomul32(&hi);

    for r in 0..24 {
        state[r] = ((hi[r] >> 16) << 1)
            + ((hi[r] as u16 as u64) << 16)
            + lo[r]
            + round_constants[r].to_u32() as u64;
        F::reduce64(&mut state[r]);
    }
}

#[inline(always)]
fn fast_cyclomul32(f: &[u64; 32]) -> [u64; 32] {
    const N: usize = 16;
    let mut ff_lo = [0i64; N];
    let mut ff_hi = [0i64; N];
    for i in 0..N {
        ff_lo[i] = f[i] as i64 + f[i + N] as i64;
        ff_hi[i] = f[i] as i64 - f[i + N] as i64;
    }

    let hh_lo = fast_cyclomul16(ff_lo);
    let hh_hi = complex_negacyclomul16(ff_hi);

    let mut hh = [0u64; 2 * N];
    for i in 0..N {
        hh[i] = ((hh_lo[i] + hh_hi[i]) >> 1) as u64;
        hh[i + N] = ((hh_lo[i] - hh_hi[i]) >> 1) as u64;
    }

    hh
}

#[inline(always)]
fn fast_cyclomul16(f: [i64; 16]) -> [i64; 16] {
    const N: usize = 8;
    let mut ff_lo = [0i64; N];
    let mut ff_hi = [0i64; N];
    for i in 0..N {
        ff_lo[i] = f[i] + f[i + N];
        ff_hi[i] = f[i] - f[i + N];
    }

    let hh_lo = fast_cyclomul8(ff_lo);
    let hh_hi = complex_negacyclomul8(ff_hi);

    let mut hh = [0i64; 2 * N];
    for i in 0..N {
        hh[i] = (hh_lo[i] + hh_hi[i]) >> 1;
        hh[i + N] = (hh_lo[i] - hh_hi[i]) >> 1;
    }

    hh
}

#[inline(always)]
fn fast_cyclomul8(f: [i64; 8]) -> [i64; 8] {
    const N: usize = 4;
    let mut ff_lo = [0i64; N];
    let mut ff_hi = [0i64; N];
    for i in 0..N {
        ff_lo[i] = f[i] + f[i + N];
        ff_hi[i] = f[i] - f[i + N];
    }

    let hh_lo = fast_cyclomul4(ff_lo);
    let hh_hi = complex_negacyclomul4(ff_hi);

    let mut hh = [0i64; 2 * N];
    for i in 0..N {
        hh[i] = (hh_lo[i] + hh_hi[i]) >> 1;
        hh[i + N] = (hh_lo[i] - hh_hi[i]) >> 1;
    }

    hh
}

#[inline(always)]
fn fast_cyclomul4(f: [i64; 4]) -> [i64; 4] {
    const N: usize = 2;
    let mut ff_lo = [0i64; N];
    let mut ff_hi = [0i64; N];
    for i in 0..N {
        ff_lo[i] = f[i] + f[i + N];
        ff_hi[i] = f[i] - f[i + N];
    }

    let hh_lo = fast_cyclomul2(ff_lo);
    let hh_hi = complex_negacyclomul2(ff_hi);

    let mut hh = [0i64; 2 * N];
    for i in 0..N {
        hh[i] = (hh_lo[i] + hh_hi[i]) >> 1;
        hh[i + N] = (hh_lo[i] - hh_hi[i]) >> 1;
    }

    hh
}

#[inline(always)]
fn fast_cyclomul2(f: [i64; 2]) -> [i64; 2] {
    let ff_lo = f[0] + f[1];
    let ff_hi = f[0] - f[1];

    let hh_lo = ff_lo * 37460020068;
    let hh_hi = ff_hi * -2147483648;

    let mut hh = [0i64; 2];
    hh[0] = (hh_lo + hh_hi) >> 1;
    hh[1] = (hh_lo - hh_hi) >> 1;

    hh
}

#[inline(always)]
fn complex_negacyclomul16(f: [i64; 16]) -> [i64; 16] {
    const N: usize = 8;

    let mut f0 = [(0i64, 0i64); N];
    // let mut f1 = [(0i64,0i64); N];

    for i in 0..N {
        f0[i] = (f[i], -f[N + i]);
        // f1[i] = (f[i],  f[N+i]);
    }

    let h0 = complex_karatsuba8(f0);
    // h1 = complex_karatsuba(f1, g1)

    // h = a * h0 + b * h1
    // where a = 2^-1 * (i*X^(n/2) + 1)
    // and  b = 2^-1 * (-i*X^(n/2) + 1)

    let mut h = [0i64; 3 * N - 1];
    for i in 0..(2 * N - 1) {
        h[i] += h0[i].0;
        h[i + N] -= h0[i].1;
        // h[i] += h0[i].0 / 2
        // h[i+N] -= h0[i].1 / 2
        // h[i] += h1[i].0 / 2
        // h[i+N] -= h1[i].1 / 2
    }

    let mut hh = [0i64; 2 * N];
    for i in 0..(2 * N) {
        hh[i] += h[i];
    }
    for i in (2 * N)..(3 * N - 1) {
        hh[i - 2 * N] -= h[i];
    }

    hh
}

#[inline(always)]
fn complex_karatsuba8(f: [(i64, i64); 8]) -> [(i64, i64); 15] {
    const N: usize = 4;

    let g = [
        (-1653474029, -764368101),
        (-494009618, 1383115546),
        (-623486820, -787285939),
        (21828258, 107898302),
        (820828241, -1013121520),
        (1013121520, 1326655406),
        (-107898302, -21828258),
        (787285939, -1523996827),
    ];

    let ff = complex_sum::<4>(f[..N].try_into().unwrap(), f[N..].try_into().unwrap());
    let gg = [
        (-832645788, -1777489621),
        (519111902, 2709770952),
        (-731385122, -809114197),
        (809114197, -1416098525),
    ];

    let lo = complex_karatsuba4(f[..N].try_into().unwrap(), g[..N].try_into().unwrap());
    let hi = complex_karatsuba4(f[N..].try_into().unwrap(), g[N..].try_into().unwrap());

    let li = complex_diff::<7>(complex_karatsuba4(ff, gg), complex_sum::<7>(lo, hi));

    let mut result = [(0i64, 0i64); 4 * N - 1];
    for i in 0..(2 * N - 1) {
        result[i].0 = lo[i].0;
        result[i].1 = lo[i].1;
    }
    for i in 0..(2 * N - 1) {
        result[N + i].0 += li[i].0;
        result[N + i].1 += li[i].1;
    }
    for i in 0..(2 * N - 1) {
        result[2 * N + i].0 += hi[i].0;
        result[2 * N + i].1 += hi[i].1;
    }

    result
}

#[inline(always)]
fn complex_negacyclomul8(f: [i64; 8]) -> [i64; 8] {
    const N: usize = 4;

    let mut f0 = [(0i64, 0i64); N];
    // let mut f1 = [(0i64,0i64); N];
    let g0 = [
        (-123384022, 726686671),
        (123384022, -1420796976),
        (-1316554499, -743987706),
        (743987706, -830929148),
    ];

    for i in 0..N {
        f0[i] = (f[i], -f[N + i]);
        // f1[i] = (f[i],  f[N+i]);
    }

    let h0 = complex_karatsuba4(f0, g0);
    // h1 = complex_karatsuba(f1, g1)

    // h = a * h0 + b * h1
    // where a = 2^-1 * (i*X^(n/2) + 1)
    // and  b = 2^-1 * (-i*X^(n/2) + 1)

    let mut h = [0i64; 3 * N - 1];
    for i in 0..(2 * N - 1) {
        h[i] += h0[i].0;
        h[i + N] -= h0[i].1;
        // h[i] += h0[i].0 / 2
        // h[i+N] -= h0[i].1 / 2
        // h[i] += h1[i].0 / 2
        // h[i+N] -= h1[i].1 / 2
    }

    let mut hh = [0i64; 2 * N];
    for i in 0..(2 * N) {
        hh[i] += h[i];
    }
    for i in (2 * N)..(3 * N - 1) {
        hh[i - 2 * N] -= h[i];
    }

    hh
}

#[inline(always)]
fn complex_karatsuba4(f: [(i64, i64); 4], g: [(i64, i64); 4]) -> [(i64, i64); 7] {
    const N: usize = 2;

    let ff = complex_sum::<2>(f[..N].try_into().unwrap(), f[N..].try_into().unwrap());
    let gg = complex_sum::<2>(g[..N].try_into().unwrap(), g[N..].try_into().unwrap());

    let lo = complex_karatsuba2(f[..N].try_into().unwrap(), g[..N].try_into().unwrap());
    let hi = complex_karatsuba2(f[N..].try_into().unwrap(), g[N..].try_into().unwrap());

    let li = complex_diff::<3>(complex_karatsuba2(ff, gg), complex_sum::<3>(lo, hi));

    let mut result = [(0i64, 0i64); 4 * N - 1];
    for i in 0..(2 * N - 1) {
        result[i].0 = lo[i].0;
        result[i].1 = lo[i].1;
    }
    for i in 0..(2 * N - 1) {
        result[N + i].0 += li[i].0;
        result[N + i].1 += li[i].1;
    }
    for i in 0..(2 * N - 1) {
        result[2 * N + i].0 += hi[i].0;
        result[2 * N + i].1 += hi[i].1;
    }

    result
}

#[inline(always)]
fn complex_negacyclomul4(f: [i64; 4]) -> [i64; 4] {
    const N: usize = 2;

    let mut f0 = [(0i64, 0i64); N];
    // let mut f1 = [(0i64,0i64); N];
    let g0 = [(-1267653833, 1858422187), (-879829814, -289061460)];

    for i in 0..N {
        f0[i] = (f[i], -f[N + i]);
        // f1[i] = (f[i],  f[N+i]);
    }

    let h0 = complex_karatsuba2(f0, g0);
    // h1 = complex_karatsuba(f1, g1)

    // h = a * h0 + b * h1
    // where a = 2^-1 * (i*X^(n/2) + 1)
    // and  b = 2^-1 * (-i*X^(n/2) + 1)

    let mut h = [0i64; 4 * N - 1];
    for i in 0..(2 * N - 1) {
        h[i] += h0[i].0;
        h[i + N] -= h0[i].1;
        // h[i] += h0[i].0 / 2
        // h[i+N] -= h0[i].1 / 2
        // h[i] += h1[i].0 / 2
        // h[i+N] -= h1[i].1 / 2
    }

    let mut hh = [0i64; 2 * N];
    for i in 0..(2 * N) {
        hh[i] += h[i];
    }
    for i in (2 * N)..(4 * N - 1) {
        hh[i - 2 * N] -= h[i];
    }

    hh
}

#[inline(always)]
fn complex_negacyclomul2(f: [i64; 2]) -> [i64; 2] {
    let f0 = (f[0], -f[1]);
    let g0 = (-32768, 4294934526);

    let h0 = complex_product(f0, g0);

    [h0.0, -h0.1]
}

#[inline(always)]
fn complex_karatsuba2(f: [(i64, i64); 2], g: [(i64, i64); 2]) -> [(i64, i64); 3] {
    const N: usize = 1;

    let ff = (f[0].0 + f[1].0, f[0].1 + f[1].1);
    let gg = (g[0].0 + g[1].0, g[0].1 + g[1].1);

    let lo = complex_product(f[0], g[0]);
    let hi = complex_product(f[1], g[1]);

    let ff_times_gg = complex_product(ff, gg);
    let lo_plus_hi = (lo.0 + hi.0, lo.1 + hi.1);

    let li = (ff_times_gg.0 - lo_plus_hi.0, ff_times_gg.1 - lo_plus_hi.1);

    let mut result = [(0i64, 0i64); 4 * N - 1];
    result[0].0 += lo.0;
    result[0].1 += lo.1;
    result[N].0 += li.0;
    result[N].1 += li.1;
    result[2 * N].0 += hi.0;
    result[2 * N].1 += hi.1;

    result
}

#[inline(always)]
fn complex_sum<const N: usize>(f: [(i64, i64); N], g: [(i64, i64); N]) -> [(i64, i64); N] {
    let mut h = [(0i64, 0i64); N];
    for i in 0..N {
        h[i].0 = f[i].0 + g[i].0;
        h[i].1 = f[i].1 + g[i].1;
    }
    h
}

#[inline(always)]
fn complex_diff<const N: usize>(f: [(i64, i64); N], g: [(i64, i64); N]) -> [(i64, i64); N] {
    let mut h = [(0i64, 0i64); N];
    for i in 0..N {
        h[i].0 = f[i].0 - g[i].0;
        h[i].1 = f[i].1 - g[i].1;
    }
    h
}

#[inline(always)]
fn complex_product(f: (i64, i64), g: (i64, i64)) -> (i64, i64) {
    // don't karatsuba; this is faster
    (f.0 * g.0 - f.1 * g.1, f.0 * g.1 + f.1 * g.0)
}

///////////////////////////////////////////////////////////////////////////////
// test
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod mds_tests {
    use super::*;
    use crate::fields::{f31::F31, utils};
    use ff::Field;

    static TESTRUNS: usize = 5;
    type Scalar = F31;

    fn matmul(input: &[Scalar], mat: &[Vec<Scalar>]) -> Vec<Scalar> {
        let t = input.len();
        debug_assert!(t <= mat.len());
        let mut out = vec![Scalar::zero(); t];
        for row in 0..t {
            for (col, inp) in input.iter().enumerate().take(t) {
                let mut tmp = mat[row][col];
                tmp.mul_assign(inp);
                out[row].add_assign(&tmp);
            }
        }
        out
    }

    fn circ_mat(row: &[u32]) -> Vec<Vec<Scalar>> {
        let t = row.len();
        let mut mat: Vec<Vec<Scalar>> = Vec::with_capacity(t);
        let mut rot: Vec<Scalar> = row.iter().map(|i| Scalar::from_u32(*i)).collect();
        mat.push(rot.clone());
        for _ in 1..t {
            rot.rotate_right(1);
            mat.push(rot.clone());
        }
        mat
    }

    #[test]
    fn kats() {
        let row = [
            87474966, 500304516, 1138910529, 1387408269, 937082352, 1410252806, 806711693,
            1520034124, 593719941, 1284124534, 1575767662, 927918294, 669885656, 1717383379,
            853820823, 1137173171, 1740948995, 2024301343, 1160738787, 60752863, 1950203872,
            1302354504, 1593997632, 136918578, 1358088042, 2071410473, 1467869360, 1941039814,
            1490713897, 1739211637, 230334003, 643163553,
        ];
        let mat = circ_mat(&row);
        let round_const = [Scalar::zero(); 24];

        for _ in 0..TESTRUNS {
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

            let output1 = matmul(&input, &mat);
            let mut output2 = input.to_owned();
            let mut output3 = input.to_owned();
            mds_multiply_with_rc(&mut output2, &round_const);
            mds_multiply(&mut output3);
            assert_eq!(output1, output2);
            assert_eq!(output1, output3);

            let mut output4 = [0u64; 24];
            for (src, des) in input.iter().zip(output4.iter_mut()) {
                *des = src.to_u32() as u64;
            }
            let mut output5 = output4.to_owned();
            mds_multiply_u64::<Scalar>(&mut output4);
            mds_multiply_with_rc_u64(&mut output5, &round_const);
            for (a, b) in output1.iter().zip(output4.iter()) {
                assert_eq!(a.to_u32(), *b as u32);
            }
            for (a, b) in output1.iter().zip(output5.iter()) {
                assert_eq!(a.to_u32(), *b as u32);
            }
        }
    }
}
