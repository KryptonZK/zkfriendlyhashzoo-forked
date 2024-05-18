use super::rescue_params::RescueParams;
use crate::merkle_tree::merkle_tree_fp::MerkleTreeHash;
use ff::PrimeField;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Rescue<S: PrimeField> {
    pub(crate) params: Arc<RescueParams<S>>,
}

impl<S: PrimeField> Rescue<S> {
    pub fn new(params: &Arc<RescueParams<S>>) -> Self {
        Rescue {
            params: Arc::clone(params),
        }
    }

    pub fn get_t(&self) -> usize {
        self.params.t
    }

    pub fn permutation(&self, input: &[S]) -> Vec<S> {
        let t = self.params.t;
        assert_eq!(input.len(), t);

        let mut current_state = input.to_owned();

        // initial RC
        current_state = Self::add_rc(&current_state, &self.params.round_constants[0]);

        for r in 0..self.params.rounds {
            current_state = self.sbox_inverse(&current_state);
            current_state = self.affine(&current_state, 2 * r + 1);
            current_state = self.sbox(&current_state);
            current_state = self.affine(&current_state, 2 * r + 2);
        }

        current_state
    }

    fn sbox(&self, input: &[S]) -> Vec<S> {
        input
            .iter()
            .map(|el| {
                let mut el2 = *el;
                el2.square();

                match self.params.d {
                    3 => {
                        let mut out = el2;
                        out.mul_assign(el);
                        out
                    }
                    5 => {
                        let mut out = el2;
                        out.square();
                        out.mul_assign(el);
                        out
                    }
                    _ => {
                        panic!();
                    }
                }
            })
            .collect()
    }

    fn sbox_inverse(&self, input: &[S]) -> Vec<S> {
        input
            .iter()
            .map(|el| {
                if *el == S::zero() {
                    *el
                } else {
                    el.pow(self.params.d_inv)
                }
            })
            .collect()
    }

    fn affine(&self, input: &[S], round: usize) -> Vec<S> {
        let mat_result = self.matmul(input, &self.params.mds);
        Self::add_rc(&mat_result, &self.params.round_constants[round])
    }

    fn matmul(&self, input: &[S], mat: &[Vec<S>]) -> Vec<S> {
        let t = mat.len();
        debug_assert!(t == input.len());
        let mut out = vec![S::zero(); t];
        // TODO check if really faster
        for row in 0..t {
            for (col, inp) in input.iter().enumerate().take(t) {
                let mut tmp = mat[row][col];
                tmp.mul_assign(inp);
                out[row].add_assign(&tmp);
            }
        }
        out
    }

    fn add_rc(input: &[S], round_constants: &[S]) -> Vec<S> {
        debug_assert!(input.len() == round_constants.len());
        input
            .iter()
            .zip(round_constants.iter())
            .map(|(a, b)| {
                let mut r = *a;
                r.add_assign(b);
                r
            })
            .collect()
    }
}

impl<F: PrimeField> MerkleTreeHash<F> for Rescue<F> {
    fn compress(&self, input: &[&F; 2]) -> F {
        self.permutation(&[input[0].to_owned(), input[1].to_owned(), F::zero()])[0]
    }
}

#[cfg(test)]
mod rescue_tests_bls12 {
    use super::*;
    use crate::fields::{bls12::FpBLS12, utils};
    use crate::rescue::rescue_instance_bls12::RESCUE_BLS_PARAMS;
    use ff::{from_hex, Field};

    type Scalar = FpBLS12;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let rescue = Rescue::new(&RESCUE_BLS_PARAMS);
        let t = rescue.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = rescue.permutation(&input1);
            let perm2 = rescue.permutation(&input1);
            let perm3 = rescue.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    #[test]
    fn kats() {
        let rescue = Rescue::new(&RESCUE_BLS_PARAMS);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = rescue.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x3a6da171e7d612f45c04bff4fb100efd6d85fbbdc78b49872947ca7c5be9e87a").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x65843d0bfa54f9891aedd014bed810acb9a7c9613c724855b312a568d9fa3b7a").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x6a724ef437d9280246325184cfa3844a90553cf5b01484a170a5d92a996e4f06").unwrap(),
        );
    }
}

#[cfg(test)]
mod rescue_tests_bn256 {
    use super::*;
    use crate::fields::{bn256::FpBN256, utils};
    use crate::rescue::rescue_instance_bn256::RESCUE_BN_PARAMS;
    use ff::{from_hex, Field};

    type Scalar = FpBN256;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let rescue = Rescue::new(&RESCUE_BN_PARAMS);
        let t = rescue.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = rescue.permutation(&input1);
            let perm2 = rescue.permutation(&input1);
            let perm3 = rescue.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    #[test]
    fn kats() {
        let rescue = Rescue::new(&RESCUE_BN_PARAMS);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = rescue.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x2262a88c8b641065446e84e3fa132210312c076e9b056c972986dbc775c42e89").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x19185415565c3847d80a72e8341b1ddbeb10d8f119a2132e127370e34b6b312e").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x16c41359b114c56cf7b00c6a22de88a9bac5e1d5957cf02790a75a66d4566060").unwrap(),
        );
    }
}

#[cfg(test)]
mod rescue_tests_st {
    use super::*;
    use crate::fields::{st::FpST, utils};
    use crate::rescue::rescue_instance_st::RESCUE_ST_PARAMS;
    use ff::{from_hex, Field};

    type Scalar = FpST;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let rescue = Rescue::new(&RESCUE_ST_PARAMS);
        let t = rescue.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = rescue.permutation(&input1);
            let perm2 = rescue.permutation(&input1);
            let perm3 = rescue.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    #[test]
    fn kats() {
        let rescue = Rescue::new(&RESCUE_ST_PARAMS);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = rescue.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x02778ceedfc5dca26a811aed72ce59f5a6721fc65f5f61d2dd8fbd333b8a6ebb").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x039c581707ec0a5ed908301088b8ebbd1ea815c8db2368d64101b7306272c45f").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x018bba1c61eedababe3d7c81ba1ee000b514c4400dd9a23bd3440765da3bfb09").unwrap(),
        );
    }
}
