use super::poseidon_params::PoseidonParams;
use crate::merkle_tree::merkle_tree_fp::MerkleTreeHash;
use ff::PrimeField;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Poseidon<S: PrimeField> {
    pub(crate) params: Arc<PoseidonParams<S>>,
}

impl<S: PrimeField> Poseidon<S> {
    pub fn new(params: &Arc<PoseidonParams<S>>) -> Self {
        Poseidon {
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
        for r in 0..self.params.rounds_f_beginning {
            current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
            current_state = self.sbox(&current_state);
            current_state = self.matmul(&current_state, &self.params.mds);
        }
        let p_end = self.params.rounds_f_beginning + self.params.rounds_p;
        current_state = self.add_rc(&current_state, &self.params.opt_round_constants[0]);
        current_state = self.matmul(&current_state, &self.params.m_i);

        for r in self.params.rounds_f_beginning..p_end {
            current_state[0] = self.sbox_p(&current_state[0]);
            if r < p_end - 1 {
                current_state[0].add_assign(
                    &self.params.opt_round_constants[r + 1 - self.params.rounds_f_beginning][0],
                );
            }
            current_state = self.cheap_matmul(&current_state, p_end - r - 1);
        }
        for r in p_end..self.params.rounds {
            current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
            current_state = self.sbox(&current_state);
            current_state = self.matmul(&current_state, &self.params.mds);
        }
        current_state
    }

    pub fn permutation_not_opt(&self, input: &[S]) -> Vec<S> {
        let t = self.params.t;
        assert_eq!(input.len(), t);

        let mut current_state = input.to_owned();

        for r in 0..self.params.rounds_f_beginning {
            current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
            current_state = self.sbox(&current_state);
            current_state = self.matmul(&current_state, &self.params.mds);
        }
        let p_end = self.params.rounds_f_beginning + self.params.rounds_p;
        for r in self.params.rounds_f_beginning..p_end {
            current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
            current_state[0] = self.sbox_p(&current_state[0]);
            current_state = self.matmul(&current_state, &self.params.mds);
        }
        for r in p_end..self.params.rounds {
            current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
            current_state = self.sbox(&current_state);
            current_state = self.matmul(&current_state, &self.params.mds);
        }
        current_state
    }

    fn sbox(&self, input: &[S]) -> Vec<S> {
        input.iter().map(|el| self.sbox_p(el)).collect()
    }

    fn sbox_p(&self, input: &S) -> S {
        let mut input2 = *input;
        input2.square();

        match self.params.d {
            3 => {
                let mut out = input2;
                out.mul_assign(input);
                out
            }
            5 => {
                let mut out = input2;
                out.square();
                out.mul_assign(input);
                out
            }
            7 => {
                let mut out = input2;
                out.square();
                out.mul_assign(&input2);
                out.mul_assign(input);
                out
            }
            _ => {
                panic!()
            }
        }
    }

    fn cheap_matmul(&self, input: &[S], r: usize) -> Vec<S> {
        let v = &self.params.v[r];
        let w_hat = &self.params.w_hat[r];
        let t = self.params.t;

        let mut new_state = vec![S::zero(); t];
        new_state[0] = self.params.mds[0][0];
        new_state[0].mul_assign(&input[0]);
        for i in 1..t {
            let mut tmp = w_hat[i - 1];
            tmp.mul_assign(&input[i]);
            new_state[0].add_assign(&tmp);
        }
        for i in 1..t {
            new_state[i] = input[0];
            new_state[i].mul_assign(&v[i - 1]);
            new_state[i].add_assign(&input[i]);
        }

        new_state
    }

    fn matmul(&self, input: &[S], mat: &[Vec<S>]) -> Vec<S> {
        let t = mat.len();
        debug_assert!(t == input.len());
        let mut out = vec![S::zero(); t];
        for row in 0..t {
            for (col, inp) in input.iter().enumerate().take(t) {
                let mut tmp = mat[row][col];
                tmp.mul_assign(inp);
                out[row].add_assign(&tmp);
            }
        }
        out
    }

    fn add_rc(&self, input: &[S], rc: &[S]) -> Vec<S> {
        input
            .iter()
            .zip(rc.iter())
            .map(|(a, b)| {
                let mut r = *a;
                r.add_assign(b);
                r
            })
            .collect()
    }
}

impl<F: PrimeField> MerkleTreeHash<F> for Poseidon<F> {
    fn compress(&self, input: &[&F; 2]) -> F {
        self.permutation(&[input[0].to_owned(), input[1].to_owned(), F::zero()])[0]
    }
}

#[cfg(test)]
mod poseidon_tests_bls12 {
    use super::*;
    use crate::fields::{bls12::FpBLS12, utils};
    use crate::poseidon::poseidon_instance_bls12::POSEIDON_BLS_PARAMS;
    use ff::{from_hex, Field};

