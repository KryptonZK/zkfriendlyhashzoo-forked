// from https://github.com/Neptune-Crypto/twenty-first/blob/master/twenty-first/src/shared_math/tip5.rs

use crate::fields::f31::Field32;
use ff::PrimeField;
use std::convert::TryInto;

/// The defining, first column of the (circulant) MDS matrix.
/// Derived from the SHA-256 hash of the ASCII string “Tip5” by dividing the digest into 16-bit
/// chunks.
// pub const MDS_MATRIX_FIRST_COLUMN: [i64; 16] = [
//     61402, 1108, 28750, 33823, 7454, 43244, 53865, 12034, 56951, 27521, 41351, 40901, 12021, 59689,
//     26798, 17845,
// ];

#[inline(always)]
pub fn mds_multiply<F: Field32 + PrimeField>(state: &mut [F; 16]) {
    mds_multiply_generic(state)
}

#[inline(always)]
pub fn mds_multiply_with_rc<F: Field32 + PrimeField>(
    state: &mut [F; 16],
    round_constants: &[F; 16],
) {
    mds_multiply_with_rc_generic(state, round_constants)
}

#[inline(always)]
pub fn mds_multiply_u64<F: Field32 + PrimeField>(state: &mut [u64; 16]) {
    mds_multiply_u64_generic::<F>(state)
}

#[inline(always)]
pub fn mds_multiply_with_rc_u64<F: Field32 + PrimeField>(
    state: &mut [u64; 16],
    round_constants: &[F; 16],
) {
    mds_multiply_with_rc_u64_generic(state, round_constants)
}

#[allow(unused)]
fn mds_multiply_generic<F: Field32 + PrimeField>(state: &mut [F; 16]) {
    let mut lo: [u64; 16] = [0; 16];
    for (i, b) in state.iter().enumerate() {
        lo[i] = b.to_u32() as u64;
    }

    lo = fast_cyclomul16(&lo);

    for r in 0..16 {
        let s = lo[r];
        state[r] = F::from_u64(s);
    }
}

#[allow(unused)]
fn mds_multiply_with_rc_generic<F: Field32 + PrimeField>(
    state: &mut [F; 16],
    round_constants: &[F; 16],
) {
    let mut lo: [u64; 16] = [0; 16];
    for (i, b) in state.iter().enumerate() {
        lo[i] = b.to_u32() as u64;
    }

    lo = fast_cyclomul16(&lo);

    for r in 0..16 {
        let s = lo[r] + round_constants[r].to_u32() as u64;
        state[r] = F::from_u64(s);
    }
}

#[allow(unused)]
fn mds_multiply_generated<F: Field32 + PrimeField>(state: &mut [F; 16]) {
    let mut lo: [u64; 16] = [0; 16];
    for (i, b) in state.iter().enumerate() {
        lo[i] = b.to_u32() as u64;
    }

    let lo = generated_function(&lo);

    for r in 0..16 {
        let s = lo[r] >> 4;
        state[r] = F::from_u64(s);
    }
}

#[allow(unused)]
fn mds_multiply_with_rc_generated<F: Field32 + PrimeField>(
    state: &mut [F; 16],
    round_constants: &[F; 16],
) {
    let mut lo: [u64; 16] = [0; 16];
    for (i, b) in state.iter().enumerate() {
        lo[i] = b.to_u32() as u64;
    }

    let lo = generated_function(&lo);

    for r in 0..16 {
        let s = (lo[r] >> 4) + round_constants[r].to_u32() as u64;
        state[r] = F::from_u64(s);
    }
}

#[allow(unused)]
fn mds_multiply_u64_generic<F: Field32 + PrimeField>(state: &mut [u64; 16]) {
    *state = fast_cyclomul16(state);

    for el in state.iter_mut() {
        F::reduce64(el);
    }
}

#[allow(unused)]
fn mds_multiply_with_rc_u64_generic<F: Field32 + PrimeField>(
    state: &mut [u64; 16],
    round_constants: &[F; 16],
) {
    let lo = fast_cyclomul16(state);

    for r in 0..16 {
        state[r] = lo[r] + round_constants[r].to_u32() as u64;
        F::reduce64(&mut state[r]);
    }
}

