use crate::fields::f64::Field64;
use ff::PrimeField;
use generic_array::typenum::{Unsigned, U4};
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake128, Shake128Reader,
};
use std::convert::TryInto;

// weird workaround for not allowed const generics in arrays
pub(crate) type BarsType = U4; // U4 = 4 bars per layer

#[derive(Clone, Debug)]
pub struct Monolith64Params<F: PrimeField + Field64, const T: usize> {
    pub(crate) round_constants: Vec<[F; T]>,
    #[allow(unused)]
    pub(crate) mds: [[F; T]; T],
    pub(crate) lookup: Vec<u16>,
}

impl<F: PrimeField + Field64, const T: usize> Monolith64Params<F, T> {
    pub const R: usize = 6;
    pub const BARS: usize = BarsType::USIZE; // Change BarsType to change the value
    pub const INIT_SHAKE: &'static str = "Monolith";

    pub fn new() -> Self {
        assert_eq!(F::NUM_BITS, 64);
        assert!(T >= 8);
        assert_eq!(T % 4, 0);
        let round_constants = Self::instantiate_rc();
        let lookup = Self::instantiate_lookup();
        let mds = Self::get_mds();

        Monolith64Params {
            round_constants,
            mds,
            lookup,
        }
    }

    fn circ_mat(row: &[u64; T]) -> [[F; T]; T] {
        let mut mat = [[F::zero(); T]; T];
        let mut rot: Vec<F> = row.iter().map(|i| F::from_u64(*i)).collect();
        mat[0].copy_from_slice(&rot);
        for row in mat.iter_mut().skip(1) {
            rot.rotate_right(1);
            row.copy_from_slice(&rot);
        }
        mat
    }

    fn get_mds() -> [[F; T]; T] {
        if T == 8 {
            let row = [23, 8, 13, 10, 7, 6, 21, 8];
            Self::circ_mat(row.as_ref().try_into().unwrap())
        } else if T == 12 {
            let row = [7, 23, 8, 26, 13, 10, 9, 7, 6, 22, 21, 8];
            Self::circ_mat(row.as_ref().try_into().unwrap())
        } else {
            let mut shake = Shake128::default();
            shake.update(Self::INIT_SHAKE.as_bytes());
            shake.update(&[T as u8, Self::R as u8]);
            shake.update(&u32::to_le_bytes(F::char().as_ref()[0] as u32));
            shake.update(&[16, 15]);
            shake.update("MDS".as_bytes());
            let mut shake = shake.finalize_xof();
            Self::cauchy_mds_matrix(&mut shake)
        }
    }

    //----------------------------------------------------------------
    // construct cauchy matrix:
    // get t distinct x_i
    // get t distinct y_i
    // if for all i, j: x_i + y_i != 0
    // then: a_i_j = (x_i + y_j)^-1 is MDS
    // construct:
    // - sample s+t y_i with floor(log_2(p)) - 1 bits
    // - set x_i to be the least significant ceil(log_2(p)) - r_bits - 2 bits of x_i
    // - x_i need to be distinct -> y_i also distinct
    // - requires sampling of s+t random values of size floor(log_2(p))
    fn cauchy_mds_matrix(shake: &mut Shake128Reader) -> [[F; T]; T] {
        let mut p = F::char().as_ref()[0].to_owned();
        let mut tmp = 0u64;
        while p != 0 {
            tmp += 1;
            p >>= 1;
        }
        // reduce two: to prevent that x_i + y_i = 0 for MDS
        let x_mask = (1u64 << (tmp - 7 - 2)) - 1;
        let y_mask = ((1u64 << tmp) - 1) >> 2;

        let mut res = [[F::zero(); T]; T];

        let y = Self::get_random_yi(shake, x_mask, y_mask);
        let mut x = y.to_owned();
        x.iter_mut().for_each(|xi| *xi &= x_mask);

        for (i, xi) in x.iter().enumerate() {
            for (j, yj) in y.iter().enumerate() {
                res[i][j] = F::from_u64(xi + yj).inverse().unwrap();
            }
        }

        res
    }

    fn get_random_yi(shake: &mut Shake128Reader, x_mask: u64, y_mask: u64) -> [u64; T] {
        let mut res = [0; T];
        for i in 0..T {
            let mut valid = false;
            while !valid {
                let mut rand = [0u8; 8];
                shake.read(&mut rand);
                let y_i = u64::from_be_bytes(rand) & y_mask;
                // check distinct x_i
                let x_i = y_i & x_mask;
                valid = true;
                for r in res.iter().take(i) {
                    if r & x_mask == x_i {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    res[i] = y_i;
                }
            }
        }

        res
    }

    fn bar0_8(limb: u8) -> u8 {
        let limbl1 = (limb >> 7) | (limb << 1); //left rot by 1
        let limbl2 = (limb >> 6) | (limb << 2); //left rot by 2
        let limbl3 = (limb >> 5) | (limb << 3); //left rot by 3

        //yi = xi +  (1 + x{i+1}) * x{i+2} * x{i+3}
        let tmp = limb ^ !limbl1 & limbl2 & limbl3;
        (tmp >> 7) | (tmp << 1) // Final rotate for less S(x) = x
    }

    // We keep a u16 lookup here
    fn instantiate_lookup() -> Vec<u16> {
        (0..=u16::MAX)
            .map(|i| {
                let a = (i >> 8) as u8;
                let b = i as u8;
                ((Self::bar0_8(a) as u16) << 8) | Self::bar0_8(b) as u16
            })
            .collect()
    }

    fn init_shake() -> Shake128Reader {
        // RCp<t><r><p in little endian><partition of sboxes>
        let mut shake = Shake128::default();
        shake.update(Self::INIT_SHAKE.as_bytes());
        shake.update(&[T as u8, Self::R as u8]);
        shake.update(&u64::to_le_bytes(F::char().as_ref()[0]));
        shake.update(&[8, 8, 8, 8, 8, 8, 8, 8]);
        shake.finalize_xof()
    }

    fn random_field_element(shake: &mut Shake128Reader) -> F {
        loop {
            let mut rnd = [0u8; 8];
            shake.read(&mut rnd);
            let repr = F::Repr::from(u64::from_le_bytes(rnd));
            let res = F::from_repr(repr);
            if let Ok(r) = res {
                return r;
            }
        }
    }

    fn instantiate_rc() -> Vec<[F; T]> {
        let mut shake = Self::init_shake();
        (0..Self::R - 1)
            .map(|_| {
                let mut rc = [F::zero(); T];
                rc.iter_mut()
                    .for_each(|el| *el = Self::random_field_element(&mut shake));
                rc
            })
            .collect()
    }

    pub fn get_t(&self) -> usize {
        T
    }

    pub fn get_rounds(&self) -> usize {
        Self::R
    }

    pub fn get_rc(&self, i: usize) -> [F; T] {
        self.round_constants[i]
    }
}

impl<F: PrimeField + Field64, const T: usize> Default for Monolith64Params<F, T> {
    fn default() -> Self {
        Self::new()
    }
}