    type Scalar = FpBLS12;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let poseidon = Poseidon::new(&POSEIDON_BLS_PARAMS);
        let t = poseidon.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = poseidon.permutation(&input1);
            let perm2 = poseidon.permutation(&input1);
            let perm3 = poseidon.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    #[test]
    fn kats() {
        let poseidon = Poseidon::new(&POSEIDON_BLS_PARAMS);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = poseidon.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x22a1f1595d99e4a04fc0a5b16be51a844a7cb5b5d69627ebbd1ee8142e7532ce").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x5f7ae6d6c380c90510de9c045ee75163eae24054ba8cd88d254cd1c343f43176").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x1b7e4da7d1ac6accb2e0470a83ba87d7bb585f4ba8c9a34f936faf3b3dfc695b").unwrap()
        );
    }
    #[test]
    fn opt_equals_not_opt() {
        let poseidon = Poseidon::new(&POSEIDON_BLS_PARAMS);
        let t = poseidon.params.t;
        for _ in 0..TESTRUNS {
            let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let perm1 = poseidon.permutation(&input);
            let perm2 = poseidon.permutation_not_opt(&input);
            assert_eq!(perm1, perm2);
        }
    }
}

#[cfg(test)]
mod poseidon_tests_bn256 {
    use super::*;
    use crate::fields::{bn256::FpBN256, utils};
    use crate::poseidon::poseidon_instance_bn256::POSEIDON_BN_PARAMS;
    use ff::{from_hex, Field};

    type Scalar = FpBN256;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let poseidon = Poseidon::new(&POSEIDON_BN_PARAMS);
        let t = poseidon.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = poseidon.permutation(&input1);
            let perm2 = poseidon.permutation(&input1);
            let perm3 = poseidon.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    #[test]
    fn kats() {
        let poseidon = Poseidon::new(&POSEIDON_BN_PARAMS);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = poseidon.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x2e72c60509a284872f62830b58ed8524a58c362dd3ddb98b2767f36b566596bd").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x180a812301272545f79ae1012b0425162a1833ac39101e070732f4d8a8bc4718").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x1828343d70eed99aae404e3ea58209f45743f3d54983fe250ce1526a9d8cf88e").unwrap()
        );
    }
    #[test]
    fn opt_equals_not_opt() {
        let poseidon = Poseidon::new(&POSEIDON_BN_PARAMS);
        let t = poseidon.params.t;
        for _ in 0..TESTRUNS {
            let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let perm1 = poseidon.permutation(&input);
            let perm2 = poseidon.permutation_not_opt(&input);
            assert_eq!(perm1, perm2);
        }
    }
}

#[cfg(test)]
mod poseidon_tests_st {
    use super::*;
    use crate::fields::{st::FpST, utils};
    use crate::poseidon::poseidon_instance_st::POSEIDON_ST_PARAMS;
    use ff::{from_hex, Field};