#[allow(unused)]
fn mds_multiply_u64_generated<F: Field32 + PrimeField>(state: &mut [u64; 16]) {
    let lo = generated_function(state);

    for r in 0..16 {
        state[r] = lo[r] >> 4;
        F::reduce64(&mut state[r]);
    }
}

#[allow(unused)]
fn mds_multiply_with_rc_u64_generated<F: Field32 + PrimeField>(
    state: &mut [u64; 16],
    round_constants: &[F; 16],
) {
    let lo = generated_function(state);

    for r in 0..16 {
        state[r] = (lo[r] >> 4) + round_constants[r].to_u32() as u64;
        F::reduce64(&mut state[r]);
    }
}

#[inline(always)]
fn fast_cyclomul16(f: &[u64; 16]) -> [u64; 16] {
    const N: usize = 8;
    let mut ff_lo = [0i64; N];
    let mut ff_hi = [0i64; N];
    for i in 0..N {
        ff_lo[i] = f[i] as i64 + f[i + N] as i64;
        ff_hi[i] = f[i] as i64 - f[i + N] as i64;
    }

    let hh_lo = fast_cyclomul8(ff_lo);
    let hh_hi = complex_negacyclomul8(ff_hi);

    let mut hh = [0u64; 2 * N];
    for i in 0..N {
        hh[i] = ((hh_lo[i] + hh_hi[i]) >> 1) as u64;
        hh[i + N] = ((hh_lo[i] - hh_hi[i]) >> 1) as u64;
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

    let hh_lo = ff_lo * 524757;
    let hh_hi = ff_hi * 52427;

    let mut hh = [0i64; 2];
    hh[0] = (hh_lo + hh_hi) >> 1;
    hh[1] = (hh_lo - hh_hi) >> 1;

    hh
}

#[inline(always)]
fn complex_negacyclomul8(f: [i64; 8]) -> [i64; 8] {
    const N: usize = 4;

    let mut f0 = [(0i64, 0i64); N];
    // let mut f1 = [(0i64,0i64); N];

    for i in 0..N {
        f0[i] = (f[i], -f[N + i]);
        // f1[i] = (f[i],  f[N+i]);
    }

    let h0 = complex_karatsuba4(f0);
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
fn complex_karatsuba4(f: [(i64, i64); 4]) -> [(i64, i64); 7] {
    const N: usize = 2;

    let g = [
        (4451, 4567),
        (-26413, 16445),
        (-12601, -27067),
        (-7078, 5811),
    ];

    let ff = complex_sum::<2>(f[..N].try_into().unwrap(), f[N..].try_into().unwrap());
    let gg = [(-8150, -22500), (-33491, 22256)];

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
    let g0 = [(98878, 10562), (-74304, -44845)];

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
    let g0 = (-12936, -26959);

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

#[inline(always)]
pub fn generated_function(input: &[u64; 16]) -> [u64; 16] {
    let node_34 = (input[0]).wrapping_add(input[8]);
    let node_38 = (input[4]).wrapping_add(input[12]);
    let node_36 = (input[2]).wrapping_add(input[10]);
    let node_40 = (input[6]).wrapping_add(input[14]);
    let node_35 = (input[1]).wrapping_add(input[9]);
    let node_39 = (input[5]).wrapping_add(input[13]);
    let node_37 = (input[3]).wrapping_add(input[11]);
    let node_41 = (input[7]).wrapping_add(input[15]);
    let node_50 = (node_34).wrapping_add(node_38);
    let node_52 = (node_36).wrapping_add(node_40);
    let node_51 = (node_35).wrapping_add(node_39);
    let node_53 = (node_37).wrapping_add(node_41);
    let node_160 = (input[0]).wrapping_sub(input[8]);
    let node_161 = (input[1]).wrapping_sub(input[9]);
    let node_165 = (input[5]).wrapping_sub(input[13]);
    let node_163 = (input[3]).wrapping_sub(input[11]);
    let node_167 = (input[7]).wrapping_sub(input[15]);
    let node_162 = (input[2]).wrapping_sub(input[10]);
    let node_166 = (input[6]).wrapping_sub(input[14]);
    let node_164 = (input[4]).wrapping_sub(input[12]);
    let node_58 = (node_50).wrapping_add(node_52);
    let node_59 = (node_51).wrapping_add(node_53);
    let node_90 = (node_34).wrapping_sub(node_38);
    let node_91 = (node_35).wrapping_sub(node_39);
    let node_93 = (node_37).wrapping_sub(node_41);
    let node_92 = (node_36).wrapping_sub(node_40);
    let node_64 = ((node_58).wrapping_add(node_59)).wrapping_mul(524757);
    let node_67 = ((node_58).wrapping_sub(node_59)).wrapping_mul(52427);
    let node_71 = (node_50).wrapping_sub(node_52);
    let node_72 = (node_51).wrapping_sub(node_53);
    let node_177 = (node_161).wrapping_add(node_165);
    let node_179 = (node_163).wrapping_add(node_167);
    let node_178 = (node_162).wrapping_add(node_166);
    let node_176 = (node_160).wrapping_add(node_164);
    let node_69 = (node_64).wrapping_add(node_67);
    let node_397 =
        ((node_71).wrapping_mul(18446744073709525744)).wrapping_sub((node_72).wrapping_mul(53918));
    let node_1857 = (node_90).wrapping_mul(395512);
    let node_99 = (node_91).wrapping_add(node_93);
    let node_1865 = (node_91).wrapping_mul(18446744073709254400);
    let node_1869 = (node_93).wrapping_mul(179380);
    let node_1873 = (node_92).wrapping_mul(18446744073709509368);
    let node_1879 = (node_160).wrapping_mul(35608);
    let node_185 = (node_161).wrapping_add(node_163);
    let node_1915 = (node_161).wrapping_mul(18446744073709340312);
    let node_1921 = (node_163).wrapping_mul(18446744073709494992);
    let node_1927 = (node_162).wrapping_mul(18446744073709450808);
    let node_228 = (node_165).wrapping_add(node_167);
    let node_1939 = (node_165).wrapping_mul(18446744073709420056);
    let node_1945 = (node_167).wrapping_mul(18446744073709505128);
    let node_1951 = (node_166).wrapping_mul(216536);
    let node_1957 = (node_164).wrapping_mul(18446744073709515080);
    let node_70 = (node_64).wrapping_sub(node_67);
    let node_702 =
        ((node_71).wrapping_mul(53918)).wrapping_add((node_72).wrapping_mul(18446744073709525744));
    let node_1961 = (node_90).wrapping_mul(18446744073709254400);
    let node_1963 = (node_91).wrapping_mul(395512);
    let node_1965 = (node_92).wrapping_mul(179380);
    let node_1967 = (node_93).wrapping_mul(18446744073709509368);
    let node_1970 = (node_160).wrapping_mul(18446744073709340312);
    let node_1973 = (node_161).wrapping_mul(35608);
    let node_1982 = (node_162).wrapping_mul(18446744073709494992);
    let node_1985 = (node_163).wrapping_mul(18446744073709450808);
    let node_1988 = (node_166).wrapping_mul(18446744073709505128);
    let node_1991 = (node_167).wrapping_mul(216536);
    let node_1994 = (node_164).wrapping_mul(18446744073709420056);
    let node_1997 = (node_165).wrapping_mul(18446744073709515080);
    let node_98 = (node_90).wrapping_add(node_92);
    let node_184 = (node_160).wrapping_add(node_162);
    let node_227 = (node_164).wrapping_add(node_166);
    let node_86 = (node_69).wrapping_add(node_397);
    let node_403 = (node_1857).wrapping_sub(
        ((((node_99).wrapping_mul(18446744073709433780)).wrapping_sub(node_1865))
            .wrapping_sub(node_1869))
        .wrapping_add(node_1873),
    );
    let node_271 = (node_177).wrapping_add(node_179);
    let node_1891 = (node_177).wrapping_mul(18446744073709208752);
    let node_1897 = (node_179).wrapping_mul(18446744073709448504);
    let node_1903 = (node_178).wrapping_mul(115728);
    let node_1909 = (node_185).wrapping_mul(18446744073709283688);
    let node_1933 = (node_228).wrapping_mul(18446744073709373568);
    let node_88 = (node_70).wrapping_add(node_702);
    let node_708 =
        ((node_1961).wrapping_add(node_1963)).wrapping_sub((node_1965).wrapping_add(node_1967));
    let node_1976 = (node_178).wrapping_mul(18446744073709448504);
    let node_1979 = (node_179).wrapping_mul(115728);
    let node_87 = (node_69).wrapping_sub(node_397);
    let node_897 = ((((node_1865).wrapping_add((node_98).wrapping_mul(353264)))
        .wrapping_sub(node_1857))
    .wrapping_sub(node_1873))
    .wrapping_sub(node_1869);
    let node_2007 = (node_184).wrapping_mul(18446744073709486416);
    let node_2013 = (node_227).wrapping_mul(180000);
    let node_89 = (node_70).wrapping_sub(node_702);
    let node_1077 = ((((node_98).wrapping_mul(18446744073709433780))
        .wrapping_add((node_99).wrapping_mul(353264)))
    .wrapping_sub((node_1961).wrapping_add(node_1963)))
    .wrapping_sub((node_1965).wrapping_add(node_1967));
    let node_2020 = (node_184).wrapping_mul(18446744073709283688);
    let node_2023 = (node_185).wrapping_mul(18446744073709486416);
    let node_2026 = (node_227).wrapping_mul(18446744073709373568);
    let node_2029 = (node_228).wrapping_mul(180000);
    let node_2035 = (node_176).wrapping_mul(18446744073709550688);
    let node_2038 = (node_176).wrapping_mul(18446744073709208752);
    let node_2041 = (node_177).wrapping_mul(18446744073709550688);
    let node_270 = (node_176).wrapping_add(node_178);
    let node_152 = (node_86).wrapping_add(node_403);
    let node_412 = (node_1879).wrapping_sub(
        (((((((node_271).wrapping_mul(18446744073709105640)).wrapping_sub(node_1891))
            .wrapping_sub(node_1897))
        .wrapping_add(node_1903))
        .wrapping_sub(
            (((node_1909).wrapping_sub(node_1915)).wrapping_sub(node_1921)).wrapping_add(node_1927),
        ))
        .wrapping_sub(
            (((node_1933).wrapping_sub(node_1939)).wrapping_sub(node_1945)).wrapping_add(node_1951),
        ))
        .wrapping_add(node_1957),
    );
    let node_154 = (node_88).wrapping_add(node_708);
    let node_717 = ((node_1970).wrapping_add(node_1973)).wrapping_sub(
        ((((node_1976).wrapping_add(node_1979)).wrapping_sub((node_1982).wrapping_add(node_1985)))
            .wrapping_sub((node_1988).wrapping_add(node_1991)))
        .wrapping_add((node_1994).wrapping_add(node_1997)),
    );
    let node_156 = (node_87).wrapping_add(node_897);
    let node_906 = ((((node_1915).wrapping_add(node_2007)).wrapping_sub(node_1879))
        .wrapping_sub(node_1927))
    .wrapping_sub(
        (((node_1897).wrapping_sub(node_1921)).wrapping_sub(node_1945)).wrapping_add(
            (((node_1939).wrapping_add(node_2013)).wrapping_sub(node_1957)).wrapping_sub(node_1951),
        ),
    );
    let node_158 = (node_89).wrapping_add(node_1077);
    let node_1086 = ((((node_2020).wrapping_add(node_2023))
        .wrapping_sub((node_1970).wrapping_add(node_1973)))
    .wrapping_sub((node_1982).wrapping_add(node_1985)))
    .wrapping_sub(
        (((node_2026).wrapping_add(node_2029)).wrapping_sub((node_1994).wrapping_add(node_1997)))
            .wrapping_sub((node_1988).wrapping_add(node_1991)),
    );
    let node_153 = (node_86).wrapping_sub(node_403);
    let node_1237 = (((((((node_1909).wrapping_sub(node_1915)).wrapping_sub(node_1921))
        .wrapping_add(node_1927))
    .wrapping_add(node_2035))
    .wrapping_sub(node_1879))
    .wrapping_sub(node_1957))
    .wrapping_sub(
        (((node_1933).wrapping_sub(node_1939)).wrapping_sub(node_1945)).wrapping_add(node_1951),
    );
    let node_155 = (node_88).wrapping_sub(node_708);
    let node_1375 = (((((node_1982).wrapping_add(node_1985))
        .wrapping_add((node_2038).wrapping_add(node_2041)))
    .wrapping_sub((node_1970).wrapping_add(node_1973)))
    .wrapping_sub((node_1994).wrapping_add(node_1997)))
    .wrapping_sub((node_1988).wrapping_add(node_1991));
    let node_157 = (node_87).wrapping_sub(node_897);
    let node_1492 = ((((node_1921).wrapping_add(
        (((node_1891).wrapping_add((node_270).wrapping_mul(114800))).wrapping_sub(node_2035))
            .wrapping_sub(node_1903),
    ))
    .wrapping_sub(
        (((node_1915).wrapping_add(node_2007)).wrapping_sub(node_1879)).wrapping_sub(node_1927),
    ))
    .wrapping_sub(
        (((node_1939).wrapping_add(node_2013)).wrapping_sub(node_1957)).wrapping_sub(node_1951),
    ))
    .wrapping_sub(node_1945);
    let node_159 = (node_89).wrapping_sub(node_1077);
    let node_1657 = ((((((node_270).wrapping_mul(18446744073709105640))
        .wrapping_add((node_271).wrapping_mul(114800)))
    .wrapping_sub((node_2038).wrapping_add(node_2041)))
    .wrapping_sub((node_1976).wrapping_add(node_1979)))
    .wrapping_sub(
        (((node_2020).wrapping_add(node_2023)).wrapping_sub((node_1970).wrapping_add(node_1973)))
            .wrapping_sub((node_1982).wrapping_add(node_1985)),
    ))
    .wrapping_sub(
        (((node_2026).wrapping_add(node_2029)).wrapping_sub((node_1994).wrapping_add(node_1997)))
            .wrapping_sub((node_1988).wrapping_add(node_1991)),
    );

    [
        (node_152).wrapping_add(node_412),
        (node_154).wrapping_add(node_717),
        (node_156).wrapping_add(node_906),
        (node_158).wrapping_add(node_1086),
        (node_153).wrapping_add(node_1237),
        (node_155).wrapping_add(node_1375),
        (node_157).wrapping_add(node_1492),
        (node_159).wrapping_add(node_1657),
        (node_152).wrapping_sub(node_412),
        (node_154).wrapping_sub(node_717),
        (node_156).wrapping_sub(node_906),
        (node_158).wrapping_sub(node_1086),
        (node_153).wrapping_sub(node_1237),
        (node_155).wrapping_sub(node_1375),
        (node_157).wrapping_sub(node_1492),
        (node_159).wrapping_sub(node_1657),
    ]
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
        let t = mat.len();
        debug_assert!(t == input.len());
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
            61402, 17845, 26798, 59689, 12021, 40901, 41351, 27521, 56951, 12034, 53865, 43244,
            7454, 33823, 28750, 1108,
        ];
        let mat = circ_mat(&row);
        let round_const = [Scalar::zero(); 16];

        for _ in 0..TESTRUNS {
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

            let output1 = matmul(&input, &mat);
            let mut output2 = input.to_owned();
            let mut output3 = input.to_owned();
            let mut output4 = input.to_owned();
            let mut output5 = input.to_owned();
            mds_multiply_with_rc_generic(&mut output2, &round_const);
            mds_multiply_with_rc_generated(&mut output3, &round_const);
            mds_multiply_generic(&mut output4);
            mds_multiply_generated(&mut output5);
            assert_eq!(output1, output2);
            assert_eq!(output1, output3);
            assert_eq!(output1, output4);
            assert_eq!(output1, output5);

            let mut output6 = [0u64; 16];
            for (src, des) in input.iter().zip(output6.iter_mut()) {
                *des = src.to_u32() as u64;
            }
            let mut output7 = output6.to_owned();
            let mut output8 = output6.to_owned();
            let mut output9 = output6.to_owned();
            mds_multiply_with_rc_u64_generic(&mut output6, &round_const);
            mds_multiply_with_rc_u64_generated(&mut output7, &round_const);
            mds_multiply_u64_generic::<Scalar>(&mut output8);
            mds_multiply_u64_generated::<Scalar>(&mut output9);
            for (a, b) in output1.iter().zip(output6.iter()) {
                assert_eq!(a.to_u32(), *b as u32);
            }
            for (a, b) in output1.iter().zip(output7.iter()) {
                assert_eq!(a.to_u32(), *b as u32);
            }
            for (a, b) in output1.iter().zip(output8.iter()) {
                assert_eq!(a.to_u32(), *b as u32);
            }
            for (a, b) in output1.iter().zip(output9.iter()) {
                assert_eq!(a.to_u32(), *b as u32);
            }
        }
    }
}
