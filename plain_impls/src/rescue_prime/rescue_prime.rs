use super::rescue_prime_params::RescuePrimeParams;
use crate::merkle_tree::merkle_tree_fp::MerkleTreeHash;
use ff::PrimeField;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct RescuePrime<S: PrimeField> {
    pub(crate) params: Arc<RescuePrimeParams<S>>,
}

impl<S: PrimeField> RescuePrime<S> {
    pub fn new(params: &Arc<RescuePrimeParams<S>>) -> Self {
        RescuePrime {
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

        for r in 0..self.params.rounds {
            current_state = self.sbox(&current_state);
            current_state = self.affine(&current_state, 2 * r);
            current_state = self.sbox_inverse(&current_state);
            current_state = self.affine(&current_state, 2 * r + 1);
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
                    7 => {
                        let mut out = el2;
                        out.square();
                        out.mul_assign(&el2);
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
                    el.pow(&self.params.d_inv[..(S::NUM_BITS as usize + 63) / 64])
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

impl<F: PrimeField> MerkleTreeHash<F> for RescuePrime<F> {
    fn compress(&self, input: &[&F; 2]) -> F {
        self.permutation(&[input[0].to_owned(), input[1].to_owned(), F::zero()])[0]
    }
}

#[cfg(test)]
mod rescue_prime_tests_bls12 {
    use super::*;
    use crate::fields::{bls12::FpBLS12, utils};
    use crate::rescue_prime::rescue_prime_instance_bls12::RESCUE_PRIME_BLS_PARAMS;
    use ff::{from_hex, Field};

    type Scalar = FpBLS12;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let rescue_prime = RescuePrime::new(&RESCUE_PRIME_BLS_PARAMS);
        let t = rescue_prime.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = rescue_prime.permutation(&input1);
            let perm2 = rescue_prime.permutation(&input1);
            let perm3 = rescue_prime.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    #[test]
    fn kats() {
        let rescue_prime = RescuePrime::new(&RESCUE_PRIME_BLS_PARAMS);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = rescue_prime.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x2e1183b4ae571061ed9514118392ede2904ae1376d61653de09083cf0b31abce").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x38f9e521c67c329a53403dd42999b19c3bfe355e594752c87ada74da35c74b85").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x69a193e3c2734c26d85d191a1e521c1bc8024c9047bb5c79835ed5cfc2d8440e").unwrap(),
        );
    }
}

#[cfg(test)]
mod rescue_prime_tests_bn256 {
    use super::*;
    use crate::fields::{bn256::FpBN256, utils};
    use crate::rescue_prime::rescue_prime_instance_bn256::RESCUE_PRIME_BN_PARAMS;
    use ff::{from_hex, Field};

    type Scalar = FpBN256;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let rescue_prime = RescuePrime::new(&RESCUE_PRIME_BN_PARAMS);
        let t = rescue_prime.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = rescue_prime.permutation(&input1);
            let perm2 = rescue_prime.permutation(&input1);
            let perm3 = rescue_prime.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    #[test]
    fn kats() {
        let rescue_prime = RescuePrime::new(&RESCUE_PRIME_BN_PARAMS);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = rescue_prime.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x0dc30ccd5d64e5bea071e99087ef86d433eb156aa0500a823298f9bb05328bd2").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x189893368d5815608c56e44cc67f7e821e093bb6254a0553f9ff69f4d99debc8").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x1acafc768221448ebc51fa2cd1e3c9b2044a0c04f3509d833b0a82c7e3462610").unwrap(),
        );
    }
}

#[cfg(test)]
mod rescue_prime_tests_st {
    use super::*;
    use crate::fields::{st::FpST, utils};
    use crate::rescue_prime::rescue_prime_instance_st::RESCUE_PRIME_ST_PARAMS;
    use ff::{from_hex, Field};

    type Scalar = FpST;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let rescue_prime = RescuePrime::new(&RESCUE_PRIME_ST_PARAMS);
        let t = rescue_prime.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = rescue_prime.permutation(&input1);
            let perm2 = rescue_prime.permutation(&input1);
            let perm3 = rescue_prime.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    #[test]
    fn kats() {
        let rescue_prime = RescuePrime::new(&RESCUE_PRIME_ST_PARAMS);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = rescue_prime.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x00b20c5d0d501d2b597b193e377c29d9fbc9185ee63c85055ec47b60f18fde12").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x0067e87512fd2301814777307dbec5aa04bfecc351d9635c76729bc961d81a6d").unwrap(),
        );
        assert_eq!(
            perm[2],
            from_hex("0x03764f69a9a92eca9b3f96043cfaa5855a0e38d61fef60a46308ef93d62954e2").unwrap(),
        );
    }
}

#[cfg(test)]
mod rescue_prime_tests_goldilocks {
    use super::*;
    use crate::fields::{f64::F64, utils};
    use crate::rescue_prime::rescue_prime_instance_goldilocks::*;
    use ff::from_hex;

    type Scalar = F64;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let rescue_prime = RescuePrime::new(&RESCUE_PRIME_GOLDILOCKS_8_PARAMS);
        let t = rescue_prime.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = rescue_prime.permutation(&input1);
            let perm2 = rescue_prime.permutation(&input1);
            let perm3 = rescue_prime.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    #[test]
    fn kats() {
        let rescue_prime = RescuePrime::new(&RESCUE_PRIME_GOLDILOCKS_8_PARAMS);
        let t = rescue_prime.params.t;
        let input: Vec<Scalar> = (0..t).map(|i| utils::from_u64(i as u64)).collect();
        let perm = rescue_prime.permutation(&input);
        assert_eq!(perm[0], from_hex("0x78611c23bb3f3511").unwrap());
        assert_eq!(perm[1], from_hex("0x747ca7c6adfb6053").unwrap());
        assert_eq!(perm[2], from_hex("0x72bab842bedc7f2b").unwrap());
        assert_eq!(perm[3], from_hex("0xff382886d0643ff1").unwrap());
        assert_eq!(perm[4], from_hex("0x53364e0ade11b65c").unwrap());
        assert_eq!(perm[5], from_hex("0xdd7d94314e8b2d24").unwrap());
        assert_eq!(perm[6], from_hex("0x70f59074a73ebd6f").unwrap());
        assert_eq!(perm[7], from_hex("0x115d7141e8c75cdd").unwrap());
    }
}