    type Scalar = FpST;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let poseidon = Poseidon::new(&POSEIDON_ST_PARAMS);
        let t = poseidon.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = poseidon.permutation(&input1);
            let perm2 = poseidon.permutation(&input1);
            let perm3 = poseidon.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    #[test]
    fn kats() {
        let poseidon = Poseidon::new(&POSEIDON_ST_PARAMS);
        let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let perm = poseidon.permutation(&input);
        assert_eq!(
            perm[0],
            from_hex("0x9c36f0505876a92c77012b9f33834dd0e74e3a50953d88eb59b0b5c2a43c52").unwrap()
        );
        assert_eq!(
            perm[1],
            from_hex("0x9a62957268edaa73a2d71fdb2b1a92481605e417bad3688e17e87c7aa41458").unwrap()
        );
        assert_eq!(
            perm[2],
            from_hex("0x1398d4f53d2356f56a35127ada7061e9a6c1bdadadc4497439a839b251b622").unwrap()
        );
    }
    #[test]
    fn opt_equals_not_opt() {
        let poseidon = Poseidon::new(&POSEIDON_ST_PARAMS);
        let t = poseidon.params.t;
        for _ in 0..TESTRUNS {
            let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let perm1 = poseidon.permutation(&input);
            let perm2 = poseidon.permutation_not_opt(&input);
            assert_eq!(perm1, perm2);
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod poseidon_tests_goldilocks {
    use super::*;
    use crate::fields::{f64::F64, utils};
    use crate::poseidon::poseidon_instance_goldilocks::POSEIDON_GOLDILOCKS_12_PARAMS;
    use ff::{from_hex, Field};
    use std::convert::TryFrom;

    type Scalar = F64;

    static TESTRUNS: usize = 5;

    #[test]
    fn consistent_perm() {
        let poseidon = Poseidon::new(&POSEIDON_GOLDILOCKS_12_PARAMS);
        let t = poseidon.params.t;
        for _ in 0..TESTRUNS {
            let input1: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let mut input2: Vec<Scalar>;
            loop {
                input2 = (0..t).map(|_| utils::random_scalar(true)).collect();
                if input1 != input2 {
                    break;
                }
            }

            let perm1 = poseidon.permutation(&input1);
            let perm2 = poseidon.permutation(&input1);
            let perm3 = poseidon.permutation(&input2);
            assert_eq!(perm1, perm2);
            assert_ne!(perm1, perm3);
        }
    }

    #[test]
    fn kats() {
        let poseidon = Poseidon::new(&POSEIDON_GOLDILOCKS_12_PARAMS);
        // let input: Vec<Scalar> = vec![Scalar::zero(), Scalar::one(), utils::from_u64::<Scalar>(2)];
        let mut input: Vec<Scalar> = vec![];
        for i in 0..poseidon.params.t {
            input.push(utils::from_u64::<Scalar>(u64::try_from(i).unwrap()));
        }
        let perm = poseidon.permutation(&input);
        assert_eq!(perm[0], from_hex("0xe9ad770762f48ef5").unwrap());
        assert_eq!(perm[1], from_hex("0xc12796961ddc7859").unwrap());
        assert_eq!(perm[2], from_hex("0xa61b71de9595e016").unwrap());
        assert_eq!(perm[3], from_hex("0xead9e6aa583aafa3").unwrap());
        assert_eq!(perm[4], from_hex("0x93e297beff76e95b").unwrap());
        assert_eq!(perm[5], from_hex("0x53abd3c5c2a0e924").unwrap());
        assert_eq!(perm[6], from_hex("0xf3bc50e655c74f51").unwrap());
        assert_eq!(perm[7], from_hex("0x246cac41b9a45d84").unwrap());
        assert_eq!(perm[8], from_hex("0xcc7f9314b2341f4f").unwrap());
        assert_eq!(perm[9], from_hex("0xf5f071587c83415c").unwrap());
        assert_eq!(perm[10], from_hex("0x09486cf35116fba3").unwrap());
        assert_eq!(perm[11], from_hex("0x9d82aaf136b5c38a").unwrap());
    }

    #[test]
    fn opt_equals_not_opt() {
        let poseidon = Poseidon::new(&POSEIDON_GOLDILOCKS_12_PARAMS);
        let t = poseidon.params.t;
        for _ in 0..TESTRUNS {
            let input: Vec<Scalar> = (0..t).map(|_| utils::random_scalar(true)).collect();

            let perm1 = poseidon.permutation(&input);
            let perm2 = poseidon.permutation_not_opt(&input);
            assert_eq!(perm1, perm2);
        }
    }
}
